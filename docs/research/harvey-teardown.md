# Harvey (ethanplusai/harvey): technical teardown

| | |
|---|---|
| Repo | https://github.com/ethanplusai/harvey (open-source sales agent; not Harvey AI the legal company) |
| Commit inspected | `e114e95471b1bf155454fca108f0730b66a47a01` (merge of PR #12, committed 2026-10-15) |
| Date inspected | 2026-10-18 |
| Package version | `0.3.0` (`pyproject.toml:7`) |
| License | MIT, "Copyright (c) 2026 Ethan Rogers" (`LICENSE`). Code, prompts (`prompts/*.md`) and skills (`skills/*.md`) are all covered; no separate license on them. Copying or adapting is allowed, including commercially, **as long as you keep the copyright and permission notice in any copy or substantial portion**. If we copy any prompt or skill text, keep the notice with it. Vendored fonts in `harvey/web/fonts/` likely have their own licenses (not checked). |
| Activity (gh api, 2026-10-18) | 1,221 stars, 221 forks, 14 watchers, 5 contributors. Created 2026-03-12, last push 2026-10-15. 11 open issues+PRs (2 issues, 9 PRs). Most of the work landed 2026-08-28 to 2026-10-15. |
| Method | Read only. Nothing was installed, built or run. Line numbers refer to the commit above. |

## Summary

- **What it is.** A single-process Python 3.11 asyncio daemon (`harvey run`) for B2B cold-email outreach. It finds small businesses, scrapes their websites, writes email sequences with an LLM, puts them in an approval queue, sends them through Gmail, Microsoft 365 or SMTP, and classifies and drafts replies.
- **How it calls a model.** It shells out to the `claude -p` CLI on the user's Claude subscription, or to a locked-down `codex app-server` on a ChatGPT plan. There is no API key and no agent framework. The model only ever returns text or JSON. It never gets tools.
- **The "agents" are Python classes.** Scout, Writer, Sender, Handler and Analyst each make zero to two prompt calls. The loop decides what to do from row counts, not from a model (`harvey/main.py:76-109`).
- **Most of the code is deterministic.** Discovery, website profiling, email-pattern finding, a pre-send content gate, a kill switch on bounce rate, and stop-on-reply all run without a model. This is the strongest part of the design.
- **State.** One SQLite file in WAL mode with versioned migrations (`harvey/state.py:52-478`). It has an `outbox` table with an approval status ladder, and an "observations" fact table governed by a trigger.
- **Who it's for.** A solo founder or small agency doing local or SMB outbound at about 50 emails a day from a secondary domain.
- **Maturity.** Beta and moving fast. It has 412 test functions and CI on 3 operating systems (`.github/workflows/tests.yml`). About a third of the tests cover the Codex provider. The core loop, Writer and Scout are barely tested, and there are real logic bugs (below).
- **Security history.** Until v0.2.1 (2026-10-13), every Claude call ran with `--dangerously-skip-permissions` on prompts that contain scraped pages and inbound email (commit `fb9c4a8`; `CHANGELOG.md:83-95`). It is fixed now, but it shows the threat model was added late.

## Architecture

### Control loop and task lifecycle

`harvey/main.py:158-352` runs one cycle like this:

```
 ┌──────────────────────────────────────────────────────────────────────┐
 │ quiet hours? ──yes──> sleep in 300s polls (main.py:206-224)          │
 │ brain.preflight() (Codex only) ──fail──> sleep interval (230-238)    │
 │ brain.is_within_budget() ──no──> sleep 1h (242-252)                  │
 │ summary = state.get_state_summary(); action = decide_next_action()   │
 │ PHASE 1 (asyncio.gather, 600s cap; main.py:112-143, 261-301):        │
 │   handle_replies | scout+analyst | writer | analyst                  │
 │   + read_replies (native mail) + profile(limit 25) when unprofiled   │
 │ PHASE 2 (main.py:303-319): sender.run only if every reply reader     │
 │   returned True and brain not paused (stop-on-reply ordering)        │
 │ log_action; sleep heartbeat_interval (default 15 min)                │
 └──── exception → exponential backoff 60s…900s (main.py:338-350) ──────┘
```

- **Priority order** (`main.py:97-106`): open conversations, then draft campaigns, then new prospects, then prospect, then idle.
- **Sequence lifecycle** (native mail):
  1. Writer creates a `draft` campaign and marks its prospects `queued`. With a native mail provider it also writes a separate first email for each prospect (`harvey/agents/writer.py:93-110, 127-169`).
  2. Sender stages one outbox row per (prospect, step), with `send_at` set by the cumulative delay, and sets the campaign to `active` (`harvey/agents/sender.py:351-405`).
  3. Each cycle, Sender drains rows that are due and `approved` (`sender.py:423-584`).
- **Outbox statuses:** `pending_review → approved → sent`, or `rejected / cancelled / failed` (`state.py:252-254`).
- **Conversation stages** (`initial_outreach → … → closing`) move forward on intent and keywords (`harvey/agents/handler.py:549-589`). Nothing moves them to `closed_won`.

### Roles and agents

| Agent | Model calls | Code |
|---|---|---|
| Scout | One batch scoring call (`think_json`). It also scrapes search engines (DDG, Bing, Google) and Hunter, and optionally drives LinkedIn through Playwright | `agents/scout.py:120-163, 1105-1180` |
| Writer | One 3-step sequence per batch, plus one email per prospect in native mode (batch cap 8) | `agents/writer.py:24-27, 171-305` |
| Sender | None | `agents/sender.py` |
| Handler | Classifies intent, then drafts a reply | `agents/handler.py:503-547, 591-672` |
| Analyst | None. Writes `data/analytics.json` | `agents/analyst.py:13-40` |
| Collectors (discover/profile) | None. Only used by the CLI, the dashboard and the per-cycle profile step | `collectors/*.py`, `pipeline.py:59-128` |

Skills are plain Markdown files, joined together and added to the prompt per agent (`brain.py:666-699`). In practice **only Writer and Handler use them**. Scout loads its skills (`scout.py:124`) and never adds them to its prompt (`scout.py:1127-1147`). The `sender` and `linkedin` entries in the skill map are never loaded.

### Model invocation

- **Claude** (`brain.py:246-370`). Runs `claude -p` with `--output-format json --tools "" --strict-mcp-config` and an optional `--model`. Prompts longer than 8,000 characters go through stdin. Timeout is 300 s. It retries twice with backoff of 5 s, then 10 s, but skips retries for auth errors. On failure it returns `""` and callers treat that as "no result".
  - It sets no `cwd` and no flags to keep out the user's or the project's `CLAUDE.md`, settings or hooks. The repo ships a 285-line `CLAUDE.md` setup-wizard persona. Whether that file gets loaded into Harvey's calls when run from the repo is an open question.
- **Codex** (`harvey/codex.py`, 1,561 lines). For each call it:
  - pins the CLI to exactly version 0.159.3 (`codex.py:95`);
  - uses a private temporary `HOME`/`CODEX_HOME` with an allowlist of environment variables (`codex.py:516-528`);
  - copies the login without the refresh token (`codex.py:359-393`);
  - checks the effective config against a required/forbidden list (`codex.py:145-180, 530-578`);
  - starts the thread read-only with approval policy `never`;
  - kills the runtime on any server-initiated request, such as a tool call (`codex.py:1168-1183`).
- **JSON handling.** `think_json` adds "Respond ONLY with valid JSON", strips code fences, and falls back to slicing from the outermost `{` or `[` (`brain.py:534-575`). Callers coerce the shape defensively (`writer.py:307-355`).
- **No conversation state with the model.** Every call is one-shot. The handler passes the last 6 thread messages inline (`handler.py:596-599`).

### Tool execution

Not present by design: the model gets no tools. All side effects (HTTP, SMTP, Gmail/Graph, DB) are made by Python code that the loop calls. A test guards the Claude no-tools flags (`tests/test_brain_no_tools.py`).

### State and memory

- **Storage.** SQLite at `data/harvey.db`, WAL mode, `synchronous=NORMAL`, 30 s busy timeout. Migrations are tracked in `PRAGMA user_version` (`state.py:445-478`).
- **Tables:** companies, prospects, campaigns, conversations (thread stored as JSON), outbox, settings (a key-value store for the kill switch and counters), signal_codes, observations, runs, usage_events, email_patterns, processed_replies.
- **Observations.** Facts are stored as rows, and a trigger rejects any signal code that isn't in the vocabulary (`state.py:300-346`).
- **Long-term memory** is just these tables plus the skill files. There is no vector store or summarisation.
- **Unique indexes** guard against duplicates:
  - `uq_outbox_campaign_step` blocks double-staging the same step (`state.py:280-282`, insert via `INSERT OR IGNORE` at `state.py:837`);
  - prospect email, LinkedIn URL and company domain are unique (`state.py:176-180`).

### Scheduling, heartbeat, retries and recovery

- **Heartbeat.** Fixed interval. Quiet hours are checked again every 300 s, because a sleep/wake cycle on the host machine once stalled the process (`main.py:35-39, 206-224`).
- **Hang protection.** Each phase of a cycle is capped at 600 s (`main.py:33, 124-135`). Codex has a 1-hour pause after usage limits and re-checks the provider every 30 minutes (`brain.py:52-58`).
- **Recovery comes from re-runnable queries, not checkpoints.** Profiling picks up whatever is stale (`pipeline.py:7-11`), Instantly deploys resume from a stored campaign ID (`sender.py:136-158`), and a stuck `runs` row is swept up (`state.py:1332`).
- **Gaps:**
  - No `sending` or claim status. A crash between `provider.send_email` and the `sent` update (`sender.py:528-544`) leaves the row `approved`, so it gets sent again (at-least-once delivery).
  - No single-instance lock for `harvey run`. Only the dashboard writes a PID file (`dashboard.py:101-113`, `cli.py:127-131`).
  - A reply that hits a non-brain exception is still marked processed and dropped (`handler.py:410-415`).
  - Gmail reads only the 50 most recent inbox messages from the last 7 days, with no pagination (`integrations/gmail.py:175-178`).

### Approval and outbound controls

These apply to the native providers (gmail, microsoft, smtp; `mail_provider.py:24`):

- **Approval.** Rows start as `pending_review` when `require_approval` is on (`sender.py:357-360`, `writer.py:135-138`, `handler.py:473-488`). Approval happens one row at a time or all at once (`state.py:904-920`).
- **Kill switch.** `settings.sending_paused` is checked once at the start of each drain (`sender.py:321-327`). The handler sets it when bounces exceed `max_bounce_rate` after at least 10 sends (`handler.py:456-467`).
- **Pre-send gate** (`harvey/gate.py:49-107`), no model involved. It checks:
  - the recipient matches the database record and the address status is `verified` (or `risky` if opted in);
  - no leftover merge tags;
  - none of 30 banned phrases;
  - subject ≤ 90 characters and body ≤ 220 words;
  - at most 1 link and no HTML.
- **Limits.** At most 8 sends per cycle and the daily cap, with 4-15 s random gaps between sends (`sender.py:29-32, 432-436, 577-580`).
- **Stop-on-reply.** Pending rows are cancelled before intent is classified (`handler.py:213-222`), and Sender re-checks prospect status (`sender.py:481-487`).
- **Hard keyword overrides** for opt-out and escalation run before the LLM classifier (`handler.py:44-78, 226-232`).
- **Optional Precleared check.** A third-party legal check for each new recipient (`sender.py:504-526`).
- **The Instantly path bypasses all of this.** It has no approval, no gate, no kill-switch check and no Precleared check: it creates campaigns, adds leads and activates them directly (`sender.py:88-303`). Handler replies go out immediately through `instantly.send_reply` (`handler.py:345-364`). The `EmailChannelConfig.provider` default is `"instantly"` (`config.py:76`); only the shipped `harvey.yaml` overrides it to gmail.

### Logging, usage tracking and debugging

- **Logs.** Python `logging` to stdout. When the dashboard starts the loop, output goes to `data/harvey.log`, and the dashboard reads the last 64 KB (`dashboard.py:441-467, 514-530`).
- **Audit trail.** An `actions` table (`state.log_action`).
- **Usage.** `usage_events` gets one row per call with tokens, cache tokens, cost, duration and agent/task attribution, parsed from Claude's result JSON `modelUsage` (`brain.py:478-532`). Codex rows record cost as 0 (`brain.py:444-459`).
- **Quota gate.**
  - Claude: reads Claude Code's OAuth access token from `~/.claude/.credentials.json` or the macOS Keychain (including `security dump-keychain`) and calls the undocumented `api.anthropic.com/api/oauth/usage` endpoint (`integrations/quota.py:1-117`).
  - If quota can't be read, it falls back to `200 × percent` calls per day (`main.py:200`).
- **Docker health check.** Only checks that the DB file was modified recently (`Dockerfile:42-43`).

### Configuration and extensibility

- **Files.** `harvey.yaml` (pydantic models, `config.py:27-268`). `harvey.local.yaml` takes precedence; the search order also includes `cwd.parent/harvey.yaml` (`config.py:415-437`). Secrets live in plain-text `.env`, including the LinkedIn and SMTP passwords (`config.py:270-297`). The Gmail refresh token is stored at `data/gmail_token.json` with mode 0600 (`gmail.py:58-65`).
- **Extension points:**
  - edit `skills/*.md` or `prompts/*.md`; both are re-read on every agent run (`brain.py:633-664`);
  - add a `MailProvider` subclass (`mail_provider.py:63-104`);
  - add a discovery provider or signal code (`signals.py`, `collectors/discover.py`).
- **Not applied:** `harvey.yaml` itself is read once at startup (`main.py:168`).
- **Dead or unused config and prompts:**
  - `product.offer.*` (booking_url, goal, etc.) is never read by any agent;
  - `linkedin.max_daily_*` is only written by setup;
  - `prompts/system.md` and `prompts/scout.md` are never loaded (`load_prompt` is only called with `writer`/`handler`: `writer.py:199,253`, `handler.py:609`).

### Dashboard

- **Stack.** FastAPI JSON API plus static HTML/JS/CSS with no build step (`harvey/dashboard.py`, `harvey/web/`). Binds to `127.0.0.1:5555` by default; `--host` can change that (`cli.py:701-705`).
- **No authentication, CSRF token or Origin check.** Endpoints that change state include:
  - `/api/outbox/approve-all` and `/api/sending/resume`, which also resets the bounce count (`dashboard.py:718-766`);
  - `/api/harvey/start`, which spawns the loop;
  - `/api/discover/run` (paid);
  - `/api/settings/env`, which rewrites `.env` keys including SMTP host and password (`dashboard.py:300-327`).
- **Escaping.** The UI escapes HTML with `escHtml` (83 call sites in `web/app.js`). Not audited exhaustively.

### Tests

- 412 `def test` functions in 25 files, run with pytest and pytest-asyncio. CI matrix is Ubuntu, macOS and Windows on Python 3.11 and 3.12. A real Codex CLI is tested against localhost mocks (`.github/workflows/tests.yml`).
- **Well covered:** Codex isolation (`test_codex*.py`, ~104 tests), brain provider (37), discovery (37), Microsoft (28), state (36), outbox, gate and bounces (`test_outbox_native.py`, 13).
- **Not present:** tests for `decide_next_action` or the heartbeat cycle, the Writer, Scout's search and scoring, handler intent and reply generation, the Instantly path, and the dashboard endpoints that change state (only the signals routes are tested).

## README vs code discrepancies

| README claim | Code reality |
|---|---|
| Priority ladder ends in "prospect > idle (analyze)" (README:97) and the sample log shows "Decision: prospect (only 3 new prospects…)" (README:16) | `idle` is unreachable. If `new_prospects > 0` the action is `write_campaign`; otherwise `new_prospects` is 0, so `< 20` is always true (`main.py:101-106`). Three new prospects would produce `write_campaign`, not `prospect`. |
| Loop finds, writes, sends and handles on its own | Once **any** conversation is `open`, every cycle picks `handle_replies`, and Scout and Writer never run again from the heartbeat (`main.py:97-98, 264-265`). Conversations only close on unsubscribe or not_interested (`handler.py:282, 309`). Interested, objection, question, wrong_person and OOO threads stay open, and nothing else closes them (no other `update_conversation(status=…)` callers). |
| "By default every outgoing email waits for your approval" (README:310) | True only for native providers. The Instantly path sends without approval or gate (`sender.py:88-303`, `handler.py:345-364`), and the code default provider is `instantly` (`config.py:76`). |
| `harvey sending pause` "stops everything mid-flight" (README:317) | The flag is read once per drain (`sender.py:321`) and not at all on the Instantly path. A drain already running (up to 8 sends) finishes. |
| Skill matrix: Scout, Sender and LinkedIn get skills (README:274-288) | Only Writer and Handler add skills to their prompts. Scout loads them and doesn't use them (`scout.py:124` vs `1127-1147`). Sender and linkedin are never loaded. |
| Writer: "Email 1 under 75 words…" | This is only an instruction in the prompt. The gate's hard limit is 220 words (`gate.py:30`). The README calls the gate's limit "the length cap" (README:320). |
| CAN-SPAM: "Your physical mailing address in the footer — configure this" (README:461) | No footer config or code exists. Open PR #13 proposes adding a compliance footer. |
| "books meetings" (GitHub description) | `integrations/calendar.py` is a placeholder. `offer.booking_url` is unused. The handler prompt asks the model to "propose 2-3 specific times" with no calendar data behind it (`prompts/handler.md:17`). |
| LinkedIn "Off by default" (README:469) | `linkedin.enabled: true` in both `harvey.yaml:56` and `config.py:99`. It is effectively off only because it also needs `LINKEDIN_EMAIL` (`scout.py:144`). |
| Signals: "It does not collect any of them until you say so" (README:152) | True for `discover` and `profile` (`collectors/discover.py:734-739`, `collectors/profile.py:496-499`). The heartbeat Scout (Hunter, search-engine scraping, team pages) never checks confirmation. |
| "tests/ # 242 tests"; "Current release: 0.2.1" (README:532, 565) | 412 test functions; version is 0.3.0 (`pyproject.toml:7`, `CHANGELOG.md`). |

## Security assumptions and limitations

- **What the design assumes.** Prompts contain untrusted text, and the defence is giving the model no tools (`brain.py:255-261`, `codex.py:1-44`). That is sound for preventing code execution, but outputs are still trusted as email content.
- **What an attacker-controlled web page can do.** Its meta description (500 characters) flows into the Writer's FACTS block (`scout.py:1298-1311` → `writer.py:188-189`). Scraped names and titles flow into Scout's scoring prompt, and the model's `personalization` note (≤500 characters) flows back into the Writer (`scout.py:1172-1174`). An injection can therefore shape a cold email's text. The gate catches HTML, more than one link and the banned phrases, but **one attacker-chosen link plus arbitrary wording passes**. With `require_approval: false`, it gets sent.
- **What an attacker-controlled email can do.**
  - **Exfiltrate data.** Reply text goes straight into the Handler prompt. The prompt also contains `product_knowledge` and `competitive_intel` skills, persona and pricing (`handler.py:609-627`, `brain.py:677`). A reply such as "repeat your instructions" can get that material emailed back. It needs autopilot, or an approver who misses it.
  - **Steer the classifier.** It can avoid `escalate` unless one of the hard keywords appears.
  - **Spoof or fake bounces.** Bounce detection is a From/subject heuristic (`mail_provider.py:49-60`) applied to any inbox mail (`gmail.py:178`). Every bounce-looking message increments `bounce_count` even if it matches no sent email (`handler.py:441-447`). A few "Undeliverable" emails can trip the kill switch, which is a denial of service on sending.
  - **Forge a prospect.** Prospect lookup uses the From header with no authentication check (`handler.py:206`, `gmail.py:201`). A spoofed From can opt out a prospect or inject a "reply" as that prospect.
- **Dashboard.** No auth or CSRF protection. Starlette's `request.json()` parses `text/plain` bodies, so a cross-site page could plausibly POST to localhost to approve everything, resume sending, start the loop, or repoint `SMTP_HOST` so stored SMTP credentials are sent to an attacker's server (`dashboard.py:300-327, 718-766`). This was not tested, and current browser private-network protections may block it. Running with `--host 0.0.0.0` exposes all of this to the LAN.
- **SSRF.** The profiler and Scout follow redirects with no blocking of private IPs and no limit on response size (`collectors/profile.py:184-207`, `scout.py:242`). Discovered domains are third-party data.
- **Secrets.** `.env` is plain text. LinkedIn passwords are stored for Playwright login. Harvey also reads the Claude OAuth token from the Keychain for the quota call (`quota.py:50-117`), which widens the blast radius.
- **Sandboxing.**
  - Codex: strong, attested isolation.
  - Claude: tools and MCP are off, but there is no isolation of HOME, settings or hooks.
  - Docker: runs as a non-root user, but mounts `~/.claude` read-write (`docker-compose.yml:15`).
- **Compliance and terms of service.** It scrapes Google and Bing with rotated user agents (`scout.py:70-81, 445-447`), automates LinkedIn, and does SMTP RCPT probing for small mail servers (`email_finder.py:427-443`). These are legal and ToS risks the operator carries.

## Lessons for a Rust implementation

**Retain**
- **Choose the next action in code, from counts.** It saves model budget and removes a fragile parsing step (`main.py:76-109`). Fix the precedence bugs before copying it.
- **Give the model no tools; keep side effects in typed code.** This is the only thing that makes untrusted input survivable (`brain.py:255-261`, commit `fb9c4a8`).
- **Use an outbox table with a status ladder and a unique (campaign, prospect, step) index.** Approval, auditing and dedup all hang off it (`state.py:250-284`).
- **Run a deterministic pre-send gate and hard keyword overrides.** Prompts can be ignored; code can't (`gate.py`, `handler.py:44-78`).
- **Phase the cycle: read replies → stop-on-reply → send, and send nothing if reading failed.** It prevents follow-ups going to people who already answered (`main.py:303-319`).
- **Make every external effect re-runnable through queries instead of hand-offs.** Recovery comes for free (`pipeline.py:7-16`).
- **Fail closed on provider problems.** Pause, don't fall back, and leave inputs untouched (`brain.py:122-175`, `handler.py:183-187`).
- **Keep a per-call usage ledger with agent/task attribution** (`brain.py:478-532`).

**Redesign**
- **Send claims.** Add a `sending` status with a lease or idempotency key before the provider call, plus a single-instance lock. As written, delivery is at-least-once (`sender.py:528-544`).
- **The kill switch** should be checked before every send and on every outbound path. Make it a typed enum, not an empty or non-empty string in a key-value table (`sender.py:321`).
- **Approval must be enforced in one place** that every provider goes through. Here, Instantly bypasses it (`sender.py:88-303`).
- **Conversation lifecycle** needs explicit terminal and dormant states with timeouts so open threads don't starve prospecting (`main.py:97`).
- **Bounce detection** should parse DSN (`multipart/report`), match on the original Message-ID, and never count unmatched messages (`handler.py:423-467`).
- **Mark failed replies for retry, not as processed** (`handler.py:410-415`).
- **Structured output.** Use schema-validated JSON (serde with deny_unknown_fields) in place of fence-stripping and brace-slicing (`brain.py:556-575`).
- **Untrusted text in prompts** should be wrapped in clear delimiters, kept separate from confidential skills in the reply path, and checked on output for links and domains that aren't on an allowlist.
- **Dashboard.** Require an auth token, check Origin, require a JSON content type, and never edit secrets over HTTP.
- **Config.** Pick safe defaults in code, not only in the sample YAML (`config.py:76, 99`). Fail on keys that are unknown or unused.

**Avoid**
- **Skipping permissions on an agent CLI that receives untrusted text.** Harvey shipped this for seven months (`CHANGELOG.md:87-95`).
- **Reading another product's OAuth token from the Keychain and calling an undocumented endpoint** (`quota.py:25-117`). It is fragile and expands what Harvey can access.
- **Pinning a 1,500-line attestation layer to one exact CLI version** (`codex.py:95`). It is a large ongoing maintenance cost for one provider. Prefer a direct API, or a provider adapter with a small surface.
- **Skill and config maps that silently do nothing** (`brain.py:674-680`, `product.offer`). Every config knob should have a code path and a test.
- **Scraping search engines with spoofed user agents, and LinkedIn automation** (`scout.py:70-81`, `integrations/linkedin.py`). This is ToS and account risk with no deterministic value.

## Open questions

1. Does `claude -p` launched from the repo directory load the repo's `CLAUDE.md` (setup-wizard persona) and the user's hooks or settings into every Harvey call? The code doesn't prevent it (`brain.py:249-299`), but the CLI behaviour wasn't verified.
2. Is the dashboard CSRF practical in current browsers given private-network-access rules? Not tested.
3. Does `GmailProvider.get_replies` change message labels, given that it holds the `gmail.modify` scope (`gmail.py:41`)? Only the query was reviewed.
4. Licenses of the vendored fonts (`harvey/web/fonts/*.woff2`) weren't checked.
5. Was the open-conversation starvation seen in the field? No issue reports it. The open issues are #9 (Antigravity provider) and #20 (an off-topic feature request).

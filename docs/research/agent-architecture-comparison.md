# Agent Architecture Comparison and Rust Ecosystem Review

Accessed: 2026-10-18 (all sources). Scope: comparable agent/automation systems, single- vs multi-agent guidance, and the Rust crate ecosystem, to inform a Rust-first, narrow business-workflow agent with human approval of consequential actions. `ethanplusai/harvey` is out of scope here (it is covered separately).

Confidence key: **H** = verified from primary source (repo, registry API, official docs) today. **M** = official source but indirect or partial. **L** = secondary or inferred. Bracketed numbers like [S12] refer to the Sources list at the end.

---

## 1. Summary of conclusions

1. **Use one agent with tools, inside a deterministic workflow.** Anthropic [S1], OpenAI [S4] and Cognition [S3] all say to start with the simplest design: workflows or a single agent loop. Anthropic's multi-agent research system beat a single agent by 90.2% on its own eval, but it used about 15x the tokens of chat and is described as a poor fit when agents must share context [S2]. Customer-inquiry handling shares context and has few parallel parts. **H**
2. **Approval must be enforced by the platform, not by the prompt.** The mature systems put approval in the runtime: n8n's per-tool human review [S8], OpenAI Agents SDK `needs_approval` with durable `RunState` [S16], OpenHands confirmation policies [S13], and Claude Agent SDK's hooks, rules and `canUseTool` [S15]. Zapier Agents' in-agent approval is an instruction in the prompt ("ask for my confirmation ... before continuing") [S10]. That is the anti-pattern to avoid. **H**
3. **Durable execution is now standard, and resume semantics differ in ways that matter.** LangGraph re-runs the whole interrupted node on resume, so side effects before `interrupt()` must be idempotent [S11]. Temporal and Restate journal each step and replay completed ones from the journal [S17, S19]. In every model, effects must be idempotent or keyed, and the approval wait must survive restarts. **H**
4. **Postgres-only durability is enough for v1.** Temporal's Rust SDK went GA in September 2026 [S18] (crate 1.0.0 on 2026-09-04, 1.1.0 on 2026-10-06) [S30], but it needs a Temporal cluster. Restate's server is BSL-1.1 and its Rust SDK is still 0.x [S20, S30]. For one product on one database, an explicit state machine in Postgres, driven by a `SKIP LOCKED` job queue [S27], is the lowest-risk option. apalis 1.0.0 (2026-10-12) or graphile_worker are possible off-the-shelf queues. **H/M**
5. **Rig is active but very unstable.** Its recent releases carry 17 to 48 entries marked breaking each (0.40 to 0.44, roughly one minor release every 1 to 6 weeks). 0.44 also changed `RUN_FORMAT`, so "runs persisted by 0.43 are refused on resume" [S21]. Never persist Rig's types as the durable record. **H**
6. **Anthropic ships no official Rust SDK.** Official client SDKs cover Python, TypeScript, C#, Go, Java, PHP and Ruby [S22]. The Claude Agent SDK exists only for Python and TypeScript [S23]. Some crates.io crates (`anthropic_rust`, `claude-agent-sdk`) list `anthropics/...` repository URLs that return 404 and are owned by individuals [S24], so treat them as unofficial. **H**
7. **Recommendation: build a thin provider layer of our own** (reqwest + serde) for one or two providers, behind our own `ModelClient` trait. Use `rmcp` (official MCP Rust SDK, 3.5.1) only if MCP becomes a requirement. Rig is worth reading as a reference, and adopting it should be revisited when it reaches 1.0. **M** (judgement)
8. **License flags for reuse or embedding:** n8n (Sustainable Use License), Dify (modified Apache), Inngest server (SSPL with a future-Apache clause) and Restate server (BSL) are all source-available with restrictions. OpenHands, LangGraph, Temporal, Activepieces core and both agent SDKs are MIT. **H**

---

## 2. Part A: Comparable systems

### 2.1 Comparison table

| System | Model | Execution | State / durability | HITL / approval | License | Lesson |
|---|---|---|---|---|---|---|
| OpenHands (SDK v1.55.1; app v1.27.1) | Agent SDK + app for software agents | Single agent loop over typed action/observation events | Append-only event log, "event sourcing with indexing", auto-save and resume [S12, S14] | `AlwaysConfirm` / `NeverConfirm` / `ConfirmRisky` policies plus an LLM risk analyzer; status `WAITING_FOR_CONFIRMATION` [S13] | MIT | Event log as source of truth; separate policy from risk classification |
| n8n (2.43.7) | Low-code workflow graph; AI Agent node = LangChain Tools Agent | Node graph, triggers/webhooks; queue mode with Redis + workers [S6] | Executions stored in DB; Wait node resumes "with the same data" [S7] | Per-tool human review via Slack, Teams, Gmail etc.; deny means "AI is informed of the rejection" [S8] | Sustainable Use License (source-available) [S5] | Per-tool approval gating and deny-as-tool-result is the right UX |
| Zapier Agents / MCP | SaaS agent plus 9,000+ app actions over MCP | Hosted; dynamic tool discovery | Opaque (SaaS) | Agent approval is **prompt-instructed**; Zap "Human in the Loop" step is a workflow step [S10] | Proprietary | Integrations breadth is a moat we can't match; prompt-based approval is insufficient |
| Temporal (Rust SDK 1.1.0) | Durable execution platform | Deterministic workflow code; LLM/tool calls as activities [S17] | Event-history replay; requires Temporal Service | Signals/Updates; cookbook "human-in-the-loop" recipe with Signals [S17] | MIT | Gold standard semantics, but heavy ops for a small product |
| LangGraph (1.2.16) | Graph of nodes over shared state | Super-step graph execution | Checkpointer per `thread_id`; Postgres/SQLite savers [S11b] | `interrupt()` + `Command(resume=…)`; node **re-executes from start** [S11] | MIT (platform commercial) | Checkpoint per step; design pre-approval code to be idempotent |
| Claude Agent SDK (py 0.2.168) / OpenAI Agents SDK (py v0.24.0) | Agent-loop libraries | Single agent loop, handoffs/subagents | Agents SDK `RunState.to_json()/from_json()` durable [S16]; Claude SDK sessions | Claude: hooks → deny → ask → mode → allow → `canUseTool` [S15]; OpenAI: `needs_approval`, interruptions, approve/reject [S16] | MIT | Layered, fail-closed permission evaluation; authenticate approvals server-side |
| Restate (server v1.8.0; Rust SDK 0.12.1) | Durable async/await runtime | Handlers with journaled `ctx.run()` steps | Journal; completed steps not re-run [S19] | Awakeables/durable promises (Rust SDK `awakeable`, `promise`) [S20b] | Server BSL-1.1; SDK MIT | Journaled side effects are the model to copy, even in-house |
| Activepieces (0.92.3) | Low-code flows + AI agent, MCP | Flow graph, TypeScript "pieces" | Paused run "keeps its place for 30 days" (vendor) [S26] | "Wait for Approval" tools / approval pieces [S26] | MIT core + EE dir | Same pattern as n8n with a permissive core |

### 2.2 Per-system notes

**OpenHands** (`OpenHands/OpenHands`, 91k stars; SDK repo `OpenHands/software-agent-sdk`, MIT, v1.55.1)
- *Architecture.* Four packages: core SDK, tools, workspace (Docker/remote) and an optional FastAPI agent server. "Same agent code works in both modes—just swap the workspace type." [S12] **H**
- *Users.* Developers who want autonomous coding agents. **H**
- *State and resume.* "Events form an append-only log". The conversation "owns and persists event log". The persistence service does "debounced writes, incremental events" [S14]. **M** (the mechanics are in `state.py` and were not read)
- *Approval.* The confirmation policy is kept separate from the security analyzer. "The analyzer does not prevent execution — it classifies risk for the policy layer to act on." UNKNOWN risk is confirmed by default. The documented gap: "`conversation.execute_tool()` bypasses the analyzer and confirmation policy." [S13] **H**
- *LLM layer.* Depends on `litellm>=1.93.0` (pyproject) [S12b]. **H**
- *Strength.* Sandboxed workspaces and an event model that can be replayed.
- *Limitation.* Built for coding, Python only, and the app is heavy.
- **Lesson.** Every model action and observation becomes an immutable event. Approval policy is a separate component from risk scoring. Approval must also cover any direct tool-execution path, so there can be no bypass API.

**n8n** (`n8n-io/n8n`, 208k stars, n8n@2.43.7 on 2026-10-16)
- *Architecture.* A node graph. The AI Agent node "lets you build an AI agent... Connect a chat model and one or more tools" and now always runs as a Tools Agent (agent types deprecated from 1.82.0) [S9]. **H**
- *Execution.* In queue mode, the main instance handles triggers and webhooks and puts execution IDs on Redis. Workers load the workflow from the database, execute it and write results back [S6]. **H**
- *HITL.* Review can be set per tool or for all tools, over Slack, Teams, Gmail, WhatsApp and other channels. The reviewer sees `$tool.name`/`$tool.parameters`. "If denied, the action is canceled and the AI is informed of the rejection." [S8] **H**
- *License.* Sustainable Use License: "You may use or modify the software only for your own internal business purposes or for non-commercial or personal use" [S5]. The FAQ allows "behind the scenes in your own product" use but forbids letting "external end users... build or configure their own workflows through your product, whether via a custom UI, our API, MCP, or an AI agent acting on the user's behalf" and forbids forking "to launch your own automation product" [S5b]. Files marked `.ee.` need an enterprise license. **H**
- *Limitation.* Workflows are not code-reviewable Rust, Node.js runtime, and the license constrains embedding.
- **Lesson.** Copy the UX: approval per tool, sent through the channel the business owner already uses, and a denial fed back to the model as a tool result. Do not embed n8n.

**Zapier (Agents, MCP; AI Actions)**
- *Integration.* Zapier MCP covers "9,000+ apps and 40,000+ actions". Each successful tool call costs two tasks. Only Streamable HTTP is supported, not SSE [S10b]. **H** (vendor docs)
- *Approval.* The Agents help page (updated 2026-10-01) has users add a sentence to the instructions, "ask for my confirmation through [messaging app] before continuing", and notes that agents "carry out instructions in the order they are presented". A Zap-level Human in the Loop step is the alternative [S10]. **H**
- *AI Actions status.* Not verified today.
- **Lesson.** Approval that lives in instructions depends on the model complying. Our approval gate must sit in code between "model proposed action" and "effect executed". Zapier MCP could also serve as an integration back end later, priced per call.

**Temporal** (server MIT, 23.8k stars; Rust SDK `temporalio-sdk` 1.1.0)
- *Execution.* Workflows are deterministic code. LLM calls and tool I/O run as activities ("every LLM call and every I/O tool call runs as a durable activity", from the ADK recipe). The cookbook includes a durable agentic loop with Claude tool calling, an OpenAI Agents SDK integration, and a human-in-the-loop approval recipe that uses Signals [S17]. **M**
- *Rust status.* Public preview was announced 2026-05-07 [S18b]. The September 2026 digest (2026-09-30) says "the Rust SDK and Standalone Activities are Generally Available" [S18]. crates.io: 1.0.0 published 2026-09-04, 1.1.0 published 2026-10-06, about 2.87M downloads. The repo has 538 stars and 97 open issues. The crate's license metadata is "non-standard", while the repo LICENSE is MIT [S30]. **H**
- *Limitation.* Production needs a Temporal Service or Temporal Cloud, workflow determinism constraints apply, and the Rust SDK is newly GA.
- **Lesson.** Model each run as a workflow, with every non-deterministic or effectful call as a journaled activity and approvals as external signals. We can get these semantics in Postgres without running Temporal.

**LangGraph** (MIT, 43k stars, 1.2.16)
- *State.* A checkpointer persists "a thread's graph state as checkpoints". Production should use `PostgresSaver`/`AsyncPostgresSaver`. Checkpoints accumulate, so plan pruning [S11b]. **H**
- *HITL.* `interrupt()` surfaces a JSON value and waits indefinitely. The caller resumes with `Command(resume=…)` on the same `thread_id`. Critically, "the runtime restarts the entire node from the beginning". The docs advise idempotent upserts before the interrupt and putting side effects after it or in a separate node [S11]. **H**
- **Lesson.** Approval should be its own step boundary. Nothing effectful should happen in the same step before the pause.

**Claude Agent SDK / OpenAI Agents SDK** (both MIT, Python and TypeScript only)
- *Claude.* Permission evaluation order is hooks → deny rules → ask rules → permission mode → allow rules → `canUseTool`. "Auto-approved tools never reach `canUseTool`." For checks that must always run, use a `PreToolUse` hook [S15]. The SDK runs on the Claude Code runtime, so it is not embeddable in Rust [S23]. **H**
- *OpenAI.* Tools declare `needs_approval` (bool or callable, failing closed). Paused runs expose `interruptions`. `state.approve/reject` (optionally `always_approve`). "`RunState` is designed to be durable" via `to_json`. Security caveat: `from_json` does "not authenticate the snapshot or the person submitting it", so "validate decisions against server-stored pending items" [S16]. **H**
- **Lesson.** Approval should be a declarative property of each tool (static or argument-dependent) and should fail closed. The approval record lives on the server and is authenticated, and decisions refer to a specific pending call ID.

**Restate** (server BSL-1.1, v1.8.0; Rust SDK MIT, 0.12.1)
- Positioned as "the foundation for AI workflows, agents, and agentic platforms". Wrap LLM calls in `ctx.run()` so that completed steps aren't repeated. Approvals "pause the agent and resume when the response arrives, even across restarts" [S19]. The Rust SDK exposes `run`, `awakeable` and `promise` [S20b]. Server license: BSL with an Additional Use Grant that excludes a "Public Restate Platform Service" [S20]. **H**
- **Lesson.** The cleanest model to copy is a journaled step with idempotent replay plus an external completion token for the approval.

**Activepieces** (MIT outside `packages/ee`, 25k stars, 0.92.3)
- TypeScript "pieces", MCP server exposure, and "Wait for Approval" tools. Vendor says a paused run "keeps its place for 30 days" [S26]. **M/L**
- **Lesson.** Paused approvals need an explicit expiry policy. Ours: escalate, then auto-expire to "not sent".

**Others, briefly.** Dify uses a modified Apache 2.0 license that bars multi-tenant use without written authorization and removal of the console logo [S25]. Inngest's server license is "Server Side Public License... Apache 2.0 Future License" [S25b]. Neither is a good fit for embedding.

### 2.3 Single-agent vs multi-agent guidance

- **Anthropic, "Building effective agents" (2024-12-19)** [S1]. Distinguishes workflows ("predefined code paths") from agents (LLMs "dynamically direct their own processes"). Recommends "finding the simplest solution possible" and warns that frameworks add "extra layers of abstraction that can obscure the underlying prompts". Patterns: prompt chaining, routing, parallelization, orchestrator-workers, evaluator-optimizer. Agents suit "open-ended problems where it's difficult or impossible to predict the required number of steps". Invest in the agent-computer interface: tool docs, poka-yoke parameters. **H**
- **Anthropic, multi-agent research system (2025-06-13)** [S2]. 90.2% better than single-agent Opus 4 on its internal research eval, but multi-agent "use[s] about 15× more tokens than chats". Domains "that require all agents to share the same context" are "not a good fit... today". Production lessons: resume rather than restart, "regular checkpoints", and rainbow deploys so in-flight agents keep their code version. **H**
- **Cognition, "Don't Build Multi-Agents" (Walden Yan, 2025-06-12)** [S3]. Principles: "Share context, and share full agent traces" and "Actions carry implicit decisions, and conflicting decisions carry bad results". Prefer a single-threaded linear agent and compress history for long tasks. **H**
- **OpenAI, "A practical guide to building agents" (2025)** [S4]. Secondary summaries agree it advises starting with a single agent and moving to multi-agent (manager or decentralized handoffs) only when complexity demands it. The PDF text could not be extracted today, so exact wording is **unverified (L)**.
- **Synthesis for us.** Customer-inquiry handling is a routing + prompt-chaining workflow with one bounded tool-using agent step. Multi-agent is not justified.

---

## 3. Part B: Rust ecosystem (crates.io API, GitHub API, 2026-10-18)

### 3.1 Crate table

| Crate | Version | Last release | License | Downloads (all / recent 90d) | Maturity signals | Notes |
|---|---|---|---|---|---|---|
| rig-core / rig (0xPlaygrounds) | 0.44.0 | 2026-10-07 | MIT | 3.62M / 1.93M | 8.9k stars, 92 open issues; 63 versions; README: "future updates **will** contain **breaking changes**" | 20+ providers incl. anthropic, openai, gemini, bedrock, ollama, openrouter; tools, structured output, streaming, OTel GenAI semconv, sans-IO `AgentRun` state machine, `rig-rmcp` MCP (native only); 0.44 adds typed `cache`/`reasoning` options |
| rmcp (official MCP SDK) | 3.5.1 | 2026-10-05 | Apache-2.0 (MIT→Apache transition) | 37.0M / 20.6M | 4.0k stars, 73 open issues; ~weekly releases | De facto MCP standard in Rust |
| async-openai | 0.42.2 | 2026-10-10 | MIT | 9.46M / 2.97M | 2.0k stars, 26 open issues; 124 versions | OpenAI-shaped only; 0.x minors break |
| genai (jeremychone) | 0.6.5 (0.7.0-rc.8 2026-10-16) | 2026-10-16 | MIT OR Apache-2.0 | 492k / 225k | 910 stars; single maintainer | 27+ native-protocol providers; Anthropic `with_cache_control` on tools, signed thinking preserved across tool turns |
| misanthropic | 1.0.0-alpha.24 | 2026-10-14 | MIT | 20k | Small, alpha | Community Anthropic client |
| anthropic-ai-sdk | 0.2.27 | 2026-01-11 | MIT | 47k | Self-described unofficial | Messages, streaming, Files API |
| anthropic_rust / claude-agent-sdk | 0.1.3 / 0.1.1 | 2025-09 | MIT | <5k | **Repo URLs point at nonexistent `anthropics/*` repos**; owned by individuals | Avoid |
| swarms-rs | 0.3.0 | 2026-09-29 | Apache-2.0 | 5.7k | 8 versions; low use | Not production-relevant |
| langchain-rust | 4.6.0 | 2024-10-06 | MIT | 159k | No crates.io release in 2 years | Stale on crates.io |
| autoagents | 0.4.0 | 2026-07-08 | MIT/Apache | 18k | Small | Niche |
| restate-sdk | 0.12.1 | 2026-09-22 | MIT | 1.07M / 738k | 88 stars; 0.x; server BSL | `run`, `awakeable`, `promise` |
| temporalio-sdk | 1.1.0 | 2026-10-06 | MIT (repo); "non-standard" metadata | 2.87M / 2.60M | GA per Temporal 2026-09-30; 97 open issues | Requires Temporal Service |
| apalis (+ apalis-postgres, apalis-workflow) | 1.0.0 | 2026-10-12 | MIT OR Apache-2.0 | 1.32M / 388k | 1.4k stars, 2 open issues; 1.0 after a long RC series | Tower middleware; apalis-workflow claims "durable and resumable" DAG workflows (vendor claim) |
| graphile_worker (rs) | 0.14.1 | 2026-10-11 | MIT | 78k | Active, single maintainer | Port of Node graphile-worker; Postgres |
| underway | 0.2.0 | 2025-07-16 | MIT/Apache | 24k | 176 stars; repo pushed 2026-10 but no release in 15 months | "Durable background workflows on Postgres" |
| sqlxmq | 0.6.0 | 2025-05-25 | MIT/Apache | 198k | Low activity | Postgres queue on sqlx |
| fang | 0.11.0 | 2026-07-02 | MIT | 111k | Low activity | Postgres/MySQL/SQLite |
| tokio | 1.53.3 | 2026-10-13 | MIT | 1.08B | Foundation | — |
| axum | 0.8.9 | 2026-04-14 | MIT | 526M | tokio-rs; 0.8 line since 2025-01 | — |
| sqlx | 0.9.0 | 2026-05-21 | MIT/Apache | 167M | Async, compile-time-checked SQL | — |
| sea-orm | 2.0.4 | 2026-09-27 | MIT/Apache | 26.9M | 2.0 GA 2026-07-19 | Async ORM on sqlx |
| diesel / diesel-async | 2.3.14 / 0.9.2 | 2026-10-07 / 2026-06-19 | MIT/Apache | 40.7M / 15.2M | Mature; async via separate crate | — |
| tracing / tracing-opentelemetry / opentelemetry | 0.1.44 / 0.34.0 / 0.33.1 | 2025-12-18 / 2026-09-23 / 2026-10-09 | MIT / MIT / Apache-2.0 | 942M / 225M / 295M | OTel crates still 0.x and break on minors | Pin versions together |
| reqwest | 0.13.5 | 2026-09-08 | MIT/Apache | 810M | Foundation | — |

### 3.2 Rig in detail
- **Cadence and breakage (H).** Minor releases: 0.39 (2026-06-19), 0.40 (07-11), 0.41 (07-28), 0.42 (08-17), 0.43 (09-30), 0.44 (10-07). Entries marked `[**breaking**]` per release in CHANGELOG: 31, 17, 32, 48, 40 for 0.40 to 0.44. In 0.44 alone, `CompletionRequest`/`Usage` lost struct literals, the error types merged, Anthropic's `with_automatic_caching` was replaced by `cache(..)`, Ollama/Cohere default routes changed, and `RUN_FORMAT` became 2, refusing runs persisted by 0.43 [S21].
- **Capabilities (H).** Provider modules for anthropic, openai, gemini, azure, bedrock, vertex, mistral, deepseek, groq, ollama, openrouter, xai and more. Typed `GenerationOptions` (reasoning, cache, service tier...) that fail with `UnsupportedOption` under `OnUnsupported::Error`. Provider-specific typed extensions for 25 providers. Native structured output on Anthropic. Streaming tool-argument deltas. OTel GenAI semantic conventions. Record/replay "cassettes" for tests. A sans-IO `AgentRun` whose `ExecutingTools` state "keeps a serialized run self-contained" for resume [S21].
- **Assessment.** Rig has caught up on provider feature lag (caching, reasoning, citations). The costs are churn and a large surface: many crates and an ECS-based harness. Its persisted-run format is tied to the Rig version, which rules it out as our durable state.

### 3.3 Recommendation: thin custom provider layer (primary), Rig as reference/fallback

| Factor | Rig | Thin custom layer (reqwest + serde, 1-2 providers) |
|---|---|---|
| Time to first call | Minutes | 1-3 days per provider (Messages + streaming + tool use) |
| Provider-specific features (prompt caching, extended thinking, citations, structured output) | Available now, but behind a unifying abstraction; mapping errors surface as `UnsupportedOption` | Exact wire fidelity; new features usable the day they ship |
| Breaking changes | 17-48 per minor release, every few weeks | Only when the provider changes its API (versioned, rare) |
| Abstraction leakage | Cross-provider message model and history adaptation (`history::adapt`) | None, but provider switching costs more |
| Durable state | Must not persist Rig types (format refused across versions) | We own the schema in Postgres |
| Testing | Cassettes built in | We write recorded-fixture tests (cheap with serde) |
| License | MIT | N/A |

**Decision rationale.** A narrow product needs one primary model provider plus perhaps one fallback, so cross-provider breadth has little value. Correctness of the persisted conversation and the approval record has high value. A provider trait of about 300-800 lines (`complete`, `stream`, typed tool-use blocks, usage and cache counters) is small enough to own. Revisit Rig when it reaches 1.0 or when more than two providers are needed. `genai` is the lighter alternative if multi-provider is needed sooner. Use `rmcp` (the official SDK) for any MCP client or server work. **M** (judgement on verified facts)

---

## 4. Implications for our Rust architecture

1. **Loop design: a workflow around one agent step.** Run each case as a deterministic pipeline: ingest → classify/route (LLM, structured output) → draft/plan (single tool-using agent loop, bounded `max_turns`) → policy gate → approval → execute effect → record outcome. Read tools (CRM lookup, order status) run freely. Write tools (send email or SMS, refund, book) are declared `requires_approval` with an optional argument predicate, as in OpenAI's `needs_approval`, and fail closed [S16]. A denial goes back to the model as a tool result, as in n8n [S8].
2. **Durability: Postgres state machine plus journal.** Tables: `runs` (status enum: `running | awaiting_approval | executing | done | failed | expired`), an append-only `run_events` log of model requests and responses, tool calls and results, and approvals (OpenHands-style event log [S14]), and a `jobs` table claimed with `FOR UPDATE SKIP LOCKED` [S27]. Each step is a job that reads state, does one thing, appends events and enqueues the next step in one transaction. Persist each LLM response before acting on it, as in Restate/Temporal journaling [S17, S19], so a crash never re-calls the model for a completed step or re-sends an effect. Version the event schema ourselves. Keep apalis 1.0 / graphile_worker as drop-in queue options. Move to Temporal (Rust GA) only if multi-service orchestration appears.
3. **Approval model: a first-class record enforced in the platform.** When the model proposes a write action, insert a `pending_action` (tool, canonical arguments, hash, proposed-by run/turn, expiry) and move the run to `awaiting_approval`. The effect executor accepts only an `approved` pending action whose argument hash matches. There is no `execute_tool` bypass (the OpenHands caveat [S13]). Authenticate the approver and validate the decision against the server-stored pending item (the OpenAI caveat [S16]). Allow edit-then-approve, which stores the edited arguments as a new hash. Set an expiry with escalation (Activepieces' 30-day pause shows the need). Never rely on prompt-instructed approval (Zapier [S10]).
4. **Idempotent effects.** Each external effect carries an idempotency key derived from `pending_action.id`. Record `effect_started` before the call and `effect_completed` after it. On recovery, an effect with `started` but not `completed` is reconciled, not blindly retried. This mirrors LangGraph's warning [S11] and Rig's own harness rule that a mutating tool call "starts only once the reply that asked for it, and a mark that it started, are on disk" [S21b].
5. **Persistence stack.** tokio 1.53 + axum 0.8.9 + sqlx 0.9 (compile-time-checked SQL, no ORM indirection for the event and journal tables). Postgres only, with no Redis.
6. **Provider layer.** A `ModelClient` trait with an Anthropic Messages implementation first (prompt caching on system prompt and tools, extended thinking as an opt-in), plus an optional OpenAI implementation. Keep raw request and response JSON in `run_events` for replay and evals. Use `tracing` + `tracing-opentelemetry` with GenAI semconv attribute names.
7. **Observability.** One trace per run and one span per step, LLM call and tool call. Costs and token/cache usage go on events. Monitor decision patterns (approval rate, edit rate, deny reasons) more than transcript contents, following Anthropic's privacy-conscious monitoring [S2].
8. **Licensing posture.** Depend only on MIT/Apache crates. Do not embed n8n, Dify, Inngest or Restate server code. Learning from their designs is fine.

---

## 5. Sources

| # | Title | Publisher | Date | URL | Type | Conf. |
|---|---|---|---|---|---|---|
| S1 | Building effective agents | Anthropic | 2024-12-19 | https://www.anthropic.com/engineering/building-effective-agents | Official guidance | H |
| S2 | How we built our multi-agent research system | Anthropic | 2025-06-13 | https://www.anthropic.com/engineering/multi-agent-research-system | Official engineering blog (vendor data) | H |
| S3 | Don't Build Multi-Agents (Walden Yan) | Cognition | 2025-06-12 | https://cognition.com/blog/dont-build-multi-agents | Vendor engineering opinion | H |
| S4 | A practical guide to building agents | OpenAI | 2025 (exact date unverified) | https://cdn.openai.com/business-guides-and-resources/a-practical-guide-to-building-agents.pdf | Official guide; text not extracted; secondary summaries used | L |
| S5 | n8n LICENSE.md (Sustainable Use License 1.0) | n8n GmbH | repo HEAD 2026-10 | https://github.com/n8n-io/n8n/blob/master/LICENSE.md | Source/license | H |
| S5b | License FAQ | n8n docs | accessed 2026-10-18 | https://docs.n8n.io/n8n-community-license/community-license/license-faq.md | Official docs | H |
| S6 | Enable queue mode | n8n docs | accessed | https://docs.n8n.io/deploy/host-n8n/configure-n8n/scaling/enable-queue-mode.md | Official docs | H |
| S7 | Wait | n8n docs | accessed | https://docs.n8n.io/build/flow-logic/wait.md | Official docs | H |
| S8 | Human-in-the-loop for tools | n8n docs | accessed | https://docs.n8n.io/advanced-ai/human-in-the-loop-tools/ | Official docs | H |
| S9 | AI Agent node | n8n docs | accessed | https://docs.n8n.io/integrations/builtin/cluster-nodes/root-nodes/n8n-nodes-langchain.agent.md | Official docs | H |
| S10 | Add approval steps to your agent's instructions | Zapier Help | updated 2026-10-01 | https://help.zapier.com/hc/en-us/articles/41776074420493 | Official docs | H |
| S10b | Zapier MCP | Zapier docs | accessed | https://docs.zapier.com/mcp/home | Official docs (vendor claims on scale) | M |
| S11 | Interrupts | LangChain/LangGraph docs | accessed | https://docs.langchain.com/oss/python/langgraph/interrupts | Official docs | H |
| S11b | Persistence | LangChain/LangGraph docs | accessed | https://docs.langchain.com/oss/python/langgraph/persistence | Official docs | H |
| S12 | SDK architecture overview | OpenHands docs | accessed | https://docs.openhands.dev/sdk/arch/overview | Official docs | H |
| S12b | openhands-sdk pyproject.toml (v1.55.1) | OpenHands | repo HEAD | https://github.com/OpenHands/software-agent-sdk | Source | H |
| S13 | Security / confirmation mode | OpenHands docs | accessed | https://docs.openhands.dev/sdk/guides/security | Official docs | H |
| S14 | Events; Conversation architecture | OpenHands docs | accessed | https://docs.openhands.dev/sdk/arch/events ; https://docs.openhands.dev/sdk/arch/conversation | Official docs | M |
| S15 | Configure permissions (Agent SDK) | Anthropic / Claude Code docs | accessed | https://code.claude.com/docs/en/agent-sdk/permissions | Official docs | H |
| S16 | Human-in-the-loop | OpenAI Agents SDK docs | accessed | https://openai.github.io/openai-agents-python/human_in_the_loop/ | Official docs | H |
| S17 | AI Cookbook | Temporal docs | accessed (undated) | https://docs.temporal.io/ai-cookbook | Official docs | M |
| S18 | Durable Digest: September highlights | Temporal | 2026-09-30 | https://temporal.io/blog/durable-digest-september-2026 | Vendor announcement | H |
| S18b | Rust SDK public preview | Temporal changelog | 2026-05-07 | https://temporal.io/changelog/rust-sdk-public-preview | Vendor announcement (via search) | M |
| S19 | AI agents with Restate | Restate docs | accessed | https://docs.restate.dev/ai | Official docs (vendor positioning) | M |
| S20 | Restate LICENSE (BSL 1.1) | Restate | repo HEAD | https://github.com/restatedev/restate/blob/main/LICENSE | License | H |
| S20b | restate sdk-rust `src/context/mod.rs` | Restate | repo HEAD (0.12.x) | https://github.com/restatedev/sdk-rust | Source code | H |
| S21 | Rig README + CHANGELOG (0.44.0, 2026-10-07) and `crates/rig-agent/src/run/mod.rs` | 0xPlaygrounds | 2026-10-07 | https://github.com/0xPlaygrounds/rig | Source code / changelog | H |
| S21b | Rig README "The rig coding agent" section | 0xPlaygrounds | repo HEAD | https://github.com/0xPlaygrounds/rig#the-rig-coding-agent | Source docs | H |
| S22 | SDKs, CLI, and libraries | Anthropic (Claude Platform docs) | accessed | https://platform.claude.com/docs/en/cli-sdks-libraries/overview | Official docs | H |
| S23 | anthropics GitHub org repo listing (claude-agent-sdk-python/-typescript; no Rust SDK; `anthropic-sdk-rust` 404) | GitHub API | 2026-10-18 | https://github.com/anthropics | Primary (registry) | H |
| S24 | crates.io API: owners of `anthropic_rust` (tmikus), `claude-agent-sdk` (dhuseby) | crates.io | 2026-10-18 | https://crates.io/api/v1/crates/anthropic_rust/owners | Primary (registry) | H |
| S25 | Dify LICENSE | LangGenius | repo HEAD | https://github.com/langgenius/dify/blob/main/LICENSE | License | H |
| S25b | Inngest LICENSE.md | Inngest | repo HEAD | https://github.com/inngest/inngest/blob/main/LICENSE.md | License | H |
| S26 | Todos MCP / AI agent builder pages | Activepieces | ~2026-02 (per search) | https://www.activepieces.com/mcp/todos ; https://www.activepieces.com/product/ai-agent-builder | Vendor pages | L/M |
| S27 | SELECT: The Locking Clause (SKIP LOCKED) | PostgreSQL Global Development Group | current docs | https://www.postgresql.org/docs/current/sql-select.html | Official docs ("can be used to avoid lock contention with multiple consumers accessing a queue-like table") | H |
| S28 | genai README | jeremychone | repo HEAD (0.7.0-rc) | https://github.com/jeremychone/rust-genai | Source docs | H |
| S29 | apalis-workflow README; underway README | apalis-dev; maxcountryman | repo HEAD | https://github.com/apalis-dev/apalis ; https://github.com/maxcountryman/underway | Source docs (maintainer claims) | M |
| S30 | crates.io API `/api/v1/crates/{name}` and `/versions` for all crates in §3.1 | crates.io | 2026-10-18 | https://crates.io/api/v1/crates/rig-core (etc.) | Primary (registry) | H |
| S31 | GitHub API repo metadata (stars, open issues, license, latest release) for all repos cited | GitHub | 2026-10-18 | https://api.github.com/repos/{owner}/{repo} | Primary | H |

Unverified items: exact wording and date of OpenAI's agents guide; the current status of Zapier AI Actions; the internals of OpenHands persistence (`state.py` not read); the 30-day Activepieces pause (vendor page via search).

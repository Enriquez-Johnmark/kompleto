# UI audit and polish, 2026-11-10

An audit of the bookkeeper's pages with the `ui-taste` skill (`reference/audit.md`), then a polish pass (`reference/polish.md`) that kept the existing look. Owner's decision: `docs/decisions.md`, 2026-11-10.

## What was inspected

- **Build:** `main` at `16c0801`, on the demo Firm (`kompleto demo-firm`, served with `DEMO_DATE=2027-01-07`).
- **Browser:** Chromium (Playwright) at 1280 wide (the showcase screenshots) and 390×844 (phone, touch emulated).
- **Pages:**
  - the bookkeeper's pages: dashboard, drafts, clients, a client's five tabs, the 2307 tracker, BIR calendar, activity, settings, import and adding a client;
  - the client's upload page.
- **Overflow:** a script measured horizontal overflow on every page at phone width.
- **Not checked:** a screen reader, zoom above 100% and real phones.

## Findings and what was done

| # | Priority | Finding | Done |
|---|---|---|---|
| 1 | P1 | On a phone the Activity log cut off its "What" column: the bookkeeper couldn't see what happened | One entry per block on a phone (`f51203e`) |
| 2 | P2 | No Activity entry said which client it concerned ("Drafted a message", but for whom?) | Each entry names its client, found from its record within the same Firm (`f51203e`) |
| 3 | P2 | On a phone a client's tab strip was centred, so its first tab ("Deadlines") was cut off and couldn't be scrolled to | The strip starts at the left (`acd5385`) |
| 4 | P2 | At the same Cut-off the dashboard listed clients with everything received before those still owing documents, by name | Clients still owing come first (`6c6c0f9`) |

**Checked and fine:**

- The dashboard, drafts, clients list, 2307 tracker, BIR calendar, settings, import and the client's upload page fit a phone with no overflow.
- Focus rings, empty states and the destructive "Close account" card follow one system.

## Checks after the fixes

- `cargo fmt --check`, clippy and `cargo test --all-features` (391 tests) passed before each commit.
- Before each web commit, `pnpm lint`, `typecheck`, `test` and `build` passed too.
- The Activity log and the client tabs were captured again on a phone with the new build and inspected.
- The showcase was remade from a fresh demo Firm.

# Roadmap

Where Kompleto stands and what comes next, in one page. Dated facts come from `docs/decisions.md`; the milestones from `docs/architecture.md`. Updated 2026-10-25.

## Where we are

**Showcase-ready, waiting on the domain.** Every milestone is built except deployment, and every acceptance criterion has a passing test.

| Milestone | What it delivers | Status |
|---|---|---|
| M1–M5 | The core loop: outbox, Firms and login, clients, BIR calendar, checklist, AI Drafts with template fallback, approval, the upload page, replies and bounces, dashboard, 2307 tracker | Done |
| M6 | Export, account closure, send budget, hardening; real email domain and hosting | Built; **deployment waits for the domain** |
| M7 | The bookkeeper's pages as a React app | Done |
| M8 | Demo Firm, ClamAV scanning, help and legal pages, screenshots, walkthrough video | Done |
| M9–M13 | Staff logins, approved sign-up, Xero/QuickBooks import, 2307 Payor matching, payroll chasing | Done |

The engine inside Kompleto was split out as [Nudgewright](https://github.com/Enriquez-Johnmark/nudgewright) (2026-10-23); Kompleto pins v0.2.0.

## Next

1. **Owner:** the Taglish test of the AI model (the one open item of the spec, needs an API key).
2. **Owner:** buy `getkompleto.com` just before pilots (the first spend; ₱0 until then).
3. **Deploy (M6 close-out):** real email from the domain through the shared Resend account, Cloudflare Tunnel or Oracle Always Free hosting, nightly backups.
4. **Pilots:** free written outreach (≤ 15–20 personalised messages a week), then 3–5 free 4-week pilots, then ₱1,500/month via GCash.

## Housekeeping before both products run live

- Make the daily send budget configurable and set it to **60** (shared Resend account with Balanse; Balanse ADR 0002).
- Move onto Nudgewright's `accounts` module (v0.3.0) and drop Kompleto's own login, staff, sign-up and closing code.
- Upgrade to Nudgewright v0.4.0 (deferred messages stay approved; `Cargo.toml` still says "v0.1.0" in a comment).

## Later products on the same engine

Balanse (in progress), then Kotasyon, Biyahe (name re-checked first), Iskedyul and Rekord, one at a time, each in its own repository.

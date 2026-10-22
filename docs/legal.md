# Help and legal pages

Four public pages, readable without logging in. Their text is Markdown in the React app, rendered by `web/src/lib/markdown.tsx`:

| Page | Text |
|---|---|
| `/help` | `web/src/features/help/help.md` |
| `/privacy` | `web/src/features/legal/privacy-notice.md` |
| `/terms` | `web/src/features/legal/terms.md` |
| `/data-processing` | `web/src/features/legal/data-processing-agreement.md` |

The login page links to help, privacy and terms; the sidebar links to help; the Client's upload page links to the privacy notice in the Client's language.

## Status: drafts for the owner

The three legal documents were drafted by Claude at M8 (showcase round, 2026-10-22) from the research in `docs/research/p1-kill-checks.md` §4 and `docs/research/p1-launch-prerequisites.md` §3, and from what the code actually does. **They have not been reviewed by a lawyer.** Each opens with a "Draft for review, not yet in force" note. Before the first pilot, the owner:

1. replaces every bracketed placeholder: `[owner's full name]`, `[business address]`, `[date it takes effect]`, the hosting choice (Oracle Cloud Singapore or the owner's Mac), and the price in the terms;
2. confirms `help@getkompleto.com` is routed to them (`SUPPORT_EMAIL`, `docs/deploy.md`);
3. removes the draft note from each document;
4. signs a one-line written designation of themself as data protection officer;
5. files the NPC sworn declaration of exemption from registration (notarised), until Kompleto processes sensitive personal information of 1,000 or more people, when it must register instead (`docs/research/p1-launch-prerequisites.md` §3).

## Keep them true

The documents describe behaviour the code enforces. Change them in the same commit when any of these change:

- retention: Uploads 30 days after received and 90 at most (F3), Reply text 90 days (G1), closing deletes after 7 days (I2), backups kept 30 days;
- what the model may see (`docs/adr/0003-the-model-never-sees-documents-or-tax-data.md`);
- the sub-processors (hosting, Resend, Cloudflare, Google Drive, Anthropic): Firms get 15 days' notice of a change;
- virus scanning, encryption and the approval rule.

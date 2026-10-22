# The bookkeeper's pages are a React app served by the Rust binary

Owner, 2026-10-22: the bookkeeper's pages move from server-rendered HTML to React, so Kompleto looks finished when it's shown to bookkeepers. This replaces "server-rendered pages first, React later" (`docs/product/p1-spec.md`, Web technology).

The React app is a Vite single-page app built to static files. The Rust binary serves those files and a JSON API under `/api/`, so there is still **one program to deploy** on ₱0 hosting. Every rule that protects an acceptance criterion stays in Rust: approval binds content (D2), the outbox sends at most once (E1), Firm isolation (A2) and the pre-send checks (D7). The React app only shows data and asks the API to act, so a bug in it can't send an unapproved message or show another Firm's data.

The **Client's upload page** (`/u/{token}`) stays server-rendered and is restyled to match. It must load fast on cheap phones over mobile data, and it is the one page strangers open, with the signed link in its address (F1, F2).

## Considered options

- **Next.js:** rejected. It needs a Node server beside the Rust one, which means two programs on ₱0 hosting, and its server features (server components, server actions) would duplicate what the Rust API already does.
- **Keep server-rendered pages and restyle them:** rejected by the owner, who wants React for the bookkeeper's side.
- **React for the upload page too:** rejected. A few hundred kilobytes of JavaScript slows the page that clients open on prepaid data, for no gain: it has one form and no live state.

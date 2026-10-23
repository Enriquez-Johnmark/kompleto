# Engine and product live in one crate, with a one-way dependency (superseded by ADR 0006)

The reusable engine is built inside Kompleto rather than as a separate library or Cargo workspace. Its parts are proven by a real product before being generalised, and a solo builder maintains one build. `src/engine` must never depend on `src/kompleto`; a CI check enforces this, so the engine can later be extracted for S1 or other products without untangling it.

## Considered options

- **Engine-first library:** rejected. It means guessing what's reusable before any product exists.
- **Workspace from day one:** rejected until a second product or a concrete build need requires it (AGENTS.md: one crate until a requirement demands more).

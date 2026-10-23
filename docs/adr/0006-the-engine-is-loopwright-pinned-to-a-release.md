# The engine is Loopwright, in its own repository, pinned to a release

Supersedes ADR 0001. The engine was built inside Kompleto, behind a one-way boundary, until a second product needed it. On 2026-10-23 the owner decided to build five more products (Balanse, Kotasyon, Biyahe, Iskedyul, Resibo), each in its own public repository (`docs/decisions.md`). So the engine moved, with its git history, to its own public repository, [Loopwright](https://github.com/Enriquez-Johnmark/loopwright), licensed AGPL-3.0-only.

Kompleto depends on a release, named in `Cargo.toml` by its version and the exact commit it was tagged at, and re-exports it as `crate::engine`, so its code keeps the same paths. Naming the commit means a moved tag can never change what Kompleto builds. Kompleto keeps its own migrations, which already create the engine's tables with an identical schema; Loopwright's own migration is for new products. Kompleto keeps its copies of the engine's tests, because they carry its acceptance criteria (E1's crash test only exists here, since it needs Kompleto's program to kill).

## Considered options

- **Keep the engine inside Kompleto:** every other product would have to depend on Kompleto.
- **One repository for the engine and all six products:** can't give each product its own public repository.

## Consequences

- An engine change takes two steps: release a new Loopwright version, then move Kompleto to it.
- The boundary is now enforced by the compiler: Loopwright can't see Kompleto at all, and its CI fails if it names any product.

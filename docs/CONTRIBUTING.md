# Contributing

## Software design principles

These apply to every change, no exceptions.

- **Single responsibility** — every function, module, and file does one thing well.
- **Small, clean functions** — short, focused, readable top-to-bottom.
- **Modularity and cohesion** — composable pieces with clear inputs/outputs; related code lives together.
- **Explicit over implicit** — named parameters, clear return types, obvious control flow.
- **Minimal coupling** — depend on interfaces, not implementations; follow layer boundaries.
- **DRY — but not prematurely** — extract after three genuine repetitions, not two.
- **Fail fast and visibly** — validate at boundaries; surface errors early.
- **Naming is documentation** — if a name needs a comment, rename it.
- **Type safety** — strict mode; validate at system boundaries.
- **Defensive async** — guard against stale state; no fire-and-forget.
- **Test coverage** — every public method has explicit tests.
- **Keep state minimal and local** — prefer derived values over stored duplicates.
- **Delete freely** — dead code is a liability; version control remembers.

## Coding standards

- Format: `cargo fmt -- --check` must pass
- Lint: `cargo clippy -- -D warnings` must pass (zero warnings)
- No `unsafe` blocks
- No `unwrap()` in library code — use `Result` or `expect()` with context
- All public functions and types have doc comments
- Tests: `cargo test` must pass, every public behavior has a test
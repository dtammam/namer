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

{{Fill in project-specific rules: language config, lint rules, test requirements, etc.}}

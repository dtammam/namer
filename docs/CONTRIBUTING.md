# Contributing

## Software design principles

These apply to every change, no exceptions.

- **Single responsibility** — every function, module, and file does one thing well.
- **Small, clean functions** — short, focused, readable top-to-bottom.
- **Modularity and cohesion** — composable pieces with clear inputs/outputs; related code lives together.
- **Explicit over implicit** — named parameters, clear return types, obvious control flow.
- **Minimal coupling** — depend on traits, not concrete types; follow layer boundaries.
- **DRY — but not prematurely** — extract after three genuine repetitions, not two.
- **Fail fast and visibly** — validate at boundaries; surface errors early.
- **Naming is documentation** — if a name needs a comment, rename it.
- **Type safety** — strict mode; validate at system boundaries.
- **Test coverage** — every public method has explicit tests.
- **Keep state minimal and local** — prefer derived values over stored duplicates.
- **Delete freely** — dead code is a liability; version control remembers.

## Coding standards

- Format: `cargo fmt -- --check` must pass
- Lint: `cargo clippy -- -D warnings` must pass (zero warnings)
- Markdown lint: `npx markdownlint-cli2 '**/*.md'` must pass (all markdown files)
- No `unsafe` blocks
- No `unwrap()` in library code — use `Result` or `expect()` with context
- All public functions and types have doc comments
- Tests: `cargo test` must pass, every public behavior has a test
- Setup: run `./setup.sh` after cloning to install git hooks

## Three-pillar code quality framework

Every function and type must be classifiable as semantic, pragmatic, or a model.
If something straddles two categories, refactor until it doesn't.

### Semantic functions

- No boolean parameters that switch behavior — use enums (e.g., `Casing::Upper` not `lowercase: bool`).
- Return types must encode invariants. If a function always returns exactly N items, use a tuple or struct, never a Vec.
- Accept the narrowest type that expresses the domain: prefer domain structs over generic slices.
- Pure functions must not transform data they don't own (e.g., `format_name` should not case the delimiter — only the words).

### Pragmatic functions

- Only `main()` and CLI/IO boundary code should be pragmatic. Everything else should be semantic and pure.
- Pragmatic functions must not leak into test expectations — test semantic functions directly, integration-test pragmatic ones via the binary.

### Models

- Eliminate `Option` fields when a default value exists — use `default_value` at the parsing boundary instead.
- Represent multi-field domain objects as named structs, not tuples or Vecs. Field names are documentation.
- If two values have different domain meanings (adjective vs noun), they must be distinguishable by type or struct field — never two bare Strings in a Vec.

### Testing

- Every test must exercise a distinct code path. Tautological tests (same inputs, same function, different variable names) must be removed.
- Semantic functions get unit tests. Pragmatic/CLI behavior gets integration tests that invoke the binary.
- Edge cases (multi-character delimiters, unicode, empty input) must have explicit tests.

### Documentation

- Only include design principles that apply to this codebase. Remove generic principles (e.g., "defensive async" for a sync CLI).
- `RELIABILITY.md` invariants must be populated — if the type system doesn't enforce it, the docs must state it.
- `ARCHITECTURE.md` data model section must match actual types in code.

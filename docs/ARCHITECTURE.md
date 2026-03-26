# Architecture

The principal-engineer agent owns this file and updates it during Design.

## High-level design

- Clap (derive) for CLI argument parsing
- Rand for random word selection
- Hardcoded word lists as `const` arrays (`ADJECTIVES`, `NOUNS`)
- Single binary, single source file (`src/main.rs`)

## Repo layout

```
src/
  main.rs              — CLI entry point, word lists, name generation, formatting, tests
Cargo.toml             — dependencies: clap (derive), rand
docs/
  ARCHITECTURE.md      — this file
  CONTRIBUTING.md      — design principles and coding standards
  RELIABILITY.md       — performance budgets and invariants
  exec-plans/          — feature execution plans
.github/
  workflows/
    ci.yml             — PR gate: fmt, clippy, test
    release.yml        — Tag-triggered cross-platform release builds
assets/
  logo/                — App icon/logo files (PNG, optional SVG)
.state/                — feature lifecycle state (managed by engineering-manager)
```

## Component relationships

All components live in `src/main.rs`:

```
main()
  |-- Cli::parse()              (clap-derived arg struct)
  |-- generate_name(rng)        (returns NameParts { adjective, noun })
  |-- format_name(parts, casing, delimiter)  (applies casing to words + joins)
  |-- println!                  (output)
```

- `generate_name` depends only on `rand` and the word list constants.
- `format_name` is a pure function with no external dependencies. Only cases the words, not the delimiter.
- `Cli` depends on `clap` derive macros.

## Data model

- **Word lists**: `ADJECTIVES` and `NOUNS` are `&[&str]` const arrays of lowercase English words.
- **`Cli` struct**: Clap-derived struct with fields `lower: bool` and `delimiter: String` (defaults to `""`).
- **`NameParts` struct**: `{ adjective: String, noun: String }` — the output of `generate_name`, unformatted lowercase.
- **`Casing` enum**: `Upper` or `Lower` — controls output casing in `format_name`.

## CI/CD

- **PR gate** (`.github/workflows/ci.yml`): Runs `cargo fmt --check`, `cargo clippy -D warnings`, and `cargo test` on every pull request to `main`. Single job on `ubuntu-latest`.
- **Release** (`.github/workflows/release.yml`): Triggers on `v*` tags. Builds release binaries for 4 targets (Linux x86-64, Windows x86-64, macOS Intel, macOS Apple Silicon) on native runners, then creates a GitHub Release with all binaries attached. Uses only the built-in `GITHUB_TOKEN`.

## Key protocols / APIs

None. CLI only -- stdin/stdout. Exit code 0 on success, non-zero on invalid arguments (clap default).
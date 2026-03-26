# Architecture

The principal-engineer agent owns this file and updates it during Design.

## High-level design

- Clap (derive) for CLI argument parsing
- Rand for random word selection
- Hardcoded word lists as `const` arrays (`ADJECTIVES`, `NOUNS`)
- Single binary, two source files (`src/main.rs` and `src/words.rs`)

## Repo layout

```
src/
  main.rs              — CLI entry point, name generation, formatting, tests
  words.rs             — Word list constants (ADJECTIVES, NOUNS) — data only, no logic
Cargo.toml             — dependencies: clap (derive), rand
docs/
  ARCHITECTURE.md      — this file
  CONTRIBUTING.md      — design principles and coding standards
  RELIABILITY.md       — performance budgets and invariants
  VERSIONING.md        — semantic versioning policy and release checklist
  exec-plans/          — feature execution plans
.github/
  workflows/
    ci.yml             — PR gate: fmt, clippy, test
    release.yml        — Tag-triggered cross-platform release builds (with auto-generated release notes)
assets/
  logo/                — App icon/logo files (PNG, optional SVG)
.state/                — feature lifecycle state (managed by engineering-manager)
```

## Component relationships

```
src/main.rs
  |-- mod words              (imports word list constants from words.rs)
  |-- Cli::parse()           (clap-derived arg struct: --lower, --delimiter, --number)
  |-- for 1..=N:
  |     |-- generate_name(rng)    (returns NameParts: adjective and noun)
  |     |-- format_name(parts, casing, delimiter)  (applies casing + joining)
  |     |-- println!              (output)

src/words.rs
  |-- ADJECTIVES: &[&str]   (777 entries)
  |-- NOUNS: &[&str]         (777 entries)
```

- `words.rs` is a data-only module — no logic, no imports, no dependencies.
- `generate_name` depends only on `rand` and the word list constants from `words.rs`.
- `format_name` is a pure function with no external dependencies.
- `Cli` depends on `clap` derive macros.

## Data model

- **Word lists** (`src/words.rs`): `ADJECTIVES` and `NOUNS` are `pub(crate) &[&str]` const arrays of 777 lowercase English words each. Data only.
- **`Cli` struct**: Clap-derived struct with fields `lower: bool`, `delimiter: String` (default `""`), and `number: u32` (default 1, range 1–1000).
- **Name generation output**: `NameParts` struct with fields `adjective: String` and `noun: String` (unformatted, lowercase).

## CI/CD

- **PR gate** (`.github/workflows/ci.yml`): Runs `cargo fmt --check`, `cargo clippy -D warnings`, and `cargo test` on every pull request to `main`. Single job on `ubuntu-latest`.
- **Release** (`.github/workflows/release.yml`): Triggers on `v*` tags. Builds release binaries for 4 targets (Linux x86-64, Windows x86-64, macOS Intel, macOS Apple Silicon) on native runners, then creates a GitHub Release with all binaries attached and auto-generated release notes (derived from PR titles via `softprops/action-gh-release`). Uses only the built-in `GITHUB_TOKEN`.

## Key protocols / APIs

None. CLI only -- stdin/stdout. Exit code 0 on success, non-zero on invalid arguments (clap default).

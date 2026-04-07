# Architecture

The principal-engineer agent owns this file and updates it during Design.

## High-level design

- Clap (derive) for CLI argument parsing
- Rand for random word selection
- Hardcoded word lists as `const` arrays, organized by category
- `ThingCategory` enum (derives `clap::ValueEnum`) selects the active noun list
- Single binary, source split across `src/main.rs` and `src/words/` module

## Repo layout

```text
src/
  main.rs              — CLI entry point, name generation, formatting, tests
  words/
    mod.rs             — re-exports word list constants from sub-modules
    adjectives.rs      — ADJECTIVES constant — data only, no logic
    objects.rs         — OBJECTS constant — data only, no logic
    produce.rs         — PRODUCE constant — data only, no logic
    animals.rs         — ANIMALS constant — data only, no logic
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

```text
src/main.rs
  |-- mod words              (imports word list constants from words/ module)
  |-- Cli::parse()           (clap-derived arg struct: --lower, --delimiter, --number, --things)
  |-- ThingCategory::nouns() (maps category enum variant to noun slice)
  |-- for 1..=N:
  |     |-- generate_name(rng, nouns)  (returns NameParts: adjective and noun)
  |     |-- format_name(parts, casing, delimiter)  (applies casing + joining)
  |     |-- println!                   (output)

src/words/
  |-- mod.rs                 (re-exports: ADJECTIVES, OBJECTS, PRODUCE, ANIMALS)
  |-- adjectives.rs          (ADJECTIVES: &[&str], ~400 entries)
  |-- objects.rs             (OBJECTS: &[&str], ~240 entries)
  |-- produce.rs             (PRODUCE: &[&str], ~200 entries)
  |-- animals.rs             (ANIMALS: &[&str], ~250 entries)
```

- `words/` is a data-only module — no logic, no imports, no dependencies.
- `generate_name` depends only on `rand` and a noun slice passed by the caller.
- `format_name` is a pure function with no external dependencies.
- `Cli` depends on `clap` derive macros.
- `ThingCategory` derives `clap::ValueEnum` for CLI validation and maps to noun
  lists via its `nouns()` method.

## Data model

- **Word lists** (`src/words/`): `ADJECTIVES`, `OBJECTS`, `PRODUCE`, and
  `ANIMALS` are `pub(crate) &[&str]` const arrays of lowercase English words.
  Data only, one file per list.
- **`ThingCategory` enum**: Variants `Objects` (default), `Produce`, `Animals`.
  Derives `Clone`, `Copy`, `clap::ValueEnum`. Has a `nouns()` method that
  returns the corresponding `&'static [&'static str]` slice.
- **`Cli` struct**: Clap-derived struct with fields `lower: bool`,
  `delimiter: String` (default `""`), `number: u32` (default 1, range 1–1000),
  and `things: ThingCategory` (default `Objects`).
- **Name generation output**: `NameParts` struct with fields
  `adjective: String` and `noun: String` (unformatted, lowercase).
- **`Casing` enum**: Variants `Upper`, `Lower`. Controls output formatting.

## CI/CD

- **PR gate** (`.github/workflows/ci.yml`): Runs `cargo fmt --check`,
  `cargo clippy -D warnings`, and `cargo test` on every pull request to `main`.
  Single job on `ubuntu-latest`.
- **Release** (`.github/workflows/release.yml`): Triggers on `v*` tags. Builds
  release binaries for 4 targets (Linux x86-64, Windows x86-64, macOS Intel,
  macOS Apple Silicon) on native runners, then creates a GitHub Release with all
  binaries attached and auto-generated release notes (derived from PR titles via
  `softprops/action-gh-release`). Uses only the built-in `GITHUB_TOKEN`.

## Key protocols / APIs

None. CLI only — stdin/stdout. Exit code 0 on success, non-zero on invalid
arguments (clap default).

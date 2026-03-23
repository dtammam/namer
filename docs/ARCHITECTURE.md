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
  main.rs          — CLI entry point, word lists, name generation, formatting, tests
Cargo.toml         — dependencies: clap (derive), rand
docs/
  ARCHITECTURE.md  — this file
  CONTRIBUTING.md  — design principles and coding standards
  RELIABILITY.md   — performance budgets and invariants
  exec-plans/      — feature execution plans
.state/            — feature lifecycle state (managed by engineering-manager)
```

## Component relationships

All components live in `src/main.rs`:

```
main()
  |-- Cli::parse()          (clap-derived arg struct)
  |-- generate_name(rng)    (returns Vec<String> of random words)
  |-- format_name(words, lowercase, delimiter)  (applies casing + joining)
  |-- println!              (output)
```

- `generate_name` depends only on `rand` and the word list constants.
- `format_name` is a pure function with no external dependencies.
- `Cli` depends on `clap` derive macros.

## Data model

- **Word lists**: `ADJECTIVES` and `NOUNS` are `&[&str]` const arrays of lowercase English words.
- **`Cli` struct**: Clap-derived struct with fields `lower: bool` and `delimiter: Option<String>`.
- **Name generation output**: `Vec<String>` of selected words (unformatted, lowercase).

## Key protocols / APIs

None. CLI only -- stdin/stdout. Exit code 0 on success, non-zero on invalid arguments (clap default).
# Output formatting: uppercase default, --lower, --delimiter, --help flags

## Goal

Extend the namer CLI so that generated names are ALL CAPS by default, with opt-in lowercase via `--lower`, configurable word delimiters via `--delimiter`, and complete `--help` output provided by clap.

## Scope

- Change the default output format from mixed/lowercase to ALL CAPS (all generated words uppercased before printing).
- Add a `--lower` boolean flag: when present, output is lowercased instead of uppercased.
- Change the default delimiter between words from hyphen to no delimiter (words concatenated with no separator).
- Add a `--delimiter <STRING>` flag: when provided, the given string is placed between each word in the output.
- Ensure `--help` (provided by clap) displays accurate, complete usage information reflecting all flags including `--lower` and `--delimiter`.
- All new CLI behaviors must have corresponding tests (`cargo test` must pass).
- All new public functions and types must have doc comments.
- `cargo fmt -- --check` and `cargo clippy -- -D warnings` must pass.

## Out of scope

- Adding any word list changes or new word categories.
- Changing the number of words generated.
- Adding any flags other than `--lower`, `--delimiter`, and `--help`.
- Adding new runtime dependencies (std, clap, rand are the only permitted dependencies).
- Any GUI, TUI, or non-CLI interface.

## Constraints

- No runtime dependencies beyond std, clap, rand.
- No `unsafe` blocks.
- No `unwrap()` in library code; use `Result` or `expect()` with context.
- `cargo fmt -- --check` must pass.
- `cargo clippy -- -D warnings` must pass (zero warnings).
- `cargo test` must pass; every public behavior introduced by this feature must have a test.
- All public functions and types introduced must have doc comments.
- Build time must remain under 10 seconds.
- Test suite must complete in under 10 seconds.

## Acceptance criteria

- [x] Running `namer` with no flags produces output that is ALL CAPS with no delimiter between words (e.g., `SWIFTPANDA`).
- [x] Running `namer --lower` produces output that is all lowercase with no delimiter between words (e.g., `swiftpanda`).
- [x] Running `namer --delimiter -` produces output that is ALL CAPS with a hyphen between words (e.g., `SWIFT-PANDA`).
- [x] Running `namer --lower --delimiter -` produces output that is all lowercase with a hyphen between words (e.g., `swift-panda`).
- [x] Running `namer --delimiter ""` (empty string) produces concatenated output with no delimiter (identical behavior to no `--delimiter` flag).
- [x] Running `namer --delimiter "_"` produces output with an underscore between words (e.g., `SWIFT_PANDA`).
- [x] Running `namer --help` (or `namer -h`) prints usage information that accurately describes all flags: `--lower`, `--delimiter`, and `--help`.
- [x] `--lower` and `--delimiter` can be combined in any order without error.
- [x] Passing an unrecognized flag produces an error message and a non-zero exit code (clap default behavior preserved).
- [x] `cargo fmt -- --check` passes with no formatting violations.
- [x] `cargo clippy -- -D warnings` passes with zero warnings.
- [x] `cargo test` passes; every new public function and every new CLI behavior introduced by this feature has at least one test.
- [x] All new public functions and types have doc comments.
- [x] No new runtime dependencies beyond std, clap, rand are introduced.

## Design

### Approach

The feature requires three changes: (1) add clap as a dependency and define a CLI arg struct, (2) refactor name generation to separate word selection from formatting, and (3) apply casing and delimiter formatting in a new formatting step before printing.

Today, `generate_name` in `src/main.rs` selects two random words and returns them joined with a hardcoded hyphen in lowercase. This function couples word selection with formatting. The design splits these concerns: `generate_name` will return a `Vec<String>` of raw lowercase words (adjective, noun), and a new `format_name` function will accept that word list plus formatting options (case, delimiter) and produce the final output string. This keeps the RNG-dependent logic isolated from pure string formatting, which simplifies testing.

Clap must be added to `Cargo.toml` since the exec plan requires `--lower`, `--delimiter`, and `--help` flags. Clap's derive API will define a `Cli` struct with these fields. The `--help` flag comes for free from clap. The binary's `main()` will parse args via `Cli::parse()`, call `generate_name` to get words, then call `format_name` with the parsed options, and print the result.

All code stays in `src/main.rs`. The codebase is small (50 lines) and splitting into multiple modules would be premature. If the project grows, a `lib.rs` extraction can happen later.

### Component changes

- **`Cargo.toml`**: Add `clap = { version = "4", features = ["derive"] }` to `[dependencies]`.
- **`src/main.rs` -- new `Cli` struct**: A clap derive struct with two optional fields:
  - `lower: bool` -- a `--lower` flag (default false). When present, output is lowercased instead of uppercased.
  - `delimiter: Option<String>` -- a `--delimiter <STRING>` option (default `None`). When `None`, words are concatenated with no separator. When `Some(s)`, `s` is placed between words.
- **`src/main.rs` -- `generate_name` refactored**: Change signature from `fn generate_name(rng: &mut impl Rng) -> String` to `fn generate_name(rng: &mut impl Rng) -> Vec<String>`. It returns `vec![adjective.to_string(), noun.to_string()]` with no formatting applied. The words are returned in their original lowercase form from the word lists.
- **`src/main.rs` -- new `format_name` function**: `pub fn format_name(words: &[String], lowercase: bool, delimiter: &str) -> String`. Joins words with the delimiter, then applies casing: if `lowercase` is true, the entire string is lowercased (a no-op since words are already lowercase, but explicit for correctness); otherwise, the entire string is uppercased. Returns the formatted string.
- **`src/main.rs` -- `main` updated**: Parses `Cli` args, calls `generate_name`, resolves the delimiter (empty string if `--delimiter` not provided), calls `format_name`, and prints.
- **`src/main.rs` -- tests updated**: Existing tests that assert on hyphen-separated lowercase output must be rewritten to match the new `generate_name` return type (`Vec<String>`). New tests added for `format_name` covering: uppercase default, lowercase flag, custom delimiter, combined lowercase + delimiter, empty delimiter.

### Data model changes

- **New struct `Cli`** (clap derive): `{ lower: bool, delimiter: Option<String> }`. This is the only new data structure.
- **`generate_name` return type changes** from `String` to `Vec<String>`.

### API changes

This is a CLI tool, not a library API. The public interface changes are:

- **CLI**: New flags `--lower` and `--delimiter <STRING>`. The `--help` / `-h` flag is provided by clap automatically.
- **Default output behavior changes**: Output goes from lowercase-hyphenated (e.g., `bold-falcon`) to uppercase-concatenated (e.g., `BOLDFALCON`). This is a breaking change to the default output format, which is the explicit goal of this feature.

### Alternatives considered

**Alternative: Keep `generate_name` returning `String` and apply formatting post-hoc in `main`.** Under this approach, `generate_name` would continue to return `"adjective-noun"` and `main` would split on hyphen, re-join with the chosen delimiter, and apply casing. This was rejected because it couples the internal separator to the word list contents (if a word ever contained a hyphen, splitting would break), it mixes concerns, and it makes `format_name` harder to test independently. Returning a `Vec<String>` from `generate_name` is cleaner and more composable.

**Alternative: Introduce a `lib.rs` module to separate library code from the binary.** This was rejected as premature for a 50-line codebase. The single-file layout is appropriate at this scale. The `pub` visibility on `generate_name` and `format_name` allows unit testing within the same file via `#[cfg(test)]`.

### Risks and mitigations

- **Risk**: Existing tests assert on the old `generate_name` signature and output format (hyphen-delimited lowercase string). They will fail after the refactor. **Mitigation**: The software-developer must update all existing tests as part of the same commit that changes `generate_name`. The test changes are straightforward since the new return type is simpler.
- **Risk**: Adding clap increases build time. **Mitigation**: Clap with derive features typically adds 3-5 seconds to a clean build. The performance budget is 10 seconds. This should remain within budget, but the build-specialist must verify after implementation. If it exceeds 10 seconds, the `derive` feature can be dropped in favor of clap's builder API, which compiles faster.
- **Risk**: The default output change (lowercase-hyphenated to uppercase-concatenated) is a breaking change for any downstream consumers. **Mitigation**: This is the stated goal of the feature. There are no known downstream consumers beyond direct CLI usage.

### Performance impact

- **Build time**: Adding clap with derive features will increase clean build time by approximately 3-5 seconds. The budget is 10 seconds. This is the primary budget risk and must be verified by the build-specialist.
- **Test suite time**: No meaningful impact. The new tests are pure string operations.
- **Runtime performance**: No meaningful impact. String formatting is negligible.

## Task breakdown

### Task 1: Add clap dependency and define Cli struct

**Files:** `Cargo.toml`, `src/main.rs`

Add `clap = { version = "4", features = ["derive"] }` to `Cargo.toml` dependencies. In `src/main.rs`, add `use clap::Parser;` and define a `Cli` struct with clap's derive macro containing:
- `lower: bool` -- a `--lower` flag (default false)
- `delimiter: Option<String>` -- a `--delimiter <STRING>` option (default `None`)

Both fields and the struct must have doc comments. Do NOT change `main()` yet.

**Done when:** `Cargo.toml` has clap dependency. `Cli` struct exists with `derive(Parser)`, has both fields with doc comments. `cargo build`, `cargo fmt -- --check`, and `cargo clippy -- -D warnings` all pass.

---

### Task 2: Refactor generate_name to return Vec<String> and update existing tests

**Files:** `src/main.rs`

Change `generate_name` from returning `String` (hyphen-joined) to returning `Vec<String>` (raw lowercase words). Update its doc comment. Update all three existing tests (`test_generate_name_format`, `test_generate_name_from_word_lists`, `test_generate_name_deterministic`) to work with the new return type. Temporarily update `main()` so the binary still compiles (join with empty string, uppercase).

**Done when:** `generate_name` returns `Vec<String>`. All three existing tests updated and passing. `cargo test`, `cargo fmt -- --check`, and `cargo clippy -- -D warnings` all pass.

---

### Task 3: Add format_name function with unit tests

**Files:** `src/main.rs`

Add `pub fn format_name(words: &[String], lowercase: bool, delimiter: &str) -> String`. Joins words with delimiter, then uppercases (default) or lowercases. Add doc comment. Add at least 6 unit tests covering: uppercase no delimiter, lowercase no delimiter, uppercase with hyphen, lowercase with hyphen, uppercase with underscore, empty delimiter equivalence.

**Done when:** `format_name` exists with doc comment. 6+ unit tests pass. `cargo test`, `cargo fmt -- --check`, and `cargo clippy -- -D warnings` all pass.

---

### Task 4: Wire Cli parsing into main and add CLI behavior tests

**Files:** `src/main.rs`

Update `main()` to parse `Cli` args, call `generate_name`, resolve delimiter (empty string if not provided), call `format_name`, and print. Add tests exercising the CLI arg combinations through `format_name`. Remove temporary `main()` code from Task 2.

**Done when:** `cargo run` produces ALL CAPS no-delimiter output. `cargo run -- --lower` produces lowercase. `cargo run -- --delimiter -` produces uppercase hyphenated. `cargo run -- --help` shows usage. `cargo test`, `cargo fmt -- --check`, and `cargo clippy -- -D warnings` all pass.

## Progress log

- 2026-03-23 — Exec plan created by product-manager during Discovery.
- 2026-03-23 — Feature completed. All 14 acceptance criteria passed. 5 tasks implemented (including 1 remediation task).

## Decision log

- 2026-03-23 — Default delimiter changed from hyphen to empty string (no separator) per feature request. Previous behavior (hyphen-separated) is now opt-in via `--delimiter -`.
- 2026-03-23 — Testing of all new public behaviors is in scope; CONTRIBUTING.md mandates this unconditionally and it cannot be deferred.

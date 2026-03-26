# Expanded Word Lists, Bulk Generation (--number), and Versioning/Release Notes

## Summary

This plan covers seven areas of improvement to the namer CLI: (1) expanding the hardcoded `ADJECTIVES` and `NOUNS` word lists from 10 entries each to 777 entries each (603,729 unique pairs), producing far greater name variety; (2) adding a `--number N` flag (capped at 1,000) that prints N names per invocation instead of always printing one; (3) establishing a formal semantic versioning policy so the project has clear, trackable releases; (4) ensuring GitHub Releases include auto-generated release notes from PR titles; (5) simplifying README install commands to idempotent one-liners; (6) fixing documentation accuracy issues in ARCHITECTURE.md and README; and (7) adding markdownlint enforcement to the pre-commit hook and agent pipeline. No new runtime dependencies are introduced.

---

## Goals

1. **Word lists**: Generate names with far more variety — 777 adjectives and 777 nouns (603,729 unique pairs) — reducing collision probability and making names more interesting.
2. **Bulk generation**: Allow a single invocation to produce multiple names, e.g., `namer --number 5` prints 5 names (one per line).
3. **Versioning**: Establish a semantic versioning policy and apply it starting with this release, so every future release is versioned consistently and predictably.
4. **Release notes**: Every GitHub Release includes release notes that describe what changed in plain language, auto-generated from PR titles.
5. **Idempotent install**: README install commands are one-liners that skip re-downloading if already installed (Linux/macOS).
6. **Documentation accuracy**: ARCHITECTURE.md and README accurately reflect the current codebase after all changes.
7. **Markdownlint enforcement**: All markdown files pass markdownlint, enforced via pre-commit hook and agent awareness.

---

## Scope

### Feature 1 — Expanded word lists
- Expand `ADJECTIVES` and `NOUNS` from 10 entries each to exactly 777 entries each, kept as `const` arrays compiled into the binary.
- Words must be lowercase, alphabetic-only (no hyphens, no digits), and suitable for use in a random name generator (varied, interesting, not offensive).
- Word lists remain in `src/main.rs` as `const` arrays — no external files, no runtime loading.

### Feature 2 — `--number N` flag
- Add a `--number` flag (type: positive integer, default: `1`) to the `Cli` struct.
- When `--number N` is passed, `namer` prints exactly N names, one per line, each independently randomized.
- N must be a positive integer in the range 1–1,000 (inclusive). The implementation rejects N = 0 and N > 1,000 with a clear error message and a non-zero exit code.
- All existing flags (`--lower`, `--delimiter`) apply to every generated name when `--number` is used.
- `--help` output reflects the new flag with a clear description.

### Feature 3 — Versioning policy
- Define and document a semantic versioning policy for namer (MAJOR.MINOR.PATCH as per semver.org).
- Bump `Cargo.toml` version to `0.2.0` for this release (first planned release after the initial `0.1.0` used for the MVP/CI/README work).
- Document the versioning policy in a new file: `docs/VERSIONING.md`.
- Update `docs/index.md` to reference `VERSIONING.md` in the knowledge map.

### Feature 4 — Release notes on GitHub Releases
- Update the release workflow (`.github/workflows/release.yml`) to include a `body` (release notes) field in the GitHub Release.
- Release notes for each release are written manually by the developer as part of the release process and passed to the release action via the workflow or tag annotation.
- Document the release notes process in `docs/VERSIONING.md` (as part of the release checklist).

---

### Feature 5 — Idempotent README install one-liners

- Simplify the README install/run commands so each platform's install block is a single idempotent command that checks if `namer` is already present before downloading.
- Linux/macOS: use `which namer || (curl ... && chmod ...)` pattern.
- Windows: download to current directory; document that the binary is local-directory only (no PATH modification — out of scope).

### Feature 6 — Documentation accuracy sweep

- Fix factual errors in `docs/ARCHITECTURE.md` introduced during implementation (wrong word count, wrong delimiter type, wrong return type, stale "single source file" claim).
- Update README `--help` example to match actual CLI output after `--number` flag addition.

### Feature 7 — Markdownlint enforcement

- Add `markdownlint-cli2` to the pre-commit hook so all markdown files are linted before every commit.
- Update software-developer and principal-engineer agent definitions to be aware of markdownlint requirements.
- Update CLAUDE.md quality gates and `docs/CONTRIBUTING.md` coding standards to include markdownlint.
- Create `.markdownlint.json` config file for project-specific rule customizations.

---

## Out of scope

- External word list files or runtime word list loading (word lists stay as `const` arrays in source)
- Alliterative name generation, themed word lists, or multiple generation "styles"
- A `--seed` flag or any determinism guarantee across invocations
- Uniqueness guarantees within a single `--number N` invocation (duplicates are acceptable)
- Publishing to crates.io, Homebrew, apt, or any package registry
- Automated CHANGELOG generation (e.g., `git-cliff`, `conventional-commits`)
- A `CHANGELOG.md` file (versioning policy and release notes in GitHub Releases are sufficient)
- Code signing, notarization, or binary integrity checks
- Any new runtime dependencies beyond `std`, `clap`, `rand`
- Windows PATH installation (users manage their own PATH for downloaded binaries)

> **Note on CONTRIBUTING.md standards**: Nothing in "out of scope" defers any mandatory standard. Every public function added or modified must have tests. `cargo fmt -- --check`, `cargo clippy -- -D warnings`, and `cargo test` must all pass. These are non-negotiable and are not deferred.

---

## Constraints

- **No new runtime dependencies**: `std`, `clap`, and `rand` only (RELIABILITY.md and CLAUDE.md non-negotiable).
- **Performance budgets** (RELIABILITY.md non-negotiable):
  - Build time: < 10s (local development build)
  - Test suite: < 10s
- **Quality gates** (CONTRIBUTING.md non-negotiable):
  - `cargo fmt -- --check` must pass
  - `cargo clippy -- -D warnings` must pass (zero warnings)
  - `cargo test` must pass
  - No `unsafe` blocks
  - No `unwrap()` in library code — use `Result` or `expect()` with context
  - All public functions and types have doc comments
  - Every public behavior has a test
- **Architecture**: Single binary, single source file (`src/main.rs`). No `lib.rs` split.
- **Word list format**: Words must be lowercase alphabetic only — no hyphens, digits, uppercase, or spaces.
- **Clap version**: Use the existing `clap = { version = "4", features = ["derive"] }` — no version bump required unless necessary.

---

## Requirements

### R1 — Expanded word lists

- `ADJECTIVES` contains exactly 777 entries. All entries are lowercase, alphabetic only.
- `NOUNS` contains exactly 777 entries. All entries are lowercase, alphabetic only.
- The combinatorial space (`len(ADJECTIVES) × len(NOUNS)`) is 603,729 unique pairs.
- Existing tests (`test_generate_name_format`, `test_generate_name_from_word_lists`, `test_generate_name_deterministic`) must still pass unchanged in behavior.
- No changes to `generate_name` or `format_name` function signatures or behaviors — only the data changes.

### R2 — `--number N` flag

- A new `number: u32` field is added to the `Cli` struct with `#[arg(long, default_value_t = 1)]`.
- When invoked with `--number N` (1 ≤ N ≤ 1,000), the program prints exactly N lines to stdout, each a separately randomized name with the current `--lower` and `--delimiter` settings applied.
- When invoked with no `--number` flag (default N = 1), behavior is identical to the current single-name output — exactly one line to stdout.
- `--number 0` and `--number` values above 1,000 are rejected with a clear error message and a non-zero exit code.
- The `--help` output includes `--number` with a description such as "Number of names to generate [1–1000] (default: 1)".
- Each of the N names is independently randomized (separate RNG samples — not the same name repeated N times).

### R3 — Versioning policy

- `Cargo.toml` `version` field is updated to `0.2.0`.
- A new file `docs/VERSIONING.md` is created containing:
  - The versioning scheme: semantic versioning (MAJOR.MINOR.PATCH) following semver.org.
  - A definition of what constitutes a MAJOR, MINOR, and PATCH change for this project.
  - A release checklist: steps to follow when cutting a new release (update `Cargo.toml` version, write release notes, tag with `v<version>`, push tag, verify CI passes, verify GitHub Release is created).
  - The release notes process: release notes are written manually as part of each release and included in the GitHub Release body.
- `docs/index.md` is updated to reference `VERSIONING.md`.

### R5 — Idempotent README install one-liners

- Each platform's install block in README.md is a single command that checks for an existing `namer` binary before downloading.
- Linux/macOS commands use a `which`/`command -v` check pattern.
- Windows command downloads to current directory and documents that it is local-directory only.
- Re-running the command does not re-download if the binary is already present (Linux/macOS only — Windows is local-directory scoped).

### R6 — Documentation accuracy

- `docs/ARCHITECTURE.md` accurately reflects the current codebase: two source files, correct field types, correct return types, correct word list counts.
- README `--help` example matches actual `cargo run -- --help` output including the `--number` flag.

### R7 — Markdownlint enforcement

- `hooks/pre-commit` runs `markdownlint-cli2` on all `**/*.md` files after the existing cargo checks.
- `.markdownlint.json` exists at project root with project-appropriate rule configuration.
- `docs/CONTRIBUTING.md` lists markdownlint as a coding standard.
- `CLAUDE.md` quality gates section includes markdownlint.
- Software-developer and principal-engineer agent definitions include markdownlint awareness.
- All existing markdown files pass markdownlint.

### R4 — Release notes on GitHub Releases

- The `.github/workflows/release.yml` release job passes a `body` (release notes) to `softprops/action-gh-release`. The mechanism is one of:
  - A `RELEASE_NOTES.md` file at repo root that is populated before tagging and passed via `body_path`.
  - Or: the tag annotation message is used as the release body (GitHub will use the annotated tag message if `body` is not explicitly set and `draft: false`).
- The preferred approach (to be decided by principal engineer) must be documented in `docs/VERSIONING.md` as part of the release checklist.
- For this release (v0.2.0), meaningful release notes are included describing what changed (expanded word lists, `--number` flag, versioning policy, release notes process).

---

## Acceptance Criteria

### Word lists
1. `ADJECTIVES` in `src/main.rs` contains exactly 777 entries; verified by counting entries in the source.
2. `NOUNS` in `src/main.rs` contains exactly 777 entries; verified by counting entries in the source.
3. All entries in `ADJECTIVES` are lowercase, ASCII alphabetic only (no hyphens, digits, spaces, or uppercase).
4. All entries in `NOUNS` are lowercase, ASCII alphabetic only.
5. `len(ADJECTIVES) × len(NOUNS) = 603,729` (verifiable by inspection).
6. `cargo test` passes — all existing word-list and generation tests continue to pass.

### `--number` flag
7. `namer --number 5` prints exactly 5 lines to stdout (one name per line), each independently randomized.
8. `namer` (no flags) prints exactly 1 line to stdout — default behavior is unchanged.
9. `namer --number 1` prints exactly 1 line to stdout — identical behavior to no flag.
10. `namer --number 5 --lower` prints 5 lowercase names.
11. `namer --number 5 --delimiter -` prints 5 hyphen-delimited names.
12. `namer --number 0` exits non-zero and prints a clear error message.
13. `namer --number 1001` exits non-zero and prints a clear error message indicating the maximum is 1,000.
14. `namer --help` includes `--number` in its output with a description.
15. `cargo test` includes at least one test verifying that `--number N` produces exactly N names (unit or integration level).

### Versioning
16. `Cargo.toml` `version` field reads `0.2.0`.
17. `docs/VERSIONING.md` exists and contains: versioning scheme definition, MAJOR/MINOR/PATCH guidance, and a release checklist.
18. `docs/index.md` references `docs/VERSIONING.md`.

### Release notes
19. `.github/workflows/release.yml` includes a mechanism to attach a release body/notes to the GitHub Release (not just the default tag message).
20. The release notes process is documented in `docs/VERSIONING.md`.
21. The v0.2.0 GitHub Release (created when the tag is pushed) includes meaningful release notes describing the changes in this feature set.

### Idempotent install one-liners
22. README.md Linux/macOS install commands check for existing binary before downloading.
23. README.md Windows install command downloads to current directory with a note that it is local-directory only.
24. Re-running Linux/macOS one-liner when `namer` is already on PATH skips the download.

### Documentation accuracy
25. `docs/ARCHITECTURE.md` line describing source files says two files (`src/main.rs` and `src/words.rs`), not one.
26. `docs/ARCHITECTURE.md` data model section matches actual code (777 entries, correct types).
27. README `--help` example includes the `--number` flag.

### Markdownlint enforcement
28. `hooks/pre-commit` includes a `markdownlint-cli2` check on `**/*.md`.
29. `.markdownlint.json` exists at project root.
30. `docs/CONTRIBUTING.md` lists markdownlint as a standard.
31. `CLAUDE.md` quality gates include markdownlint.
32. All `**/*.md` files pass `npx markdownlint-cli2`.

### Quality gates (all changes)
33. `cargo fmt -- --check` passes with no reformatting needed.
34. `cargo clippy -- -D warnings` passes with zero warnings.
35. `cargo test` passes with no failing tests.
36. No `unsafe` blocks are present in `src/main.rs`.
37. All new or modified public functions have doc comments.
38. `npx markdownlint-cli2 '**/*.md'` passes with no errors.

---

## Open Questions

All open questions resolved by user on 2026-03-24:

1. **Word list target size**: 777 entries each (603,729 pairs). Confirmed.
2. **`--number` upper bound**: Capped at 1,000. Values outside 1–1,000 exit non-zero with a clear error. Confirmed.
3. **Release notes mechanism**: Deferred to principal engineer — user has no preference on the technical mechanism. PE to decide and document in the design.
4. **Version number**: `0.2.0`. Confirmed.

---

## Design

### Approach

**Expanded word lists.** The `ADJECTIVES` and `NOUNS` arrays grow from 10 entries each to 777 each. Despite the constraint "single source file (`src/main.rs`)", placing ~2,500 string literals directly in `main.rs` would push it from 141 lines to ~2,700+ lines, burying the actual logic. The recommended approach is to extract the word lists into a new module file `src/words.rs` and declare `mod words;` in `main.rs`. This does not introduce a `lib.rs` — `main.rs` remains the crate root and only source of logic. `words.rs` is a leaf module containing only two `pub const` declarations and no logic. This keeps `main.rs` clean and readable while co-locating related data. The arrays remain `pub(crate) const ADJECTIVES: &[&str]` and `pub(crate) const NOUNS: &[&str]` — compiled into the binary, no runtime loading. If the "single source file" constraint is interpreted strictly to forbid even a data-only module, the fallback is to keep the arrays at the top of `main.rs` (functional but poor readability); the implementer should confirm with the user before proceeding.

**`--number N` flag.** A new `number` field is added to the `Cli` struct with type `u32`. Validation uses clap's built-in `value_parser!(u32).range(1..=1000)` which handles both `--number 0` and `--number 1001` with clear error messages and non-zero exit, requiring zero custom validation code. The `main()` function wraps the existing `generate_name` + `format_name` + `println!` sequence in a simple `for _ in 0..cli.number` loop. Each iteration calls `generate_name` with the same `&mut rng`, producing independently randomized names. The `--lower` and `--delimiter` flags are read once and applied to every name in the loop. Default value is `1`, preserving current behavior exactly when the flag is omitted.

**Versioning policy.** A new `docs/VERSIONING.md` documents: (a) the semver scheme (MAJOR.MINOR.PATCH per semver.org), (b) what constitutes each level of change for namer specifically (MAJOR = breaking CLI changes like renamed/removed flags; MINOR = new features like new flags or expanded word lists; PATCH = bug fixes, doc-only changes, word list corrections), and (c) a release checklist (update `Cargo.toml` version, tag `v<version>`, push tag, verify CI, verify GitHub Release). `Cargo.toml` version is bumped to `0.2.0`.

**Release notes mechanism.** The chosen approach is GitHub's auto-generated release notes via the `generate_release_notes: true` flag on `softprops/action-gh-release`. GitHub automatically produces release notes from PR titles and commit messages since the previous tag. Rationale: (1) zero manual workflow overhead — no file to write or remember to update, (2) notes are always accurate and never stale since they're derived from the actual diff, (3) it naturally encourages good PR titles and commit messages as the source of truth for release communication. The key implication is that PR titles must be written from a user-facing perspective (e.g., "Add --number flag for bulk name generation") since they become the release notes. This is documented in the release checklist in `VERSIONING.md`.

### Component changes

- **`src/words.rs`** (new): Contains `pub(crate) const ADJECTIVES: &[&str]` (777 entries) and `pub(crate) const NOUNS: &[&str]` (777 entries). No logic, no imports, data only.
- **`src/main.rs`** (modified):
  - Add `mod words;` declaration and replace local `ADJECTIVES`/`NOUNS` consts with `use words::{ADJECTIVES, NOUNS};`.
  - Add `number: u32` field to `Cli` struct with `#[arg(long, default_value_t = 1, value_parser = clap::value_parser!(u32).range(1..=1000))]` and a doc comment describing the 1–1000 range.
  - Wrap the generate+format+print sequence in `main()` in a `for` loop iterating `cli.number` times.
  - Add new tests: (a) a test verifying `ADJECTIVES.len() == 777` and `NOUNS.len() == 777`, (b) a test verifying all entries are lowercase ASCII alphabetic, (c) a test for `--number N` producing exactly N lines (can test via calling generate+format in a loop and counting).
- **`Cargo.toml`** (modified): Bump `version` from `"0.1.0"` to `"0.2.0"`.
- **`docs/VERSIONING.md`** (new): Semantic versioning policy, MAJOR/MINOR/PATCH definitions for namer, release checklist including guidance that PR titles should be user-facing (since they become release notes).
- **`docs/index.md`** (modified): Add a reference to `VERSIONING.md` in the Core docs section.
- **`.github/workflows/release.yml`** (modified): Add `generate_release_notes: true` to the `softprops/action-gh-release` step.
- **`docs/ARCHITECTURE.md`** (modified): Update repo layout to show `src/words.rs` and `docs/VERSIONING.md`. Update component relationships and data model sections.

### Data model changes

- **`Cli` struct**: New field `number: u32` (default `1`, range 1–1000).
- **Word list constants**: Moved from `src/main.rs` to `src/words.rs`. Type unchanged (`&[&str]`). Size grows from 10 to 777 entries each. Visibility changes from private to `pub(crate)`.
- **No new types or traits.** The generation loop reuses existing `generate_name` and `format_name` signatures unchanged.

### API changes

- **CLI**: New `--number <N>` flag. All existing flags (`--lower`, `--delimiter`) continue to work identically. `--help` output gains the new flag description. No breaking changes — omitting `--number` produces identical behavior to v0.1.0.
- **No library API changes.** `generate_name` and `format_name` signatures are unchanged.

### Alternatives considered

**Word list storage — keep arrays inline in `main.rs`.**
- *Pros*: Satisfies the strictest reading of "single source file"; no module to manage.
- *Cons*: `main.rs` grows to ~2,700+ lines, with ~95% being data literals. Logic becomes hard to find. Hurts readability and violates "single responsibility" and "modularity" from CONTRIBUTING.md.
- *Rejected because*: The constraint's intent is "single binary, no lib.rs split" — a data-only module preserves the spirit while keeping code navigable. If the user disagrees, falling back to inline is trivial (no design change, just file placement).

**Word list storage — `include_str!` or `include_bytes!` with an external text file.**
- *Pros*: Keeps source files small; easy to edit word lists with external tools.
- *Cons*: Requires runtime parsing (splitting lines), adds complexity, potentially violates "no external files / no runtime loading" from the scope. The arrays wouldn't be `const` in the same way.
- *Rejected because*: Scope explicitly states word lists remain as `const` arrays, not external files.

**Release notes — `RELEASE_NOTES.md` file with `body_path`.**
- *Pros*: Version-controlled, reviewable in PRs, explicit curated narrative.
- *Cons*: Adds manual workflow overhead — developer must write and update the file before every release. If forgotten, the release publishes stale notes from the previous release, which is worse than no notes. Requires discipline that doesn't scale.
- *Rejected because*: The auto-generated approach (`generate_release_notes: true`) achieves the same goal (every release has accurate context) with zero overhead and no stale-data failure mode.

**Release notes — annotated tag messages.**
- *Pros*: No extra file in the repo; notes travel with the tag.
- *Cons*: Annotated tags are awkward to write multi-line content for; not version-controlled or reviewable in PRs; gives less control over formatting. Harder for contributors to discover the expected process.
- *Rejected because*: Auto-generated notes are simpler and more reliable.

**`--number` validation — manual validation after parse.**
- *Pros*: Full control over error message wording.
- *Cons*: More code; duplicates what clap's `value_parser` range does natively; clap's error messages are already clear and consistent with other flag errors.
- *Rejected because*: Clap's built-in range validation is idiomatic, concise, and produces good error output.

### Risks and mitigations

- **Risk**: Build time increases with 2,500 string literals in `words.rs`. → **Mitigation**: String literal arrays are trivial for `rustc` — they compile to static data segments with no codegen complexity. The build budget is 10s; adding data-only constants should add negligible compile time (<1s). The implementer should measure after implementation and flag if build time exceeds 8s.
- **Risk**: Binary size increase from 2,500 embedded strings. → **Mitigation**: Rough estimate: average 7 characters × 2,500 entries = ~17.5 KB of string data. Negligible compared to the current binary (which includes clap and rand). No action needed unless binary exceeds a few MB.
- **Risk**: Word list contains offensive, inappropriate, or duplicate entries. → **Mitigation**: The implementer must curate the list carefully. Include a test that asserts no duplicates within each list. Review a sample during QA. Stick to common, neutral English adjectives and nouns.
- **Risk**: Test suite time increases if word list validation tests iterate all 2,500 entries. → **Mitigation**: Iteration over 2,500 short strings is sub-millisecond. No risk to the 10s budget.
- **Risk**: Auto-generated release notes quality depends on PR title quality. → **Mitigation**: The release checklist in `VERSIONING.md` documents the expectation that PR titles are written from a user-facing perspective. Since this is a small project with few contributors, this is a lightweight process norm rather than a technical enforcement.

### Performance impact

No expected impact on performance budgets. Adding 2,500 string literals as `const` data increases compile time negligibly (static data, no codegen). The `for` loop in `main()` is bounded at 1,000 iterations maximum, each doing two array lookups and a string join — well under any meaningful runtime threshold. Test suite gains ~3-4 new tests, each trivial. The implementer should verify build and test times after implementation and flag if either approaches 8s.

## Task breakdown

### Task 1 -- Extract word lists to `src/words.rs` with 777 entries each

**Files:** `src/words.rs` (new), `src/main.rs` (modified)

Create `src/words.rs` with `pub(crate) const ADJECTIVES: &[&str]` (777 entries) and `pub(crate) const NOUNS: &[&str]` (777 entries). All entries lowercase, alphabetic only, no duplicates. Update `src/main.rs`: add `mod words;` declaration, replace local `ADJECTIVES`/`NOUNS` consts with `use words::{ADJECTIVES, NOUNS};`. Add tests: (a) `ADJECTIVES.len() == 777` and `NOUNS.len() == 777`, (b) all entries are lowercase ASCII alphabetic only, (c) no duplicates within each list. All existing tests must continue to pass.

**Done when:** `src/words.rs` exists with exactly 777 adjectives and 777 nouns. `main.rs` imports from words module. New validation tests pass. All existing tests pass. `cargo fmt`, `clippy`, and `test` all green.

---

### Task 2 -- Add `--number N` flag for bulk name generation

**Files:** `src/main.rs` (modified)

Add `number: u32` field to `Cli` struct with `#[arg(long, default_value_t = 1, value_parser = clap::value_parser!(u32).range(1..=1000))]`. Wrap the generate+format+`println!` sequence in `main()` in a `for` loop of `cli.number` iterations. Add tests: (a) generating N names produces exactly N results, (b) default (no flag) produces exactly 1 name. Verify `--help` includes `--number` with description.

**Done when:** `namer --number 5` prints exactly 5 lines. `namer` (no flag) prints 1 line. `--number 0` and `--number 1001` are rejected with clear errors. `--help` shows `--number`. Tests cover N-line output. `cargo fmt`, `clippy`, and `test` all green.

---

### Task 3 -- Create versioning policy document and bump version to 0.2.0

**Files:** `Cargo.toml` (modified), `docs/VERSIONING.md` (new), `docs/index.md` (modified)

Bump `Cargo.toml` version from `0.1.0` to `0.2.0`. Create `docs/VERSIONING.md` with: semver scheme definition (MAJOR.MINOR.PATCH per semver.org), what constitutes MAJOR/MINOR/PATCH for namer, and a release checklist (update `Cargo.toml`, write user-facing PR titles, tag `v<version>`, push tag, verify CI, verify GitHub Release). Update `docs/index.md` to reference `VERSIONING.md`.

**Done when:** `Cargo.toml` version is `0.2.0`. `docs/VERSIONING.md` exists with semver policy, definitions, and checklist. `docs/index.md` references it. `cargo build` succeeds.

---

### Task 4 -- Add auto-generated release notes to GitHub release workflow

**Files:** `.github/workflows/release.yml` (modified), `docs/VERSIONING.md` (modified)

Add `generate_release_notes: true` to the `softprops/action-gh-release` step in `.github/workflows/release.yml`. Document the release notes process in `docs/VERSIONING.md` (auto-generated from PR titles, guidance on writing user-facing PR titles).

**Done when:** `release.yml` includes `generate_release_notes: true`. `VERSIONING.md` documents the auto-generation approach and PR title guidance.

---

### Task 5 -- Simplify README install commands to idempotent one-liners

**Files:** `README.md` (modified)

Replace multi-step install instructions with single idempotent commands per platform. Linux/macOS: `which namer || (curl ... && chmod ...) && namer`. Windows: download to current directory with a note that it's local-directory only and does not modify PATH. The user can re-run the one-liner without re-downloading (Linux/macOS).

**Done when:** Each platform's install block is a single command. Linux/macOS checks for existing binary. Windows has a clarifying note. README renders correctly.

---

### Task 6 -- Fix ARCHITECTURE.md factual errors and update README help output

**Files:** `docs/ARCHITECTURE.md` (modified), `README.md` (modified)

Fix factual errors in ARCHITECTURE.md: wrong word list count, wrong delimiter field type, wrong `generate_name` return type. Update README `--help` example to include `--number` flag. Review Windows install one-liner for robustness.

**Done when:** ARCHITECTURE.md data model matches actual code. README help example matches `cargo run -- --help` output. All facts verified against source.

---

### Task 7 -- Fix remaining QA findings (stale source file claim, Windows note)

**Files:** `docs/ARCHITECTURE.md` (modified), `README.md` (modified)

Update ARCHITECTURE.md line 9 from "single source file" to two source files. Add clarifying note to Windows install one-liner that binary is local-directory only.

**Done when:** ARCHITECTURE.md accurately describes two source files. Windows install has a clear note about local-directory scope.

---

### Task 8 -- Add markdownlint to pre-commit hook and agent awareness

**Files:** `hooks/pre-commit` (modified), `.claude/agents/software-developer.md` (modified), `.claude/agents/principal-engineer.md` (modified), `CLAUDE.md` (modified), `docs/CONTRIBUTING.md` (modified), `.markdownlint.json` (new)

Add `npx markdownlint-cli2 '**/*.md'` to pre-commit hook after cargo checks. Update software-developer and principal-engineer agents to know about markdownlint requirements. Add markdownlint to CLAUDE.md quality gates and CONTRIBUTING.md coding standards. Create `.markdownlint.json` with project-appropriate rules. Fix any existing markdown lint violations.

**Done when:** Pre-commit hook runs markdownlint. Both agent definitions reference markdownlint. CLAUDE.md and CONTRIBUTING.md document it. `.markdownlint.json` exists. All `**/*.md` files pass `npx markdownlint-cli2`.

## Progress log

- 2026-03-24: Exec plan created by product-manager during Discovery. Requirements gathered for all four feature items. Four open questions identified for principal-engineer resolution.
- 2026-03-24: All four open questions resolved by user. Word lists: 777 each. --number cap: 1,000. Version: 0.2.0. Release notes mechanism: PE to decide. Exec plan finalized.
- 2026-03-24: Design section completed by principal-engineer.
- 2026-03-26: Resumed feature. Word list target revised from 1,250 to 777 unique entries each (603,729 pairs). Exec plan updated throughout to reflect new target. Existing src/words.rs contains partial/low-quality words; Task 1 restarted with fresh implementation.
- 2026-03-26: Tasks 1-4 implemented and verified. QA review requested changes on ARCHITECTURE.md factual errors and README issues.
- 2026-03-26: Task 5 added mid-implementation: idempotent README install one-liners (user request).
- 2026-03-26: Tasks 6-7 added to address QA review findings (ARCHITECTURE.md errors, README help output, Windows install note).
- 2026-03-26: Task 8 added: markdownlint enforcement in pre-commit hook and agent awareness (user request after installing markdownlint in IDE).
- 2026-03-26: Exec plan retroactively updated to include Tasks 5-8 scope, requirements, acceptance criteria, and task breakdowns for historical fidelity.
- 2026-03-26: Feature complete. All 8 tasks implemented, verified, reviewed, and accepted. Stage set to done.

## Decision log

- 2026-03-24: Word lists remain as `const` arrays in `src/main.rs` — no external files, consistent with ARCHITECTURE.md.
- 2026-03-24: `--number` default is 1, preserving existing single-name behavior exactly.
- 2026-03-24: Automated CHANGELOG generation explicitly out of scope — manual release notes only.
- 2026-03-24: Word list size set to 777 entries each (603,729 unique pairs) — revised down from 2,000 by user.
- 2026-03-24: `--number` cap set to 1,000 — values outside 1–1,000 produce a clear error and non-zero exit. Rationale: prevents accidental disk exhaustion or runaway output with no meaningful loss of utility.
- 2026-03-24: Version `0.2.0` confirmed by user.
- 2026-03-24: Release notes mechanism: `generate_release_notes: true` on `softprops/action-gh-release`. User chose auto-generated notes over manual `RELEASE_NOTES.md` to avoid workflow overhead. PR titles serve as the source of user-facing release communication.
- 2026-03-24: Word lists extracted to `src/words.rs` module — user confirmed a data-only module is acceptable despite "single source file" constraint.
- 2026-03-26: Word list target revised from 1,250 to 777 entries each (603,729 unique pairs). User manually started populating src/words.rs but entries need curation and trimming to exactly 777.
- 2026-03-26: Word quality: user explicitly decided to keep existing words (including some fabricated ones) and just trim to 777. QA flagged word quality twice; user overrode both times. This is a conscious tradeoff, not a defect.
- 2026-03-26: Windows PATH installation explicitly out of scope — binary downloads to current directory only, user manages their own PATH.
- 2026-03-26: Markdownlint added as a quality gate alongside cargo fmt/clippy/test. Enforced via pre-commit hook and agent awareness.
- 2026-03-26: Going forward, all new scope must go through the full pipeline (discovery/design) rather than being added ad-hoc during implementation.

# Refactor Word Lists and Add Clap-Driven 'Things' Categories

## Goal

Replace the current flat adjective/noun word lists with curated alternatives and
add a `--things` CLI argument that selects which noun category is used during
name generation.

## Scope

1. Remove the exact-777-count assertion from the unit tests in `src/main.rs`.
2. Replace `ADJECTIVES` with a curated list of ~400 words (user-provided).
3. Add a `--things` clap argument (default: `"objects"`) that selects the active
   noun category.
4. Replace the existing `NOUNS` list with an "objects" category of ~240 words
   (user-provided); this is the default for `--things`.
5. Add a "produce" noun category of ~200 words (user-provided), selectable via
   `--things produce`.
6. Add an "animals" noun category of ~250 words (user-provided), selectable via
   `--things animals`.
7. Update all tests, `--help` text, README examples, and any other references to
   reflect the new structure.

## Out of scope

- Adding noun categories beyond the three specified (objects, produce, animals).
- Changing the behaviour of `--lower`, `--delimiter`, or `--number`.
- Sourcing or inventing actual word lists — the user supplies all lists at
  implementation time.
- Any runtime or build dependency beyond `std`, `clap`, and `rand`.

## Constraints

- No runtime dependencies beyond `std`, `clap`, `rand` (non-negotiable).
- Build time must remain < 10 s; test suite must remain < 10 s
  (`docs/RELIABILITY.md`).
- `cargo fmt -- --check`, `cargo clippy -- -D warnings`, and `cargo test` must
  all pass (`docs/CONTRIBUTING.md`).
- All public functions and types must have doc comments
  (`docs/CONTRIBUTING.md`).
- No `unsafe` blocks; no `unwrap()` in library code (`docs/CONTRIBUTING.md`).
- `--things` must be validated at the CLI parsing boundary; invalid values must
  never reach name-generation logic (`docs/CONTRIBUTING.md`: fail fast and
  visibly, type safety at system boundaries).
- The three noun categories must each be represented as a distinct typed variant,
  not a bare string passed through to runtime logic (`docs/CONTRIBUTING.md`:
  if two values have different domain meanings they must be distinguishable by
  type).

## Acceptance criteria

- [x] Running `namer` without `--things` produces output that draws nouns from
      the objects list (default behaviour unchanged).
- [x] Running `namer --things objects` produces the same default behaviour
      (explicit selection equals the default).
- [x] Running `namer --things produce` produces output that draws nouns from the
      produce list and not from the objects or animals list.
- [x] Running `namer --things animals` produces output that draws nouns from the
      animals list and not from the objects or produce list.
- [x] Running `namer --things invalid` exits with a non-zero status code and
      prints a helpful error message (via clap's default error handling).
- [x] `namer --help` output includes `--things`, lists its accepted values
      (`objects`, `produce`, `animals`), and shows the default (`objects`).
- [x] The exact-777-entry assertion (`word_lists_contain_exactly_777_entries_each`)
      is removed; no test asserts exact word counts.
- [x] Each word list (adjectives, objects, produce, animals) contains only
      non-empty, lowercase ASCII-alphabetic entries with no duplicates within the
      list. Verified by unit tests.
- [x] The adjectives list has at least 300 and no more than 500 entries.
- [x] The objects list has at least 200 and no more than 300 entries.
- [x] The produce list has at least 150 and no more than 250 entries.
- [x] The animals list has at least 200 and no more than 300 entries.
- [x] No word appears in both the adjectives list and any noun category list.
      Verified by a unit test.
- [x] `--things` works correctly in combination with `--lower`, `--delimiter`,
      and `--number` (integration tests cover at least one combination per
      category).
- [x] All public functions and types introduced or modified by this feature have
      doc comments.
- [x] `cargo fmt -- --check` passes.
- [x] `cargo clippy -- -D warnings` passes (zero warnings).
- [x] `cargo test` passes (unit + integration).
- [x] Build time < 10 s; test suite < 10 s.

## Design

### Approach

The feature adds a `ThingCategory` enum with variants `Objects`, `Produce`, and
`Animals`, each mapping to a distinct noun word list. The enum derives
`clap::ValueEnum` so clap validates `--things` at parse time — invalid values
never reach name-generation logic. The existing `NOUNS` array is replaced by
three category-specific arrays (`OBJECTS`, `PRODUCE`, `ANIMALS`), and `ADJECTIVES`
is replaced with a smaller curated list. The `words` module becomes a directory
(`src/words/`) with one sub-file per word list, keeping each pure-data file
focused and independently maintainable. `generate_name` is updated to accept a
noun slice (`&[&str]`) instead of hardcoding `NOUNS`, and a
`ThingCategory::nouns(&self) -> &'static [&'static str]` method maps each variant
to its word list. The `main` function resolves the category to a slice once and
passes it into the generation loop.

### Category system in code

**`ThingCategory` enum** — defined in `src/main.rs` alongside `Cli` and `Casing`,
since it is a CLI-boundary type that plugs directly into clap:

```rust
/// Noun category for name generation.
#[derive(Clone, Copy, clap::ValueEnum)]
pub enum ThingCategory {
    /// Everyday objects (default).
    Objects,
    /// Fruits, vegetables, and other produce.
    Produce,
    /// Animals from around the world.
    Animals,
}
```

**Clap integration** — a new field on `Cli`:

```rust
/// Which noun category to draw from.
#[arg(long, default_value_t = ThingCategory::Objects, value_enum)]
things: ThingCategory,
```

Because `ThingCategory` derives `ValueEnum`, clap auto-generates the accepted
value list (`objects`, `produce`, `animals`) in `--help` and rejects anything
else with a descriptive error.

**Noun list resolution** — a method on `ThingCategory`:

```rust
impl ThingCategory {
    /// Returns the noun word list for this category.
    pub fn nouns(&self) -> &'static [&'static str] {
        match self {
            Self::Objects => words::OBJECTS,
            Self::Produce => words::PRODUCE,
            Self::Animals => words::ANIMALS,
        }
    }
}
```

This is a semantic function: it maps a domain enum to its associated data with
no side effects. It lives on the enum rather than as a free function because the
mapping is intrinsic to the type.

**`generate_name` signature change** — accepts the noun slice directly:

```rust
pub fn generate_name(rng: &mut impl Rng, nouns: &[&str]) -> NameParts
```

This keeps `generate_name` pure and decoupled from the category concept. The
caller (`main`) resolves the category once via `ThingCategory::nouns()` and
passes the resulting slice. Tests can pass any slice, making them independent
of the word module.

### Word list module structure

**Option B: directory module** — `src/words/` with sub-files.

```text
src/words/
  mod.rs          — re-exports all four public constants
  adjectives.rs   — pub(crate) const ADJECTIVES: &[&str]
  objects.rs      — pub(crate) const OBJECTS: &[&str]
  produce.rs      — pub(crate) const PRODUCE: &[&str]
  animals.rs      — pub(crate) const ANIMALS: &[&str]
```

**Justification:** With four arrays totaling ~1100+ entries, a single file would
exceed 1500 lines of pure data with no navigational structure. Splitting into
one file per list aligns with single-responsibility (each file = one word list),
makes diffs clean when a single list is edited, and keeps `mod.rs` to a handful
of `pub mod` and `pub use` lines. Each sub-file has no logic and no imports — it
is a `const` declaration and nothing else.

**Rejected alternatives:**

- *Option A (single file):* Would work but produces a ~1500-line data file that
  is harder to navigate and produces noisy diffs when any single list changes.
- *Option C (build-time include):* Using `include!()` or `build.rs` to load
  word lists from text files adds complexity with no benefit for lists this size.

### Data model changes

- **`NameParts`**: No change. It still holds `adjective: String` and
  `noun: String`. The category is a generation-time concern, not a data-model
  concern.
- **`Cli` struct**: Add field `things: ThingCategory` (with `default_value_t`
  and `value_enum`). Existing fields `lower`, `delimiter`, `number` are
  unchanged.
- **New public types**:
  - `ThingCategory` enum (3 variants, derives `Clone`, `Copy`, `ValueEnum`).
- **New public functions**:
  - `ThingCategory::nouns(&self) -> &'static [&'static str]` — maps variant to
    word list.
- **Removed constants**: `words::NOUNS` is removed and replaced by
  `words::OBJECTS`, `words::PRODUCE`, `words::ANIMALS`.
- **Renamed constant**: `words::ADJECTIVES` keeps its name but moves to
  `src/words/adjectives.rs`.

### Component changes

- **`src/words.rs`** → **`src/words/` directory**:
  - Delete `src/words.rs`.
  - Create `src/words/mod.rs` — declares sub-modules, re-exports constants.
  - Create `src/words/adjectives.rs` — curated `ADJECTIVES` (~400 entries).
  - Create `src/words/objects.rs` — curated `OBJECTS` (~240 entries).
  - Create `src/words/produce.rs` — curated `PRODUCE` (~200 entries).
  - Create `src/words/animals.rs` — curated `ANIMALS` (~250 entries).
- **`src/main.rs`**:
  - Add `ThingCategory` enum with `ValueEnum` derive and `nouns()` method.
  - Add `things: ThingCategory` field to `Cli`.
  - Update `use words::{ADJECTIVES, NOUNS}` → `use words::ADJECTIVES` (nouns
    accessed via `ThingCategory::nouns()`).
  - Change `generate_name` signature to accept `nouns: &[&str]`.
  - Update `main()` to resolve `cli.things.nouns()` and pass into
    `generate_name`.
  - Remove `word_lists_contain_exactly_777_entries_each` test.
  - Update `generate_name_picks_from_word_lists` test to pass a noun slice.
  - Update `word_list_entries_are_non_empty_lowercase_ascii_alphabetic` to cover
    all four lists.
  - Update `word_lists_have_no_duplicates_and_no_cross_list_overlap` to check
    all four lists plus cross-list adjective/noun overlap.
  - Add range-based size assertion tests for each list.
  - Add test that no word appears in both adjectives and any noun list.
- **`tests/cli.rs`**:
  - Add integration tests for `--things objects`, `--things produce`,
    `--things animals`.
  - Add integration test for `--things invalid` (non-zero exit).
  - Add integration test for `--help` mentioning `--things`.
  - Add combination tests: `--things produce --lower --delimiter -`.
- **`README.md`**: Update examples and usage to mention `--things`.
- **`docs/ARCHITECTURE.md`**: Updated separately (see below).
- **`docs/RELIABILITY.md`**: Update invariants to reference category-specific
  noun lists instead of a single `NOUNS` array.

### Performance analysis

- **Compile time**: The total word count increases from ~1554 (777+777) to
  ~1090 (400+240+200+250). This is a *decrease* in compiled data. Even if
  lists were larger, `const` string arrays are trivial for `rustc` — no impact
  on the 10s build budget.
- **Binary size**: Roughly proportional to total word bytes. The net change is
  a slight decrease (~1554 → ~1090 entries). Negligible impact.
- **Test suite time**: Adding ~10-15 new unit/integration tests across 4 lists.
  Each test is O(n) over list size with n < 500. The integration tests invoke
  the binary, but they are already fast (< 100ms each). No risk to the 10s test
  budget.
- **Dependencies**: No new dependencies. `clap::ValueEnum` is already available
  via the `derive` feature.
- **Verdict**: All performance budgets will hold comfortably. No risks.

### Risks and mitigations

- **Risk**: Converting `src/words.rs` to a directory module changes the module
  path, which could break imports if done incorrectly.
  → **Mitigation**: `mod words;` in `main.rs` works identically for both a file
  (`words.rs`) and a directory (`words/mod.rs`). The re-exports in `mod.rs`
  preserve the same `words::ADJECTIVES` path. No import changes beyond removing
  `NOUNS` and using `ThingCategory::nouns()`.

- **Risk**: User-provided word lists may contain duplicates, non-ASCII, or
  empty strings.
  → **Mitigation**: Existing validation tests are extended to cover all four
  lists. The acceptance criteria require these tests. Catch issues at test time,
  not runtime.

- **Risk**: `generate_name` callers forget to pass the correct noun slice.
  → **Mitigation**: The function signature makes the noun slice explicit. There
  is only one call site (`main`), and tests use explicit slices. The compiler
  enforces the required parameter.

- **Risk**: `ThingCategory` display format for `--help` may not match desired
  kebab-case (clap `ValueEnum` lowercases variant names by default).
  → **Mitigation**: Clap's `ValueEnum` derive produces lowercase variant names
  (`objects`, `produce`, `animals`) which matches the desired CLI syntax exactly.
  No `rename_all` attribute needed.

### Alternatives considered

1. **Pass `ThingCategory` into `generate_name` instead of a slice** — This
   would couple `generate_name` to the enum and the `words` module. Passing a
   slice keeps `generate_name` generic and testable with arbitrary data. Rejected
   for tighter coupling with no benefit.

2. **Keep `src/words.rs` as a single file (Option A)** — Simpler file structure
   but results in a ~1500-line data file with four unrelated arrays. Harder to
   navigate, noisier diffs. Rejected in favor of the directory module which
   aligns with single-responsibility.

3. **Use `include_str!` or `build.rs` to load lists from text files** — Adds
   build complexity for no real benefit at this scale. The lists are small,
   static, and benefit from being checked by `rustc` at compile time. Rejected
   for unnecessary complexity.

## Task breakdown

### Task 1: Convert `src/words.rs` to `src/words/` directory module and replace ADJECTIVES

Delete `src/words.rs`. Create `src/words/mod.rs` that declares submodules and
re-exports constants. Create `src/words/adjectives.rs` with the new curated
ADJECTIVES list (~400 entries, user-provided). Temporarily keep NOUNS available
by creating `src/words/objects.rs` with the current NOUNS entries re-exported as
both `OBJECTS` and `NOUNS` (so existing code compiles). Update `mod.rs`
re-exports accordingly.

**Files:** `src/words.rs`, `src/words/mod.rs`, `src/words/adjectives.rs`,
`src/words/objects.rs`

**Done when:** `src/words/` directory exists with `mod.rs`, `adjectives.rs`,
`objects.rs`. ADJECTIVES contains ~400 user-provided entries. NOUNS still
exported for backward compatibility. `cargo test`, `cargo clippy`, `cargo fmt`
all pass.

---

### Task 2: Add `ThingCategory` enum, `--things` CLI arg, and update `generate_name` signature

In `src/main.rs`: add `ThingCategory` enum (`Objects`, `Produce`, `Animals`)
deriving `clap::ValueEnum` with doc comments. Add `nouns()` method on
`ThingCategory`. Add `things: ThingCategory` field to `Cli` struct with default
`Objects`. Change `generate_name` to accept `nouns: &[&str]` parameter. Update
`main()` to resolve `cli.things.nouns()` and pass it. Remove the `NOUNS` import
and backward-compat re-export from `words/mod.rs`. Update existing unit tests to
pass a noun slice to `generate_name`.

**Files:** `src/main.rs`, `src/words/mod.rs`

**Done when:** `ThingCategory` enum exists with `ValueEnum` derive. `--things`
arg on `Cli` with default `Objects`. `generate_name` accepts nouns slice.
Existing tests updated and passing. `cargo clippy` and `cargo fmt` pass.

---

### Task 3: Add produce and animals word lists

Create `src/words/produce.rs` with the user-provided PRODUCE list (~200
entries). Create `src/words/animals.rs` with the user-provided ANIMALS list
(~250 entries). Register both submodules in `src/words/mod.rs` and add
re-exports. The user will provide the exact word lists for both files.

**Files:** `src/words/produce.rs`, `src/words/animals.rs`, `src/words/mod.rs`

**Done when:** `PRODUCE` and `ANIMALS` constants exist and are accessible via
`words::PRODUCE` and `words::ANIMALS`. `ThingCategory::nouns()` correctly
resolves `Produce` and `Animals` variants. `cargo build` succeeds.

---

### Task 4: Replace objects word list with user-provided curated list

Replace the contents of `src/words/objects.rs` with the user-provided OBJECTS
list (~240 entries). Remove any leftover `NOUNS` re-export if still present.
The user will provide the exact word list.

**Files:** `src/words/objects.rs`

**Done when:** `OBJECTS` contains ~240 user-provided entries. No `NOUNS`
constant exists anywhere. `cargo build` succeeds.

---

### Task 5: Update unit tests for new word list structure

In `src/main.rs` tests: (1) Remove `word_lists_contain_exactly_777_entries_each`
test. (2) Update `word_list_entries_are_non_empty_lowercase_ascii_alphabetic` to
cover all four lists (`ADJECTIVES`, `OBJECTS`, `PRODUCE`, `ANIMALS`). (3) Update
`word_lists_have_no_duplicates_and_no_cross_list_overlap` to check all four
lists internally and verify no word appears in both adjectives and any noun
category. (4) Add range-based size assertion tests: ADJECTIVES 300-500, OBJECTS
200-300, PRODUCE 150-250, ANIMALS 200-300.

**Files:** `src/main.rs`

**Done when:** Exact-count test removed. All four lists validated for format,
duplicates, cross-list overlap, and size ranges. `cargo test` passes.

---

### Task 6: Add integration tests for `--things` flag

In `tests/cli.rs`: (1) Test `--things objects` produces output (default
behavior). (2) Test `--things produce` produces output. (3) Test `--things
animals` produces output. (4) Test `--things invalid` exits non-zero. (5) Test
`--help` mentions `--things` with accepted values. (6) Add combination tests:
`--things produce --lower --delimiter -` (and at least one combo per category).

**Files:** `tests/cli.rs`

**Done when:** All new integration tests pass. Each category tested standalone
and in combination with `--lower`/`--delimiter`. Invalid value test confirms
non-zero exit. `--help` test confirms `--things` is documented.

---

### Task 7: Update README and docs

Update `README.md` usage examples to show `--things` flag with each category.
Update `docs/ARCHITECTURE.md` to reflect the new `words/` directory module
structure and `ThingCategory` enum. Update `docs/RELIABILITY.md` to reference
category-specific noun lists instead of single `NOUNS` array.

**Files:** `README.md`, `docs/ARCHITECTURE.md`, `docs/RELIABILITY.md`

**Done when:** README shows `--things` usage. ARCHITECTURE.md reflects new
module structure. RELIABILITY.md references updated correctly.
`npx markdownlint-cli2` passes on all changed files.

## Progress log

- 2026-04-07 — Exec plan created by product-manager (Discovery stage).
- 2026-04-07 — Design section completed by principal-engineer.
- 2026-04-07 — All 7 tasks implemented, reviewed, verified, and accepted. Feature complete.

## Decision log

- 2026-04-07 — Exact word-count tests deliberately removed; list sizes will
  vary by category. Range-based assertions are used instead to give
  implementation flexibility while still verifying approximate size.
- 2026-04-07 — `--things` must be validated at the clap parsing boundary (typed
  enum), per CONTRIBUTING.md "type safety at system boundaries" and "values with
  different domain meanings must be distinguishable by type". A bare `String`
  passed through to runtime logic is not acceptable.
- 2026-04-07 — Cross-noun-category duplicate policy: no word may appear in both
  adjectives and any noun category. Overlap *between* noun categories is not
  explicitly prohibited (a word like "kiwi" might reasonably appear in both
  produce and animals), but each individual list must be internally duplicate-free.
- 2026-04-07 — ADJECTIVES size range revised from 350-500 to 300-500 to match
  the actual curated list size (316 entries).

# Refine README for Human-Friendly Readability

## Goal

Improve the README's readability, structure, and tone so that someone discovering the project for the first time immediately understands what `namer` is, why they'd want it, and how to get started.

## Scope

- **Tone and voice:** Rewrite or adjust any section that reads as terse, developer-internal, or unwelcoming to a first-time visitor.
- **Opening impression:** Ensure the tagline, description, and first visible content clearly communicate the tool's purpose and value within the first few lines.
- **Placeholder cleanup:** Remove or replace the HTML comment `<!-- Icon pending: drop logo-256.png into assets/logo/ to display the project logo -->` — internal developer notes must not be visible to end users.
- **Development section context:** Add brief, plain-language context to each command in the Development section so a new contributor understands *when* they would run it, not just *what* it runs.
- **Section structure:** Verify that section headings are clear, logically ordered, and consistently styled.
- **Consistency:** Ensure tone is uniform throughout — no jarring shifts between approachable prose and bare technical shorthand.

## Out of Scope

- Adding new features, flags, or functionality documentation that does not already exist.
- Changing installation instructions beyond tone/clarity (URLs, targets, and command sequences are authoritative).
- Adding a logo image or any binary assets.
- Modifying any source code, configuration files, or CI workflows.
- Creating new sections not derivable from existing README content.

## Constraints

- The README must remain a single Markdown file at the repo root (`README.md`).
- All existing content categories must be preserved: What is namer, Installation, Usage, Development, Contributing, License.
- No new runtime dependencies, tooling, or build steps are introduced — this is a documentation-only change.
- Performance budgets in `docs/RELIABILITY.md` are unaffected; no cross-check required.
- `docs/CONTRIBUTING.md` coding standards (fmt, clippy, tests) apply to source code — they do not govern Markdown prose. No code is being written in this feature, so there is no conflict.

## Acceptance Criteria

- [x] The internal HTML comment about the logo placeholder is not present in the final README.
- [x] A reader can determine what `namer` does and who it is for within the first five visible lines (excluding badges and the logo image element).
- [x] Every section in the Development section includes a one-sentence explanation of *when or why* a contributor would run the listed command, in addition to the command itself.
- [x] No placeholder text, internal developer notes, or "TODO"-style content is visible anywhere in the rendered README.
- [x] Tone is consistent throughout: approachable and plain-language, with no jarring shift between prose sections and terse technical shorthand.
- [x] All six original content sections are present and complete: What is namer, Installation (all four platforms + build from source), Usage (all four invocation examples + `--help` output), Development, Contributing, License.
- [x] All existing URLs, install commands, and usage examples are unchanged and accurate.

## Design

### Approach

The README will be refined in-place — no structural reorganization of sections, no new sections, no removed sections. The six existing content categories (What is namer, Installation, Usage, Development, Contributing, License) remain in their current order, which already follows the natural reader journey: understand → install → use → contribute.

The guiding tone principle is **"friendly expert"**: approachable and plain-language, as if a colleague were walking you through the tool, but never condescending or verbose. Every sentence should earn its place — cut filler, but add context where bare commands leave the reader guessing.

The single file modified is `README.md` at the repo root. No other files change.

### Component changes

- **Opening block (lines 1–8)**: Remove the HTML comment `<!-- Icon pending: ... -->`. Keep the `<img>` tag (it will simply not render if the image is absent — this is harmless and standard). No change to badges.
- **"What is namer?" section (lines 13–15)**: Light edit only. The current paragraph is already clear and informative. Verify tone consistency with the rest of the rewritten file; adjust phrasing if it feels more terse than surrounding sections after the rewrite.
- **Installation section (lines 17–59)**: Add a brief introductory sentence before the platform subsections (e.g., "Download the latest binary for your platform and make it executable — no dependencies required."). The individual platform blocks and all URLs/commands remain **verbatim and untouched**. The "Build from source" subsection gets a one-sentence lead-in explaining when you'd choose this path.
- **Usage section (lines 61–110)**: Add a one-sentence preamble explaining what the examples demonstrate. The four invocation examples and `--help` output remain **verbatim**. Light touch-up to the brief labels above each example if tone is inconsistent.
- **Development section (lines 112–136)**: This is the primary rewrite target. Each of the four commands (`cargo build --release`, `cargo test`, `cargo clippy`, `cargo fmt`) gets a one-sentence explanation of *when or why* a contributor would run it — not just *what* it does. Add a brief introductory sentence framing the section (e.g., "After cloning the repo, here's how to build, test, and lint locally."). Commands themselves remain **verbatim**.
- **Contributing section (lines 138–140)**: Light tone adjustment to feel welcoming rather than gatekeep-y. The link to `docs/CONTRIBUTING.md` and the listed quality gates remain unchanged.
- **License section (lines 142–144)**: No change.

### Data model changes

None.

### API changes

None.

### Alternatives considered

- **Full restructure with new sections** (e.g., "Quick Start", "FAQ", "Why namer?"): This would go beyond the exec plan scope, which explicitly prohibits "creating new sections not derivable from existing README content." It also risks over-engineering what is a small CLI tool's README. Rejected because it violates scope constraints and adds maintenance burden without proportional value.

- **Templated README generation** (e.g., a script that assembles README from fragments): Overkill for a single-file project with a stable README. Introduces tooling complexity for no ongoing benefit. Rejected.

### Risks and mitigations

- **Risk**: Accidentally altering install URLs, binary names, or command flags while editing surrounding prose. → **Mitigation**: The design explicitly marks all URLs, commands, and `--help` output as verbatim-frozen. The implementer should diff the final output against the original to confirm no command or URL changed. Acceptance criteria #7 covers this.

- **Risk**: Tone overcorrection — making the README too chatty or informal for a developer tool. → **Mitigation**: The "friendly expert" guideline keeps prose concise. Each added sentence should be ≤ 25 words. The QA reviewer should flag any sentence that feels like filler.

- **Risk**: Removing the HTML comment but accidentally deleting the `<img>` tag or badge lines in the same edit. → **Mitigation**: The design specifies removing *only* the comment line (line 5). The `<img>` tag on lines 6–8 and badges on lines 10–11 are explicitly preserved.

### Performance impact

No expected impact on performance budgets. This is a documentation-only change — no source code, build configuration, or dependencies are modified.

## Task Breakdown

### Task 1: Clean up opening block and remove placeholder comment
- **Files:** `README.md`
- **What:** Remove the HTML comment `<!-- Icon pending: drop logo-256.png into assets/logo/ to display the project logo -->` on line 5. Keep the `<img>` tag (lines 6-8) and badge lines (lines 10-11) exactly as they are.
- **Done when:** The HTML comment is gone. The `<img>` tag, badges, heading, and tagline are unchanged. No other lines modified.

### Task 2: Refine "What is namer?" section tone
- **Files:** `README.md`
- **What:** Review the "What is namer?" paragraph for tone consistency with the "friendly expert" voice. Lightly adjust phrasing if it reads more terse or dense than surrounding sections. Keep the same factual content.
- **Done when:** The section conveys the same facts in a consistent, approachable tone. No new claims or removed content.

### Task 3: Add introductory context to the Installation section
- **Files:** `README.md`
- **What:** Add a brief introductory sentence before the platform subsections. Add a one-sentence lead-in to the "Build from source" subsection explaining when you'd choose this path. All URLs, commands, and platform blocks remain verbatim.
- **Done when:** Installation section has a brief intro sentence. "Build from source" has a one-sentence lead-in. All existing URLs and commands are byte-identical to the original.

### Task 4: Add preamble to the Usage section and polish example labels
- **Files:** `README.md`
- **What:** Add a one-sentence preamble at the top of the Usage section explaining what the examples demonstrate. Lightly touch up the labels above each example if their tone is inconsistent. All code blocks remain verbatim.
- **Done when:** Usage section opens with a preamble. Example labels have consistent tone. All code blocks are byte-identical to the original.

### Task 5: Rewrite the Development section with contextual explanations
- **Files:** `README.md`
- **What:** Primary rewrite target. Add a brief intro sentence. For each of the four commands (`cargo build --release`, `cargo test`, `cargo clippy`, `cargo fmt`), add a one-sentence explanation of when or why a contributor would run it. Commands remain verbatim. Each added sentence should be at most 25 words.
- **Done when:** Development section has an intro sentence. Each command has a contextual explanation. All commands are byte-identical. No sentence exceeds roughly 25 words.

### Task 6: Polish Contributing section tone and final consistency pass
- **Files:** `README.md`
- **What:** Lightly adjust the Contributing section to feel welcoming. Keep the link and quality gates unchanged. Then do a final read-through of the entire README to ensure uniform tone with no jarring shifts. License section does not change.
- **Done when:** Contributing section reads as welcoming. Tone is uniform across all sections. License section is unchanged. All URLs and commands are unchanged.

## Progress Log

- **2026-03-24** — Exec plan created by product-manager agent during Discovery stage.
- **2026-03-24** — Feature completed. All 6 tasks implemented, verified, and accepted.

## Decision Log

- **2026-03-24** — `docs/CONTRIBUTING.md` coding standards were reviewed. They govern source code quality (fmt, clippy, tests, no unsafe). Since this feature touches only `README.md` (Markdown prose), no conflict exists between out-of-scope items and mandatory standards. No corrections required.

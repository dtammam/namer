# Exec Plan: Fix README Install One-Liners — Platform-Split Path and Inline Comments

## Goal

Improve the Unix install blocks in `README.md`: use platform-appropriate install
directories, add inline comments so users understand every step, and explicitly
explain why `sudo` is needed on macOS (and how to avoid it if preferred).

## Scope

- `README.md` only — the three Unix install blocks (macOS Apple Silicon, macOS Intel,
  Linux x86-64)
- Inline comments added to each step within those blocks
- No other files touched

## Out of Scope

- Windows install instructions (different mechanism; no `sudo` concern)
- Build-from-source instructions
- Any Rust source, CI, or configuration file changes
- Auto-modification of user shell profiles (`~/.bashrc`, `~/.zshrc`, etc.)
- Runtime dependency changes

## Constraints

- `npx markdownlint-cli2 '**/*.md'` must pass — mandatory per `docs/CONTRIBUTING.md`
- Changes are limited to `README.md`; no code changes permitted
- No new runtime dependencies introduced

## Install Path Decision

**Platform-split strategy:**

| Platform | Install directory | sudo required | mkdir -p needed |
|----------|-------------------|---------------|-----------------|
| macOS (Apple Silicon) | `/usr/local/bin` | Yes | Yes — not guaranteed on fresh macOS (especially Apple Silicon) |
| macOS (Intel) | `/usr/local/bin` | Yes | Yes — not guaranteed on fresh macOS (especially Apple Silicon) |
| Linux (x86-64) | `~/.local/bin` | No | Yes — user directory, may not exist |

**Rationale:**
- macOS: `/usr/local/bin` is on `PATH` by default and `sudo` for a one-time
  system-wide binary install is the normal macOS convention. No PATH update
  required for the user. Note: `/usr/local/bin` is NOT guaranteed to exist on
  fresh macOS installations (especially Apple Silicon Macs that have never had
  Homebrew or Xcode CLI tools installed), so `sudo mkdir -p /usr/local/bin` is
  included as the first step.
- Linux: `~/.local/bin` is XDG-conventional, user-writable without elevation, and on
  `PATH` by default on most Linux distributions. `mkdir -p` creates it if absent.
- The previous single-path approach (`~/.local/bin` everywhere) was rejected because
  it is not on macOS's default `PATH`, creating silent post-install confusion.

## Requirements

### R1 — macOS install directory

Both macOS install blocks use `/usr/local/bin` as the install target. `sudo` is
used for the `mkdir -p`, `curl`, and `chmod` steps. `sudo mkdir -p /usr/local/bin`
is required as the first step because `/usr/local/bin` is not guaranteed to exist
on fresh macOS installations (especially Apple Silicon Macs).

### R2 — Linux install directory

The Linux install block uses `~/.local/bin` as the install target, created with
`mkdir -p ~/.local/bin`. No `sudo` is used.

### R3 — Inline comments

Each Unix install block is reformatted as a multi-line shell block with `#` comments
explaining every step:

- The `which namer` idempotency check (skip if already installed)
- The `mkdir -p` step on Linux (creates user bin directory if absent)
- The `curl` download step (what URL pattern it uses and why)
- The `chmod +x` step (makes the binary executable)
- The final binary invocation
- On macOS blocks: an explicit comment explaining **why** `sudo` is required
  (writing to `/usr/local/bin` is a root-owned system directory) and a short
  comment noting how to avoid `sudo` if preferred (install to `~/.local/bin`
  and add it to `PATH`)

### R4 — Syntactic validity

Each updated install block must be syntactically valid `sh`/`bash` (verifiable
with `bash -n` or manual inspection).

### R5 — Markdown lint

`npx markdownlint-cli2 '**/*.md'` passes on the updated `README.md`.

### R6 — Windows and build-from-source unchanged

The Windows install block and the Build from source block are not modified.

## Acceptance Criteria

- [x] The macOS Apple Silicon install block uses `/usr/local/bin` with `sudo`,
      includes `sudo mkdir -p /usr/local/bin` as the first command inside the
      subshell, and includes comments on every step — including an explicit comment
      stating why `sudo` is needed and a short note on how to avoid it
- [x] The macOS Intel install block uses `/usr/local/bin` with `sudo`, includes
      `sudo mkdir -p /usr/local/bin` as the first command inside the subshell,
      and includes comments on every step — including an explicit comment stating
      why `sudo` is needed and a short note on how to avoid it
- [x] The Linux x86-64 install block uses `~/.local/bin` with `mkdir -p` and no
      `sudo`, and includes comments on every step
- [x] Each updated install block is syntactically valid shell (parseable by `bash -n`)
- [x] `npx markdownlint-cli2 '**/*.md'` passes with zero errors
- [x] The Windows install block is unchanged
- [x] The Build from source block is unchanged
- [x] No files other than `README.md` are modified

## Design

### Approach

This is a docs-only change affecting three shell code blocks in `README.md`
(lines 22–24, 28–30, 34–36). Each single-line install command is replaced with
a multi-line shell block containing `#` comments on every step. The surrounding
prose, Windows block, and build-from-source block are unchanged (R6).

**macOS blocks (Apple Silicon + Intel)** — target directory stays `/usr/local/bin`.
`sudo mkdir -p /usr/local/bin` is included as the first command inside the
subshell because `/usr/local/bin` is not guaranteed to exist on fresh macOS
installations, especially Apple Silicon Macs that have never had Homebrew or
Xcode CLI tools installed (R1). `sudo` is used on `mkdir -p`, `curl -L … -o`,
and `chmod +x` because `/usr/local/bin` is root-owned. A comment block at the
top of each code fence explains why `sudo` is needed and offers a one-line
alternative path (`~/.local/bin` + PATH update) for users who prefer to avoid
`sudo`. Every step (`which` check, `mkdir -p`, `curl`, `chmod`, invocation)
gets its own inline `#` comment (R3).

**Linux block** — target directory changes from `/usr/local/bin` to `~/.local/bin`.
`sudo` is removed entirely. `mkdir -p ~/.local/bin` is added as the first action
step (R2). Every step gets its own inline `#` comment (R3).

All three blocks use the `sh` code fence language tag (unchanged) and must parse
cleanly under `bash -n` (R4). Lines stay under 100 characters where possible to
avoid horizontal scroll in rendered markdown (R5).

**Shell history / up-arrow re-run:** The `which` check comment is placed inline
on the `which namer || (` line (not on a standalone line above it). This ensures
the entire block — from `which namer || (` to `) && namer` — is a single
compound command and a single shell history entry. Pressing up-arrow recalls the
full install block, not just the last line.

#### Exact block structures

**macOS Apple Silicon (`README.md` lines 22–24):**

```sh
which namer || (  # Skip if already installed
  # /usr/local/bin is root-owned on macOS, so sudo is required.
  # To avoid sudo: install to ~/.local/bin and add it to your PATH.
  sudo mkdir -p /usr/local/bin &&    # Ensure install directory exists
  sudo curl -L https://github.com/dtammam/namer/releases/latest/download/namer-aarch64-apple-darwin \
    -o /usr/local/bin/namer &&       # Download the binary
  sudo chmod +x /usr/local/bin/namer # Make it executable
) && namer  # Generate a name
```

**macOS Intel (`README.md` lines 28–30):**

```sh
which namer || (  # Skip if already installed
  # /usr/local/bin is root-owned on macOS, so sudo is required.
  # To avoid sudo: install to ~/.local/bin and add it to your PATH.
  sudo mkdir -p /usr/local/bin &&    # Ensure install directory exists
  sudo curl -L https://github.com/dtammam/namer/releases/latest/download/namer-x86_64-apple-darwin \
    -o /usr/local/bin/namer &&       # Download the binary
  sudo chmod +x /usr/local/bin/namer # Make it executable
) && namer  # Generate a name
```

**Linux x86-64 (`README.md` lines 34–36):**

```sh
which namer || (  # Skip if already installed
  mkdir -p ~/.local/bin &&       # Create user bin directory if absent
  curl -L https://github.com/dtammam/namer/releases/latest/download/namer-x86_64-unknown-linux-gnu \
    -o ~/.local/bin/namer &&     # Download the binary
  chmod +x ~/.local/bin/namer    # Make it executable
) && namer  # Generate a name
```

### Component changes

- **`README.md`** (lines 22–36, the three Unix install code blocks): Replace
  each single-line install command with a multi-line commented shell block as
  specified above. The introductory prose on line 18 ("Paste the one-liner for
  your platform…") should be updated to say "Paste the snippet for your
  platform…" since the blocks are no longer one-liners. No other sections of
  `README.md` are modified.

### Data model changes

None — this is a docs-only change. No Rust source, schema, or configuration
files are affected.

### API changes

None.

### Alternatives considered

1. **Single comment block at top, no per-line comments.** Pros: fewer total
   lines, less visual noise. Cons: users who copy only part of the block lose
   context; per-line comments are more discoverable when scanning. Rejected
   because per-line comments better serve the goal of "understand every step."

2. **Collapsible `<details>` sections with explanations below each block.**
   Pros: keeps the code block clean; explanation is opt-in. Cons: GitHub-flavored
   markdown only — breaks in other renderers; adds HTML to a markdown file.
   Rejected for portability and simplicity.

3. **Separate "Advanced install" documentation page.** Pros: README stays
   concise. Cons: most users never navigate to secondary docs; the primary goal
   is to explain the commands users actually paste. Rejected — inline comments
   are the lowest-friction way to educate.

### Risks and mitigations

- **Risk**: Shell comments after `&&` on the same line may confuse some
  minimal POSIX shells. → **Mitigation**: Comments are placed after the `&&`
  continuation or on their own lines, both of which are valid in `sh` and
  `bash`. Verify with `bash -n` during implementation (R4).

- **Risk**: Line continuations (`\`) followed by trailing whitespace would
  break the shell block. → **Mitigation**: Implementation must ensure no
  trailing whitespace after `\`. Markdownlint rule MD009 (no trailing spaces)
  catches this automatically.

- **Risk**: Changing the introductory prose ("one-liner" → "snippet") could
  drift from the original voice. → **Mitigation**: Change is minimal and
  scoped to one word; reviewer can verify tone consistency.

### Performance impact

No expected impact on performance budgets. This change modifies only
`README.md` — no Rust source, build, or test changes.

## Task Breakdown

### Task 1: Replace three Unix install blocks and update introductory prose in README.md

**Files:** `README.md`

**What to do:**

1. Replace the macOS Apple Silicon install block (currently a single-line command)
   with the multi-line commented shell block from the Design section above.
2. Replace the macOS Intel install block with its corresponding multi-line
   commented shell block from the Design section.
3. Replace the Linux x86-64 install block with its corresponding multi-line
   commented shell block from the Design section.
4. Update the introductory prose (line ~18) from "one-liner" to "snippet" since
   the blocks are no longer one-liners.
5. Leave the Windows install block and build-from-source block completely
   untouched.

**Definition of done:**

- All three Unix blocks match the exact structures in the Design section
- `bash -n` parses each block without errors (syntactically valid shell)
- `npx markdownlint-cli2 '**/*.md'` passes with zero errors
- No trailing whitespace after line continuations (`\`)
- Windows and build-from-source blocks are unchanged
- No files other than `README.md` are modified

## Progress Log

- 2026-03-27 — Initial exec plan written with `~/.local/bin` everywhere.
- 2026-03-27 — Install path decision revised to platform-split strategy after user
  feedback: macOS users should not need a PATH update. Plan updated in full.
- 2026-03-27 — Design section completed by principal-engineer.
- 2026-03-27 — Feature completed. All 8 acceptance criteria verified PASS.

## Decision Log

- 2026-03-27 — Initial decision: `~/.local/bin` for all Unix platforms (sudo-free).
  Revised after identifying that `~/.local/bin` is not on macOS's default PATH,
  which would require users to update their shell profile — the friction we were
  trying to eliminate.
- 2026-03-27 — Final decision: platform-split. macOS uses `/usr/local/bin` with
  `sudo` (on PATH by default, pre-existing directory, normal macOS convention).
  Linux uses `~/.local/bin` without `sudo` (XDG-conventional, on PATH by default
  on most distros). macOS `sudo` comments must explicitly explain the reason and
  offer a sudo-free alternative for users who prefer it.
- 2026-03-27 — "One-liner" blocks will be reformatted as multi-line shell blocks
  with `#` comments. Shell does not support inline comments on a single logical line;
  multi-line blocks are the only readable alternative. Code fence language tag (`sh`)
  is preserved.
- 2026-03-27 — Design correction: the original design assumed `/usr/local/bin`
  always exists on macOS and explicitly prohibited `mkdir -p`. This assumption is
  wrong. On fresh Apple Silicon Macs (no Homebrew, no Xcode CLI tools),
  `/usr/local/bin` does not exist, and `curl -o /usr/local/bin/namer` will fail.
  The QA review correctly identified this. AC-1 and AC-2 updated to REQUIRE
  `sudo mkdir -p /usr/local/bin` as the first command inside the subshell, and
  R1 updated accordingly. The implementation already included this fix (task 2).

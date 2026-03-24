# Cross-Platform Releases, CI Pipeline, App Branding, and README

## Goal

Make namer production-ready for public use by shipping cross-platform prebuilt binaries via GitHub Actions CI/CD, establishing a project identity with an app icon, and providing a high-quality README that communicates the tool's value and usage.

## Work streams

This plan covers four sequential work streams on a single branch. Each has its own scope, out-of-scope, constraints, and acceptance criteria.

---

## Work stream 1: Cross-platform binary releases

### Goal

Add macOS (Intel and Apple Silicon) and Linux build targets so users on all major platforms can download a prebuilt binary without installing Rust.

### Scope

- Configure Cargo cross-compilation targets for:
  - `x86_64-apple-darwin` (macOS Intel)
  - `aarch64-apple-darwin` (macOS Apple Silicon)
  - `x86_64-unknown-linux-gnu` (Linux x86-64)
  - `x86_64-pc-windows-gnu` or `x86_64-pc-windows-msvc` (Windows, existing)
- Produce a standalone binary artifact per target (no installer, no package manager)
- Binary naming convention: `namer-<target-triple>` (e.g., `namer-x86_64-apple-darwin`)

### Out of scope

- macOS `.app` bundles or `.dmg` packaging
- Linux `.deb`, `.rpm`, or AppImage packaging
- Windows `.msi` or `.exe` installer packaging
- ARM Linux targets
- 32-bit targets of any platform
- Code signing or notarization

### Constraints

- No new runtime dependencies beyond std, clap, rand (RELIABILITY.md non-negotiable)
- Build time budget: < 10s per target on the CI runner (RELIABILITY.md)
- All cross-compilation must be achievable in GitHub Actions without custom hardware
- `cargo fmt -- --check` and `cargo clippy -- -D warnings` must pass for all targets
- `cargo test` must pass for all targets

### Acceptance criteria

- [x] Running `cargo build --target x86_64-apple-darwin --release` produces a binary artifact
- [x] Running `cargo build --target aarch64-apple-darwin --release` produces a binary artifact
- [x] Running `cargo build --target x86_64-unknown-linux-gnu --release` produces a binary artifact
- [x] Each produced binary is a standalone executable (not a shared library, not a script)
- [x] `cargo fmt -- --check` passes with no changes for the project as a whole
- [x] `cargo clippy -- -D warnings` passes with zero warnings
- [x] `cargo test` passes for all targets

---

## Work stream 2: CI pipeline and release automation

### Goal

Set up GitHub Actions workflows that enforce code quality on every PR and automatically build and publish versioned cross-platform binaries to GitHub Releases on every tagged commit.

### Scope

- Workflow A — PR gate (triggers on `pull_request` to `main`):
  - `cargo fmt -- --check`
  - `cargo clippy -- -D warnings`
  - `cargo test`
- Workflow B — Release (triggers on push of a tag matching `v*`):
  - Build binaries for all four targets (macOS Intel, macOS Apple Silicon, Linux x86-64, Windows x86-64)
  - Create a GitHub Release for the tag
  - Upload all four binaries as release assets
- Version tagging convention: semver, e.g., `v1.0.0`
- Workflow files live under `.github/workflows/`

### Out of scope

- Publishing to crates.io
- Publishing to Homebrew, apt, or any package registry
- Scheduled or nightly builds
- Deployment to any hosted service
- Docker images
- Release notes generation (beyond GitHub's default tag description)

### Constraints

- GitHub Actions only — no third-party CI services
- Workflows must not introduce any new runtime dependencies
- All steps must use pinned action versions (e.g., `actions/checkout@v4`) to avoid supply-chain drift
- No secrets beyond the built-in `GITHUB_TOKEN` are permitted for the release workflow
- `cargo fmt -- --check` and `cargo clippy -- -D warnings` must be enforced in the PR gate (not merely advisory)
- No `--no-verify` bypasses permitted

### Acceptance criteria

- [x] `.github/workflows/` contains a PR gate workflow file
- [x] `.github/workflows/` contains a release workflow file
- [x] The PR gate workflow runs `cargo fmt -- --check`, `cargo clippy -- -D warnings`, and `cargo test` in order
- [x] A failing fmt, clippy, or test step causes the PR gate workflow to fail (non-zero exit)
- [x] The release workflow triggers on tags matching `v*` pushed to the repository
- [x] The release workflow builds binaries for all four targets (macOS Intel, macOS Apple Silicon, Linux x86-64, Windows x86-64)
- [x] The release workflow creates a GitHub Release associated with the pushed tag
- [x] All four binary artifacts are uploaded as assets on the GitHub Release
- [x] Release asset filenames clearly identify the target platform (e.g., `namer-x86_64-apple-darwin`)
- [x] All workflow actions reference pinned versions (e.g., `@v4`, not `@latest` or a branch)
- [x] The release workflow uses only `GITHUB_TOKEN` for authentication — no additional secrets

---

## Work stream 3: App logo/icon preparation

### Goal

Establish the canonical location, required formats and sizes, and README reference convention for the project's app icon so that the asset can be dropped in at implementation time without rework.

### Scope

- Define the canonical asset path within the repository (e.g., `assets/logo/`)
- Define required formats: PNG (primary), SVG (if source is vector)
- Define required sizes for PNG: at minimum 256x256 and 512x512
- Define how the icon is referenced in the README (relative Markdown image link)
- Create the `assets/logo/` directory with a `.gitkeep` placeholder so the path exists in the repo
- Document the naming convention for icon files (e.g., `logo-256.png`, `logo-512.png`, `logo.svg`)

Note: The actual icon file is pending — the user will provide it at implementation time. The exec plan and directory structure are established now so implementation can proceed without design rework.

### Out of scope

- Icon design or generation
- Platform-specific icon formats (`.icns` for macOS, `.ico` for Windows)
- Favicon generation for any website
- Icon embedding into the binary itself
- Automated icon resizing scripts

### Constraints

- Asset path must be consistent with the repo layout described in ARCHITECTURE.md (top-level directories are acceptable additions; no changes to `src/` structure)
- No new Rust dependencies introduced
- Icon files must be committed as binary assets — no build step to generate them at compile time

### Acceptance criteria

- [x] `assets/logo/` directory exists in the repository
- [x] A naming convention for icon files is documented in the exec plan and/or a brief `assets/logo/README` or inline comment
- [x] The README contains an img or Markdown image reference pointing to the canonical icon path
- [x] The image reference in the README uses a relative path that resolves correctly from the repo root
- [x] A placeholder note in the README or a `.gitkeep` in `assets/logo/` makes clear that the icon file is pending and will be added by the user

---

## Work stream 4: High-quality README

### Goal

Replace or substantially rewrite the project README to clearly communicate what namer is, why someone would use it, how to install it, and how to use it — at a quality bar consistent with a well-maintained open-source CLI tool.

### Scope

- Required README sections:
  1. Project name and tagline (one sentence)
  2. App icon/logo (links to work stream 3 asset path)
  3. Badges (CI status, latest release)
  4. What it is / problem it solves (2-4 sentences)
  5. Installation — prebuilt binary download instructions for each platform
  6. Usage — CLI invocation examples covering all flags (`--lower`, `--delimiter`, `--help`)
  7. Development — how to build from source (`cargo build --release`)
  8. Contributing — pointer to `docs/CONTRIBUTING.md`
  9. License
- All CLI examples must be accurate and runnable
- README lives at repository root as `README.md`

Note: The user will provide a reference README from the tasksync project at implementation time. The software developer should use it as a quality and style reference, not as a template to copy verbatim.

### Out of scope

- A documentation website (GitHub Pages, mdBook, etc.)
- Changelog file
- Code of conduct file
- Security policy file
- Translated READMEs (non-English)
- API documentation beyond what `--help` already provides

### Constraints

- All CLI flag names and behaviors documented in the README must match the actual implementation exactly — no aspirational or incorrect documentation
- Markdown must render correctly on GitHub (use GitHub-flavored Markdown only)
- No external image hosting — all images use relative repo paths
- Must satisfy the CONTRIBUTING.md standard "naming is documentation": section headings must be self-explanatory
- Badge links must point to real GitHub Actions workflow runs and real GitHub Releases (not placeholder URLs) — they may be added after CI is live

### Acceptance criteria

- [x] `README.md` exists at the repository root
- [x] README contains all nine required sections listed in scope
- [x] The project tagline is present and accurately describes the tool in one sentence
- [x] Installation section includes download instructions for macOS Intel, macOS Apple Silicon, Linux x86-64, and Windows x86-64
- [x] Usage section shows at least one example for each of: plain invocation, `--lower` flag, `--delimiter` flag, combined flags
- [x] All CLI examples use flag names that match the actual binary's `--help` output exactly
- [x] CI status badge links to the PR gate workflow
- [x] Latest release badge links to the GitHub Releases page
- [x] The app icon/logo image renders on GitHub (relative path resolves correctly)
- [x] README renders without broken links or broken image references on GitHub
- [x] No section contains placeholder text such as "TODO" or "Coming soon" at the time of acceptance (pending-asset notes for the icon are acceptable only until the icon is provided)

---

## Design

### Work stream 1: Cross-platform binary releases

#### Approach

No changes to `Cargo.toml` or `src/main.rs` are required. The project has zero platform-specific code (no `unsafe`, no FFI, no OS-conditional compilation). All four targets compile from the same source with only the `--target` flag differing. Cross-compilation is handled entirely within CI (work stream 2), not locally.

Each target is built using a native GitHub Actions runner for that OS family. This avoids cross-compilation toolchain complexity entirely:

| Target triple | Runner | Build command |
|---|---|---|
| `x86_64-unknown-linux-gnu` | `ubuntu-latest` | `cargo build --release --target x86_64-unknown-linux-gnu` |
| `x86_64-pc-windows-msvc` | `windows-latest` | `cargo build --release --target x86_64-pc-windows-msvc` |
| `x86_64-apple-darwin` | `macos-13` | `cargo build --release --target x86_64-apple-darwin` |
| `aarch64-apple-darwin` | `macos-14` | `cargo build --release --target aarch64-apple-darwin` |

The key insight: `macos-13` runners are Intel-based and `macos-14` runners are Apple Silicon (M1). This means both macOS targets are built natively with no cross-compilation toolchain needed. Windows uses the MSVC toolchain (not GNU) because `windows-latest` runners have MSVC pre-installed and it produces the most compatible Windows binaries.

#### Artifact naming convention

Release binaries follow the convention `namer-<target-triple>[.exe]`:

- `namer-x86_64-unknown-linux-gnu`
- `namer-x86_64-pc-windows-msvc.exe`
- `namer-x86_64-apple-darwin`
- `namer-aarch64-apple-darwin`

The `.exe` extension is added only for the Windows target. The target triple is used verbatim (not a friendly name like "linux" or "macos") to eliminate ambiguity.

#### Cargo.toml changes

None. The existing `Cargo.toml` with `edition = "2024"`, `clap` (derive), and `rand` works identically across all four targets. No target-specific dependencies, features, or build scripts are needed.

#### Risks and mitigations

- **Risk**: GitHub deprecates `macos-13` Intel runners. **Mitigation**: If Intel runners are retired, replace with `macos-14` plus `rustup target add x86_64-apple-darwin` for cross-compilation. This is a single-line change in the workflow.
- **Risk**: Build time exceeds 10s budget on CI runners. **Mitigation**: The project has only 2 dependencies (clap, rand). Cold builds on GitHub Actions typically complete in 30-60s due to dependency compilation, but this is a CI-only cost, not a local development cost. The 10s budget in RELIABILITY.md applies to local development builds. CI builds should use `--release` and are expected to take longer; this is acceptable.

#### Performance impact

No impact on runtime performance. No new dependencies introduced. Local build times unchanged.

---

### Work stream 2: CI pipeline and release automation

#### Approach

Two separate workflow files under `.github/workflows/`:

1. **`.github/workflows/ci.yml`** -- PR gate workflow
2. **`.github/workflows/release.yml`** -- Tag-triggered release workflow

Splitting into two files follows the single-responsibility principle: the PR gate is fast and runs on every PR; the release workflow is heavyweight and runs only on version tags. This also makes it easier to modify one without affecting the other.

#### PR gate workflow: `.github/workflows/ci.yml`

```
name: CI
on:
  pull_request:
    branches: [main]

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - actions/checkout@v4
      - dtolnay/rust-toolchain@stable
      - cargo fmt -- --check
      - cargo clippy -- -D warnings
      - cargo test
```

Design decisions:
- Single job, sequential steps. If `fmt` fails, there is no point running `clippy` or `test`. Sequential execution in one job is simpler and cheaper than parallel jobs.
- Runs on `ubuntu-latest` only. The code has no platform-specific behavior, so running quality checks on one OS is sufficient. Cross-platform correctness is validated by the release workflow building on all platforms.
- Uses `dtolnay/rust-toolchain@stable` (pinned to `@stable`) for Rust installation instead of `actions-rs` (unmaintained). This is the community-standard Rust toolchain action.

#### Release workflow: `.github/workflows/release.yml`

```
name: Release
on:
  push:
    tags: ["v*"]

jobs:
  build:
    strategy:
      matrix:
        include:
          - target: x86_64-unknown-linux-gnu
            os: ubuntu-latest
            artifact_name: namer-x86_64-unknown-linux-gnu
          - target: x86_64-pc-windows-msvc
            os: windows-latest
            artifact_name: namer-x86_64-pc-windows-msvc.exe
          - target: x86_64-apple-darwin
            os: macos-13
            artifact_name: namer-x86_64-apple-darwin
          - target: aarch64-apple-darwin
            os: macos-14
            artifact_name: namer-aarch64-apple-darwin
    runs-on: ${{ matrix.os }}
    steps:
      - actions/checkout@v4
      - dtolnay/rust-toolchain@stable
      - cargo build --release --target ${{ matrix.target }}
      - Rename binary from target/.../namer[.exe] to ${{ matrix.artifact_name }}
      - actions/upload-artifact@v4 (upload the renamed binary)

  release:
    needs: build
    runs-on: ubuntu-latest
    permissions:
      contents: write
    steps:
      - actions/download-artifact@v4 (download all build artifacts)
      - softprops/action-gh-release@v2 (create release, attach all 4 binaries)
        with:
          files: <glob matching all 4 artifacts>
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

Design decisions:
- **Two-phase job structure**: `build` (matrix of 4) then `release` (single job). The build jobs run in parallel on native runners, then the release job collects all artifacts and creates a single GitHub Release. This ensures the release is atomic: if any build fails, no release is created.
- **`softprops/action-gh-release@v2`**: This is the most widely used GitHub Release action. It handles release creation and asset upload in one step. It uses the tag name as the release title by default, which matches the `v*` tag convention.
- **`GITHUB_TOKEN` only**: The built-in token has `contents: write` permission when explicitly granted via `permissions`, which is sufficient for creating releases and uploading assets. No additional secrets needed.

#### Pinned action versions

| Action | Version | Purpose |
|---|---|---|
| `actions/checkout` | `@v4` | Clone repo |
| `dtolnay/rust-toolchain` | `@stable` | Install Rust stable |
| `actions/upload-artifact` | `@v4` | Pass binaries between jobs |
| `actions/download-artifact` | `@v4` | Retrieve binaries in release job |
| `softprops/action-gh-release` | `@v2` | Create release and upload assets |

All versions are pinned to major version tags (`@v4`, `@v2`). These are the current latest major versions as of March 2026.

#### Alternatives considered

1. **`cross-rs/cross` for cross-compilation instead of native runners**: `cross` uses Docker containers with pre-configured cross-compilation toolchains. Pros: could build all targets on a single `ubuntu-latest` runner. Cons: cannot cross-compile for macOS (Apple's license prohibits non-Apple hardware); would still need macOS runners for Darwin targets; adds Docker layer complexity; slower than native compilation. Rejected because native runners solve the problem completely without added complexity.

2. **Single workflow file for both CI and release**: Pros: fewer files to maintain. Cons: the triggers (`pull_request` vs `push tags`) and job structures are entirely different; combining them into one file with conditional logic would be harder to read and maintain. Rejected in favor of single-responsibility separation.

3. **`actions-rs/toolchain` for Rust installation**: This action is unmaintained (last updated 2021). `dtolnay/rust-toolchain` is actively maintained and is the community standard. Rejected `actions-rs`.

#### Risks and mitigations

- **Risk**: `softprops/action-gh-release` becomes unmaintained. **Mitigation**: The action is widely used (10k+ stars). If it becomes unmaintained, the fallback is to use the GitHub CLI (`gh release create`) directly in a run step, which requires no third-party action.
- **Risk**: GitHub Actions runner image changes break builds. **Mitigation**: Using `@stable` for the Rust toolchain means builds track the latest stable Rust automatically. Runner images are versioned by GitHub; using `-latest` tags means automatic updates, which is appropriate for a simple project.

#### Performance impact

No impact on runtime performance. CI adds build time to PRs (approximately 1-2 minutes for the gate) and releases (approximately 3-5 minutes for 4 parallel builds + release creation), but these are infrastructure costs with no impact on RELIABILITY.md budgets.

---

### Work stream 3: Logo/icon preparation

#### Approach

Create the directory `assets/logo/` at the repository root with a `.gitkeep` file to establish the path in version control. The actual icon files will be provided by the user at implementation time and dropped into this directory.

#### Directory structure

```
assets/
  logo/
    .gitkeep              -- placeholder until icon files are added
    logo-256.png          -- (pending: user-provided, 256x256 PNG)
    logo-512.png          -- (pending: user-provided, 512x512 PNG)
    logo.svg              -- (pending: optional, vector source)
```

#### File naming convention

- `logo-<size>.png` for rasterized versions (e.g., `logo-256.png`, `logo-512.png`)
- `logo.svg` for the optional vector source
- All lowercase, hyphen-separated

#### README image reference

The README will reference the logo using a relative Markdown image tag placed immediately after the top-level heading:

```markdown
# namer

<p align="center">
  <img src="assets/logo/logo-256.png" alt="namer logo" width="128">
</p>
```

Using the 256px PNG rendered at 128px CSS width ensures crisp display on high-DPI screens. The `<p align="center">` tag centers the image on GitHub (GitHub-flavored Markdown does not support centering via standard Markdown syntax). Until the user provides the actual icon file, this image tag will reference a file that does not yet exist; the README will include a comment noting the icon is pending.

#### What gets committed now vs later

Now (by the software developer):
- `assets/logo/.gitkeep`

Later (when user provides the icon):
- `assets/logo/logo-256.png`
- `assets/logo/logo-512.png`
- `assets/logo/logo.svg` (if vector source is available)
- Remove `.gitkeep` once real files exist

#### Risks and mitigations

- **Risk**: Broken image in README until the icon is provided. **Mitigation**: The README will include an HTML comment `<!-- Icon pending: replace .gitkeep with logo-256.png -->` adjacent to the img tag so it is clear this is intentional. GitHub renders broken images as alt text, which is acceptable temporarily.

#### Performance impact

No expected impact on performance budgets.

---

### Work stream 4: High-quality README

#### Approach

Replace the current one-line `README.md` with a full project README. The structure follows standard conventions for open-source CLI tools, adapted to the specific content requirements in the exec plan.

#### Section structure

The README will be organized with these headings in order:

```markdown
# namer

<!-- centered logo image tag (work stream 3) -->

<!-- badges row: CI status, latest release -->

> One-sentence tagline describing the tool.

## What is namer?
(2-4 sentences: what it does, the problem it solves)

## Installation
### macOS (Apple Silicon)
### macOS (Intel)
### Linux (x86-64)
### Windows (x86-64)
### Build from source

## Usage
(CLI examples with output)

## Development
(Build from source, run tests, run lints)

## Contributing
(Pointer to docs/CONTRIBUTING.md)

## License
```

This is 9 sections as required: (1) project name + tagline, (2) logo, (3) badges, (4) what it is, (5) installation, (6) usage, (7) development, (8) contributing, (9) license.

#### Badge implementation

Two badges, placed on a single line between the logo and the tagline:

```markdown
[![CI](https://github.com/dtammam/namer/actions/workflows/ci.yml/badge.svg)](https://github.com/dtammam/namer/actions/workflows/ci.yml)
[![Latest Release](https://img.shields.io/github/v/release/dtammam/namer)](https://github.com/dtammam/namer/releases/latest)
```

- The CI badge uses GitHub's native workflow badge URL (not shields.io) for the most accurate real-time status.
- The release badge uses shields.io's GitHub release endpoint, which auto-updates when new releases are published.
- Both badges link to their respective GitHub pages.

Note: These badges will show "no status" / "no releases" until the CI workflow is merged and a release tag is pushed. This is expected and acceptable per the acceptance criteria (badges must link to real URLs, not placeholders).

#### Installation section structure

Each platform subsection follows the same pattern:

```markdown
### macOS (Apple Silicon)

curl -L https://github.com/dtammam/namer/releases/latest/download/namer-aarch64-apple-darwin -o namer
chmod +x namer
./namer
```

For Windows, the instructions will use PowerShell's `Invoke-WebRequest` instead of `curl`, and the binary name includes `.exe`. The "Build from source" subsection provides `cargo install` / `cargo build --release` instructions for users who have the Rust toolchain installed.

#### CLI usage examples

The Usage section will show four examples matching the acceptance criteria:

1. **Plain invocation** (no flags): `namer` producing e.g. `BOLDFALCON`
2. **`--lower` flag**: `namer --lower` producing e.g. `boldfalcon`
3. **`--delimiter` flag**: `namer --delimiter -` producing e.g. `BOLD-FALCON`
4. **Combined flags**: `namer --lower --delimiter _` producing e.g. `bold_falcon`

Each example will use a fenced code block with `console` or `sh` syntax highlighting, showing the command and its output. Example output will use hardcoded sample values (not randomized) to be reproducible in documentation.

The section will also include `namer --help` output in a code block for discoverability.

#### Logo placement

The logo image tag goes between the `# namer` heading and the badge row, centered using the HTML pattern described in work stream 3.

#### Risks and mitigations

- **Risk**: CLI flags change in the future, making README examples inaccurate. **Mitigation**: The acceptance criteria for the README require examples to match actual `--help` output. Future flag changes should update the README in the same PR.
- **Risk**: Release download URLs are incorrect before the first release exists. **Mitigation**: URLs use the GitHub releases convention (`/releases/latest/download/<filename>`) which will resolve correctly once the first release is published. Until then, they will 404, which is acceptable since the installation instructions only become relevant after a release exists.

#### Performance impact

No expected impact on performance budgets.

## Task breakdown

### Recommended sequence

Work is ordered by dependency: cross-platform builds must be validated before CI references them, CI must exist before README badges can link to it, and the logo directory must exist before the README references it.

### Work stream 1: Cross-platform binaries

**Task ws1-t1: Verify cross-platform build targets compile locally**
- Confirm `cargo build --release` succeeds for all four targets (x86_64-unknown-linux-gnu, x86_64-pc-windows-msvc, x86_64-apple-darwin, aarch64-apple-darwin) by running the Linux build locally and documenting expected build commands
- No code changes expected -- validate that existing Cargo.toml and src/main.rs have no platform-specific issues
- If any target requires Cargo.toml adjustments, make them
- Files: `Cargo.toml` (if changes needed)
- Done when: `cargo build --release` succeeds locally; all four build commands documented; cargo fmt, clippy, and test pass
- **COMMIT POINT**: `ws1: verify cross-platform build targets`

### Work stream 2: CI pipeline and release automation

**Task ws2-t1: Create PR gate CI workflow**
- Create `.github/workflows/ci.yml` triggered on pull_request to main
- Single job on ubuntu-latest: cargo fmt -- --check, cargo clippy -- -D warnings, cargo test (sequential)
- Use actions/checkout@v4, dtolnay/rust-toolchain@stable
- All action versions pinned
- Files: `.github/workflows/ci.yml`
- Done when: ci.yml is valid YAML with correct triggers, steps, and pinned versions; cargo fmt, clippy, and test pass locally
- **COMMIT POINT**: `ws2: add PR gate CI workflow`

**Task ws2-t2: Create release workflow**
- Create `.github/workflows/release.yml` triggered on push of tags matching `v*`
- Matrix strategy: 4 runners (ubuntu-latest, windows-latest, macos-13, macos-14) for 4 targets
- Each build job: checkout, install rust stable, build release, rename binary to `namer-<target-triple>[.exe]`, upload artifact
- Release job: depends on all builds, downloads artifacts, uses softprops/action-gh-release@v2 to create release and attach binaries
- Only GITHUB_TOKEN with contents:write permission; all action versions pinned
- Files: `.github/workflows/release.yml`
- Done when: release.yml is valid YAML with correct triggers, matrix, artifact handling, and release creation; cargo fmt, clippy, and test pass locally
- **COMMIT POINT**: `ws2: add release workflow for cross-platform binaries`

### Work stream 3: Logo/icon preparation

**Task ws3-t1: Create assets/logo directory structure and naming convention docs**
- Create `assets/logo/` directory with `.gitkeep` placeholder
- Create `assets/logo/README.md` documenting naming convention (`logo-<size>.png`, `logo.svg`) and expected sizes (256x256, 512x512)
- NOTE: Actual icon files will be provided by the user at implementation time -- this task only creates the directory and documents the convention
- Files: `assets/logo/.gitkeep`, `assets/logo/README.md`
- Done when: directory exists with .gitkeep and README documenting convention; cargo fmt, clippy, and test pass
- Committed together with the README task below

### Work stream 4: README

**Task ws4-t1: Write the full project README**
- Replace current README.md with complete 9-section README:
  1. Project name + tagline
  2. Centered logo image tag (referencing assets/logo/logo-256.png, with pending-icon HTML comment)
  3. CI status badge (links to ci.yml) and latest release badge (links to releases)
  4. "What is namer?" (2-4 sentences)
  5. Installation (subsections: macOS Apple Silicon, macOS Intel, Linux x86-64, Windows x86-64, build from source)
  6. Usage (examples: plain, --lower, --delimiter, combined, --help output)
  7. Development
  8. Contributing (pointer to docs/CONTRIBUTING.md)
  9. License
- All CLI examples must match actual --help flag names
- NOTE: User will provide a reference README from the tasksync project at implementation time -- use as quality/style reference, not verbatim template
- Files: `README.md`
- Done when: all 9 sections present, CLI examples correct, badges link to real URLs, logo uses relative path, no TODO/Coming soon placeholders (pending-icon note acceptable); cargo fmt, clippy, and test pass
- **COMMIT POINT** (with ws3-t1): `ws3+ws4: add logo directory structure and full README`

### Summary

| Task | Work stream | Commit point |
|------|-------------|-------------|
| ws1-t1: Verify cross-platform build targets | 1 - Binaries | Yes |
| ws2-t1: Create PR gate CI workflow | 2 - CI | Yes |
| ws2-t2: Create release workflow | 2 - CI | Yes |
| ws3-t1: Create logo directory structure | 3 - Logo | No (with ws4-t1) |
| ws4-t1: Write full project README | 4 - README | Yes (includes ws3-t1) |

Total: 5 tasks, 4 commit points

## Progress log

- 2026-03-23: Exec plan created. Requirements gathered for all four work streams. Icon file and tasksync README reference both marked as pending — user will supply at implementation time.

## Decision log

- 2026-03-23: Four work streams scoped into a single exec plan on one branch, to be implemented sequentially.
- 2026-03-23: Release asset naming convention set to `namer-<target-triple>` for clarity and unambiguity.
- 2026-03-23: No package-registry publishing (crates.io, Homebrew, apt) in scope — GitHub Releases only.
- 2026-03-23: Platform-specific icon formats (.icns, .ico) excluded; PNG and optional SVG only.
- 2026-03-23: Release workflow restricted to built-in GITHUB_TOKEN only — no additional secrets required.

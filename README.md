# namer

> Generate a random, memorable name in an instant — ALL CAPS by default, configurable to your needs.

<p align="center">
  <img src="assets/logo/logo-256.png" alt="namer logo" width="128">
</p>

[![CI](https://github.com/dtammam/namer/actions/workflows/ci.yml/badge.svg)](https://github.com/dtammam/namer/actions/workflows/ci.yml)
[![Latest Release](https://img.shields.io/github/v/release/dtammam/namer)](https://github.com/dtammam/namer/releases/latest)

## What is namer?

namer is a command-line tool that generates random two-word names by combining an adjective with a noun. By default, output is ALL CAPS with no separator — great for project names, environment identifiers, or anywhere you need a quick, human-readable handle. You can configure both the casing and the separator between words using flags.

## Installation

Paste the snippet for your platform — it checks whether `namer` is already installed and skips the download if so. No dependencies required.

### macOS (Apple Silicon)

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

### macOS (Intel)

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

### Linux (x86-64)

```sh
which namer || (  # Skip if already installed
  mkdir -p ~/.local/bin &&           # Create user bin directory if absent
  curl -L https://github.com/dtammam/namer/releases/latest/download/namer-x86_64-unknown-linux-gnu \
    -o ~/.local/bin/namer &&         # Download the binary
  chmod +x ~/.local/bin/namer        # Make it executable
) && ~/.local/bin/namer  # Generate a name
```

### Windows (x86-64)

```powershell
if (-not (Get-Command namer -ErrorAction SilentlyContinue)) { Invoke-WebRequest -Uri https://github.com/dtammam/namer/releases/latest/download/namer-x86_64-pc-windows-msvc.exe -OutFile namer.exe }; .\namer.exe
```

This downloads `namer.exe` to your current directory and runs it from there.

### Build from source

Choose this path if you want the latest unreleased code or plan to contribute to the project.

Requires [Rust stable](https://rustup.rs).

```sh
git clone https://github.com/dtammam/namer.git
cd namer
cargo build --release
./target/release/namer
```

## Usage

The examples below cover every flag namer supports — start with the simplest invocation and combine flags as needed.

**Plain invocation** — ALL CAPS, no separator:

```console
$ namer
BOLDFALCON
```

**`--lower`** — lowercase output:

```console
$ namer --lower
boldfalcon
```

**`--delimiter`** — insert a separator between words:

```console
$ namer --delimiter -
BOLD-FALCON
```

**Combined flags** — lowercase with a custom separator:

```console
$ namer --lower --delimiter _
bold_falcon
```

**`--help`** — full flag reference:

```console
$ namer --help
Generates a random name from a curated list of adjectives and nouns.

By default the output is ALL CAPS with no delimiter between words. Use `--lower` for lowercase output and `--delimiter` to insert a separator. Use `--number` to generate multiple names at once (up to 1000).

Usage: namer [OPTIONS]

Options:
      --lower
          Output the name in lowercase instead of the default ALL CAPS

      --delimiter <DELIMITER>
          String placed between words in the output. Defaults to no separator

          [default: ""]

      --number <NUMBER>
          Number of names to generate (1-1000)

          [default: 1]

  -h, --help
          Print help (see a summary with '-h')
```

## Development

After cloning the repo, here is how to build, test, and lint locally.

**Build** — compile an optimized binary; run this when verifying performance or preparing a release:

```sh
cargo build --release
```

**Test** — run the full test suite; do this before opening a pull request to catch regressions:

```sh
cargo test
```

**Lint** — run Clippy with warnings as errors; fix any issues it flags before committing:

```sh
cargo clippy -- -D warnings
```

**Format check** — verify code is formatted consistently; the CI gate requires this to pass:

```sh
cargo fmt -- --check
```

## Contributing

Contributions are welcome! Read [docs/CONTRIBUTING.md](docs/CONTRIBUTING.md) for design principles and coding standards. Before opening a pull request, make sure `cargo fmt -- --check`, `cargo clippy -- -D warnings`, and `cargo test` all pass.

## License

[MIT](LICENSE)

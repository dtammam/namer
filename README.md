# namer

> Generate a random, memorable name in an instant — ALL CAPS by default, configurable to your needs.

<!-- Icon pending: drop logo-256.png into assets/logo/ to display the project logo -->
<p align="center">
  <img src="assets/logo/logo-256.png" alt="namer logo" width="128">
</p>

[![CI](https://github.com/dtammam/namer/actions/workflows/ci.yml/badge.svg)](https://github.com/dtammam/namer/actions/workflows/ci.yml)
[![Latest Release](https://img.shields.io/github/v/release/dtammam/namer)](https://github.com/dtammam/namer/releases/latest)

## What is namer?

namer is a command-line tool that generates random two-word names by combining an adjective with a noun. Output is ALL CAPS with no separator by default — ideal for project names, environment identifiers, or any context where you need a quick, human-readable handle. Both the casing and the separator between words are configurable via flags.

## Installation

### macOS (Apple Silicon)

```sh
curl -L https://github.com/dtammam/namer/releases/latest/download/namer-aarch64-apple-darwin -o namer
chmod +x namer
./namer
```

### macOS (Intel)

```sh
curl -L https://github.com/dtammam/namer/releases/latest/download/namer-x86_64-apple-darwin -o namer
chmod +x namer
./namer
```

### Linux (x86-64)

```sh
curl -L https://github.com/dtammam/namer/releases/latest/download/namer-x86_64-unknown-linux-gnu -o namer
chmod +x namer
./namer
```

### Windows (x86-64)

```powershell
Invoke-WebRequest -Uri https://github.com/dtammam/namer/releases/latest/download/namer-x86_64-pc-windows-msvc.exe -OutFile namer.exe
.\namer.exe
```

### Build from source

Requires [Rust stable](https://rustup.rs).

```sh
git clone https://github.com/dtammam/namer.git
cd namer
cargo build --release
./target/release/namer
```

## Usage

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

By default the output is ALL CAPS with no delimiter between words. Use `--lower` for lowercase output and `--delimiter` to insert a separator.

Usage: namer [OPTIONS]

Options:
      --lower
          Output the name in lowercase instead of the default ALL CAPS

      --delimiter <DELIMITER>
          String placed between words in the output. Defaults to no separator

  -h, --help
          Print help (see a summary with '-h')
```

## Development

Build the project:

```sh
cargo build --release
```

Run the test suite:

```sh
cargo test
```

Run the linter:

```sh
cargo clippy -- -D warnings
```

Check formatting:

```sh
cargo fmt -- --check
```

## Contributing

See [docs/CONTRIBUTING.md](docs/CONTRIBUTING.md) for design principles and coding standards. All contributions must pass `cargo fmt -- --check`, `cargo clippy -- -D warnings`, and `cargo test` before merging.

## License

[MIT](LICENSE)

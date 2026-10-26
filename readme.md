# Typie

A terminal typing game built while learning Rust.

Choose a session length, type random words, and get instant character
feedback. Check your score, time, and WPM after each session.

## Tech stack

Rust, Crossterm, and Rand.

Game logic and terminal rendering are separated into modules, with unit
tests covering typing, scoring, backspace, and word progression.

## Download

Download the Windows version from [Releases](https://github.com/wojpo/typie/releases),
extract the ZIP, and run `typie.exe`.

The word list is embedded in the executable, so no additional files are needed.

## Getting started

Install [Rust](https://rustup.rs/), then clone the repository:

```bash
git clone https://github.com/wojpo/typie.git
cd typie
cargo run --release
```

## Build

```bash
cargo build --release
```

The executable is available in `target/release`.

## Development

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```
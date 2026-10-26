# Typie

A terminal typing game built with Rust and Crossterm.

Practice typing with word-count sessions, instant character feedback,
and speed statistics.

## Features

- Preset sessions: 10, 20, or 50 words
- Custom sessions: 1–1000 words
- Green/red character feedback
- Backspace support
- Upcoming word preview
- Session time and WPM results
- Retry and return to the main menu
- Animated headers and terminal resize handling

## Download

Download the Windows version from [Releases](https://github.com/wojpo/typie/releases),
extract the ZIP, and run `typie.exe`.


## Run from source

Install [Rust](https://rustup.rs/), then run:

```sh
git clone https://github.com/wojpo/typie.git
cd typie
cargo run --release
```

## Development

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```
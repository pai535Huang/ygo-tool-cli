# ygo-tool-cli

A simple CLI tool to search Yu-Gi-Oh! cards and render their images directly in the terminal via ygocdb API.

## Features
- Search card info (Password, Type, Race, Attribute, Level/Rank, Effects).
- Native terminal image rendering (Kitty / Sixel protocols).
- Fallback to URL output if protocols are unsupported.
- Fetches HD official SC/JP card arts.

## Dependencies
- [Rust / Cargo](https://rustup.rs/) (to build/install)
- Terminal with Kitty or Sixel graphics protocol support (for `img` command)

## Usage

### Search Card Info
```bash
cargo run -- search "Blue-Eyes White Dragon"
# or
cargo run -- search 89631139
```

### Render Card Image
```bash
cargo run -- img "Ash Blossom"
```

## Installation

Install globally via cargo:
```bash
cargo install --path .
```
Then use it directly anywhere:
```bash
ygotool search "Dark Magician"
ygotool img "Dark Magician"
```
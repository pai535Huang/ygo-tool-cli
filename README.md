# ygo-tool-cli

A simple CLI tool to search Yu-Gi-Oh! cards and render their images directly in the terminal via [ygocdb](https://ygocdb.com/) API.

## Features
- Search card info (Password, Type, Race, Attribute, Level/Rank, Effects).
- Native terminal image rendering (Kitty / Sixel protocols).
- Fallback to URL output if protocols are unsupported.
- Fetches HD official SC/JP card arts.

## Dependencies
- [Rust / Cargo](https://rustup.rs/) (to build/install)
- [fzf](https://github.com/junegunn/fzf) (for interactive search)
- Terminal with Kitty or Sixel graphics protocol support (for `img` command)

## Usage

Install globally via cargo:
```bash
cargo install --path .
```
Then use it directly anywhere:

### Interactive search
```bash
ygotool
```
This mode invokes `fzf` to search cards interactively.

### Search Card Info
```bash
ygotool search "Dark Magician"
```
As the API supports fuzzy search, this mode prints the first search result from the API feedback if multiple results are found.

### Render Card Image
```bash
ygotool img "Dark Magician"
```
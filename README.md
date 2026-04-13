# YGOTOOL

A simple CLI tool to search Yu-Gi-Oh! cards and render their images directly in the terminal via [ygocdb](https://ygocdb.com/) API.

## Features
- Search card info (Type, Race, Attribute, Level/Rank, Effects).
- Native terminal image rendering (Kitty protocol).
- Fallback to URL output if protocols are unsupported.
- Fetches HD official SC/JP card arts.

## Dependencies
- [Rust / Cargo](https://rustup.rs/) (to build/install)
- [fzf](https://github.com/junegunn/fzf) (for interactive search)
- Terminal with Kitty graphics protocol support (for `img` command)

## Usage

Directly run the binary.
```bash
chmod +x ./ygotool
```
then
```bash
./ygotool
```
If the directory is added into `$PATH` environment variable, simply invoke
```bash
ygotool
```

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

## Manual Installation

Simply run 
```bash
cargo install --path .
```
and then use it anywhere. 
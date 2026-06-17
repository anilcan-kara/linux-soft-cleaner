# Linux Soft Cleaner

A fast, safe, and minimal disk cleanup tool for Linux servers — written in Rust.

## Features

- **Safe by default** — dry-run mode shows what would be deleted before touching anything
- **Fast** — written in Rust for maximum I/O performance
- **Zero dependencies** — single static binary, no runtime requirements
- **Targeted cleanup** — `/tmp` files, package manager caches, old log archives

## Usage

```bash
# Dry-run (safe preview)
linux-soft-cleaner --dry-run

# Actual cleanup
sudo linux-soft-cleaner

# Set age threshold (days)
linux-soft-cleaner --dry-run --days 14
```

## What gets cleaned?

| Target | Description |
|---|---|
| `/tmp` | Temporary files older than N days |
| `/var/cache/apt/archives` | APT package cache |
| `/var/cache/dnf` | DNF/YUM package cache |

## Installation

### From source

```bash
cargo install linux-soft-cleaner
```

## License

MIT — Anilcan Kara

# Linux Soft Cleaner

<p align="center">
  <img src="assets/banner.png" alt="Linux Soft Cleaner Banner" width="100%">
</p>

A fast, safe, and minimal disk cleanup tool for Linux servers — written in Rust for maximum performance, memory safety, and power efficiency.

## Features

- **Safe by default** — dry-run mode (`--dry-run` / `-n`) shows what would be deleted before touching anything.
- **Blazingly Fast** — leveraging Rust's zero-cost abstractions and concurrent file system traversal.
- **Zero dependencies** — single static binary with no external library requirements.
- **Comprehensive Scans**:
  - `/tmp` files (using configurable age filter).
  - Package manager caches (APT, YUM, DNF, Pacman).
  - Rotated log archives under `/var/log`.
  - Docker caches (`--docker` flag to prune images, containers, and build cache).
  - NPM global cache (`--npm` flag).

---

## Installation & Usage

### 1. Run instantly via npx (Node.js required)
You can execute it instantly on any machine with Node.js installed without installing the package:
```bash
# Preview cleanable space (safe dry-run)
npx linux-soft-cleaner --dry-run

# Run actual cleanup
sudo npx linux-soft-cleaner
```

### 2. Install globally via NPM
```bash
npm install -g linux-soft-cleaner

# Use the command directly
linux-soft-cleaner --dry-run
```

### 3. Install via curl shell script (Linux/macOS)
Installs the precompiled binary directly into your path:
```bash
curl -fsSL https://raw.githubusercontent.com/anilcan-kara/linux-soft-cleaner/master/install.sh | sh
```

### 4. Install from source via Cargo (Rust toolchain required)
```bash
cargo install linux-soft-cleaner
```

### 5. Download statically linked binaries
Get the precompiled static binaries for your system directly from the [GitHub Releases Page](https://github.com/anilcan-kara/linux-soft-cleaner/releases).

---

## Command Line Options

```bash
=== Linux Soft Cleaner v0.1.0 ===
A fast, safe, and minimal disk cleanup tool for Linux servers

Usage: linux-soft-cleaner [OPTIONS]

Options:
  -n, --dry-run        Run in dry-run mode (preview changes, no files will be deleted)
  -d, --days <DAYS>    Threshold in days for temporary files and logs [default: 7]
      --docker         Also clean Docker system caches (containers, images, volumes)
      --npm            Also clean global NPM cache
  -h, --help           Print help
  -V, --version        Print version
```

## License

MIT — Anilcan Kara

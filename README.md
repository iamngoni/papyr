# Papyr

Cross-platform document scanning library for Flutter/Dart, powered by Rust.

## Project Structure

```
papyr/
├── papyr/           # Dart/Flutter package (FFI bindings)
└── papyr_core/      # Rust core library (scanner backends)
```

## Components

### 📱 `papyr/` - Dart Package

Flutter package that provides a high-level API for document scanning.

- **Language**: Dart/Flutter
- **API**: Easy-to-use async scanner interface
- **Integration**: Uses FFI to call Rust core

**Status**: In Development

### ⚙️ `papyr_core/` - Rust Core

Low-level scanning engine with support for multiple protocols.

- **Language**: Rust
- **Backends**: 
  - eSCL/AirScan (network, all platforms)
  - WIA (Windows)
  - ICA (macOS)
  - SANE (Linux)
- **FFI**: C-compatible interface for Dart integration

**Status**: ✅ Core implementation complete

## Quick Start

### For Dart Developers

```bash
cd papyr
flutter pub get
# Use the Papyr API in your Flutter app
```

### For Rust Developers

```bash
cd papyr_core

# Build the library
cargo build --release

# Run tests
cargo test

# Test scanner discovery
cargo run --bin test_scanner
```

## Platform Support

| Platform | Backends | Status |
|----------|----------|--------|
| Windows  | WIA, eSCL | ✅ Ready |
| macOS    | ICA, eSCL | ✅ Ready |
| Linux    | SANE, eSCL | ✅ Ready |

## Features

- 🔍 **Auto-discovery** - Automatically find USB and network scanners
- 🌐 **Network scanning** - eSCL/AirScan support (driverless)
- 📄 **Multi-page** - ADF and duplex scanning
- 🎨 **Multiple modes** - Color, grayscale, black & white
- ⚡ **High performance** - Rust-powered core
- 🛡️ **Memory safe** - No memory leaks or crashes

## Development

### CI/CD

The project includes automated workflows:

- **Rust CI** - Runs on every commit to `papyr_core/`
  - Format checks
  - Linter (Clippy)
  - Tests on Linux, macOS, Windows
  - Documentation builds

- **Rust Release** - Triggered by tags like `core-v0.1.0`
  - Builds optimized binaries for all platforms
  - Creates GitHub releases
  - Uploads `.dll`, `.dylib`, `.so` files

- **Dart CI** - Runs on commits to `papyr/`
  - Package validation
  - Spell checking
  - Semantic PR checks

### Building from Source

#### Prerequisites

**Rust:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**Platform-specific:**
- **Linux**: `sudo apt-get install libsane-dev`
- **macOS**: Xcode Command Line Tools
- **Windows**: Visual Studio Build Tools

#### Build Steps

```bash
# Clone the repo
git clone https://github.com/yourusername/papyr.git
cd papyr

# Build Rust core
cd papyr_core
cargo build --release

# Build Dart package
cd ../papyr
flutter pub get
```

## Testing

### Without Hardware

```bash
# Rust tests (no scanner needed)
cd papyr_core
cargo test

# Scanner discovery test (shows "no scanners found")
cargo run --bin test_scanner
```

### With Hardware

Connect a scanner and run the test binary:

```bash
cd papyr_core
cargo run --bin test_scanner
```

Expected output:
```
🔍 Papyr Core - Scanner Backend Test

📡 Discovering scanners...

✅ Found 1 scanner(s):

  1. HP LaserJet Pro MFP (escl_192_168_1_100)
     Backend: Escl
     ...
```

## Documentation

- **Rust Core**: See [`papyr_core/README.md`](papyr_core/README.md)
- **Dart Package**: See [`papyr/README.md`](papyr/README.md)
- **API Docs**: Run `cargo doc --open` in `papyr_core/`

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Ensure tests pass: `cargo test` and `cargo fmt --all --check`
5. Submit a pull request

## License

Copyright (c) 2025 Codecraft Solutions. All rights reserved.

## Roadmap

- [x] Rust core implementation
- [x] eSCL backend (network scanning)
- [x] SANE backend (Linux)
- [x] WIA backend (Windows stub)
- [x] ICA backend (macOS)
- [x] CI/CD pipeline
- [ ] Dart FFI bindings
- [ ] Flutter UI components
- [ ] Image processing (crop, rotate)
- [ ] PDF generation
- [ ] OCR integration

## Support

- **Issues**: [GitHub Issues](https://github.com/yourusername/papyr/issues)
- **Discussions**: [GitHub Discussions](https://github.com/yourusername/papyr/discussions)

---

**Built with ❤️ using Rust and Dart**

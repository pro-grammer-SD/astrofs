# AstroFS Build Scripts

This directory contains cross-platform build and installation scripts for AstroFS Python bindings.

## Scripts Overview

### 🔨 `build_bindings.ps1` (Windows PowerShell)

Build AstroFS wheels on Windows with automatic type stub generation.

**Usage:**
```powershell
# Basic build
.\build_bindings.ps1

# Build to custom directory
.\build_bindings.ps1 -OutputDir build

# Debug build
.\build_bindings.ps1 -Release $false

# Skip type stub generation
.\build_bindings.ps1 -GenerateStubs $false
```

**Features:**
- Automatic dependency checking
- Environment variable setup for Python 3.10-3.14 compatibility
- Type stub generation included
- Colored output with progress indicators
- File listing after build

**Requirements:**
- PowerShell 5.1+ (built into Windows)
- maturin: `pip install maturin`
- Python 3.10+
- Rust 1.70+

---

### 🔨 `build_bindings.sh` (Linux/macOS)

Build AstroFS wheels on Unix-like systems with automatic type stub generation.

**Usage:**
```bash
chmod +x scripts/build_bindings.sh

# Basic build
./build_bindings.sh

# Build to custom directory
./build_bindings.sh -o build

# Debug build
./build_bindings.sh --debug

# Skip type stub generation
./build_bindings.sh --skip-stubs

# Show help
./build_bindings.sh --help
```

**Features:**
- Cross-platform compatibility (Linux, macOS)
- Automatic dependency checking
- Multiple build modes (debug/release)
- Type stub generation
- Progress output with timestamps

**Requirements:**
- bash 4.0+
- maturin: `pip install maturin`
- Python 3.10+
- Rust 1.70+

---

### 🐍 `build_helper.py` (Cross-Platform Python)

Unified Python-based build helper for all platforms.

**Usage:**
```bash
python scripts/build_helper.py

# Build to custom directory
python scripts/build_helper.py -o build

# Debug build
python scripts/build_helper.py --debug

# Skip type stub generation
python scripts/build_helper.py --skip-stubs

# Skip dependency check
python scripts/build_helper.py --skip-deps-check

# Show help
python scripts/build_helper.py --help
```

**Features:**
- Single script works on Windows, Linux, macOS
- Automatic platform detection
- Dependency validation
- Type stub generation
- Detailed build output

**Requirements:**
- Python 3.7+
- maturin: `pip install maturin`
- Rust 1.70+

---

### 📦 `install.py` (Cross-Platform Installation)

Simple installation helper for AstroFS Python bindings.

**Usage:**
```bash
# Install from local wheels (default)
python scripts/install.py

# Install from specific wheel directory
python scripts/install.py --wheel-dir /path/to/wheels

# Install from PyPI
python scripts/install.py --pypi

# Install specific version from PyPI
python scripts/install.py --pypi --version 0.1.0

# Build and install from source
python scripts/install.py --source

# Install for current user only
python scripts/install.py --user
```

**Features:**
- Auto-detection of wheel directory
- PyPI installation support
- Source building and installation
- Platform-aware build script detection
- Helpful next steps after installation

**Requirements:**
- Python 3.7+
- pip (usually included with Python)

---

## Quick Start Guide

### Windows

```powershell
# 1. Build
.\scripts\build_bindings.ps1 -Release

# 2. Install
python scripts\install.py

# 3. Verify
python -c "from pyastrofs import PyAstroFS; print('✓ Ready!')"
```

### Linux/macOS

```bash
# 1. Build
chmod +x scripts/build_bindings.sh
./scripts/build_bindings.sh --release

# 2. Install
python scripts/install.py

# 3. Verify
python -c "from astrofs import PyAstroFS; print('✓ Ready!')"
```

### All Platforms

```bash
# 1. Build (unified Python script)
python scripts/build_helper.py

# 2. Install
python scripts/install.py

# 3. Run examples
python examples_python_bindings.py
```

---

## Output Directory Structure

All build scripts output wheels and artifacts to the `whl/` directory:

```
whl/
├── astrofs-0.1.1-cp310-abi3-linux_x86_64.whl
├── astrofs-0.1.1-cp310-abi3-macosx_10_7_x86_64.whl
├── astrofs-0.1.1-cp310-abi3-win_amd64.whl
├── astrofs-0.1.1.tar.gz
└── pyastrofs.pyi
```

The `pyastrofs.pyi` file contains type stubs for IDE support and type checking.

---

## Build Options

### Release vs. Debug Builds

**Release mode** (default):
- Optimized for production
- Larger file size (~2-3 MB)
- Better runtime performance
- Recommended for distribution

**Debug mode**:
- Unoptimized for faster compilation
- Smaller file size
- Better for development and debugging

### Type Stub Generation

Type stubs (`pyastrofs.pyi`) provide:
- IDE autocomplete support
- Type checking with mypy
- Better developer experience

Generated automatically by all scripts unless disabled.

---

## Environment Variables

### Build Environment

The scripts automatically set:

```bash
export PYO3_USE_ABI3_FORWARD_COMPATIBILITY=1
```

This enables forward compatibility with Python 3.10-3.14 using the stable ABI.

### Custom Configuration

You can override settings:

```bash
# Use a specific Rust toolchain
export RUSTFLAGS="-C target-cpu=native"

# Set custom output directory
export CARGO_TARGET_DIR=./my_target
```

---

## Troubleshooting

### maturin not found

```bash
pip install --upgrade maturin
```

### Rust not installed

```bash
# Linux/macOS/Windows (WSL)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows (native)
# Download from https://rustup.rs/
```

### Python version issues

Ensure Python 3.10+ is installed:

```bash
python --version
# Should show Python 3.10.x or higher
```

### Permission denied (Linux/macOS)

```bash
chmod +x scripts/build_bindings.sh
```

### Build directory issues

Clean and rebuild:

```bash
rm -rf target/
python scripts/build_helper.py --release
```

---

## Supported Python Versions

All build scripts support and create wheels for:

- Python 3.10
- Python 3.11
- Python 3.12
- Python 3.13
- Python 3.14

Using the stable ABI (abi3), a single wheel works across multiple Python versions.

---

## Platform Support

All scripts are tested and supported on:

| Platform | PowerShell | Bash | Python | Recommended |
|----------|-----------|------|--------|-------------|
| **Windows** | ✅ | ⚠️ (WSL) | ✅ | PowerShell |
| **Linux** | ⚠️ (pwsh) | ✅ | ✅ | Bash |
| **macOS** | ⚠️ (pwsh) | ✅ | ✅ | Bash |

✅ = Fully supported  
⚠️ = Works with additional setup

---

## Contributing

When modifying scripts:

1. Test on all supported platforms
2. Maintain backward compatibility
3. Update this README
4. Document new options with `--help`

---

## Related Documentation

- [PYTHON_BINDINGS.md](../PYTHON_BINDINGS.md) - Python bindings documentation
- [README.md](../README.md) - Main project README
- [examples_python_bindings.py](../examples_python_bindings.py) - Usage examples

---

**Made with ❤️ for cross-platform compatibility**

# 🚀 Quick Start Guide

## Prerequisites

Make sure you have Rust installed. If not:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Building and Running

### 1. Navigate to the project directory

```bash
cd astrofs
```

### 2. Build the project (first time)

```bash
# This will download dependencies and compile
# Takes a few minutes on first build
cargo build --release
```

### 3. Run the application

```bash
cargo run --release
```

Or directly run the compiled binary:

```bash
./target/release/astrofs
```

## First Steps

1. **Navigate**: Use `↑/↓` or `j/k` to move through files
2. **Open folders**: Press `Enter` on a directory
3. **Go back**: Press `Backspace` or `h`
4. **Search**: Press `/` and start typing
5. **Toggle hidden files**: Press `.`
6. **Get help**: Press `?`
7. **Quit**: Press `q`

## Troubleshooting

### Build errors

If you encounter build errors, try:

```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

### Terminal issues

If the UI looks broken:

- Make sure your terminal supports colors and Unicode
- Try a different terminal (recommended: iTerm2, Windows Terminal, Alacritty)
- Increase terminal size (minimum 80x24 recommended)

### Performance issues

For large directories:

- The first load might be slow
- Search results are cached for better performance
- Use `F5` to refresh if data seems stale

## Performance Tips

- **Release mode**: Always use `--release` for best performance
- **Large directories**: Search is parallelized and very fast
- **Git repos**: Git status updates automatically when changing directories

## Next Steps

- Read the full [README.md](README.md) for all features
- Check the keyboard shortcuts with `?` in the app
- Customize colors in `src/theme.rs`
- Contribute features you'd like to see!

---

Happy exploring! 🎉

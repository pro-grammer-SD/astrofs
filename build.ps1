Write-Host "🦀 Building AstroFS..."
Write-Host ""

if (-not (Get-Command cargo -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Cargo not found. Please install Rust from https://rustup.rs/"
    exit 1
}

Write-Host "✓ Rust/Cargo found"
Write-Host ""

Write-Host "📦 Compiling in release mode (this may take a few minutes)..."
cargo build --release

if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "✅ Build successful!"
    Write-Host ""

    if (-not (Test-Path -Path ".\dist")) {
        New-Item -ItemType Directory -Path ".\dist" | Out-Null
    }

    Copy-Item -Path ".\target\release\astrofs.exe" -Destination ".\dist\" -Force

    Write-Host "📂 Binary copied to .\dist\astrofs.exe"
    Write-Host ""
    Write-Host "🚀 Run the application with:"
    Write-Host "   .\dist\astrofs.exe"
    Write-Host ""
    Write-Host "Or use:"
    Write-Host "   cargo run --release"
    Write-Host ""
} else {
    Write-Host ""
    Write-Host "❌ Build failed. Check the errors above."
    exit 1
}

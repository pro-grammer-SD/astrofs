#!/usr/bin/env python3
"""
Installation helper for AstroFS Python bindings.

Simplifies installation from built wheels or PyPI.
"""

import argparse
import subprocess
import sys
from pathlib import Path


def install_from_wheel(wheel_dir="whl"):
    """Install from local wheel directory."""
    wheel_path = Path(wheel_dir)
    
    if not wheel_path.exists():
        print(f"✗ Wheel directory not found: {wheel_dir}")
        return False
    
    wheels = list(wheel_path.glob("*.whl"))
    
    if not wheels:
        print(f"✗ No wheels found in {wheel_dir}")
        return False
    
    # Sort by modification time, use latest
    latest_wheel = sorted(wheels, key=lambda x: x.stat().st_mtime)[-1]
    
    print(f"📦 Installing from: {latest_wheel}")
    
    try:
        result = subprocess.run(
            [sys.executable, "-m", "pip", "install", str(latest_wheel)],
            check=True
        )
        print("✅ Installation successful!")
        return True
    except subprocess.CalledProcessError as e:
        print(f"✗ Installation failed: {e}")
        return False


def install_from_pypi(version=None):
    """Install from PyPI."""
    cmd = [sys.executable, "-m", "pip", "install", "astrofs"]
    
    if version:
        cmd[-1] = f"astrofs=={version}"
    
    print(f"📦 Installing from PyPI: {cmd[-1]}")
    
    try:
        result = subprocess.run(cmd, check=True)
        print("✅ Installation successful!")
        return True
    except subprocess.CalledProcessError as e:
        print(f"✗ Installation failed: {e}")
        return False


def install_from_source():
    """Build and install from source."""
    print("🔨 Building from source...")
    
    import platform
    
    # Check platform and run appropriate build script
    system = platform.system()
    
    if system == "Windows":
        # Try PowerShell script
        script = Path("scripts/build_bindings.ps1")
        if not script.exists():
            print("✗ Build script not found: scripts/build_bindings.ps1")
            return False
        
        print("Running PowerShell build script...")
        try:
            result = subprocess.run(
                ["powershell", "-ExecutionPolicy", "RemoteSigned", 
                 "-File", str(script)],
                check=True
            )
        except subprocess.CalledProcessError as e:
            print(f"✗ Build failed: {e}")
            return False
    else:
        # Try bash script
        script = Path("scripts/build_bindings.sh")
        if not script.exists():
            print("✗ Build script not found: scripts/build_bindings.sh")
            return False
        
        print("Running bash build script...")
        try:
            result = subprocess.run(
                ["bash", str(script), "--release"],
                check=True
            )
        except subprocess.CalledProcessError as e:
            print(f"✗ Build failed: {e}")
            return False
    
    # Install from built wheel
    return install_from_wheel()


def main():
    """Main entry point."""
    parser = argparse.ArgumentParser(
        description="Install AstroFS Python bindings",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python scripts/install.py                    # Install from whl/
  python scripts/install.py --pypi             # Install from PyPI
  python scripts/install.py --source           # Build and install from source
  python scripts/install.py --pypi --version 0.1.0  # Install specific version
        """
    )
    
    source_group = parser.add_mutually_exclusive_group()
    source_group.add_argument(
        "--wheel",
        action="store_true",
        default=True,
        help="Install from local wheels (default)"
    )
    source_group.add_argument(
        "--wheel-dir",
        type=str,
        metavar="DIR",
        help="Custom wheel directory"
    )
    source_group.add_argument(
        "--pypi",
        action="store_true",
        help="Install from PyPI"
    )
    source_group.add_argument(
        "--source",
        action="store_true",
        help="Build and install from source"
    )
    
    parser.add_argument(
        "--version",
        type=str,
        help="Specific version to install (with --pypi)"
    )
    parser.add_argument(
        "--user",
        action="store_true",
        help="Install for current user only"
    )
    
    args = parser.parse_args()
    
    print("╔" + "=" * 60 + "╗")
    print("║" + " AstroFS Python Bindings - Installation Helper ".center(60) + "║")
    print("╚" + "=" * 60 + "╝")
    print()
    
    if args.wheel_dir:
        success = install_from_wheel(args.wheel_dir)
    elif args.pypi:
        success = install_from_pypi(args.version)
    elif args.source:
        success = install_from_source()
    else:
        success = install_from_wheel()
    
    if not success:
        sys.exit(1)
    
    print()
    print("Next steps:")
    print("  • Import: python -c 'from astrofs import PyAstroFS; print(\"✓ Ready!\")'")
    print("  • Examples: python examples_python_bindings.py")
    print("  • Docs: https://github.com/pro-grammer-SD/astrofs/blob/main/PYTHON_BINDINGS.md")
    print()


if __name__ == "__main__":
    main()

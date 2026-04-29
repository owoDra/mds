#!/usr/bin/env python3
"""vendor_binary.py — Copy the native mds binary into the vendor/ directory
for packaging with the Python mds-cli wrapper.

Usage:
    python scripts/vendor_binary.py [--source PATH]

If --source is not given, attempts to find the binary from:
    1. ../../crates/target/release/mds
    2. `which mds` on PATH
"""

from __future__ import annotations

import argparse
import os
import platform
import shutil
import stat
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def find_binary(explicit: str | None) -> Path:
    if explicit:
        p = Path(explicit)
        if not p.exists():
            print(f"error: specified binary not found: {explicit}", file=sys.stderr)
            sys.exit(1)
        return p.resolve()

    # Try workspace release build
    release = ROOT.parent / "crates" / "target" / "release" / "mds"
    if release.exists():
        return release

    # Try PATH
    which = shutil.which("mds")
    if which:
        return Path(which).resolve()

    print(
        "error: native mds binary not found. "
        "Build with `cd crates && cargo build --release` first.",
        file=sys.stderr,
    )
    sys.exit(1)


def main() -> None:
    parser = argparse.ArgumentParser(description="Vendor native mds binary")
    parser.add_argument("--source", help="Path to mds binary")
    args = parser.parse_args()

    binary = find_binary(args.source)
    vendor_dir = ROOT / "mds_cli" / "mds_cli" / "vendor"
    vendor_dir.mkdir(parents=True, exist_ok=True)

    dest = vendor_dir / "mds"
    shutil.copy2(binary, dest)
    dest.chmod(dest.stat().st_mode | stat.S_IEXEC | stat.S_IXGRP | stat.S_IXOTH)
    print(f"copied {binary} -> {dest}")


if __name__ == "__main__":
    main()

from __future__ import annotations

import os
import shutil
import subprocess
import sys
from pathlib import Path


def _candidates() -> list[str]:
    here = Path(__file__).resolve().parent
    values = [
        str(here / "mds"),
        str(here.parent / "vendor" / "mds"),
        os.environ.get("MDS_NATIVE_BIN"),
        shutil.which("mds"),
    ]
    return [value for value in values if value]


def main() -> int:
    for candidate in _candidates():
        path = Path(candidate)
        if path.name == "mds" and path.resolve() == Path(sys.argv[0]).resolve():
            continue
        if not path.exists() and os.sep in candidate:
            continue
        try:
            return subprocess.call([candidate, *sys.argv[1:]])
        except FileNotFoundError:
            continue
        except OSError as error:
            print(f"environment error: failed to run native mds: {error}", file=sys.stderr)
            return 4
    print("environment error: native mds binary is not available", file=sys.stderr)
    return 4


if __name__ == "__main__":
    raise SystemExit(main())

#!/usr/bin/env bash
set -euo pipefail

python3 - "$@" <<'PY'
from __future__ import annotations

import argparse
import hashlib
import json
import pathlib
import re
import sys
import tomllib


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(add_help=False)
    parser.add_argument("--manifest", default="release.mds.toml")
    parser.add_argument("--verbose", action="store_true")
    parser.add_argument("-h", "--help", action="store_true")
    args, unknown = parser.parse_known_args()
    if args.help:
        print("usage: ./.github/script/release-check.sh [--manifest <path>] [--verbose]")
        sys.exit(0)
    if unknown:
        joined = " ".join(unknown)
        print(f"error: unknown option(s): {joined}", file=sys.stderr)
        sys.exit(2)
    return args


def sha256_file(path: pathlib.Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(65536), b""):
            digest.update(chunk)
    return digest.hexdigest()


def read_checksum_digest(path: pathlib.Path) -> str | None:
    content = path.read_text(encoding="utf-8")
    for part in content.split():
        if re.fullmatch(r"[0-9a-fA-F]{64}", part):
            return part.lower()
    return None


def verify_json(path: pathlib.Path) -> bool:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except Exception:
        return False
    return isinstance(value, dict) and (
        "bomFormat" in value or "spdxVersion" in value or "SPDXID" in value
    )


def verify_json_or_jsonl(path: pathlib.Path) -> bool:
    content = path.read_text(encoding="utf-8").strip()
    if not content:
        return False
    try:
        json.loads(content)
        return True
    except Exception:
        pass
    for line in content.splitlines():
        line = line.strip()
        if not line:
            continue
        try:
            json.loads(line)
        except Exception:
            return False
    return True


def resolve(base: pathlib.Path, value: str) -> pathlib.Path:
    return (base / value).resolve()


def error(errors: list[str], path: pathlib.Path | None, message: str) -> None:
    if path is None:
        errors.append(message)
    else:
        errors.append(f"{path}: {message}")


def main() -> int:
    args = parse_args()
    manifest_path = pathlib.Path(args.manifest).resolve()
    if not manifest_path.exists():
        print(f"error: manifest not found: {manifest_path}", file=sys.stderr)
        return 1

    try:
        data = tomllib.loads(manifest_path.read_text(encoding="utf-8"))
    except Exception as exc:
        print(f"error: failed to parse {manifest_path}: {exc}", file=sys.stderr)
        return 1

    artifacts = data.get("artifacts")
    if not isinstance(artifacts, list) or not artifacts:
        print(
            f"error: {manifest_path}: release quality manifest requires [[artifacts]] entries",
            file=sys.stderr,
        )
        return 1

    base_dir = manifest_path.parent
    errors: list[str] = []

    print("Release quality gate:")
    for artifact in artifacts:
        name = artifact.get("name", "<unnamed>")
        channel = artifact.get("channel", "unknown")
        print(f"- {name} ({channel})")

        required = ["path", "checksum", "signature", "sbom", "provenance"]
        resolved: dict[str, pathlib.Path] = {}
        for field in required:
            value = artifact.get(field)
            if not isinstance(value, str):
                error(errors, None, f"release artifact `{name}` is missing `{field}`")
                continue
            path = resolve(base_dir, value)
            resolved[field] = path
            if not path.exists():
                error(errors, path, f"release artifact field `{field}` does not exist for `{name}`")

        smoke = artifact.get("smoke")
        if smoke not in (True, "ok"):
            error(errors, None, f"release artifact `{name}` requires successful smoke = true")

        artifact_path = resolved.get("path")
        checksum_path = resolved.get("checksum")
        signature_path = resolved.get("signature")
        sbom_path = resolved.get("sbom")
        provenance_path = resolved.get("provenance")

        if artifact_path and checksum_path and artifact_path.exists() and checksum_path.exists():
            digest = read_checksum_digest(checksum_path)
            if digest is None:
                error(errors, checksum_path, f"checksum for `{name}` must contain a SHA-256 hex digest")
            elif artifact_path.is_file():
                actual = sha256_file(artifact_path)
                if actual != digest:
                    error(
                        errors,
                        checksum_path,
                        f"checksum mismatch for `{name}`: expected {digest}, actual {actual}",
                    )

        if signature_path and signature_path.exists() and signature_path.stat().st_size == 0:
            error(errors, signature_path, f"signature for `{name}` must not be empty")

        if sbom_path and sbom_path.exists() and not verify_json(sbom_path):
            error(errors, sbom_path, f"SBOM for `{name}` must look like SPDX or CycloneDX JSON")

        if provenance_path and provenance_path.exists() and not verify_json_or_jsonl(provenance_path):
            error(errors, provenance_path, f"provenance for `{name}` must be valid JSON or JSONL")

    if errors:
        for line in errors:
            print(f"error: {line}", file=sys.stderr)
        return 1

    print("release quality ok")
    return 0


raise SystemExit(main())
PY
"""mds Python language adapter.

Provides Python-specific file naming conventions, quality tool defaults,
and test runner integration metadata for mds.
"""

from __future__ import annotations

__all__ = [
    "LANG",
    "MD_EXT",
    "OUTPUT_EXTENSIONS",
    "DEFAULT_QUALITY",
    "TOOLS",
    "output_path",
]

LANG = "py"
"""Language identifier used in mds.config.toml [quality.py] sections."""

MD_EXT = ".py.md"
"""File extension for Python implementation markdown."""

OUTPUT_EXTENSIONS: dict[str, str] = {
    "source": ".py",
    "types": ".pyi",
    "test": ".py",  # prefixed with test_
}
"""Default output file extensions by output kind."""

DEFAULT_QUALITY: dict[str, str | list[str]] = {
    "linter": "ruff check",
    "fixer": "ruff format",
    "test_runner": "pytest",
    "required": ["python3", "ruff", "pytest"],
    "optional": [],
}
"""Default quality tool configuration for Python."""

TOOLS: dict[str, list[str]] = {
    "linters": ["ruff check"],
    "fixers": ["ruff format", "black"],
    "test_runners": ["pytest", "unittest"],
}
"""Available quality tools for Python."""


def output_path(md_rel_path: str, kind: str) -> str:
    """Return the expected output path for a given implementation markdown path.

    Args:
        md_rel_path: Markdown-root relative path (e.g. "greet.py.md").
        kind: One of "source", "types", or "test".

    Returns:
        The expected output file path.
    """
    base = md_rel_path.removesuffix(".md").removesuffix(".py")
    parts = base.rsplit("/", 1)
    directory = parts[0] + "/" if len(parts) > 1 else ""
    name = parts[-1]

    if kind == "types":
        return f"{base}.pyi"
    if kind == "test":
        return f"{directory}test_{name}.py"
    return f"{base}.py"

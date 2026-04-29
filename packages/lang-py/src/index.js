/**
 * @mds/lang-py — Python language adapter for mds.
 *
 * Provides Python-specific file naming conventions, quality tool
 * defaults, and test runner integration metadata.
 */

/** Language identifier used in mds.config.toml [quality.py] sections. */
export const LANG = "py";

/** File extension for Python implementation markdown. */
export const MD_EXT = ".py.md";

/** Default output file extensions by output kind. */
export const OUTPUT_EXTENSIONS = {
  source: ".py",
  types: ".pyi",
  test: ".py", // prefixed with test_
};

/** Default quality tool configuration for Python. */
export const DEFAULT_QUALITY = {
  linter: "ruff check",
  fixer: "ruff format",
  test_runner: "pytest",
  required: ["python3", "ruff", "pytest"],
  optional: [],
};

/** Available quality tools for Python. */
export const TOOLS = {
  linters: ["ruff check"],
  fixers: ["ruff format", "black"],
  test_runners: ["pytest", "unittest"],
};

/**
 * Return the expected output path for a given implementation markdown path.
 * @param {string} mdRelPath - Markdown-root relative path (e.g. "greet.py.md")
 * @param {"source"|"types"|"test"} kind
 * @returns {string}
 */
export function outputPath(mdRelPath, kind) {
  const base = mdRelPath.replace(/\.md$/, "").replace(/\.py$/, "");
  const name = base.split("/").pop() || base;
  const dir = base.includes("/") ? base.slice(0, base.lastIndexOf("/") + 1) : "";
  switch (kind) {
    case "types":
      return `${base}.pyi`;
    case "test":
      return `${dir}test_${name}.py`;
    case "source":
    default:
      return `${base}.py`;
  }
}

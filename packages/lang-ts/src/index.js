/**
 * mds-lang-ts — TypeScript language adapter for mds.
 *
 * Provides TypeScript-specific file naming conventions, quality tool
 * defaults, and test runner integration metadata.
 */

/** Language identifier used in mds.config.toml [quality.ts] sections. */
export const LANG = "ts";

/** File extension for TypeScript implementation markdown. */
export const MD_EXT = ".ts.md";

/** Default output file extensions by output kind. */
export const OUTPUT_EXTENSIONS = {
  source: ".ts",
  types: ".types.ts",
  test: ".test.ts",
};

/** Default quality tool configuration for TypeScript. */
export const DEFAULT_QUALITY = {
  linter: "eslint",
  fixer: "prettier --write",
  test_runner: "vitest run",
  required: ["node", "eslint", "prettier", "vitest"],
  optional: [],
};

/** Available quality tools for TypeScript. */
export const TOOLS = {
  linters: ["eslint", "biome"],
  fixers: ["prettier --write", "biome check --write"],
  test_runners: ["vitest run", "jest"],
};

/**
 * Return the expected output path for a given implementation markdown path.
 * @param {string} mdRelPath - Markdown-root relative path (e.g. "greet.ts.md")
 * @param {"source"|"types"|"test"} kind
 * @returns {string}
 */
export function outputPath(mdRelPath, kind) {
  const base = mdRelPath.replace(/\.md$/, "").replace(/\.ts$/, "");
  switch (kind) {
    case "types":
      return `${base}.types.ts`;
    case "test":
      return `${base}.test.ts`;
    case "source":
    default:
      return `${base}.ts`;
  }
}

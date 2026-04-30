/**
 * mds-lang-rs — Rust language adapter for mds.
 *
 * Provides Rust-specific file naming conventions, quality tool
 * defaults, and test runner integration metadata.
 */

/** Language identifier used in mds.config.toml [quality.rs] sections. */
export const LANG = "rs";

/** File extension for Rust implementation markdown. */
export const MD_EXT = ".rs.md";

/** Default output file extensions by output kind. */
export const OUTPUT_EXTENSIONS = {
  source: ".rs",
  types: "_types.rs",
  test: "_test.rs",
};

/** Module block markers used by mds-lang-rs for lib.rs generation. */
export const MODULE_MARKERS = {
  begin: "// mds:begin generated modules",
  end: "// mds:end generated modules",
};

/** Default quality tool configuration for Rust. */
export const DEFAULT_QUALITY = {
  linter: "cargo clippy",
  fixer: "rustfmt",
  test_runner: "cargo test",
  required: ["rustc", "cargo", "rustfmt"],
  optional: ["clippy-driver"],
};

/** Available quality tools for Rust. */
export const TOOLS = {
  linters: ["cargo clippy"],
  fixers: ["rustfmt"],
  test_runners: ["cargo test", "cargo nextest run"],
};

/**
 * Return the expected output path for a given implementation markdown path.
 * @param {string} mdRelPath - Markdown-root relative path (e.g. "greet.rs.md")
 * @param {"source"|"types"|"test"} kind
 * @returns {string}
 */
export function outputPath(mdRelPath, kind) {
  const base = mdRelPath.replace(/\.md$/, "").replace(/\.rs$/, "");
  const parts = base.split("/").filter(Boolean);
  switch (kind) {
    case "types":
      return `${base}_types.rs`;
    case "test":
      return `${parts.join("_")}_test.rs`;
    case "source":
    default:
      return `${base}.rs`;
  }
}

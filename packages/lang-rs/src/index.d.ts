export declare const LANG: "rs";
export declare const MD_EXT: ".rs.md";

export declare const OUTPUT_EXTENSIONS: {
  source: ".rs";
  types: "_types.rs";
  test: "_test.rs";
};

export declare const MODULE_MARKERS: {
  begin: string;
  end: string;
};

export declare const DEFAULT_QUALITY: {
  linter: string;
  fixer: string;
  test_runner: string;
  required: string[];
  optional: string[];
};

export declare const TOOLS: {
  linters: string[];
  fixers: string[];
  test_runners: string[];
};

export declare function outputPath(
  mdRelPath: string,
  kind: "source" | "types" | "test"
): string;

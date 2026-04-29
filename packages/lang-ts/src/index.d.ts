export declare const LANG: "ts";
export declare const MD_EXT: ".ts.md";

export declare const OUTPUT_EXTENSIONS: {
  source: ".ts";
  types: ".types.ts";
  test: ".test.ts";
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

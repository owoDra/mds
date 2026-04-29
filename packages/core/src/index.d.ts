/**
 * Result of an mds CLI command execution.
 */
export interface MdsResult {
  exitCode: number;
  stdout: string;
  stderr: string;
}

export interface RunOptions {
  cwd?: string;
}

export interface CheckOptions {
  verbose?: boolean;
}

export interface BuildOptions {
  dryRun?: boolean;
  verbose?: boolean;
}

export interface LintOptions {
  fix?: boolean;
  check?: boolean;
  verbose?: boolean;
}

export interface TestOptions {
  verbose?: boolean;
}

export interface DoctorOptions {
  format?: "text" | "json";
  verbose?: boolean;
}

export interface PackageSyncOptions {
  check?: boolean;
  verbose?: boolean;
}

/**
 * Run an mds CLI command with arbitrary arguments.
 */
export function run(args: string[], options?: RunOptions): MdsResult;

/**
 * Run `mds check` on a package.
 */
export function check(packagePath: string, options?: CheckOptions): MdsResult;

/**
 * Run `mds build` on a package.
 */
export function build(
  packagePath: string,
  options?: BuildOptions
): MdsResult;

/**
 * Run `mds lint` on a package.
 */
export function lint(packagePath: string, options?: LintOptions): MdsResult;

/**
 * Run `mds test` on a package.
 */
export function test(packagePath: string, options?: TestOptions): MdsResult;

/**
 * Run `mds doctor` on a package.
 */
export function doctor(
  packagePath: string,
  options?: DoctorOptions
): MdsResult;

/**
 * Run `mds package sync` on a package.
 */
export function packageSync(
  packagePath: string,
  options?: PackageSyncOptions
): MdsResult;

/**
 * Get the mds version string, or null if not available.
 */
export function version(): string | null;

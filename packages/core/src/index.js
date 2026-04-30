/**
 * mds-core — programmatic access to mds CLI commands.
 *
 * This module wraps the native mds binary and exposes a JS API for
 * check, build, lint, test, doctor, and package-sync operations.
 */

import { spawnSync } from "node:child_process";
import { existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";

const __dirname = dirname(fileURLToPath(import.meta.url));

/**
 * Resolve the native mds binary path.
 * Search order: sibling vendor, mds-cli vendor, MDS_NATIVE_BIN env, PATH.
 * @returns {string} Resolved binary path or "mds" for PATH lookup.
 */
function resolveBinary() {
  const candidates = [
    join(__dirname, "..", "..", "cli", "vendor", "mds"),
    join(__dirname, "..", "..", "cli", "bin", "mds"),
    process.env.MDS_NATIVE_BIN,
    "mds",
  ].filter(Boolean);

  for (const candidate of candidates) {
    if (candidate !== "mds" && !existsSync(candidate)) continue;
    return candidate;
  }
  return "mds";
}

/**
 * Run an mds CLI command and return the result.
 * @param {string[]} args - CLI arguments (e.g. ["check", "--package", "./my-pkg"])
 * @param {object} [options]
 * @param {string} [options.cwd] - Working directory.
 * @returns {{ exitCode: number, stdout: string, stderr: string }}
 */
export function run(args, options = {}) {
  const bin = resolveBinary();
  const result = spawnSync(bin, args, {
    cwd: options.cwd,
    encoding: "utf-8",
    stdio: ["ignore", "pipe", "pipe"],
  });
  if (result.error) {
    return {
      exitCode: 4,
      stdout: "",
      stderr: `environment error: failed to run native mds: ${result.error.message}\n`,
    };
  }
  return {
    exitCode: result.status === null ? 3 : result.status,
    stdout: result.stdout || "",
    stderr: result.stderr || "",
  };
}

/**
 * Run `mds check` on a package.
 * @param {string} packagePath - Path to the mds package directory.
 * @param {object} [options]
 * @param {boolean} [options.verbose]
 * @returns {{ exitCode: number, stdout: string, stderr: string }}
 */
export function check(packagePath, options = {}) {
  const args = ["check", "--package", packagePath];
  if (options.verbose) args.push("--verbose");
  return run(args);
}

/**
 * Run `mds build` on a package.
 * @param {string} packagePath
 * @param {object} [options]
 * @param {boolean} [options.dryRun]
 * @param {boolean} [options.verbose]
 * @returns {{ exitCode: number, stdout: string, stderr: string }}
 */
export function build(packagePath, options = {}) {
  const args = ["build", "--package", packagePath];
  if (options.dryRun) args.push("--dry-run");
  if (options.verbose) args.push("--verbose");
  return run(args);
}

/**
 * Run `mds lint` on a package.
 * @param {string} packagePath
 * @param {object} [options]
 * @param {boolean} [options.fix]
 * @param {boolean} [options.check]
 * @param {boolean} [options.verbose]
 * @returns {{ exitCode: number, stdout: string, stderr: string }}
 */
export function lint(packagePath, options = {}) {
  const args = ["lint", "--package", packagePath];
  if (options.fix) args.push("--fix");
  if (options.check) args.push("--check");
  if (options.verbose) args.push("--verbose");
  return run(args);
}

/**
 * Run `mds test` on a package.
 * @param {string} packagePath
 * @param {object} [options]
 * @param {boolean} [options.verbose]
 * @returns {{ exitCode: number, stdout: string, stderr: string }}
 */
export function test(packagePath, options = {}) {
  const args = ["test", "--package", packagePath];
  if (options.verbose) args.push("--verbose");
  return run(args);
}

/**
 * Run `mds doctor` on a package.
 * @param {string} packagePath
 * @param {object} [options]
 * @param {"text"|"json"} [options.format]
 * @param {boolean} [options.verbose]
 * @returns {{ exitCode: number, stdout: string, stderr: string }}
 */
export function doctor(packagePath, options = {}) {
  const args = ["doctor", "--package", packagePath];
  if (options.format) args.push("--format", options.format);
  if (options.verbose) args.push("--verbose");
  return run(args);
}

/**
 * Run `mds package sync` on a package.
 * @param {string} packagePath
 * @param {object} [options]
 * @param {boolean} [options.check]
 * @param {boolean} [options.verbose]
 * @returns {{ exitCode: number, stdout: string, stderr: string }}
 */
export function packageSync(packagePath, options = {}) {
  const args = ["package", "sync", "--package", packagePath];
  if (options.check) args.push("--check");
  if (options.verbose) args.push("--verbose");
  return run(args);
}

/**
 * Get the mds version string.
 * @returns {string|null}
 */
export function version() {
  const result = run(["--version"]);
  if (result.exitCode === 0) {
    return result.stdout.trim();
  }
  return null;
}

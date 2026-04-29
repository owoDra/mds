#!/usr/bin/env node
const { existsSync } = require("node:fs");
const { dirname, join, resolve } = require("node:path");
const { spawnSync } = require("node:child_process");

const here = dirname(__filename);
const candidates = [
  join(here, "mds"),
  join(here, "..", "vendor", "mds"),
  process.env.MDS_NATIVE_BIN,
  "mds",
].filter(Boolean);

for (const candidate of candidates) {
  if (candidate !== "mds" && !existsSync(candidate)) continue;
  if (candidate === "mds" && process.argv[1] && resolve(process.argv[1]) === resolve(__filename)) continue;
  const result = spawnSync(candidate, process.argv.slice(2), { stdio: "inherit" });
  if (result.error && result.error.code === "ENOENT") continue;
  if (result.error) {
    console.error(`environment error: failed to run native mds: ${result.error.message}`);
    process.exit(4);
  }
  process.exit(result.status === null ? 3 : result.status);
}

console.error("environment error: native mds binary is not available");
process.exit(4);

# src/adapter.rs

## Purpose

Migrated implementation source for `src/adapter.rs`.

## Contract

- Preserve the behavior of the pre-migration Rust source.
- This file is synchronized into `.build/rust/mds/core/src/adapter.rs`.

## Source

````rs
use std::io;
use std::path::Path;
use std::process::{Command as ProcessCommand, Stdio};

use crate::descriptor;
use crate::diagnostics::{Diagnostic, RunState};
use crate::model::{ImplDoc, OutputKind, QualityConfig};
````

````rs
/// Compute the output file path relative to the selected output root.
/// Resolves through the built-in language descriptor.

pub(crate) fn output_relative_path(doc: &ImplDoc, kind: OutputKind) -> std::path::PathBuf {
    descriptor::output_relative_path(&doc.markdown_relative_path, &doc.lang, kind)
}
````

````rs
pub(crate) fn run_toolchain_command(
    command: &str,
    file: Option<&Path>,
    cwd: &Path,
    config: &QualityConfig,
    diagnostic_path: &Path,
    state: &mut RunState,
) -> Result<io::Result<()>, String> {
    let Some((program, args)) = split_command(command) else {
        return Ok(Ok(()));
    };
    if !tool_available(program) {
        state.environment_missing = true;
        state.diagnostics.push(Diagnostic::error(
            Some(diagnostic_path.to_path_buf()),
            format!("LINT001_TOOLCHAIN_FAILED: required toolchain `{program}` is not available"),
        ));
        return Ok(Err(io::Error::new(io::ErrorKind::NotFound, program)));
    }
    let mut process = ProcessCommand::new(program);
    process
        .args(args)
        .current_dir(cwd)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    if let Some(file) = file {
        process.arg(file);
    }
    let output = process
        .output()
        .map_err(|error| format!("failed to run toolchain: {error}"))?;
    if !output.status.success() {
        let detail = tool_output_detail(&output.stdout, &output.stderr, file, diagnostic_path, cwd);
        state.diagnostics.push(Diagnostic::error(
            Some(diagnostic_path.to_path_buf()),
            format!("LINT001_TOOLCHAIN_FAILED: toolchain command failed: {detail}"),
        ));
        return Ok(Err(io::Error::other("toolchain command failed")));
    }
    for optional in &config.optional {
        if !tool_available(optional) {
            state.diagnostics.push(Diagnostic::warning(
                Some(diagnostic_path.to_path_buf()),
                format!("optional toolchain `{optional}` is not available"),
            ));
        }
    }
    Ok(Ok(()))
}
````

````rs
fn tool_output_detail(
    stdout: &[u8],
    stderr: &[u8],
    file: Option<&Path>,
    diagnostic_path: &Path,
    cwd: &Path,
) -> String {
    let raw = if stderr.is_empty() { stdout } else { stderr };
    let detail = String::from_utf8_lossy(raw).trim().to_string();
    let Some(file) = file else {
        return detail;
    };
    replace_path_variants(&detail, file, diagnostic_path, cwd)
}
````

````rs
fn replace_path_variants(output: &str, from: &Path, to: &Path, cwd: &Path) -> String {
    let mut replaced = output.to_string();
    let to_display = to.display().to_string();
    for variant in path_variants(from, cwd) {
        replaced = replaced.replace(&variant, &to_display);
    }
    replaced
}
````

````rs
fn path_variants(path: &Path, cwd: &Path) -> Vec<String> {
    let mut variants = vec![path.display().to_string()];
    if let Ok(relative) = path.strip_prefix(cwd) {
        variants.push(relative.display().to_string());
    }
    variants.sort();
    variants.dedup();
    variants
}
````

````rs
pub(crate) fn split_command(command: &str) -> Option<(&str, Vec<&str>)> {
    let mut parts = command.split_whitespace();
    let program = parts.next()?;
    Some((program, parts.collect()))
}
````

````rs
pub(crate) fn tool_available(program: &str) -> bool {
    if program.contains('/') {
        return Path::new(program).exists();
    }
    let Some(path) = std::env::var_os("PATH") else {
        return false;
    };
    std::env::split_paths(&path).any(|dir| dir.join(program).exists())
}
````
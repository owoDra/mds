use std::io;
use std::path::Path;
use std::process::{Command as ProcessCommand, Stdio};

use crate::diagnostics::{Diagnostic, RunState};
use crate::model::{ImplDoc, QualityConfig};

/// Compute the output file path relative to the source root.
/// Strips the `.md` suffix from the markdown-relative path.
/// e.g. `greet.ts.md` → `greet.ts`
pub(crate) fn output_relative_path(doc: &ImplDoc) -> std::path::PathBuf {
    strip_md_extension(&doc.markdown_relative_path)
}

pub(crate) fn strip_md_extension(path: &Path) -> std::path::PathBuf {
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    let stripped = name.strip_suffix(".md").unwrap_or(&name);
    path.with_file_name(stripped)
}

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
        state.diagnostics.push(Diagnostic::error(
            Some(diagnostic_path.to_path_buf()),
            format!(
                "LINT001_TOOLCHAIN_FAILED: toolchain command failed: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ),
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

pub(crate) fn split_command(command: &str) -> Option<(&str, Vec<&str>)> {
    let mut parts = command.split_whitespace();
    let program = parts.next()?;
    Some((program, parts.collect()))
}

pub(crate) fn tool_available(program: &str) -> bool {
    if program.contains('/') {
        return Path::new(program).exists();
    }
    let Some(path) = std::env::var_os("PATH") else {
        return false;
    };
    std::env::split_paths(&path).any(|dir| dir.join(program).exists())
}

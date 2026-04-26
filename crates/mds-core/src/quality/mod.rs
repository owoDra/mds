use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::process::{Command as ProcessCommand, Stdio};

use crate::adapter::imports_for;
use crate::diagnostics::{Diagnostic, RunState};
use crate::diff::unified_diff;
use crate::fs_utils::is_excluded;
use crate::model::{ImplDoc, Lang, OutputKind, Package, QualityConfig};

#[derive(Debug, Clone, Copy)]
pub(crate) enum QualityOperation {
    Lint,
    Fix { check: bool },
    Test,
}

pub(crate) fn run_quality(
    package: &Package,
    docs: &[ImplDoc],
    operation: QualityOperation,
    state: &mut RunState,
) -> Result<(), String> {
    match operation {
        QualityOperation::Lint | QualityOperation::Test => {
            for doc in docs {
                run_doc_quality(package, doc, operation, state)?;
            }
            if !state.has_errors() && !state.environment_missing {
                let name = match operation {
                    QualityOperation::Lint => "lint",
                    QualityOperation::Test => "test",
                    QualityOperation::Fix { .. } => unreachable!(),
                };
                state.stdout.push_str(&format!("{name} ok\n"));
            }
        }
        QualityOperation::Fix { check } => {
            for doc in docs {
                fix_doc(package, doc, check, state)?;
            }
            if !state.has_errors() && !state.environment_missing {
                state.stdout.push_str("lint --fix ok\n");
            }
        }
    }
    Ok(())
}

fn run_doc_quality(
    package: &Package,
    doc: &ImplDoc,
    operation: QualityOperation,
    state: &mut RunState,
) -> Result<(), String> {
    let Some(config) = package.config.quality.get(&doc.lang) else {
        return Ok(());
    };
    let command = match operation {
        QualityOperation::Lint => config.lint.as_deref(),
        QualityOperation::Test => config.test.as_deref(),
        QualityOperation::Fix { .. } => None,
    };
    let Some(command) = command else {
        return Ok(());
    };
    for kind in [OutputKind::Types, OutputKind::Source, OutputKind::Test] {
        let Some(code) = doc.code.get(&kind) else {
            continue;
        };
        let path = temp_code_path(package, doc.lang, kind);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("failed to create {}: {error}", parent.display()))?;
        }
        let imports = imports_for(package, doc, kind, &path, state);
        fs::write(&path, format!("{imports}{code}"))
            .map_err(|error| format!("failed to write {}: {error}", path.display()))?;
        let _ = run_tool(
            command,
            Some(&path),
            &package.root,
            config,
            &doc.path,
            state,
        )?;
    }
    Ok(())
}

fn fix_doc(
    package: &Package,
    doc: &ImplDoc,
    check: bool,
    state: &mut RunState,
) -> Result<(), String> {
    let Some(config) = package.config.quality.get(&doc.lang) else {
        return Ok(());
    };
    let Some(command) = config.fix.as_deref() else {
        return Ok(());
    };
    let old = fs::read_to_string(&doc.path)
        .map_err(|error| format!("failed to read {}: {error}", doc.path.display()))?;
    let mut replacements = Vec::new();
    for block in code_block_ranges(&old, &package.config.label_overrides) {
        let path = temp_code_path(package, doc.lang, OutputKind::Source);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("failed to create {}: {error}", parent.display()))?;
        }
        fs::write(&path, block.content)
            .map_err(|error| format!("failed to write {}: {error}", path.display()))?;
        if run_tool(
            command,
            Some(&path),
            &package.root,
            config,
            &doc.path,
            state,
        )?
        .is_ok()
        {
            let fixed = fs::read_to_string(&path)
                .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
            replacements.push((block.start, block.end, fixed));
        }
    }
    let new = apply_replacements(&old, &replacements);
    if old != new {
        state.stdout.push_str(&unified_diff(&doc.path, &old, &new));
        if check {
            state.diagnostics.push(Diagnostic::error(
                Some(doc.path.clone()),
                "lint --fix --check found code block changes",
            ));
        } else {
            fs::write(&doc.path, new)
                .map_err(|error| format!("failed to write {}: {error}", doc.path.display()))?;
            state.generated.push(doc.path.clone());
        }
    }
    Ok(())
}

fn run_tool(
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
    if !tool_available(&program) {
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

fn temp_code_path(package: &Package, lang: Lang, kind: OutputKind) -> PathBuf {
    let ext = match lang {
        Lang::TypeScript => "ts",
        Lang::Python => "py",
        Lang::Rust => "rs",
    };
    let path = package
        .root
        .join(".mds/tmp")
        .join(format!("{}.{}", kind.manifest_kind(), ext));
    if is_excluded(&package.root, &path, &package.config.excludes) {
        package
            .root
            .join(".mds-tmp")
            .join(format!("{}.{}", kind.manifest_kind(), ext))
    } else {
        path
    }
}

#[derive(Debug)]
struct CodeBlock<'a> {
    start: usize,
    end: usize,
    content: &'a str,
}

fn code_block_ranges<'a>(
    text: &'a str,
    label_overrides: &std::collections::HashMap<String, String>,
) -> Vec<CodeBlock<'a>> {
    let mut ranges = Vec::new();
    let mut in_block = false;
    let mut in_quality_section = false;
    let mut content_start = 0;
    let mut cursor = 0;
    for line in text.split_inclusive('\n') {
        let line_start = cursor;
        cursor += line.len();
        if !in_block {
            if let Some(title) = line.trim_end().strip_prefix("## ") {
                let title = canonical_quality_section(title.trim(), label_overrides);
                in_quality_section = matches!(title.as_str(), "Types" | "Source" | "Test");
            }
        }
        if line.trim_start().starts_with("```") {
            if in_block {
                if in_quality_section {
                    ranges.push(CodeBlock {
                        start: content_start,
                        end: line_start,
                        content: &text[content_start..line_start],
                    });
                }
                in_block = false;
            } else {
                in_block = true;
                content_start = cursor;
            }
        }
    }
    ranges
}

fn canonical_quality_section(
    title: &str,
    label_overrides: &std::collections::HashMap<String, String>,
) -> String {
    for canonical in ["Types", "Source", "Test"] {
        if title == canonical {
            return canonical.to_string();
        }
        if label_overrides
            .get(&canonical.to_ascii_lowercase())
            .is_some_and(|override_label| override_label.trim() == title)
        {
            return canonical.to_string();
        }
    }
    title.to_string()
}

fn apply_replacements(old: &str, replacements: &[(usize, usize, String)]) -> String {
    let mut output = String::new();
    let mut cursor = 0;
    for (start, end, replacement) in replacements {
        output.push_str(&old[cursor..*start]);
        output.push_str(replacement);
        cursor = *end;
    }
    output.push_str(&old[cursor..]);
    output
}

use std::fs;
use std::path::PathBuf;

use crate::adapter::{imports_for, run_toolchain_command};
use crate::diagnostics::{Diagnostic, RunState};
use crate::diff::unified_diff;
use crate::fs_utils::is_excluded;
use crate::model::{ImplDoc, Lang, OutputKind, Package};

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
            let mut any_configured = false;
            for doc in docs {
                if has_quality_config(package, doc, operation) {
                    any_configured = true;
                }
                run_doc_quality(package, doc, operation, state)?;
            }
            if !any_configured && !docs.is_empty() {
                let name = match operation {
                    QualityOperation::Lint => "lint",
                    QualityOperation::Test => "test",
                    QualityOperation::Fix { .. } => unreachable!(),
                };
                state.stdout.push_str(&format!(
                    "warning: no {name} tool configured in mds.config.toml [quality.*]; skipping\n"
                ));
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
            let mut any_configured = false;
            for doc in docs {
                if package
                    .config
                    .quality
                    .get(&doc.lang)
                    .and_then(|c| c.fix.as_ref())
                    .is_some()
                {
                    any_configured = true;
                }
                fix_doc(package, doc, check, state)?;
            }
            if !any_configured && !docs.is_empty() {
                state.stdout.push_str(
                    "warning: no fix tool configured in mds.config.toml [quality.*]; skipping\n",
                );
            }
            if !state.has_errors() && !state.environment_missing {
                state.stdout.push_str("lint --fix ok\n");
            }
        }
    }
    Ok(())
}

fn has_quality_config(package: &Package, doc: &ImplDoc, operation: QualityOperation) -> bool {
    let Some(config) = package.config.quality.get(&doc.lang) else {
        return false;
    };
    match operation {
        QualityOperation::Lint => config.lint.is_some(),
        QualityOperation::Test => config.test.is_some(),
        QualityOperation::Fix { .. } => config.fix.is_some(),
    }
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
    let target_kinds: &[OutputKind] = match operation {
        QualityOperation::Lint => &[OutputKind::Types, OutputKind::Source, OutputKind::Test],
        QualityOperation::Test => &[OutputKind::Test],
        QualityOperation::Fix { .. } => &[],
    };
    for kind in target_kinds {
        let Some(code) = doc.code.get(kind) else {
            continue;
        };
        let path = temp_code_path(package, &doc.lang, *kind);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("failed to create {}: {error}", parent.display()))?;
        }
        let imports = imports_for(package, doc, *kind, &path, state);
        fs::write(&path, format!("{imports}{code}"))
            .map_err(|error| format!("failed to write {}: {error}", path.display()))?;
        let _ = run_toolchain_command(
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
        let path = temp_code_path(package, &doc.lang, OutputKind::Source);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("failed to create {}: {error}", parent.display()))?;
        }
        fs::write(&path, block.content)
            .map_err(|error| format!("failed to write {}: {error}", path.display()))?;
        if run_toolchain_command(
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

fn temp_code_path(package: &Package, lang: &Lang, kind: OutputKind) -> PathBuf {
    let file_name = match (lang, kind) {
        (Lang::TypeScript, OutputKind::Test) => "test.test.ts".to_string(),
        (Lang::Python, OutputKind::Test) => "test_test.py".to_string(),
        (Lang::Rust, OutputKind::Test) => "test.rs".to_string(),
        (Lang::TypeScript, _) => format!("{}.ts", kind.manifest_kind()),
        (Lang::Python, _) => format!("{}.py", kind.manifest_kind()),
        (Lang::Rust, _) => format!("{}.rs", kind.manifest_kind()),
        (Lang::Other(ext), OutputKind::Test) => format!("test.{ext}"),
        (Lang::Other(ext), _) => format!("{}.{ext}", kind.manifest_kind()),
    };
    let path = package.root.join(".mds/tmp").join(&file_name);
    if is_excluded(&package.root, &path, &package.config.excludes) {
        package.root.join(".mds-tmp").join(file_name)
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

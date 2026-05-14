use regex::{Regex};
use std::{fs};
use std::path::{Path};
use std::path::{PathBuf};
use crate::adapter::{path_variants};
use crate::adapter::{replace_path_variants};
use crate::adapter::{run_toolchain_command};
use crate::adapter::{tool_available};
use crate::adapter::{tool_output_detail};
use crate::adapter::{ToolInvocation};
use crate::adapter::{ToolRunOutput};
use crate::adapter::{ToolRunStatus};
use crate::descriptor::{self};
use crate::descriptor::{ToolBehavior};
use crate::descriptor::{ToolInputMode};
use crate::descriptor::{ToolOutputMode};
use crate::diagnostics::{Diagnostic};
use crate::diagnostics::{RunState};
use crate::diff::{unified_diff};
use crate::fs_utils::{is_excluded};
use crate::model::{DocKind};
use crate::model::{ImplDoc};
use crate::model::{Lang};
use crate::model::{OutputKind};
use crate::model::{Package};
use crate::model::{SourceMap};
use crate::model::{SourceSpan};

struct PreparedQualityInput {
    source: String,
    source_map: SourceMap,
}

#[derive(Debug, Clone, Copy)]

pub(crate) enum QualityOperation {
    Typecheck,
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
        QualityOperation::Typecheck | QualityOperation::Lint | QualityOperation::Test => {
            let mut any_configured = false;
            for doc in docs {
                if has_quality_config(package, doc, operation) {
                    any_configured = true;
                }
                run_doc_quality(package, doc, operation, state)?;
            }
            if !any_configured && !docs.is_empty() {
                let name = match operation {
                    QualityOperation::Typecheck => "typecheck",
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
                    QualityOperation::Typecheck => "typecheck",
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
        QualityOperation::Typecheck => config.type_check.is_some(),
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
    let descriptor = descriptor::builtin_descriptor(&doc.lang);
    let Some(config) = package.config.quality.get(&doc.lang) else {
        return Ok(());
    };
    let command = match operation {
        QualityOperation::Typecheck => config.type_check.as_deref(),
        QualityOperation::Lint => config.lint.as_deref(),
        QualityOperation::Test => config.test.as_deref(),
        QualityOperation::Fix { .. } => None,
    };
    let Some(command) = command else {
        return Ok(());
    };
    let path = temp_code_path(package, &doc.lang);
    let input = padded_code_from_markdown(doc, &path)?;
    let behavior = resolve_tool_behavior(&descriptor, operation, command);
    let _ = execute_quality_command(
        package,
        doc,
        command,
        config,
        &behavior,
        &path,
        &input.source,
        Some(&input.source_map),
        state,
    )?;
    Ok(())
}

fn fix_doc(
    package: &Package,
    doc: &ImplDoc,
    check: bool,
    state: &mut RunState,
) -> Result<(), String> {
    let descriptor = descriptor::builtin_descriptor(&doc.lang);
    let Some(config) = package.config.quality.get(&doc.lang) else {
        return Ok(());
    };
    let Some(command) = config.fix.as_deref() else {
        return Ok(());
    };
    let behavior = resolve_tool_behavior(&descriptor, QualityOperation::Fix { check }, command);
    let path = temp_code_path(package, &doc.lang);
    let old = fs::read_to_string(&doc.path)
        .map_err(|error| format!("failed to read {}: {error}", doc.path.display()))?;
    let mut replacements = Vec::new();
    for block in code_block_ranges(&old) {
        let source_map = source_map_for_code_block(doc, &path, &block);
        if let Some(fixed) = execute_quality_command(
            package,
            doc,
            command,
            config,
            &behavior,
            &path,
            block.content,
            Some(&source_map),
            state,
        )? {
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

fn resolve_tool_behavior(
    descriptor: &descriptor::Descriptor,
    operation: QualityOperation,
    command: &str,
) -> ToolBehavior {
    descriptor::tool_behavior_for_command(command).unwrap_or_else(|| match operation {
        QualityOperation::Typecheck => descriptor.typecheck_behavior().clone(),
        QualityOperation::Lint => descriptor.lint_behavior().clone(),
        QualityOperation::Test => descriptor.test_behavior().clone(),
        QualityOperation::Fix { .. } => descriptor.fix_behavior().clone(),
    })
}

fn execute_quality_command(
    package: &Package,
    doc: &ImplDoc,
    command: &str,
    config: &crate::model::QualityConfig,
    behavior: &ToolBehavior,
    input_path: &Path,
    source: &str,
    source_map: Option<&SourceMap>,
    state: &mut RunState,
) -> Result<Option<String>, String> {
    if needs_tempfile(behavior) {
        if let Some(parent) = input_path.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| format!("failed to create {}: {error}", parent.display()))?;
        }
        fs::write(input_path, source)
            .map_err(|error| format!("failed to write {}: {error}", input_path.display()))?;
    }

    let file_arg = behavior.append_file_arg().then_some(input_path);
    let stdin = match behavior.input_mode() {
        ToolInputMode::Stdin => Some(source),
        ToolInputMode::TempFile | ToolInputMode::Inline => None,
    };
    let inline_arg = match behavior.input_mode() {
        ToolInputMode::Inline => Some(source),
        ToolInputMode::TempFile | ToolInputMode::Stdin => None,
    };

    let status = run_toolchain_command(ToolInvocation {
        command,
        file_arg,
        cwd: &package.root,
        stdin,
        inline_arg,
    })?;

    warn_missing_optional_tools(config, &doc.path, state);

    match status {
        ToolRunStatus::EmptyCommand => Ok(None),
        ToolRunStatus::MissingTool(program) => {
            state.environment_missing = true;
            state.diagnostics.push(Diagnostic::error(
                Some(doc.path.clone()),
                format!(
                    "LINT001_TOOLCHAIN_FAILED: required toolchain `{program}` is not available"
                ),
            ));
            Ok(None)
        }
        ToolRunStatus::Completed(output) => {
            if !output.success {
                report_tool_failure(
                    &output,
                    behavior,
                    &doc.path,
                    input_path,
                    source_map,
                    &package.root,
                    state,
                )?;
                return Ok(None);
            }
            let fixed = match behavior.output_mode() {
                ToolOutputMode::None => None,
                ToolOutputMode::Stdout => Some(output.stdout),
                ToolOutputMode::TempFile => Some(
                    fs::read_to_string(input_path)
                        .map_err(|error| format!("failed to read {}: {error}", input_path.display()))?,
                ),
            };
            Ok(fixed)
        }
    }
}

fn warn_missing_optional_tools(
    config: &crate::model::QualityConfig,
    path: &std::path::Path,
    state: &mut RunState,
) {
    for optional in &config.optional {
        if !tool_available(optional) {
            state.diagnostics.push(Diagnostic::warning(
                Some(path.to_path_buf()),
                format!("optional toolchain `{optional}` is not available"),
            ));
        }
    }
}

fn report_tool_failure(
    output: &ToolRunOutput,
    behavior: &ToolBehavior,
    markdown_path: &std::path::Path,
    input_path: &std::path::Path,
    source_map: Option<&SourceMap>,
    cwd: &std::path::Path,
    state: &mut RunState,
) -> Result<(), String> {
    let detail = tool_output_detail(output);
    let diagnostics = capture_tool_diagnostics(
        &detail,
        behavior,
        markdown_path,
        input_path,
        source_map,
        cwd,
    )?;
    if diagnostics.is_empty() {
        let rendered = source_map
            .and_then(|map| remap_generic_tool_failure(&detail, input_path, markdown_path, map, cwd))
            .unwrap_or_else(|| replace_path_variants(&detail, input_path, markdown_path, cwd));
        state.diagnostics.push(Diagnostic::error(
            Some(markdown_path.to_path_buf()),
            format!("LINT001_TOOLCHAIN_FAILED: toolchain command failed: {rendered}"),
        ));
        return Ok(());
    }
    state.diagnostics.extend(diagnostics);
    Ok(())
}

fn remap_generic_tool_failure(
    detail: &str,
    input_path: &Path,
    markdown_path: &Path,
    source_map: &SourceMap,
    cwd: &Path,
) -> Option<String> {
    let regex = Regex::new(r"^(?P<path>.+?):(?P<line>\d+):(?P<column>\d+):(?P<rest>.*)$").ok()?;
    let captures = regex.captures(detail.lines().next()?)?;
    let line = captures.name("line")?.as_str().parse::<usize>().ok()?;
    let column = captures.name("column")?.as_str();
    let rest = captures.name("rest")?.as_str();
    let generated_path = diagnostic_input_path(
        captures.name("path").map(|value| value.as_str()),
        input_path,
        cwd,
    )?;
    let (remapped_path, markdown_line) = source_map.remap_generated_line(generated_path, line)?;
    Some(format!(
        "{}:{}:{}:{}",
        remapped_path.display(),
        markdown_line,
        column,
        rest
    ))
}

fn capture_tool_diagnostics(
    detail: &str,
    behavior: &ToolBehavior,
    markdown_path: &std::path::Path,
    input_path: &std::path::Path,
    source_map: Option<&SourceMap>,
    cwd: &std::path::Path,
) -> Result<Vec<Diagnostic>, String> {
    let mut diagnostics = Vec::new();
    for raw_line in detail.lines().filter(|line| !line.trim().is_empty()) {
        for rule in &behavior.diagnostics {
            let regex = Regex::new(&rule.pattern)
                .map_err(|error| format!("invalid diagnostic regex `{}`: {error}", rule.pattern))?;
            let Some(captures) = regex.captures(raw_line) else {
                continue;
            };
            let mut diagnostic = match rule.severity.as_str() {
                "warning" => Diagnostic::warning(
                    Some(markdown_path.to_path_buf()),
                    capture_message(&captures, rule, raw_line),
                ),
                _ => Diagnostic::error(
                    Some(markdown_path.to_path_buf()),
                    capture_message(&captures, rule, raw_line),
                ),
            };
            let line = captures
                .name(&rule.line_group)
                .and_then(|value| value.as_str().parse::<usize>().ok())
                .map(|value| apply_line_offset(value, rule.line_offset));
            if let Some(column) = captures
                .name(&rule.column_group)
                .and_then(|value| value.as_str().parse::<usize>().ok())
            {
                diagnostic = diagnostic.at_column(column);
            }
            let remapped = line
                .and_then(|generated_line| {
                    diagnostic_input_path(
                        captures.name(&rule.path_group).map(|value| value.as_str()),
                        input_path,
                        cwd,
                    )
                    .and_then(|generated_path| {
                        source_map.and_then(|map| map.remap_generated_line(generated_path, generated_line))
                    })
                })
                .map(|(path, markdown_line)| (path.to_path_buf(), markdown_line));
            let remapped_path = remapped.as_ref().map(|(path, _)| path.clone());
            let remapped_line = remapped.as_ref().map(|(_, markdown_line)| *markdown_line);
            if let (Some(path), Some(markdown_line)) = (remapped_path, remapped_line) {
                diagnostic.path = Some(path);
                diagnostic = diagnostic.at_line(markdown_line);
            } else if let Some(line) = line {
                diagnostic = diagnostic.at_line(line);
            }
            if let Some(path_match) = captures.name(&rule.path_group) {
                let captured = std::path::PathBuf::from(path_match.as_str());
                let rewritten = path_variants(input_path, cwd)
                    .into_iter()
                    .any(|variant| variant == captured.display().to_string());
                if remapped.is_none() && !rewritten {
                    diagnostic.path = Some(std::path::PathBuf::from(replace_path_variants(
                        path_match.as_str(),
                        input_path,
                        markdown_path,
                        cwd,
                    )));
                }
            }
            diagnostics.push(diagnostic);
            break;
        }
    }
    Ok(diagnostics)
}

fn diagnostic_input_path<'a>(captured_path: Option<&str>, input_path: &'a Path, cwd: &Path) -> Option<&'a Path> {
    match captured_path {
        Some(path) => path_variants(input_path, cwd)
            .into_iter()
            .any(|variant| variant == path)
            .then_some(input_path),
        None => Some(input_path),
    }
}

fn capture_message(
    captures: &regex::Captures<'_>,
    rule: &crate::descriptor::DiagnosticCaptureRule,
    raw_line: &str,
) -> String {
    captures
        .name(&rule.message_group)
        .map(|value| value.as_str().to_string())
        .unwrap_or_else(|| raw_line.to_string())
}


fn apply_line_offset(line: usize, offset: isize) -> usize {
    if offset >= 0 {
        line.saturating_add(offset as usize)
    } else {
        line.saturating_sub(offset.unsigned_abs())
    }
}


fn needs_tempfile(behavior: &ToolBehavior) -> bool {
    matches!(behavior.input_mode(), ToolInputMode::TempFile)
        || matches!(behavior.output_mode(), ToolOutputMode::TempFile)
}

fn temp_code_path(package: &Package, lang: &Lang) -> PathBuf {
    let ext = lang.file_ext();
    let file_name = format!("source.{ext}");
    let path = package.root.join(".build/mds/tmp").join(&file_name);
    if is_excluded(&package.root, &path, &package.config.excludes) {
        package.root.join(".build/mds-tmp").join(file_name)
    } else {
        path
    }
}


#[derive(Debug)]
struct CodeBlock<'a> {
    fence_index: usize,
    start: usize,
    end: usize,
    content: &'a str,
    content_start_line: usize,
    content_end_line: usize,
}

fn code_block_ranges(text: &str) -> Vec<CodeBlock<'_>> {
    let mut ranges = Vec::new();
    let mut in_block = false;
    let mut content_start = 0;
    let mut content_start_line = 1;
    let mut fence_index = 0;
    let mut cursor = 0;
    let mut line_number: usize = 1;
    for line in text.split_inclusive('\n') {
        let line_start = cursor;
        cursor += line.len();
        if line.trim_start().starts_with("```") {
            if in_block {
                ranges.push(CodeBlock {
                    fence_index,
                    start: content_start,
                    end: line_start,
                    content: &text[content_start..line_start],
                    content_start_line,
                    content_end_line: line_number.saturating_sub(1),
                });
                fence_index += 1;
                in_block = false;
            } else {
                in_block = true;
                content_start = cursor;
                content_start_line = line_number + 1;
            }
        }
        line_number += 1;
    }
    ranges
}

fn padded_code_from_markdown(doc: &ImplDoc, input_path: &Path) -> Result<PreparedQualityInput, String> {
    let text = fs::read_to_string(&doc.path)
        .map_err(|error| format!("failed to read {}: {error}", doc.path.display()))?;
    let blocks = code_block_ranges(&text);
    if blocks.is_empty() {
        return Ok(PreparedQualityInput {
            source: doc.code.clone(),
            source_map: SourceMap::new(),
        });
    }
    let mut output = String::new();
    let mut source_map = SourceMap::new();
    let mut output_line = 1;
    for block in blocks {
        while output_line < block.content_start_line {
            output.push('\n');
            output_line += 1;
        }
        output.push_str(block.content);
        if let Some(span) = source_span_for_code_block(doc, input_path, &block, output_line) {
            source_map.extend([span]);
        }
        output_line += content_line_count(block.content);
    }
    Ok(PreparedQualityInput {
        source: output,
        source_map,
    })
}

fn source_map_for_code_block(doc: &ImplDoc, input_path: &Path, block: &CodeBlock<'_>) -> SourceMap {
    let mut source_map = SourceMap::new();
    if let Some(span) = source_span_for_code_block(doc, input_path, block, 1) {
        source_map.extend([span]);
    }
    source_map
}

fn source_span_for_code_block(
    doc: &ImplDoc,
    input_path: &Path,
    block: &CodeBlock<'_>,
    generated_start_line: usize,
) -> Option<SourceSpan> {
    let line_count = content_line_count(block.content);
    if line_count == 0 {
        return None;
    }
    Some(SourceSpan {
        markdown_path: doc.path.clone(),
        markdown_start_line: block.content_start_line,
        markdown_end_line: block.content_end_line,
        generated_path: input_path.to_path_buf(),
        generated_start_line,
        generated_end_line: generated_start_line + line_count - 1,
        output_kind: quality_output_kind(doc, block.fence_index),
        extension_key: doc.lang.key().to_string(),
        fence_index: block.fence_index,
    })
}

fn quality_output_kind(doc: &ImplDoc, fence_index: usize) -> OutputKind {
    if doc.test_blocks.iter().any(|block| block.fence_index == fence_index) {
        OutputKind::Test
    } else if doc.source_blocks.iter().any(|block| block.fence_index == fence_index) {
        OutputKind::Source
    } else {
        match doc.doc_kind {
            DocKind::Test => OutputKind::Test,
            DocKind::Source => OutputKind::Source,
        }
    }
}

fn content_line_count(text: &str) -> usize {
    text.lines().count()
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

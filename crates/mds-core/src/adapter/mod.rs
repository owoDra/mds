use std::collections::BTreeMap;
use std::fs;
use std::path::{Component, Path, PathBuf};

use crate::diagnostics::{Diagnostic, RunState};
use crate::hash::sha256;
use crate::model::{
    GeneratedFile, GeneratedKind, ImplDoc, Lang, OutputKind, Package, UseFrom, UseRow,
};

pub(crate) fn output_relative_path(doc: &ImplDoc, kind: OutputKind) -> PathBuf {
    let rel = strip_md_extension(&doc.markdown_relative_path);
    match (doc.lang, kind) {
        (Lang::TypeScript, OutputKind::Types) => with_suffix(&rel, ".types.ts"),
        (Lang::TypeScript, OutputKind::Test) => with_suffix(&rel, ".test.ts"),
        (Lang::TypeScript, OutputKind::Source) => rel,
        (Lang::Python, OutputKind::Types) => with_suffix(&rel, ".pyi"),
        (Lang::Python, OutputKind::Test) => prefixed_file(&rel, "test_", ".py"),
        (Lang::Python, OutputKind::Source) => rel,
        (Lang::Rust, OutputKind::Types) => with_suffix(&rel, "_types.rs"),
        (Lang::Rust, OutputKind::Test) => PathBuf::from(flattened_test_name(&rel)),
        (Lang::Rust, OutputKind::Source) => rel,
    }
}

pub(crate) fn strip_md_extension(path: &Path) -> PathBuf {
    let name = path.file_name().unwrap_or_default().to_string_lossy();
    let stripped = name.strip_suffix(".md").unwrap_or(&name);
    path.with_file_name(stripped)
}

pub(crate) fn with_suffix(path: &Path, suffix: &str) -> PathBuf {
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    path.with_file_name(format!("{stem}{suffix}"))
}

pub(crate) fn prefixed_file(path: &Path, prefix: &str, suffix: &str) -> PathBuf {
    let stem = path.file_stem().unwrap_or_default().to_string_lossy();
    path.with_file_name(format!("{prefix}{stem}{suffix}"))
}

pub(crate) fn flattened_test_name(path: &Path) -> String {
    let mut parts = path
        .with_extension("")
        .components()
        .filter_map(|component| match component {
            Component::Normal(value) => Some(value.to_string_lossy().to_string()),
            _ => None,
        })
        .collect::<Vec<_>>();
    if parts.is_empty() {
        parts.push("generated".to_string());
    }
    format!("{}_test.rs", parts.join("_"))
}

pub(crate) fn imports_for(
    package: &Package,
    doc: &ImplDoc,
    kind: OutputKind,
    output_path: &Path,
) -> String {
    let uses = doc.uses.get(&kind).cloned().unwrap_or_default();
    let mut grouped: Vec<UseRow> = Vec::new();
    for from in [
        UseFrom::Builtin,
        UseFrom::Package,
        UseFrom::Workspace,
        UseFrom::Internal,
    ] {
        let mut named_targets: Vec<UseRow> = Vec::new();
        for row in uses.iter().filter(|row| row.from == from) {
            if row.exposes.is_empty() {
                grouped.push(row.clone());
                continue;
            }
            if let Some(existing) = named_targets
                .iter_mut()
                .find(|existing| existing.target == row.target)
            {
                for expose in &row.exposes {
                    if !existing.exposes.contains(expose) {
                        existing.exposes.push(expose.clone());
                    }
                }
            } else {
                named_targets.push(row.clone());
            }
        }
        grouped.extend(named_targets);
    }

    grouped
        .iter()
        .map(|row| match doc.lang {
            Lang::TypeScript => ts_import(package, row, kind, output_path),
            Lang::Python => py_import(row),
            Lang::Rust => rs_import(row),
        })
        .collect::<Vec<_>>()
        .join("\n")
        + if grouped.is_empty() { "" } else { "\n" }
}

pub(crate) fn ts_import(
    package: &Package,
    row: &UseRow,
    kind: OutputKind,
    output_path: &Path,
) -> String {
    let target = if row.from == UseFrom::Internal {
        let target_path = package
            .root
            .join(&package.config.roots.source)
            .join(format!("{}.ts", row.target));
        relative_module(output_path.parent().unwrap_or(&package.root), &target_path)
    } else {
        row.target.clone()
    };
    if row.exposes.is_empty() {
        format!("import \"{target}\";")
    } else if kind == OutputKind::Types {
        format!(
            "import type {{ {} }} from \"{target}\";",
            row.exposes.join(", ")
        )
    } else {
        format!("import {{ {} }} from \"{target}\";", row.exposes.join(", "))
    }
}

pub(crate) fn py_import(row: &UseRow) -> String {
    let target = if row.from == UseFrom::Internal {
        row.target.replace('/', ".")
    } else {
        row.target.clone()
    };
    if row.exposes.is_empty() {
        format!("import {target}")
    } else {
        format!("from {target} import {}", row.exposes.join(", "))
    }
}

pub(crate) fn rs_import(row: &UseRow) -> String {
    let target = if row.from == UseFrom::Internal {
        format!("crate::{}", row.target.replace('/', "::"))
    } else {
        row.target.replace('/', "::")
    };
    if row.exposes.is_empty() {
        format!("use {target};")
    } else {
        format!("use {target}::{{{}}};", row.exposes.join(", "))
    }
}

pub(crate) fn relative_module(from_dir: &Path, to_file: &Path) -> String {
    let to_without_extension = to_file.with_extension("");
    let from_components = from_dir.components().collect::<Vec<_>>();
    let to_components = to_without_extension.components().collect::<Vec<_>>();
    let mut common = 0;
    while common < from_components.len()
        && common < to_components.len()
        && from_components[common] == to_components[common]
    {
        common += 1;
    }
    let mut parts = Vec::new();
    for _ in common..from_components.len() {
        parts.push("..".to_string());
    }
    for component in &to_components[common..] {
        if let Component::Normal(value) = component {
            parts.push(value.to_string_lossy().to_string());
        }
    }
    if parts.is_empty() {
        ".".to_string()
    } else if parts[0] == ".." {
        parts.join("/")
    } else {
        format!("./{}", parts.join("/"))
    }
}

pub(crate) fn plan_rust_modules(
    package: &Package,
    docs: &[ImplDoc],
    state: &mut RunState,
) -> Option<GeneratedFile> {
    let path = package
        .root
        .join(&package.config.roots.source)
        .join("lib.rs");
    let mut modules = Vec::new();
    for doc in docs.iter().filter(|doc| doc.lang == Lang::Rust) {
        let source = output_relative_path(doc, OutputKind::Source);
        let types = output_relative_path(doc, OutputKind::Types);
        modules.push(module_path(&source));
        modules.push(module_path(&types));
    }
    modules.sort();
    modules.dedup();
    let block = rust_module_block(&modules);
    let content = if path.exists() {
        let old = match fs::read_to_string(&path) {
            Ok(old) => old,
            Err(error) => {
                state.diagnostics.push(Diagnostic::error(
                    Some(path),
                    format!("failed to read Rust module file: {error}"),
                ));
                return None;
            }
        };
        replace_or_append_module_block(&old, &block, &path, state)?
    } else {
        format!(
            "// Generated by mds. Do not edit. Source: src-md. Source-Hash: {}.\n\n{}",
            sha256(&block),
            block
        )
    };
    Some(GeneratedFile {
        path,
        content,
        kind: GeneratedKind::RustModule,
        source_path: None,
    })
}

pub(crate) fn module_path(path: &Path) -> Vec<String> {
    path.with_extension("")
        .components()
        .filter_map(|component| match component {
            Component::Normal(value) => Some(value.to_string_lossy().to_string()),
            _ => None,
        })
        .collect()
}

pub(crate) fn rust_module_block(modules: &[Vec<String>]) -> String {
    let mut tree: BTreeMap<String, Vec<Vec<String>>> = BTreeMap::new();
    for module in modules {
        if let Some((head, tail)) = module.split_first() {
            tree.entry(head.clone()).or_default().push(tail.to_vec());
        }
    }
    let mut out = String::from("// mds:begin generated modules\n");
    for (name, children) in tree {
        if children.iter().all(Vec::is_empty) {
            out.push_str(&format!("pub mod {name};\n"));
        } else {
            out.push_str(&format!("pub mod {name} {{\n"));
            for child in children.into_iter().filter(|child| !child.is_empty()) {
                write_nested_module(&mut out, &child, 1);
            }
            out.push_str("}\n");
        }
    }
    out.push_str("// mds:end generated modules\n");
    out
}

pub(crate) fn write_nested_module(out: &mut String, module: &[String], depth: usize) {
    let indent = "    ".repeat(depth);
    if module.len() == 1 {
        out.push_str(&format!("{indent}pub mod {};\n", module[0]));
    } else {
        out.push_str(&format!("{indent}pub mod {} {{\n", module[0]));
        write_nested_module(out, &module[1..], depth + 1);
        out.push_str(&format!("{indent}}}\n"));
    }
}

pub(crate) fn replace_or_append_module_block(
    old: &str,
    block: &str,
    path: &Path,
    state: &mut RunState,
) -> Option<String> {
    let begin = "// mds:begin generated modules";
    let end = "// mds:end generated modules";
    let begin_pos = old.find(begin);
    let end_pos = old.find(end);
    match (begin_pos, end_pos) {
        (Some(begin_pos), Some(end_pos)) if begin_pos < end_pos => {
            let end_after = end_pos + end.len();
            let mut content = String::new();
            content.push_str(old[..begin_pos].trim_end());
            content.push('\n');
            content.push_str(block.trim_end());
            content.push('\n');
            content.push_str(old[end_after..].trim_start_matches('\n'));
            Some(content)
        }
        (None, None) => Some(format!("{}\n\n{}", old.trim_end(), block)),
        _ => {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                "Rust module block begin/end markers are inconsistent",
            ));
            None
        }
    }
}

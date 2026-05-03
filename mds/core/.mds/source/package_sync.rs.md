# src/package_sync.rs

## Purpose

Migrated implementation source for `src/package_sync.rs`.

## Contract

- Preserve the behavior of the pre-migration Rust source.
- This file is synchronized into `.build/rust/mds/core/src/package_sync.rs`.

## Imports

| Kind | From | Target | Symbols | Via | Summary | Code |
| --- | --- | --- | --- | --- | --- | --- |
| rust-use | builtin | std::collections | BTreeMap | std |  | `use std::collections::BTreeMap;` |
| rust-use | builtin | std | fs | std |  | `use std::fs;` |
| rust-use | builtin | std::path | Path, PathBuf | std |  | `use std::path::{Path, PathBuf};` |
| rust-use | internal | crate::diagnostics | Diagnostic, RunState | crate |  | `use crate::diagnostics::{Diagnostic, RunState};` |
| rust-use | internal | crate::diff | unified_diff | crate |  | `use crate::diff::unified_diff;` |
| rust-use | internal | crate::model | Package | crate |  | `use crate::model::Package;` |
| rust-use | internal | crate::package | read_package_metadata | crate |  | `use crate::package::read_package_metadata;` |


## Source


````rs
pub(crate) fn sync_package_md(
    package: &Package,
    check: bool,
    state: &mut RunState,
) -> Result<(), String> {
    if package.config.package_sync_hook_enabled {
        let command = package
            .config
            .package_sync_hook
            .as_deref()
            .unwrap_or("mds package sync");
        state
            .stdout
            .push_str(&format!("package sync hook command: {command}\n"));
    }
    let Some((path, old, new)) = planned_package_overview(package, state) else {
        return Ok(());
    };
    if old == new {
        state
            .stdout
            .push_str(&format!("package sync ok: {}\n", package.root.display()));
        return Ok(());
    }
    state.stdout.push_str(&unified_diff(&path, &old, &new));
    if check {
        state.diagnostics.push(Diagnostic::error(
            Some(path),
            "dependency snapshot is not synchronized with package metadata; run `mds package sync`",
        ));
    } else {
        fs::write(&path, &new)
            .map_err(|error| format!("failed to write {}: {error}", path.display()))?;
        state.generated.push(path.clone());
        state
            .stdout
            .push_str(&format!("package sync ok: {}\n", package.root.display()));
    }
    Ok(())
}
````

````rs
pub(crate) fn planned_package_overview(
    package: &Package,
    state: &mut RunState,
) -> Option<(PathBuf, String, String)> {
    let path = package.root.join(".mds/source/overview.md");
    let old = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) => {
            state.diagnostics.push(Diagnostic::error(
                Some(path.clone()),
                format!("failed to read source overview for package sync: {error}"),
            ));
            return None;
        }
    };
    let metadata = read_package_metadata(package, state)?;
    let new = replace_managed_region(
        &old,
        "package-summary",
        &package_summary_table(&metadata.name, &metadata.version),
        &path,
        state,
    )
    .and_then(|text| {
        replace_managed_region(
            &text,
            "dependencies",
            &dependency_table(&metadata.dependencies),
            &path,
            state,
        )
    })
    .and_then(|text| {
        replace_managed_region(
            &text,
            "dev-dependencies",
            &dependency_table(&metadata.dev_dependencies),
            &path,
            state,
        )
    })?;
    Some((path, old, new))
}
````

````rs
fn replace_managed_region(
    text: &str,
    name: &str,
    replacement: &str,
    path: &Path,
    state: &mut RunState,
) -> Option<String> {
    if has_managed_section(text, name) {
        return replace_managed_section(text, name, replacement, path, state);
    }
    if text.contains(&format!("<!-- mds:begin {name} -->")) {
        return replace_managed_block(text, name, replacement, path, state);
    }
    state.diagnostics.push(Diagnostic::error(
        Some(path.to_path_buf()),
        format!(
            "source overview is missing managed section `{}`",
            managed_section_heading(name)
        ),
    ));
    None
}
````

````rs
fn has_managed_section(text: &str, name: &str) -> bool {
    let heading = format!("### {}", managed_section_heading(name));
    text.lines().any(|line| line.trim() == heading)
}
````

````rs
fn replace_managed_section(
    text: &str,
    name: &str,
    replacement: &str,
    path: &Path,
    state: &mut RunState,
) -> Option<String> {
    let heading = format!("### {}", managed_section_heading(name));
    let lines = text.lines().collect::<Vec<_>>();
    let Some(start) = lines.iter().position(|line| line.trim() == heading) else {
        state.diagnostics.push(Diagnostic::error(
            Some(path.to_path_buf()),
            format!(
                "source overview is missing managed section `{}`",
                managed_section_heading(name)
            ),
        ));
        return None;
    };
    let mut end = start + 1;
    while end < lines.len() {
        let trimmed = lines[end].trim();
        if trimmed.starts_with("### ") || trimmed.starts_with("## ") {
            break;
        }
        end += 1;
    }

    let mut output = String::new();
    output.push_str(&lines[..=start].join("\n"));
    output.push_str("\n\n");
    output.push_str(replacement.trim_end());
    output.push('\n');
    if end < lines.len() {
        output.push('\n');
        output.push_str(&lines[end..].join("\n"));
    }
    Some(output)
}
````

````rs
fn replace_managed_block(
    text: &str,
    name: &str,
    replacement: &str,
    path: &Path,
    state: &mut RunState,
) -> Option<String> {
    let begin = format!("<!-- mds:begin {name} -->");
    let end = format!("<!-- mds:end {name} -->");
    let Some(start) = text.find(&begin) else {
        return None;
    };
    let search_from = start + begin.len();
    let Some(relative_end) = text[search_from..].find(&end) else {
        state.diagnostics.push(Diagnostic::error(
            Some(path.to_path_buf()),
            format!("source overview is missing end marker for managed block `{name}`"),
        ));
        return None;
    };
    let end_index = search_from + relative_end;
    let mut output = String::new();
    output.push_str(&text[..start]);
    output.push_str(&begin);
    output.push('\n');
    output.push_str(replacement.trim_end());
    output.push('\n');
    output.push_str(&text[end_index..]);
    Some(output)
}
````

````rs
fn managed_section_heading(name: &str) -> &'static str {
    match name {
        "package-summary" => "Package Summary",
        "dependencies" => "Dependencies",
        "dev-dependencies" => "Dev Dependencies",
        _ => "Managed Section",
    }
}
````

````rs
fn package_summary_table(name: &str, version: &str) -> String {
    format!("| Name | Version |\n| --- | --- |\n| {name} | {version} |\n")
}
````

````rs
fn dependency_table(dependencies: &std::collections::HashMap<String, String>) -> String {
    let sorted = dependencies
        .iter()
        .map(|(name, version)| (name.clone(), version.clone()))
        .collect::<BTreeMap<_, _>>();
    let mut output = String::from("| Name | Version | Summary |\n| --- | --- | --- |\n");
    for (name, version) in sorted {
        output.push_str(&format!("| {name} | {version} |  |\n"));
    }
    output
}
````



# src/package_sync/mod.rs

## Purpose

Migrated implementation source for `src/package_sync/mod.rs`.

## Contract

- Preserve the behavior of the pre-migration Rust source.
- This file is synchronized into `.build/rust/mds-core/src/package_sync/mod.rs`.

## Source

````rs
use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use crate::diagnostics::{Diagnostic, RunState};
use crate::diff::unified_diff;
use crate::model::Package;
use crate::package::read_package_metadata;

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
            .unwrap_or("mds package sync --check");
        state
            .stdout
            .push_str(&format!("package sync hook command: {command}\n"));
    }
    let path = package.root.join("package.md");
    let old = fs::read_to_string(&path)
        .map_err(|error| format!("failed to read {}: {error}", path.display()))?;
    let Some(new) = planned_package_md(package, &old, &path, state) else {
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
            "package.md is not synchronized with package metadata",
        ));
    } else {
        fs::write(&path, new)
            .map_err(|error| format!("failed to write {}: {error}", path.display()))?;
        state.generated.push(path);
    }
    Ok(())
}

fn planned_package_md(
    package: &Package,
    old: &str,
    path: &Path,
    state: &mut RunState,
) -> Option<String> {
    let metadata = read_package_metadata(package, state)?;
    let mut output = String::new();
    let title = old
        .lines()
        .find(|line| line.starts_with("# "))
        .unwrap_or("# Package");
    output.push_str(title.trim_end());
    output.push_str("\n\n");
    output.push_str("## Package\n\n");
    output.push_str("| Name | Version |\n| --- | --- |\n");
    output.push_str(&format!("| {} | {} |\n\n", metadata.name, metadata.version));
    output.push_str("## Dependencies\n\n");
    output.push_str(&dependency_table(&metadata.dependencies));
    output.push_str("\n## Dev Dependencies\n\n");
    output.push_str(&dependency_table(&metadata.dev_dependencies));

    for (name, body) in sections_in_order(old) {
        if matches!(
            name.as_str(),
            "Package" | "Dependencies" | "Dev Dependencies"
        ) {
            continue;
        }
        output.push('\n');
        output.push_str(&format!("## {name}\n"));
        if !body.trim().is_empty() {
            output.push('\n');
            output.push_str(body.trim_matches('\n'));
            output.push('\n');
        }
    }
    if !output.ends_with('\n') {
        output.push('\n');
    }
    if !old.contains("## Package")
        || !old.contains("## Dependencies")
        || !old.contains("## Dev Dependencies")
    {
        state.diagnostics.push(Diagnostic::error(
            Some(path.to_path_buf()),
            "package.md requires Package, Dependencies, and Dev Dependencies sections for sync",
        ));
        return None;
    }
    for (name, body) in sections_in_order(old) {
        if matches!(
            name.as_str(),
            "Package" | "Dependencies" | "Dev Dependencies"
        ) && contains_manual_mixed_content(&body)
        {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("package.md ## {name} contains hand-written content inside managed sync section"),
            ));
            return None;
        }
    }
    Some(output)
}

fn contains_manual_mixed_content(body: &str) -> bool {
    body.lines().any(|line| {
        let line = line.trim();
        !line.is_empty() && !line.starts_with('|')
    })
}

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

fn sections_in_order(text: &str) -> Vec<(String, String)> {
    let mut sections = Vec::new();
    let mut current = None;
    let mut body = String::new();
    for line in text.lines() {
        if let Some(title) = line.strip_prefix("## ") {
            if let Some(name) = current.replace(title.trim().to_string()) {
                sections.push((name, body.clone()));
                body.clear();
            }
        } else if current.is_some() {
            body.push_str(line);
            body.push('\n');
        }
    }
    if let Some(name) = current {
        sections.push((name, body));
    }
    sections
}
````

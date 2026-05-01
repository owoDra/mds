# src/package/mod.rs

## Purpose

Migrated implementation source for `src/package/mod.rs`.

## Contract

- Preserve the behavior of the pre-migration Rust source.
- This file is synchronized into `.build/rust/mds-core/src/package/mod.rs`.

## Source

````rs
use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use crate::config::merge_config_file;
use crate::diagnostics::{Diagnostic, RunState};
use crate::fs_utils::{collect_files, is_excluded};
use crate::markdown::{sections_with_labels, validate_markdown_links};
use crate::model::{MetadataKind, Package, PackageMetadata};
use crate::table::parse_table_with_labels;

pub fn discover_packages(
    cwd: &Path,
    package: Option<&Path>,
    state: &mut RunState,
) -> Result<Vec<Package>, String> {
    let root_config = cwd.join("mds.config.toml");
    let mut base_config = crate::model::Config::default();
    if root_config.exists() {
        merge_config_file(&mut base_config, &root_config, state);
    }
    if let Some(package) = package {
        let root = if package.is_absolute() {
            package.to_path_buf()
        } else {
            cwd.join(package)
        };
        return Ok(load_package(&root, &base_config, state)
            .into_iter()
            .collect());
    }

    let mut packages = Vec::new();
    for path in collect_files(cwd, true)? {
        if path.file_name() == Some(OsStr::new("mds.config.toml")) {
            let Some(root) = path.parent() else {
                continue;
            };
            if let Some(package) = load_package(root, &base_config, state) {
                packages.push(package);
            }
        }
    }
    packages.sort_by(|left, right| left.root.cmp(&right.root));
    Ok(packages)
}

pub fn load_package(
    root: &Path,
    base_config: &crate::model::Config,
    state: &mut RunState,
) -> Option<Package> {
    let config_path = root.join("mds.config.toml");
    if !config_path.exists() {
        state.diagnostics.push(Diagnostic::error(
            Some(root.to_path_buf()),
            "mds.config.toml is required for an mds package",
        ));
        return None;
    }

    let mut config = base_config.clone();
    merge_config_file(&mut config, &config_path, state)?;
    if !config.enabled {
        return None;
    }

    let package_md = root.join("package.md");
    if !package_md.exists() {
        state.diagnostics.push(Diagnostic::error(
            Some(package_md),
            "enabled package requires package.md",
        ));
        return None;
    }

    let metadata_kind = match metadata_kind(root) {
        Some(kind) => kind,
        None => {
            state.diagnostics.push(Diagnostic::error(
                Some(root.to_path_buf()),
                "enabled package requires package.json, pyproject.toml, or Cargo.toml",
            ));
            return None;
        }
    };

    Some(Package {
        root: root.to_path_buf(),
        config,
        metadata_kind,
    })
}

pub fn metadata_kind(root: &Path) -> Option<MetadataKind> {
    if root.join("package.json").exists() {
        Some(MetadataKind::Node)
    } else if root.join("pyproject.toml").exists() {
        Some(MetadataKind::Python)
    } else if root.join("Cargo.toml").exists() {
        Some(MetadataKind::Rust)
    } else {
        None
    }
}

pub fn validate_package_md(package: &Package, state: &mut RunState) {
    let path = package.root.join("package.md");
    let text = match fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) => {
            state.diagnostics.push(Diagnostic::error(
                Some(path),
                format!("failed to read package.md: {error}"),
            ));
            return;
        }
    };
    let sections = sections_with_labels(&text, &package.config.label_overrides);
    validate_markdown_links(&path, &text, state);
    for required in ["Package", "Dependencies", "Dev Dependencies", "Rules"] {
        if !sections.contains_key(required) {
            state.diagnostics.push(Diagnostic::error(
                Some(path.clone()),
                format!("package.md requires ## {required}"),
            ));
        }
    }

    if let Some(package_section) = sections.get("Package") {
        if let Some(rows) = parse_table_with_labels(
            package_section,
            &["Name", "Version"],
            &path,
            &package.config.label_overrides,
            state,
        ) {
            if rows.is_empty() {
                state.diagnostics.push(Diagnostic::error(
                    Some(path.clone()),
                    "package.md Package table must contain at least one row",
                ));
            }
            if let Some(metadata) = read_package_metadata(package, state) {
                if let Some(row) = rows.first() {
                    let doc_name = row.get("name").map(String::as_str).unwrap_or_default();
                    let doc_version = row.get("version").map(String::as_str).unwrap_or_default();
                    if doc_name != metadata.name {
                        state.diagnostics.push(Diagnostic::error(
                            Some(path.clone()),
                            format!(
                                "package.md Package.Name `{doc_name}` does not match metadata `{}`",
                                metadata.name
                            ),
                        ));
                    }
                    if doc_version != metadata.version {
                        state.diagnostics.push(Diagnostic::error(
                            Some(path.clone()),
                            format!(
                                "package.md Package.Version `{doc_version}` does not match metadata `{}`",
                                metadata.version
                            ),
                        ));
                    }
                }
            }
        } else {
            state.diagnostics.push(Diagnostic::error(
                Some(path.clone()),
                "package.md Package section requires Name and Version table columns",
            ));
        }
    }

    validate_dependency_section(package, &sections, "Dependencies", false, &path, state);
    validate_dependency_section(package, &sections, "Dev Dependencies", true, &path, state);
}

pub fn read_package_metadata(package: &Package, state: &mut RunState) -> Option<PackageMetadata> {
    match package.metadata_kind {
        MetadataKind::Node => read_node_metadata(&package.root.join("package.json"), state),
        MetadataKind::Python => read_python_metadata(&package.root.join("pyproject.toml"), state),
        MetadataKind::Rust => {
            read_toml_metadata(&package.root.join("Cargo.toml"), &["package"], state)
        }
    }
}

pub fn read_node_metadata(path: &Path, state: &mut RunState) -> Option<PackageMetadata> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) => {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("failed to read package metadata: {error}"),
            ));
            return None;
        }
    };
    let value = match serde_json::from_str::<serde_json::Value>(&text) {
        Ok(value) => value,
        Err(error) => {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("failed to parse package.json: {error}"),
            ));
            return None;
        }
    };
    let name = json_string_value(value.get("name"));
    let version = json_string_value(value.get("version"));
    match (name, version) {
        (Some(name), Some(version)) => Some(PackageMetadata {
            name,
            version,
            dependencies: json_dependency_object(value.get("dependencies")),
            dev_dependencies: json_dependency_object(value.get("devDependencies")),
        }),
        _ => {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                "package metadata requires name and version",
            ));
            None
        }
    }
}

fn json_string_value(value: Option<&serde_json::Value>) -> Option<String> {
    value?.as_str().map(ToOwned::to_owned)
}

fn json_dependency_object(value: Option<&serde_json::Value>) -> HashMap<String, String> {
    let Some(object) = value.and_then(serde_json::Value::as_object) else {
        return HashMap::new();
    };
    object
        .iter()
        .map(|(name, value)| (name.clone(), json_dependency_version(value)))
        .collect()
}

fn json_dependency_version(value: &serde_json::Value) -> String {
    if let Some(version) = value.as_str() {
        return version.to_string();
    }
    if let Some(version) = value.get("version").and_then(serde_json::Value::as_str) {
        return version.to_string();
    }
    value.to_string()
}

pub fn read_toml_metadata(
    path: &Path,
    table_path: &[&str],
    state: &mut RunState,
) -> Option<PackageMetadata> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) => {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("failed to read package metadata: {error}"),
            ));
            return None;
        }
    };
    let value = match text.parse::<toml::Value>() {
        Ok(value) => value,
        Err(error) => {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("failed to parse TOML package metadata: {error}"),
            ));
            return None;
        }
    };
    let section_name = table_path.join(".");
    let fields = toml_table_path(&value, table_path);
    let name = fields.and_then(|fields| toml_string_value(fields.get("name")));
    let version = fields.and_then(|fields| toml_string_value(fields.get("version")));
    match (name, version) {
        (Some(name), Some(version)) => Some(PackageMetadata {
            name,
            version,
            dependencies: toml_dependency_section(&value, "dependencies"),
            dev_dependencies: toml_dependency_section(&value, "dev-dependencies"),
        }),
        _ => {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("package metadata requires [{section_name}] name and version"),
            ));
            None
        }
    }
}

pub fn read_python_metadata(path: &Path, state: &mut RunState) -> Option<PackageMetadata> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) => {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("failed to read package metadata: {error}"),
            ));
            return None;
        }
    };
    let value = match text.parse::<toml::Value>() {
        Ok(value) => value,
        Err(error) => {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("failed to parse pyproject.toml: {error}"),
            ));
            return None;
        }
    };
    let project = toml_table_path(&value, &["project"]);
    let name = project.and_then(|project| toml_string_value(project.get("name")));
    let version = project.and_then(|project| toml_string_value(project.get("version")));
    match (name, version) {
        (Some(name), Some(version)) => Some(PackageMetadata {
            name,
            version,
            dependencies: project
                .and_then(|project| project.get("dependencies"))
                .map(toml_dependency_array)
                .unwrap_or_default(),
            dev_dependencies: pyproject_optional_dependencies(&value, "dev"),
        }),
        _ => {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                "package metadata requires [project] name and version",
            ));
            None
        }
    }
}

fn toml_table_path<'a>(
    value: &'a toml::Value,
    table_path: &[&str],
) -> Option<&'a toml::map::Map<String, toml::Value>> {
    let mut current = value;
    for segment in table_path {
        current = current.get(*segment)?;
    }
    current.as_table()
}

fn toml_string_value(value: Option<&toml::Value>) -> Option<String> {
    value?.as_str().map(ToOwned::to_owned)
}

fn toml_dependency_section(value: &toml::Value, section_name: &str) -> HashMap<String, String> {
    let Some(table) = toml_table_path(value, &[section_name]) else {
        return HashMap::new();
    };
    table
        .iter()
        .map(|(name, value)| (name.clone(), toml_dependency_version(value)))
        .collect()
}

fn toml_dependency_version(value: &toml::Value) -> String {
    if let Some(version) = value.as_str() {
        return version.to_string();
    }
    if let Some(version) = value.get("version").and_then(toml::Value::as_str) {
        return version.to_string();
    }
    value.to_string()
}

fn pyproject_optional_dependencies(value: &toml::Value, group: &str) -> HashMap<String, String> {
    toml_table_path(value, &["project", "optional-dependencies"])
        .and_then(|section| section.get(group))
        .map(toml_dependency_array)
        .unwrap_or_default()
}

fn toml_dependency_array(value: &toml::Value) -> HashMap<String, String> {
    let Some(array) = value.as_array() else {
        return HashMap::new();
    };
    array
        .iter()
        .filter_map(toml::Value::as_str)
        .map(split_dependency_spec)
        .collect()
}

fn split_dependency_spec(value: &str) -> (String, String) {
    for marker in [">=", "<=", "==", "~=", "!=", ">", "<"] {
        if let Some((name, version)) = value.split_once(marker) {
            return (
                name.trim().to_string(),
                format!("{marker}{}", version.trim()),
            );
        }
    }
    (value.trim().to_string(), String::new())
}

fn validate_dependency_section(
    package: &Package,
    sections: &HashMap<String, String>,
    section_name: &str,
    dev: bool,
    path: &Path,
    state: &mut RunState,
) {
    let Some(metadata) = read_package_metadata(package, state) else {
        return;
    };
    let expected = if dev {
        metadata.dev_dependencies
    } else {
        metadata.dependencies
    };
    let Some(section) = sections.get(section_name) else {
        return;
    };
    let Some(rows) = parse_table_with_labels(
        section,
        &["Name", "Version", "Summary"],
        path,
        &package.config.label_overrides,
        state,
    ) else {
        state.diagnostics.push(Diagnostic::error(
            Some(path.to_path_buf()),
            format!(
                "package.md {section_name} section requires Name, Version, and Summary table columns"
            ),
        ));
        return;
    };
    let actual = rows
        .iter()
        .filter_map(|row| {
            let name = row.get("name")?.trim();
            if name.is_empty() {
                return None;
            }
            Some((
                name.to_string(),
                row.get("version").cloned().unwrap_or_default(),
            ))
        })
        .collect::<HashMap<_, _>>();
    for (name, version) in &expected {
        match actual.get(name) {
            Some(actual_version) if actual_version == version => {}
            Some(actual_version) => state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!(
                    "package.md {section_name} dependency `{name}` version `{actual_version}` does not match metadata `{version}`"
                ),
            )),
            None => state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("package.md {section_name} is missing dependency `{name}`"),
            )),
        }
    }
    for name in actual.keys() {
        if !expected.contains_key(name) {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!(
                    "package.md {section_name} contains dependency `{name}` not present in metadata"
                ),
            ));
        }
    }
}

pub fn rust_expose_modules(package: &Package, state: &mut RunState) -> Vec<Vec<String>> {
    let markdown_root = package.root.join(&package.config.roots.markdown);
    let Ok(files) = collect_files(&markdown_root, false) else {
        return Vec::new();
    };
    let mut modules = Vec::new();
    for path in files
        .into_iter()
        .filter(|path| !is_excluded(&package.root, path, &package.config.excludes))
        .filter(|path| path.file_name() == Some(OsStr::new("index.md")))
    {
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        validate_markdown_links(&path, &text, state);
        let sections = sections_with_labels(&text, &package.config.label_overrides);
        let Some(exposes_section) = sections.get("Exposes") else {
            continue;
        };
        let Some(rows) = parse_table_with_labels(
            exposes_section,
            &["Kind", "Name", "Target", "Summary"],
            &path,
            &package.config.label_overrides,
            state,
        ) else {
            continue;
        };
        for row in rows {
            let target = row.get("target").map(String::as_str).unwrap_or_default();
            if target.is_empty() {
                continue;
            }
            modules.push(
                target
                    .split('/')
                    .filter(|part| !part.is_empty())
                    .map(ToOwned::to_owned)
                    .collect(),
            );
        }
    }
    modules
}

pub fn validate_index_docs(package: &Package, state: &mut RunState) {
    let markdown_root = package.root.join(&package.config.roots.markdown);
    if !markdown_root.exists() {
        return;
    }
    let Ok(files) = collect_files(&markdown_root, false) else {
        return;
    };
    for path in files
        .into_iter()
        .filter(|path| !is_excluded(&package.root, path, &package.config.excludes))
        .filter(|path| path.file_name() == Some(OsStr::new("index.md")))
    {
        let text = match fs::read_to_string(&path) {
            Ok(text) => text,
            Err(error) => {
                state.diagnostics.push(Diagnostic::error(
                    Some(path),
                    format!("failed to read index.md: {error}"),
                ));
                continue;
            }
        };
        let sections = sections_with_labels(&text, &package.config.label_overrides);
        for required in ["Purpose", "Architecture", "Exposes", "Rules"] {
            if !sections.contains_key(required) {
                state.diagnostics.push(Diagnostic::error(
                    Some(path.clone()),
                    format!("index.md requires ## {required}"),
                ));
            }
        }
        if let Some(exposes_section) = sections.get("Exposes") {
            let Some(rows) = parse_table_with_labels(
                exposes_section,
                &["Kind", "Name", "Target", "Summary"],
                &path,
                &package.config.label_overrides,
                state,
            ) else {
                state.diagnostics.push(Diagnostic::error(
                    Some(path.clone()),
                    "index.md Exposes section requires Kind, Name, Target, and Summary table columns",
                ));
                continue;
            };
            validate_expose_rows(&rows, &path, state);
        }
    }
}

pub fn validate_expose_rows(rows: &[HashMap<String, String>], path: &Path, state: &mut RunState) {
    let mut seen = HashSet::new();
    for row in rows {
        let kind = row.get("kind").map(String::as_str).unwrap_or_default();
        let name = row.get("name").map(String::as_str).unwrap_or_default();
        if !matches!(kind, "type" | "value" | "function" | "class" | "module") {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!(
                    "Expose.Kind must be one of type, value, function, class, module: `{kind}`"
                ),
            ));
        }
        if name.is_empty() {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                "Expose.Name is required",
            ));
        }
        if !seen.insert((kind.to_string(), name.to_string())) {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                "duplicate Expose row with the same Kind and Name",
            ));
        }
    }
}
````

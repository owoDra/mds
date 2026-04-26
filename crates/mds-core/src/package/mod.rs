use std::collections::{HashMap, HashSet};
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};

use crate::config::{merge_config_file, parse_string};
use crate::diagnostics::{Diagnostic, RunState};
use crate::fs_utils::collect_files;
use crate::markdown::sections;
use crate::model::{MetadataKind, Package, PackageMetadata};
use crate::table::parse_table;

pub(crate) fn discover_packages(
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

pub(crate) fn load_package(
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

    if metadata_kind == MetadataKind::Python && config.roots.source == PathBuf::from("src") {
        config.roots.source = PathBuf::from("src");
    }

    Some(Package {
        root: root.to_path_buf(),
        config,
        metadata_kind,
    })
}

pub(crate) fn metadata_kind(root: &Path) -> Option<MetadataKind> {
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

pub(crate) fn validate_package_md(package: &Package, state: &mut RunState) {
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
    let sections = sections(&text);
    for required in ["Package", "Dependencies", "Dev Dependencies", "Rules"] {
        if !sections.contains_key(required) {
            state.diagnostics.push(Diagnostic::error(
                Some(path.clone()),
                format!("package.md requires ## {required}"),
            ));
        }
    }

    if let Some(package_section) = sections.get("Package") {
        if let Some(rows) = parse_table(package_section, &["Name", "Version"], &path, state) {
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

pub(crate) fn read_package_metadata(
    package: &Package,
    state: &mut RunState,
) -> Option<PackageMetadata> {
    match package.metadata_kind {
        MetadataKind::Node => read_node_metadata(&package.root.join("package.json"), state),
        MetadataKind::Python => {
            read_toml_metadata(&package.root.join("pyproject.toml"), &["project"], state)
        }
        MetadataKind::Rust => {
            read_toml_metadata(&package.root.join("Cargo.toml"), &["package"], state)
        }
    }
}

pub(crate) fn read_node_metadata(path: &Path, state: &mut RunState) -> Option<PackageMetadata> {
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
    let name = json_string_field(&text, "name");
    let version = json_string_field(&text, "version");
    match (name, version) {
        (Some(name), Some(version)) => Some(PackageMetadata {
            name,
            version,
            dependencies: json_object_field(&text, "dependencies"),
            dev_dependencies: json_object_field(&text, "devDependencies"),
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

fn json_object_field(text: &str, key: &str) -> HashMap<String, String> {
    let mut values = HashMap::new();
    let pattern = format!("\"{key}\"");
    let Some(after_key) = text.split_once(&pattern).map(|(_, value)| value) else {
        return values;
    };
    let Some(after_colon) = after_key
        .split_once(':')
        .map(|(_, value)| value.trim_start())
    else {
        return values;
    };
    let Some(mut body) = after_colon.strip_prefix('{') else {
        return values;
    };
    let Some(end) = body.find('}') else {
        return values;
    };
    body = &body[..end];
    for entry in body.split(',') {
        let Some((raw_key, raw_value)) = entry.split_once(':') else {
            continue;
        };
        let key = raw_key.trim().trim_matches('"');
        let value = raw_value.trim().trim_matches('"');
        if !key.is_empty() {
            values.insert(key.to_string(), value.to_string());
        }
    }
    values
}

pub(crate) fn json_string_field(text: &str, key: &str) -> Option<String> {
    let pattern = format!("\"{key}\"");
    let after_key = text.split_once(&pattern)?.1;
    let after_colon = after_key.split_once(':')?.1.trim_start();
    let value = after_colon.strip_prefix('"')?;
    let end = value.find('"')?;
    Some(value[..end].to_string())
}

pub(crate) fn read_toml_metadata(
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
    let section_name = table_path.join(".");
    let fields = simple_toml_section(&text, &section_name);
    let name = fields.get("name").cloned();
    let version = fields.get("version").cloned();
    match (name, version) {
        (Some(name), Some(version)) => Some(PackageMetadata {
            name,
            version,
            dependencies: simple_toml_section(&text, "dependencies"),
            dev_dependencies: simple_toml_section(&text, "dev-dependencies"),
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
    let Some(rows) = parse_table(section, &["Name", "Version"], path, state) else {
        state.diagnostics.push(Diagnostic::error(
            Some(path.to_path_buf()),
            format!("package.md {section_name} section requires Name and Version table columns"),
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

pub(crate) fn rust_expose_modules(package: &Package, state: &mut RunState) -> Vec<Vec<String>> {
    let markdown_root = package.root.join(&package.config.roots.markdown);
    let Ok(files) = collect_files(&markdown_root, false) else {
        return Vec::new();
    };
    let mut modules = Vec::new();
    for path in files
        .into_iter()
        .filter(|path| path.file_name() == Some(OsStr::new("index.md")))
    {
        let Ok(text) = fs::read_to_string(&path) else {
            continue;
        };
        let sections = sections(&text);
        let Some(exposes_section) = sections.get("Exposes") else {
            continue;
        };
        let Some(rows) = parse_table(
            exposes_section,
            &["Kind", "Name", "Target", "Summary"],
            &path,
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

pub(crate) fn simple_toml_section(text: &str, section_name: &str) -> HashMap<String, String> {
    let mut current = String::new();
    let mut fields = HashMap::new();
    for raw_line in text.lines() {
        let line = raw_line
            .split_once('#')
            .map(|(line, _)| line)
            .unwrap_or(raw_line)
            .trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            current = line.trim_matches(['[', ']']).to_string();
            continue;
        }
        if current != section_name {
            continue;
        }
        if let Some((key, value)) = line.split_once('=') {
            fields.insert(key.trim().to_string(), parse_string(value.trim()));
        }
    }
    fields
}

pub(crate) fn validate_index_docs(package: &Package, state: &mut RunState) {
    let markdown_root = package.root.join(&package.config.roots.markdown);
    if !markdown_root.exists() {
        return;
    }
    let Ok(files) = collect_files(&markdown_root, false) else {
        return;
    };
    for path in files
        .into_iter()
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
        let sections = sections(&text);
        for required in ["Purpose", "Architecture", "Exposes", "Rules"] {
            if !sections.contains_key(required) {
                state.diagnostics.push(Diagnostic::error(
                    Some(path.clone()),
                    format!("index.md requires ## {required}"),
                ));
            }
        }
        if let Some(exposes_section) = sections.get("Exposes") {
            let Some(rows) = parse_table(
                exposes_section,
                &["Kind", "Name", "Target", "Summary"],
                &path,
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

pub(crate) fn validate_expose_rows(
    rows: &[HashMap<String, String>],
    path: &Path,
    state: &mut RunState,
) {
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

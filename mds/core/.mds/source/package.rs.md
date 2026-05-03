# src/package.rs

## Purpose

Migrated implementation source for `src/package.rs`.

## Contract

- Preserve the behavior of the pre-migration Rust source.
- This file is synchronized into `.build/rust/mds/core/src/package.rs`.

## Imports

| Kind | From | Target | Symbols | Via | Summary | Code |
| --- | --- | --- | --- | --- | --- | --- |
| rust-use | builtin | std::collections | HashMap, HashSet | std |  | `use std::collections::{HashMap, HashSet};` |
| rust-use | builtin | std::ffi | OsStr | std |  | `use std::ffi::OsStr;` |
| rust-use | builtin | std | fs | std |  | `use std::fs;` |
| rust-use | builtin | std::path | Path | std |  | `use std::path::Path;` |
| rust-use | external | regex | Regex | regex |  | `use regex::Regex;` |
| rust-use | internal | crate::config | merge_config_file | crate |  | `use crate::config::merge_config_file;` |
| rust-use | internal | crate | descriptor | crate |  | `use crate::descriptor;` |
| rust-use | internal | crate::diagnostics | Diagnostic, RunState | crate |  | `use crate::diagnostics::{Diagnostic, RunState};` |
| rust-use | internal | crate::fs_utils | collect_files, is_excluded | crate |  | `use crate::fs_utils::{collect_files, is_excluded};` |
| rust-use | internal | crate::markdown | sections_with_labels, source_markdown_root, validate_markdown_links | crate |  | `use crate::markdown::{sections_with_labels, source_markdown_root, validate_markdown_links};` |
| rust-use | internal | crate::model | Package, PackageMetadata | crate |  | `use crate::model::{Package, PackageMetadata};` |
| rust-use | internal | crate::table | parse_table_with_labels | crate |  | `use crate::table::parse_table_with_labels;` |


## Source


````rs
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
````

````rs
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

    let package_manager = match descriptor::detect_package_manager(root) {
        Some(manager) => manager,
        None => {
            state.diagnostics.push(Diagnostic::error(
                Some(root.to_path_buf()),
                "enabled package requires a recognized package manager metadata file",
            ));
            return None;
        }
    };

    Some(Package {
        root: root.to_path_buf(),
        config,
        package_manager_id: package_manager.id,
    })
}
````

````rs
pub fn validate_package_md(package: &Package, state: &mut RunState) {
    let Some((path, old, new)) = crate::package_sync::planned_package_overview(package, state)
    else {
        return;
    };
    if old != new {
        state.diagnostics.push(Diagnostic::error(
            Some(path),
            "dependency snapshot is not synchronized with package metadata; run `mds package sync`",
        ));
    }
}
````

````rs
pub fn read_package_metadata(package: &Package, state: &mut RunState) -> Option<PackageMetadata> {
    let Some(manager) = descriptor::package_manager_for_id(&package.package_manager_id) else {
        state.diagnostics.push(Diagnostic::error(
            Some(package.root.clone()),
            format!("unknown package manager `{}`", package.package_manager_id),
        ));
        return None;
    };
    let Some(path) = manager.metadata_path(&package.root) else {
        state.diagnostics.push(Diagnostic::error(
            Some(package.root.clone()),
            format!("package manager `{}` metadata file is missing", manager.id),
        ));
        return None;
    };
    match manager.metadata_reader.as_str() {
        "node-package-json" | "vcpkg-json" => read_node_metadata(&path, state),
        "pyproject-toml" => read_python_metadata(&path, state),
        "cargo-toml" => read_toml_metadata(&path, &["package"], state),
        "pubspec-yaml" => read_pubspec_metadata(&path, state),
        "dotnet-xml" => read_dotnet_metadata(&path, state),
        "cmake-text" => read_cmake_metadata(&path, state),
        "meson-text" => read_meson_metadata(&path, state),
        "conan-text" => read_conan_metadata(&path, state),
        "bundler" => read_bundler_metadata(&package.root, &path, state),
        "zig-zon" => read_zig_metadata(&path, state),
        _ => read_generic_metadata(&package.root, &path),
    }
}
````

````rs
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
````

````rs
fn json_string_value(value: Option<&serde_json::Value>) -> Option<String> {
    value?.as_str().map(ToOwned::to_owned)
}
````

````rs
fn json_dependency_object(value: Option<&serde_json::Value>) -> HashMap<String, String> {
    let Some(object) = value.and_then(serde_json::Value::as_object) else {
        return HashMap::new();
    };
    object
        .iter()
        .map(|(name, value)| (name.clone(), json_dependency_version(value)))
        .collect()
}
````

````rs
fn json_dependency_version(value: &serde_json::Value) -> String {
    if let Some(version) = value.as_str() {
        return version.to_string();
    }
    if let Some(version) = value.get("version").and_then(serde_json::Value::as_str) {
        return version.to_string();
    }
    value.to_string()
}
````

````rs
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
````

````rs
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
````

````rs
pub fn read_pubspec_metadata(path: &Path, state: &mut RunState) -> Option<PackageMetadata> {
    let text = fs::read_to_string(path).ok()?;
    let value = match serde_yaml::from_str::<serde_yaml::Value>(&text) {
        Ok(value) => value,
        Err(error) => {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("failed to parse pubspec.yaml: {error}"),
            ));
            return None;
        }
    };
    let name = yaml_string(value.get("name"));
    let version = yaml_string(value.get("version")).unwrap_or_else(|| "0.1.0".to_string());
    Some(PackageMetadata {
        name: name.unwrap_or_else(|| fallback_package_name(path)),
        version,
        dependencies: yaml_dependency_object(value.get("dependencies")),
        dev_dependencies: yaml_dependency_object(value.get("dev_dependencies")),
    })
}
````

````rs
pub fn read_dotnet_metadata(path: &Path, state: &mut RunState) -> Option<PackageMetadata> {
    let text = fs::read_to_string(path).ok()?;
    let name = capture_first(&text, &[r"<AssemblyName>([^<]+)</AssemblyName>", r"<PackageId>([^<]+)</PackageId>"])
        .unwrap_or_else(|| fallback_package_name(path));
    let version = capture_first(&text, &[r"<Version>([^<]+)</Version>"])
        .unwrap_or_else(|| "0.1.0".to_string());
    Some(PackageMetadata {
        name,
        version,
        dependencies: capture_many_pairs(
            &text,
            r#"<PackageReference[^>]*Include=\"([^\"]+)\"[^>]*Version=\"([^\"]+)\"[^>]*/?>"#,
            state,
        ),
        dev_dependencies: HashMap::new(),
    })
}
````

````rs
pub fn read_cmake_metadata(path: &Path, _state: &mut RunState) -> Option<PackageMetadata> {
    let text = fs::read_to_string(path).ok()?;
    let name = capture_first(&text, &[r"project\(([^\s\)]+)", r"set\(PROJECT_NAME\s+([^\s\)]+)"])
        .unwrap_or_else(|| fallback_package_name(path));
    let version = capture_first(&text, &[r"VERSION\s+([^\s\)]+)"]).unwrap_or_else(|| "0.1.0".to_string());
    Some(PackageMetadata {
        name,
        version,
        dependencies: capture_many_names(&text, r"find_package\(([^\s\)]+)"),
        dev_dependencies: HashMap::new(),
    })
}
````

````rs
pub fn read_meson_metadata(path: &Path, _state: &mut RunState) -> Option<PackageMetadata> {
    let text = fs::read_to_string(path).ok()?;
    let name = capture_first(&text, &[r#"project\(['\"]([^'\"]+)['\"]"#])
        .unwrap_or_else(|| fallback_package_name(path));
    let version = capture_first(&text, &[r#"version\s*:\s*['\"]([^'\"]+)['\"]"#])
        .unwrap_or_else(|| "0.1.0".to_string());
    Some(PackageMetadata {
        name,
        version,
        dependencies: HashMap::new(),
        dev_dependencies: HashMap::new(),
    })
}
````

````rs
pub fn read_conan_metadata(path: &Path, _state: &mut RunState) -> Option<PackageMetadata> {
    let text = fs::read_to_string(path).ok()?;
    let mut metadata = read_generic_metadata(path.parent().unwrap_or(path), path)?;
    metadata.dependencies = text
        .lines()
        .filter_map(|line| line.split_once('/'))
        .map(|(name, version)| (name.trim().to_string(), version.trim().to_string()))
        .collect();
    Some(metadata)
}
````

````rs
pub fn read_bundler_metadata(root: &Path, path: &Path, _state: &mut RunState) -> Option<PackageMetadata> {
    if path.extension().and_then(|value| value.to_str()) == Some("gemspec") {
        let text = fs::read_to_string(path).ok()?;
        let name = capture_first(&text, &[r#"\.name\s*=\s*['\"]([^'\"]+)['\"]"#])
            .unwrap_or_else(|| fallback_package_name(path));
        let version = capture_first(&text, &[r#"\.version\s*=\s*['\"]([^'\"]+)['\"]"#])
            .unwrap_or_else(|| "0.1.0".to_string());
        return Some(PackageMetadata {
            name,
            version,
            dependencies: HashMap::new(),
            dev_dependencies: HashMap::new(),
        });
    }

    if let Ok(entries) = fs::read_dir(root) {
        for entry in entries.filter_map(|entry| entry.ok()) {
            let candidate = entry.path();
            if candidate.extension().and_then(|value| value.to_str()) == Some("gemspec") {
                if let Some(metadata) = read_bundler_metadata(root, &candidate, _state) {
                    return Some(metadata);
                }
            }
        }
    }

    read_generic_metadata(root, path)
}
````

````rs
pub fn read_zig_metadata(path: &Path, _state: &mut RunState) -> Option<PackageMetadata> {
    let text = fs::read_to_string(path).ok()?;
    let name = capture_first(&text, &[r#"\.name\s*=\s*\"([^\"]+)\""#])
        .unwrap_or_else(|| fallback_package_name(path));
    let version = capture_first(&text, &[r#"\.version\s*=\s*\"([^\"]+)\""#])
        .unwrap_or_else(|| "0.1.0".to_string());
    Some(PackageMetadata {
        name,
        version,
        dependencies: HashMap::new(),
        dev_dependencies: HashMap::new(),
    })
}
````

````rs
fn read_generic_metadata(root: &Path, _path: &Path) -> Option<PackageMetadata> {
    Some(PackageMetadata {
        name: root
            .file_name()
            .and_then(|value| value.to_str())
            .unwrap_or("mds-package")
            .to_string(),
        version: "0.1.0".to_string(),
        dependencies: HashMap::new(),
        dev_dependencies: HashMap::new(),
    })
}
````

````rs
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
````

````rs
fn toml_string_value(value: Option<&toml::Value>) -> Option<String> {
    value?.as_str().map(ToOwned::to_owned)
}
````

````rs
fn toml_dependency_section(value: &toml::Value, section_name: &str) -> HashMap<String, String> {
    let Some(table) = toml_table_path(value, &[section_name]) else {
        return HashMap::new();
    };
    table
        .iter()
        .map(|(name, value)| (name.clone(), toml_dependency_version(value)))
        .collect()
}
````

````rs
fn toml_dependency_version(value: &toml::Value) -> String {
    if let Some(version) = value.as_str() {
        return version.to_string();
    }
    if let Some(version) = value.get("version").and_then(toml::Value::as_str) {
        return version.to_string();
    }
    value.to_string()
}
````

````rs
fn pyproject_optional_dependencies(value: &toml::Value, group: &str) -> HashMap<String, String> {
    toml_table_path(value, &["project", "optional-dependencies"])
        .and_then(|section| section.get(group))
        .map(toml_dependency_array)
        .unwrap_or_default()
}
````

````rs
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
````

````rs
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
````

````rs
fn yaml_string(value: Option<&serde_yaml::Value>) -> Option<String> {
    value?.as_str().map(ToOwned::to_owned)
}
````

````rs
fn yaml_dependency_object(value: Option<&serde_yaml::Value>) -> HashMap<String, String> {
    let Some(mapping) = value.and_then(serde_yaml::Value::as_mapping) else {
        return HashMap::new();
    };
    mapping
        .iter()
        .filter_map(|(key, value)| Some((key.as_str()?.to_string(), yaml_dependency_version(value))))
        .collect()
}
````

````rs
fn yaml_dependency_version(value: &serde_yaml::Value) -> String {
    value
        .as_str()
        .map(ToOwned::to_owned)
        .unwrap_or_else(|| serde_yaml::to_string(value).unwrap_or_default().trim().to_string())
}
````

````rs
fn capture_first(text: &str, patterns: &[&str]) -> Option<String> {
    for pattern in patterns {
        let regex = Regex::new(pattern).ok()?;
        if let Some(captures) = regex.captures(text) {
            if let Some(value) = captures.get(1) {
                return Some(value.as_str().trim().to_string());
            }
        }
    }
    None
}
````

````rs
fn capture_many_names(text: &str, pattern: &str) -> HashMap<String, String> {
    let Ok(regex) = Regex::new(pattern) else {
        return HashMap::new();
    };
    regex
        .captures_iter(text)
        .filter_map(|captures| captures.get(1).map(|value| (value.as_str().trim().to_string(), String::new())))
        .collect()
}
````

````rs
fn capture_many_pairs(
    text: &str,
    pattern: &str,
    _state: &mut RunState,
) -> HashMap<String, String> {
    let Ok(regex) = Regex::new(pattern) else {
        return HashMap::new();
    };
    regex
        .captures_iter(text)
        .filter_map(|captures| {
            Some((
                captures.get(1)?.as_str().trim().to_string(),
                captures.get(2)?.as_str().trim().to_string(),
            ))
        })
        .collect()
}
````

````rs
fn fallback_package_name(path: &Path) -> String {
    path.parent()
        .and_then(|value| value.file_name())
        .and_then(|value| value.to_str())
        .unwrap_or("mds-package")
        .to_string()
}
````

````rs
pub fn rust_expose_modules(package: &Package, state: &mut RunState) -> Vec<Vec<String>> {
    let markdown_root = source_markdown_root(package);
    let Ok(files) = collect_files(&markdown_root, false) else {
        return Vec::new();
    };
    let mut modules = Vec::new();
    for path in files
        .into_iter()
        .filter(|path| !is_excluded(&package.root, path, &package.config.excludes))
        .filter(|path| is_source_overview_doc(path))
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
````

````rs
pub fn validate_index_docs(package: &Package, state: &mut RunState) {
    let markdown_root = source_markdown_root(package);
    if !markdown_root.exists() {
        return;
    }
    let Ok(files) = collect_files(&markdown_root, false) else {
        return;
    };
    for path in files
        .into_iter()
        .filter(|path| !is_excluded(&package.root, path, &package.config.excludes))
        .filter(|path| is_source_overview_doc(path))
    {
        let text = match fs::read_to_string(&path) {
            Ok(text) => text,
            Err(error) => {
                state.diagnostics.push(Diagnostic::error(
                    Some(path),
                    format!("failed to read source overview: {error}"),
                ));
                continue;
            }
        };
        let sections = sections_with_labels(&text, &package.config.label_overrides);
        for required in ["Purpose", "Architecture", "Exposes", "Rules"] {
            if !sections.contains_key(required) {
                state.diagnostics.push(Diagnostic::error(
                    Some(path.clone()),
                    format!("source overview requires ## {required}"),
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
                    "source overview Exposes section requires Kind, Name, Target, and Summary table columns",
                ));
                continue;
            };
            validate_expose_rows(&rows, &path, state);
        }
    }
}
````

````rs
fn is_source_overview_doc(path: &Path) -> bool {
    path.file_name() == Some(OsStr::new("overview.md"))
}
````

````rs
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



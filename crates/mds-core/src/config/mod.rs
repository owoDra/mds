use std::fs;
use std::path::{Path, PathBuf};

use crate::diagnostics::{Diagnostic, RunState};
use crate::model::{Config, Lang};

pub fn merge_config_file(config: &mut Config, path: &Path, state: &mut RunState) -> Option<()> {
    let text = match fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) => {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("failed to read config: {error}"),
            ));
            return None;
        }
    };
    let value = match text.parse::<toml::Value>() {
        Ok(value) => value,
        Err(error) => {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("failed to parse mds.config.toml: {error}"),
            ));
            return None;
        }
    };
    let Some(root) = value.as_table() else {
        state.diagnostics.push(Diagnostic::error(
            Some(path.to_path_buf()),
            "mds.config.toml must contain TOML tables",
        ));
        return None;
    };

    for key in root.keys() {
        if !is_supported_top_level_table(key) {
            state.diagnostics.push(Diagnostic::warning(
                Some(path.to_path_buf()),
                format!("ignoring unsupported config table `{key}`"),
            ));
        }
    }

    if let Some(package) = root.get("package").and_then(toml::Value::as_table) {
        for (key, value) in package {
            match key.as_str() {
                "enabled" => config.enabled = bool_value(value, path, key, state),
                "allow_raw_source" => config.allow_raw_source = bool_value(value, path, key, state),
                "mds_version" | "mds-version" => {
                    config.mds_version = Some(string_value(value, path, key, state));
                }
                _ => warn_unsupported(path, state, "package config", key),
            }
        }
    }

    if let Some(roots) = root.get("roots").and_then(toml::Value::as_table) {
        for (key, value) in roots {
            match key.as_str() {
                "markdown" => {
                    config.roots.markdown = PathBuf::from(string_value(value, path, key, state))
                }
                "source" => {
                    config.roots.source = PathBuf::from(string_value(value, path, key, state))
                }
                "types" => {
                    config.roots.types = PathBuf::from(string_value(value, path, key, state))
                }
                "test" => config.roots.test = PathBuf::from(string_value(value, path, key, state)),
                "exclude" | "excludes" => {
                    config.excludes = string_array_value(value, path, key, state)
                }
                _ => warn_unsupported(path, state, "roots config", key),
            }
        }
    }

    if let Some(adapters) = root.get("adapters").and_then(toml::Value::as_table) {
        for (adapter, value) in adapters {
            let Some(lang) = lang_from_key(adapter) else {
                warn_unsupported(path, state, "adapter config", adapter);
                continue;
            };
            let Some(table) = value.as_table() else {
                state.diagnostics.push(Diagnostic::error(
                    Some(path.to_path_buf()),
                    format!("adapter config `{adapter}` must be a table"),
                ));
                continue;
            };
            for (key, value) in table {
                if key == "enabled" {
                    config
                        .adapters
                        .insert(lang.clone(), bool_value(value, path, key, state));
                } else {
                    warn_unsupported(path, state, &format!("adapter config `{adapter}`"), key);
                }
            }
        }
    }

    if let Some(quality) = root.get("quality").and_then(toml::Value::as_table) {
        for (adapter, value) in quality {
            let Some(lang) = lang_from_key(adapter) else {
                warn_unsupported(path, state, "quality config", adapter);
                continue;
            };
            let Some(table) = value.as_table() else {
                state.diagnostics.push(Diagnostic::error(
                    Some(path.to_path_buf()),
                    format!("quality config `{adapter}` must be a table"),
                ));
                continue;
            };
            let entry = config
                .quality
                .entry(lang)
                .or_insert_with(|| crate::model::QualityConfig {
                    lint: None,
                    fix: None,
                    test: None,
                    required: Vec::new(),
                    optional: Vec::new(),
                });
            for (key, value) in table {
                match key.as_str() {
                    "lint" | "linter" => {
                        entry.lint = optional_command_value(value, path, key, state)
                    }
                    "fix" | "fixer" => entry.fix = optional_command_value(value, path, key, state),
                    "test" | "test_runner" => {
                        entry.test = optional_command_value(value, path, key, state)
                    }
                    "required" => entry.required = string_array_value(value, path, key, state),
                    "optional" => entry.optional = string_array_value(value, path, key, state),
                    _ => warn_unsupported(path, state, &format!("quality config `{adapter}`"), key),
                }
            }
        }
    }

    if let Some(doctor) = root.get("doctor").and_then(toml::Value::as_table) {
        for (key, value) in doctor {
            match key.as_str() {
                "required" => {
                    for command in string_array_value(value, path, key, state) {
                        for quality in config.quality.values_mut() {
                            if !quality.required.contains(&command) {
                                quality.required.push(command.clone());
                            }
                        }
                    }
                }
                "optional" => {
                    for command in string_array_value(value, path, key, state) {
                        for quality in config.quality.values_mut() {
                            if !quality.optional.contains(&command) {
                                quality.optional.push(command.clone());
                            }
                        }
                    }
                }
                _ => warn_unsupported(path, state, "doctor config", key),
            }
        }
    }

    for table_name in ["package_sync", "package-sync"] {
        if let Some(package_sync) = root.get(table_name).and_then(toml::Value::as_table) {
            for (key, value) in package_sync {
                match key.as_str() {
                    "hook_enabled" | "hook-enabled" => {
                        config.package_sync_hook_enabled = bool_value(value, path, key, state);
                        if config.package_sync_hook_enabled && config.package_sync_hook.is_none() {
                            config.package_sync_hook = Some("mds package sync --check".to_string());
                        }
                    }
                    "hook" | "post_hook" | "post-command" | "post_command" | "hook_command"
                    | "hook-command" => {
                        config.package_sync_hook = Some(string_value(value, path, key, state));
                    }
                    _ => warn_unsupported(path, state, "package sync config", key),
                }
            }
        }
    }

    for table_name in ["labels", "label_overrides", "label-overrides"] {
        if let Some(labels) = root.get(table_name).and_then(toml::Value::as_table) {
            for (key, value) in labels {
                let canonical = key.to_ascii_lowercase();
                if is_supported_label(&canonical) {
                    config
                        .label_overrides
                        .insert(canonical, string_value(value, path, key, state));
                } else {
                    state.diagnostics.push(Diagnostic::error(
                        Some(path.to_path_buf()),
                        format!("unsupported label override `{key}`"),
                    ));
                }
            }
        }
    }

    Some(())
}

fn is_supported_label(key: &str) -> bool {
    matches!(
        key,
        "purpose"
            | "contract"
            | "types"
            | "source"
            | "cases"
            | "test"
            | "expose"
            | "exposes"
            | "from"
            | "target"
            | "summary"
            | "kind"
            | "name"
            | "version"
    )
}

fn is_supported_top_level_table(section: &str) -> bool {
    matches!(
        section,
        "package"
            | "roots"
            | "adapters"
            | "quality"
            | "doctor"
            | "package_sync"
            | "package-sync"
            | "labels"
            | "label_overrides"
            | "label-overrides"
    )
}

fn lang_from_key(key: &str) -> Option<Lang> {
    match key {
        "ts" | "typescript" => Some(Lang::TypeScript),
        "py" | "python" => Some(Lang::Python),
        "rs" | "rust" => Some(Lang::Rust),
        other
            if !other.is_empty()
                && other
                    .chars()
                    .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_') =>
        {
            Some(Lang::Other(other.to_string()))
        }
        _ => None,
    }
}

fn bool_value(value: &toml::Value, path: &Path, key: &str, state: &mut RunState) -> bool {
    match value.as_bool() {
        Some(value) => value,
        None => {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("config `{key}` must be a boolean"),
            ));
            false
        }
    }
}

fn string_value(value: &toml::Value, path: &Path, key: &str, state: &mut RunState) -> String {
    match value.as_str() {
        Some(value) => value.to_string(),
        None => {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("config `{key}` must be a string"),
            ));
            String::new()
        }
    }
}

fn optional_command_value(
    value: &toml::Value,
    path: &Path,
    key: &str,
    state: &mut RunState,
) -> Option<String> {
    if value.as_bool() == Some(false) {
        return None;
    }
    let command = string_value(value, path, key, state);
    if command.is_empty() || command == "false" {
        None
    } else {
        Some(command)
    }
}

fn string_array_value(
    value: &toml::Value,
    path: &Path,
    key: &str,
    state: &mut RunState,
) -> Vec<String> {
    let Some(values) = value.as_array() else {
        state.diagnostics.push(Diagnostic::error(
            Some(path.to_path_buf()),
            format!("config `{key}` must be an array of strings"),
        ));
        return Vec::new();
    };
    values
        .iter()
        .filter_map(|value| match value.as_str() {
            Some(value) => Some(value.to_string()),
            None => {
                state.diagnostics.push(Diagnostic::error(
                    Some(path.to_path_buf()),
                    format!("config `{key}` must contain only strings"),
                ));
                None
            }
        })
        .filter(|value| !value.is_empty())
        .collect()
}

fn warn_unsupported(path: &Path, state: &mut RunState, scope: &str, key: &str) {
    state.diagnostics.push(Diagnostic::warning(
        Some(path.to_path_buf()),
        format!("ignoring unsupported {scope} `{key}`"),
    ));
}

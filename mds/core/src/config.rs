use std::{fs};
use std::path::{Path};
use std::path::{PathBuf};
use crate::diagnostics::{Diagnostic};
use crate::diagnostics::{RunState};
use crate::model::{CANONICAL_SOURCE_MD_ROOT};
use crate::model::{CANONICAL_TEST_MD_ROOT};
use crate::model::{Config};
use crate::model::{Lang};
use crate::model::{OutputKind};
use crate::model::{OutputOverride};
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
                "copy_source_assets" | "copy-source-assets" => {
                    config.copy_source_assets = bool_value(value, path, key, state)
                }
                "mds_version" | "mds-version" => {
                    config.mds_version = Some(string_value(value, path, key, state));
                }
                _ => warn_unsupported(path, state, "package config", key),
            }
        }
    }

    for table_name in ["check", "checks"] {
        if let Some(check) = root.get(table_name).and_then(toml::Value::as_table) {
            for (key, value) in check {
                match key.as_str() {
                    "code_blocks_required" | "code_block_required" => {
                        config.check.code_blocks_required = bool_value(value, path, key, state)
                    }
                    "code_fence_integrity" | "code_fences" => {
                        config.check.code_fence_integrity = bool_value(value, path, key, state)
                    }
                    "duplicate_h2_sections" | "duplicate_sections" => {
                        config.check.duplicate_h2_sections = bool_value(value, path, key, state)
                    }
                    "markdown_links" | "links" => {
                        config.check.markdown_links = bool_value(value, path, key, state)
                    }
                    "import_with_implementation" | "imports_with_implementation" => {
                        config.check.import_with_implementation = bool_value(value, path, key, state)
                    }
                    "top_level_fence_required" | "multiple_top_level_implementations" => {
                        config.check.top_level_fence_required = bool_value(value, path, key, state)
                    }
                    "doc_comments_outside_code" | "doc_comment_outside_code" => {
                        config.check.doc_comments_outside_code = bool_value(value, path, key, state)
                    }
                    "documented_sections" | "documentation_sections" => {
                        config.check.documented_sections = bool_value(value, path, key, state)
                    }
                    "documented_exports" | "export_documentation" => {
                        config.check.documented_exports = bool_value(value, path, key, state)
                    }
                    _ => warn_unsupported(path, state, "check config", key),
                }
            }
        }
    }

    if let Some(roots) = root.get("roots").and_then(toml::Value::as_table) {
        for (key, value) in roots {
            match key.as_str() {
                "source_md" => {
                    if let Some(root) = canonical_root_value(
                        value,
                        path,
                        key,
                        CANONICAL_SOURCE_MD_ROOT,
                        state,
                    ) {
                        config.roots.source_md = root;
                    }
                }
                "test_md" => {
                    if let Some(root) = canonical_root_value(
                        value,
                        path,
                        key,
                        CANONICAL_TEST_MD_ROOT,
                        state,
                    ) {
                        config.roots.test_md = root;
                    }
                }
                "source_out" => {
                    config.roots.source_out = PathBuf::from(string_value(value, path, key, state))
                }
                "test_out" => {
                    config.roots.test_out = PathBuf::from(string_value(value, path, key, state))
                }
                "exclude" | "excludes" => {
                    config.excludes = string_array_value(value, path, key, state)
                }
                _ => warn_unsupported(path, state, "roots config", key),
            }
        }
    }

    if let Some(output) = root.get("output").and_then(toml::Value::as_table) {
        for (key, value) in output {
            match key.as_str() {
                "source" => config.output.source = Some(string_value(value, path, key, state)),
                "test" => config.output.test = Some(string_value(value, path, key, state)),
                "override" => {
                    config.output.overrides = output_override_array_value(value, path, state)
                }
                _ => warn_unsupported(path, state, "output config", key),
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
                    type_check: None,
                    lint: None,
                    fix: None,
                    test: None,
                    required: Vec::new(),
                    optional: Vec::new(),
                });
            for (key, value) in table {
                match key.as_str() {
                    "type_check" | "type_checker" => {
                        entry.type_check = optional_command_value(value, path, key, state)
                    }
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
                            config.package_sync_hook = Some("mds package sync".to_string());
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
            | "check"
            | "checks"
            | "roots"
            | "output"
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

fn canonical_root_value(
    value: &toml::Value,
    path: &Path,
    key: &str,
    canonical: &str,
    state: &mut RunState,
) -> Option<PathBuf> {
    let Some(value) = value.as_str() else {
        state.diagnostics.push(Diagnostic::error(
            Some(path.to_path_buf()),
            format!("config `{key}` must be a string"),
        ));
        return None;
    };
    if value == canonical {
        Some(PathBuf::from(value))
    } else {
        state.diagnostics.push(Diagnostic::error(
            Some(path.to_path_buf()),
            format!("config `{key}` must be `{canonical}`"),
        ));
        None
    }
}

fn output_override_array_value(
    value: &toml::Value,
    path: &Path,
    state: &mut RunState,
) -> Vec<OutputOverride> {
    let Some(values) = value.as_array() else {
        state.diagnostics.push(Diagnostic::error(
            Some(path.to_path_buf()),
            "config `output.override` must be an array of tables",
        ));
        return Vec::new();
    };

    values
        .iter()
        .enumerate()
        .filter_map(|(index, value)| {
            let Some(table) = value.as_table() else {
                state.diagnostics.push(Diagnostic::error(
                    Some(path.to_path_buf()),
                    "config `output.override` must be an array of tables",
                ));
                return None;
            };

            let mut match_pattern = None;
            let mut kind = None;
            let mut override_path = None;

            for (key, value) in table {
                match key.as_str() {
                    "match" => {
                        match_pattern = Some(string_value(value, path, "output.override.match", state))
                    }
                    "kind" => {
                        kind = output_kind_value(value, path, "output.override.kind", state)
                    }
                    "path" => {
                        override_path = Some(string_value(value, path, "output.override.path", state))
                    }
                    _ => warn_unsupported(path, state, "output.override", key),
                }
            }

            match (match_pattern, kind, override_path) {
                (Some(match_pattern), Some(kind), Some(path_pattern))
                    if !match_pattern.is_empty() && !path_pattern.is_empty() =>
                {
                    Some(OutputOverride {
                        match_pattern,
                        kind,
                        path: path_pattern,
                    })
                }
                _ => {
                    state.diagnostics.push(Diagnostic::error(
                        Some(path.to_path_buf()),
                        format!(
                            "config `output.override[{index}]` requires `match`, `kind`, and `path`"
                        ),
                    ));
                    None
                }
            }
        })
        .collect()
}

fn output_kind_value(
    value: &toml::Value,
    path: &Path,
    key: &str,
    state: &mut RunState,
) -> Option<OutputKind> {
    let Some(value) = value.as_str() else {
        state.diagnostics.push(Diagnostic::error(
            Some(path.to_path_buf()),
            format!("config `{key}` must be a string"),
        ));
        return None;
    };

    match value {
        "source" => Some(OutputKind::Source),
        "test" => Some(OutputKind::Test),
        _ => {
            state.diagnostics.push(Diagnostic::error(
                Some(path.to_path_buf()),
                format!("config `{key}` must be `source` or `test`"),
            ));
            None
        }
    }
}

fn lang_from_key(key: &str) -> Option<Lang> {
    if !key.is_empty()
        && key
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
    {
        Some(Lang::Other(key.to_string()))
    } else {
        None
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

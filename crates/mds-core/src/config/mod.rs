use std::fs;
use std::path::{Path, PathBuf};

use crate::diagnostics::{Diagnostic, RunState};
use crate::model::{Config, Lang};

pub(crate) fn merge_config_file(
    config: &mut Config,
    path: &Path,
    state: &mut RunState,
) -> Option<()> {
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
    let mut section = String::new();
    for (idx, raw_line) in text.lines().enumerate() {
        let line = raw_line
            .split_once('#')
            .map(|(line, _)| line)
            .unwrap_or(raw_line)
            .trim();
        if line.is_empty() {
            continue;
        }
        if line.starts_with('[') && line.ends_with(']') {
            section = line.trim_matches(['[', ']']).to_string();
            if !is_supported_section(&section) {
                state.diagnostics.push(Diagnostic::warning(
                    Some(path.to_path_buf()),
                    format!("ignoring unsupported config table `{section}`"),
                ));
            }
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            state.diagnostics.push(
                Diagnostic::error(Some(path.to_path_buf()), "invalid config assignment")
                    .at_line(idx + 1),
            );
            continue;
        };
        let key = key.trim();
        let value = value.trim();
        match section.as_str() {
            "package" => match key {
                "enabled" => config.enabled = parse_bool(value, path, idx + 1, state),
                "allow_raw_source" => {
                    config.allow_raw_source = parse_bool(value, path, idx + 1, state)
                }
                _ => state.diagnostics.push(Diagnostic::warning(
                    Some(path.to_path_buf()),
                    format!("ignoring unsupported package config `{key}`"),
                )),
            },
            "roots" => match key {
                "markdown" => config.roots.markdown = PathBuf::from(parse_string(value)),
                "source" => config.roots.source = PathBuf::from(parse_string(value)),
                "types" => config.roots.types = PathBuf::from(parse_string(value)),
                "test" => config.roots.test = PathBuf::from(parse_string(value)),
                "exclude" | "excludes" => config.excludes = parse_array(value),
                _ => state.diagnostics.push(Diagnostic::warning(
                    Some(path.to_path_buf()),
                    format!("ignoring unsupported roots config `{key}`"),
                )),
            },
            "adapters.ts"
            | "adapters.typescript"
            | "adapters.py"
            | "adapters.python"
            | "adapters.rs"
            | "adapters.rust" => {
                if key != "enabled" {
                    state.diagnostics.push(Diagnostic::warning(
                        Some(path.to_path_buf()),
                        format!("ignoring unsupported adapter config `{section}.{key}`"),
                    ));
                    continue;
                }
                let lang = match section.as_str() {
                    "adapters.ts" | "adapters.typescript" => Lang::TypeScript,
                    "adapters.py" | "adapters.python" => Lang::Python,
                    _ => Lang::Rust,
                };
                config
                    .adapters
                    .insert(lang, parse_bool(value, path, idx + 1, state));
            }
            "quality.ts" | "quality.typescript" | "quality.py" | "quality.python"
            | "quality.rs" | "quality.rust" => {
                let lang = lang_from_section(&section);
                let entry =
                    config
                        .quality
                        .entry(lang)
                        .or_insert_with(|| crate::model::QualityConfig {
                            lint: None,
                            fix: None,
                            test: None,
                            required: Vec::new(),
                            optional: Vec::new(),
                        });
                match key {
                    "lint" | "linter" => entry.lint = optional_command(value),
                    "fix" | "fixer" => entry.fix = optional_command(value),
                    "test" | "test_runner" => entry.test = optional_command(value),
                    "required" => entry.required = parse_array(value),
                    "optional" => entry.optional = parse_array(value),
                    _ => state.diagnostics.push(Diagnostic::warning(
                        Some(path.to_path_buf()),
                        format!("ignoring unsupported quality config `{section}.{key}`"),
                    )),
                }
            }
            "doctor" => match key {
                "required" => {
                    for command in parse_array(value) {
                        for quality in config.quality.values_mut() {
                            if !quality.required.contains(&command) {
                                quality.required.push(command.clone());
                            }
                        }
                    }
                }
                "optional" => {
                    for command in parse_array(value) {
                        for quality in config.quality.values_mut() {
                            if !quality.optional.contains(&command) {
                                quality.optional.push(command.clone());
                            }
                        }
                    }
                }
                _ => state.diagnostics.push(Diagnostic::warning(
                    Some(path.to_path_buf()),
                    format!("ignoring unsupported doctor config `{key}`"),
                )),
            },
            "package_sync" | "package-sync" => match key {
                "hook" | "post_hook" | "post-command" | "post_command" => {
                    config.package_sync_hook = Some(parse_string(value));
                }
                _ => state.diagnostics.push(Diagnostic::warning(
                    Some(path.to_path_buf()),
                    format!("ignoring unsupported package sync config `{key}`"),
                )),
            },
            "labels" | "label_overrides" | "label-overrides" => {
                config
                    .label_overrides
                    .insert(key.to_ascii_lowercase(), parse_string(value));
            }
            _ => state.diagnostics.push(Diagnostic::warning(
                Some(path.to_path_buf()),
                format!("ignoring unsupported config key `{key}`"),
            )),
        }
    }

    Some(())
}

pub(crate) fn parse_bool(value: &str, path: &Path, line: usize, state: &mut RunState) -> bool {
    match value {
        "true" => true,
        "false" => false,
        _ => {
            state.diagnostics.push(
                Diagnostic::error(
                    Some(path.to_path_buf()),
                    "boolean config value must be true or false",
                )
                .at_line(line),
            );
            false
        }
    }
}

pub(crate) fn parse_string(value: &str) -> String {
    value.trim().trim_matches('"').to_string()
}

fn is_supported_section(section: &str) -> bool {
    matches!(
        section,
        "package"
            | "roots"
            | "adapters.ts"
            | "adapters.typescript"
            | "adapters.py"
            | "adapters.python"
            | "adapters.rs"
            | "adapters.rust"
            | "quality.ts"
            | "quality.typescript"
            | "quality.py"
            | "quality.python"
            | "quality.rs"
            | "quality.rust"
            | "doctor"
            | "package_sync"
            | "package-sync"
            | "labels"
            | "label_overrides"
            | "label-overrides"
    )
}

fn lang_from_section(section: &str) -> Lang {
    match section {
        "quality.ts" | "quality.typescript" => Lang::TypeScript,
        "quality.py" | "quality.python" => Lang::Python,
        _ => Lang::Rust,
    }
}

fn optional_command(value: &str) -> Option<String> {
    let command = parse_string(value);
    if command.is_empty() || command == "false" {
        None
    } else {
        Some(command)
    }
}

pub(crate) fn parse_array(value: &str) -> Vec<String> {
    let value = value.trim();
    let Some(inner) = value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
    else {
        return Vec::new();
    };
    inner
        .split(',')
        .map(parse_string)
        .filter(|value| !value.is_empty())
        .collect()
}

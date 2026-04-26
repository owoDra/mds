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
            if !matches!(
                section.as_str(),
                "package"
                    | "roots"
                    | "adapters.ts"
                    | "adapters.typescript"
                    | "adapters.py"
                    | "adapters.python"
                    | "adapters.rs"
                    | "adapters.rust"
            ) {
                state.diagnostics.push(Diagnostic::warning(
                    Some(path.to_path_buf()),
                    format!("MVP ignores unsupported config table `{section}`"),
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
                    format!("MVP ignores unsupported package config `{key}`"),
                )),
            },
            "roots" => match key {
                "markdown" => config.roots.markdown = PathBuf::from(parse_string(value)),
                "source" => config.roots.source = PathBuf::from(parse_string(value)),
                "types" => config.roots.types = PathBuf::from(parse_string(value)),
                "test" => config.roots.test = PathBuf::from(parse_string(value)),
                _ => state.diagnostics.push(Diagnostic::warning(
                    Some(path.to_path_buf()),
                    format!("MVP ignores unsupported roots config `{key}`"),
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
                        format!("MVP ignores unsupported adapter config `{section}.{key}`"),
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
            _ => state.diagnostics.push(Diagnostic::warning(
                Some(path.to_path_buf()),
                format!("MVP ignores unsupported config key `{key}`"),
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

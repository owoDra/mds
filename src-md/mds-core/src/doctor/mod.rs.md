# src/doctor/mod.rs

## Purpose

Migrated implementation source for `src/doctor/mod.rs`.

## Contract

- Preserve the behavior of the pre-migration Rust source.
- This file is synchronized into `.build/rust/mds-core/src/doctor/mod.rs`.

## Source

````rs
use std::process::Command as ProcessCommand;

use crate::adapter::tool_available;
use crate::diagnostics::{Diagnostic, RunState};
use crate::model::{DoctorFormat, Package};

pub(crate) fn run_doctor(packages: &[Package], format: DoctorFormat, state: &mut RunState) {
    let mut checks = Vec::new();
    let current_version = env!("CARGO_PKG_VERSION");
    checks.push(DoctorCheck::ok("mds", current_version.to_string()));
    checks.push(DoctorCheck::ok("packages", packages.len().to_string()));
    for package in packages {
        // Check mds_version compatibility
        if let Some(ref expected_version) = package.config.mds_version {
            if expected_version != current_version {
                checks.push(DoctorCheck::warning(
                    "mds_version",
                    format!(
                        "project expects mds {expected_version}, but running {current_version}. Run `mds update --version {expected_version}` to match."
                    ),
                ));
            }
        }
        checks.push(DoctorCheck::ok(
            "package",
            package.root.display().to_string(),
        ));
        for lang in package.config.quality.keys() {
            if !package.config.adapters.get(lang).copied().unwrap_or(true) {
                continue;
            }
            let Some(config) = package.config.quality.get(lang) else {
                continue;
            };
            for command in &config.required {
                if tool_available(command) {
                    if let Some(required) = minimum_version(command) {
                        match command_version(command) {
                            Some(version) if version_at_least(&version, required) => checks.push(
                                DoctorCheck::ok(command, render_version(&version).to_string()),
                            ),
                            Some(version) => {
                                state.environment_missing = true;
                                checks.push(DoctorCheck::error(
                                    command,
                                    format!(
                                        "{} is below required {}.{}",
                                        render_version(&version),
                                        required.0,
                                        required.1
                                    ),
                                ));
                                state.diagnostics.push(Diagnostic::error(
                                    Some(package.root.clone()),
                                    format!(
                                        "DOCTOR002_VERSION_TOO_OLD: `{command}` version {} is below required {}.{}",
                                        render_version(&version),
                                        required.0,
                                        required.1
                                    ),
                                ));
                            }
                            None => checks.push(DoctorCheck::warning(
                                command,
                                "version unavailable".to_string(),
                            )),
                        }
                    } else {
                        checks.push(DoctorCheck::ok(command, "available".to_string()));
                    }
                } else {
                    state.environment_missing = true;
                    checks.push(DoctorCheck::error(command, "missing".to_string()));
                    state.diagnostics.push(Diagnostic::error(
                        Some(package.root.clone()),
                        format!("DOCTOR001_TOOLCHAIN_MISSING: required toolchain `{command}` is not available"),
                    ));
                }
            }
            for command in &config.optional {
                if tool_available(command) {
                    checks.push(DoctorCheck::ok(command, "available".to_string()));
                } else {
                    checks.push(DoctorCheck::warning(command, "missing".to_string()));
                    state.diagnostics.push(Diagnostic::warning(
                        Some(package.root.clone()),
                        format!("optional toolchain `{command}` is not available"),
                    ));
                }
            }
        }
    }
    match format {
        DoctorFormat::Text => render_text(&checks, state),
        DoctorFormat::Json => render_json(&checks, state),
    }
}

fn minimum_version(command: &str) -> Option<(u32, u32)> {
    let name = command.rsplit('/').next().unwrap_or(command);
    match name {
        "node" => Some((24, 0)),
        "python" | "python3" => Some((3, 13)),
        "rustc" | "cargo" => Some((1, 86)),
        _ => None,
    }
}

fn command_version(command: &str) -> Option<(u32, u32, u32)> {
    let output = ProcessCommand::new(command)
        .arg("--version")
        .output()
        .ok()?;
    let text = if output.stdout.is_empty() {
        String::from_utf8_lossy(&output.stderr).to_string()
    } else {
        String::from_utf8_lossy(&output.stdout).to_string()
    };
    parse_version(&text)
}

fn parse_version(text: &str) -> Option<(u32, u32, u32)> {
    let start = text.find(|ch: char| ch.is_ascii_digit())?;
    let version = text[start..]
        .split(|ch: char| !ch.is_ascii_digit() && ch != '.')
        .next()?;
    let mut parts = version.split('.');
    let major = parts.next()?.parse().ok()?;
    let minor = parts.next().unwrap_or("0").parse().ok()?;
    let patch = parts.next().unwrap_or("0").parse().ok()?;
    Some((major, minor, patch))
}

fn version_at_least(version: &(u32, u32, u32), minimum: (u32, u32)) -> bool {
    version.0 > minimum.0 || version.0 == minimum.0 && version.1 >= minimum.1
}

fn render_version(version: &(u32, u32, u32)) -> String {
    format!("{}.{}.{}", version.0, version.1, version.2)
}

#[derive(Debug)]
struct DoctorCheck {
    name: String,
    status: &'static str,
    detail: String,
}

impl DoctorCheck {
    fn ok(name: &str, detail: String) -> Self {
        Self {
            name: name.to_string(),
            status: "ok",
            detail,
        }
    }

    fn warning(name: &str, detail: String) -> Self {
        Self {
            name: name.to_string(),
            status: "warning",
            detail,
        }
    }

    fn error(name: &str, detail: String) -> Self {
        Self {
            name: name.to_string(),
            status: "error",
            detail,
        }
    }
}

fn render_text(checks: &[DoctorCheck], state: &mut RunState) {
    state.stdout.push_str("Doctor summary:\n");
    for check in checks {
        state.stdout.push_str(&format!(
            "- {}: {} ({})\n",
            check.name, check.status, check.detail
        ));
    }
}

fn render_json(checks: &[DoctorCheck], state: &mut RunState) {
    state.stdout.push_str("{\"checks\":[");
    for (index, check) in checks.iter().enumerate() {
        if index > 0 {
            state.stdout.push(',');
        }
        state.stdout.push_str(&format!(
            "{{\"name\":\"{}\",\"status\":\"{}\",\"detail\":\"{}\"}}",
            escape_json(&check.name),
            check.status,
            escape_json(&check.detail)
        ));
    }
    state.stdout.push_str("]}\n");
}

fn escape_json(value: &str) -> String {
    value
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
}
````

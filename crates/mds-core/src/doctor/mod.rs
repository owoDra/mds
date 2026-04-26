use crate::diagnostics::{Diagnostic, RunState};
use crate::model::{DoctorFormat, Lang, Package};
use crate::quality::tool_available;

pub(crate) fn run_doctor(packages: &[Package], format: DoctorFormat, state: &mut RunState) {
    let mut checks = Vec::new();
    checks.push(DoctorCheck::ok(
        "mds",
        env!("CARGO_PKG_VERSION").to_string(),
    ));
    checks.push(DoctorCheck::ok("packages", packages.len().to_string()));
    for package in packages {
        checks.push(DoctorCheck::ok(
            "package",
            package.root.display().to_string(),
        ));
        for lang in [Lang::TypeScript, Lang::Python, Lang::Rust] {
            if !package.config.adapters.get(&lang).copied().unwrap_or(true) {
                continue;
            }
            let Some(config) = package.config.quality.get(&lang) else {
                continue;
            };
            for command in &config.required {
                if tool_available(command) {
                    checks.push(DoctorCheck::ok(command, "available".to_string()));
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

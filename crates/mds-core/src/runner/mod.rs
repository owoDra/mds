use crate::diagnostics::{Diagnostic, RunState};
use crate::diff::{render_dry_run, write_generated};
use crate::doctor::run_doctor;
use crate::generation::plan_generation;
use crate::manifest::validate_manifest;
use crate::markdown::load_implementation_docs;
use crate::model::{BuildMode, CliRequest, CliResult, Command, Package};
use crate::package::{discover_packages, validate_index_docs, validate_package_md};
use crate::package_sync::sync_package_md;
use crate::quality::{run_quality, QualityOperation};

pub fn execute(request: CliRequest) -> CliResult {
    match execute_inner(request) {
        Ok(state) => render_result(state),
        Err(error) => CliResult {
            stdout: String::new(),
            stderr: format!("internal error: {error}\n"),
            exit_code: 3,
        },
    }
}

fn execute_inner(request: CliRequest) -> Result<RunState, String> {
    let mut state = RunState::default();
    let packages = discover_packages(&request.cwd, request.package.as_deref(), &mut state)?;
    if packages.is_empty() {
        state
            .diagnostics
            .push(Diagnostic::error(None, "no mds enabled packages found"));
        return Ok(state);
    }

    if let Command::Doctor { format } = request.command {
        run_doctor(&packages, format, &mut state);
    } else {
        for package in packages {
            run_package(&package, request.command, request.verbose, &mut state)?;
        }
    }

    Ok(state)
}

fn render_result(mut state: RunState) -> CliResult {
    if !state.generated.is_empty() {
        state.stdout.push_str("Generated files:\n");
        for path in &state.generated {
            state.stdout.push_str("- ");
            state.stdout.push_str(&path.display().to_string());
            state.stdout.push('\n');
        }
    }

    let stderr = state
        .diagnostics
        .iter()
        .map(Diagnostic::render)
        .collect::<String>();
    let exit_code = if state.environment_missing {
        4
    } else if state.has_errors() {
        1
    } else {
        0
    };
    CliResult {
        stdout: state.stdout,
        stderr,
        exit_code,
    }
}

pub(crate) fn run_package(
    package: &Package,
    command: Command,
    verbose: bool,
    state: &mut RunState,
) -> Result<(), String> {
    if verbose {
        state
            .stdout
            .push_str(&format!("Checking package {}\n", package.root.display()));
    }
    if let Command::PackageSync { check } = command {
        sync_package_md(package, check, state)?;
        return Ok(());
    }

    validate_manifest(package, state);
    validate_package_md(package, state);
    validate_index_docs(package, state);

    let docs = load_implementation_docs(package, state)?;
    let generated = plan_generation(package, &docs, state);

    match command {
        Command::Check => {
            if !state.has_errors() {
                state.stdout.push_str(&format!(
                    "check ok: {} ({} implementation files)\n",
                    package.root.display(),
                    docs.len()
                ));
            }
        }
        Command::Build { mode } => {
            if state.has_errors() {
                return Ok(());
            }
            match mode {
                BuildMode::DryRun => render_dry_run(&generated, state),
                BuildMode::Write => write_generated(&generated, state)?,
            }
        }
        Command::Lint { fix, check } => {
            if state.has_errors() {
                return Ok(());
            }
            let operation = if fix {
                QualityOperation::Fix { check }
            } else {
                QualityOperation::Lint
            };
            run_quality(package, &docs, operation, state)?;
        }
        Command::Test => {
            if state.has_errors() {
                return Ok(());
            }
            run_quality(package, &docs, QualityOperation::Test, state)?;
        }
        Command::PackageSync { .. } => unreachable!(),
        Command::Doctor { .. } => {}
    }
    Ok(())
}

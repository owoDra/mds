# src/runner.rs

## Purpose

Migrated implementation source for `src/runner.rs`.

## Contract

- Preserve the behavior of the pre-migration Rust source.
- This file is synchronized into `.build/rust/mds/core/src/runner.rs`.

## Imports

| Kind | From | Target | Symbols | Via | Summary | Code |
| --- | --- | --- | --- | --- | --- | --- |
| rust-use | builtin | std | fs | std |  | `use std::fs;` |
| rust-use | builtin | std::path | Path | std |  | `use std::path::Path;` |
| rust-use | internal | crate::descriptor | set_workspace_descriptor_root | crate |  | `use crate::descriptor::set_workspace_descriptor_root;` |
| rust-use | internal | crate::diagnostics | Diagnostic, RunState | crate |  | `use crate::diagnostics::{Diagnostic, RunState};` |
| rust-use | internal | crate::diff | render_dry_run, write_generated | crate |  | `use crate::diff::{render_dry_run, write_generated};` |
| rust-use | internal | crate::doctor | run_doctor | crate |  | `use crate::doctor::run_doctor;` |
| rust-use | internal | crate::generation | plan_generation | crate |  | `use crate::generation::plan_generation;` |
| rust-use | internal | crate::init | run_init | crate |  | `use crate::init::run_init;` |
| rust-use | internal | crate::manifest | validate_manifest | crate |  | `use crate::manifest::validate_manifest;` |
| rust-use | internal | crate::markdown | load_implementation_docs | crate |  | `use crate::markdown::load_implementation_docs;` |
| rust-use | internal | crate::model | BuildMode, CliRequest, CliResult, Command, Package | crate |  | `use crate::model::{BuildMode, CliRequest, CliResult, Command, Package};` |
| rust-use | internal | crate::new | run_new | crate |  | `use crate::new::run_new;` |
| rust-use | internal | crate::package | discover_packages, validate_index_docs, validate_package_md | crate |  | `use crate::package::{discover_packages, validate_index_docs, validate_package_md};` |
| rust-use | internal | crate::package_sync | sync_package_md | crate |  | `use crate::package_sync::sync_package_md;` |
| rust-use | internal | crate::quality | run_quality, QualityOperation | crate |  | `use crate::quality::{run_quality, QualityOperation};` |


## Source


````rs
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
````

````rs
fn execute_inner(request: CliRequest) -> Result<RunState, String> {
    let mut state = RunState::default();
    let descriptor_root = request
        .package
        .as_deref()
        .map(|path| {
            if path.is_absolute() {
                path.to_path_buf()
            } else {
                request.cwd.join(path)
            }
        })
        .unwrap_or_else(|| request.cwd.clone());
    set_workspace_descriptor_root(Some(&descriptor_root));
    match &request.command {
        Command::Init { options } => {
            run_init(
                &request.cwd,
                request.package.as_deref(),
                options,
                request.verbose,
                &mut state,
            )?;
            return Ok(state);
        }
        Command::New { options } => {
            run_new(
                &request.cwd,
                request.package.as_deref(),
                options,
                request.verbose,
                &mut state,
            )?;
            return Ok(state);
        }
        Command::Update { .. } => {
            // Handled by CLI directly, should not reach here
            return Ok(state);
        }
        _ => {}
    }
    let packages = discover_packages(&request.cwd, request.package.as_deref(), &mut state)?;
    if packages.is_empty() {
        state
            .diagnostics
            .push(Diagnostic::error(None, "no mds enabled packages found"));
        return Ok(state);
    }

    if let Command::Doctor { format } = &request.command {
        run_doctor(&packages, *format, &mut state);
    } else {
        for package in &packages {
            run_package(
                package,
                request.command.clone(),
                request.verbose,
                &mut state,
            )?;
        }
        if matches!(request.command, Command::Build { mode: BuildMode::Write })
            && !state.has_errors()
        {
            sync_self_hosted_rust_workspace(&request.cwd, &mut state)?;
        }
    }

    Ok(state)
}
````

````rs
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
````

````rs
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
        Command::Typecheck => {
            if state.has_errors() {
                return Ok(());
            }
            run_quality(package, &docs, QualityOperation::Typecheck, state)?;
        }
        Command::Test => {
            if state.has_errors() {
                return Ok(());
            }
            run_quality(package, &docs, QualityOperation::Test, state)?;
        }
        Command::PackageSync { .. } => unreachable!(),
        Command::Doctor { .. } => {}
        Command::Init { .. }
        | Command::New { .. }
        | Command::Update { .. } => unreachable!(),
    }
    Ok(())
}
````

````rs
fn sync_self_hosted_rust_workspace(
    workspace_root: &Path,
    state: &mut RunState,
) -> Result<(), String> {
    let workspace_manifest = workspace_root.join("Cargo.toml");
    if !workspace_manifest.exists() {
        return Ok(());
    }

    let package_paths = ["mds/core", "mds/cli", "mds/lsp"];
    let existing_packages: Vec<&str> = package_paths
        .into_iter()
        .filter(|relative| workspace_root.join(relative).join("Cargo.toml").exists())
        .collect();
    if existing_packages.is_empty() {
        return Ok(());
    }

    let mirror_root = workspace_root.join(".build/rust");
    if mirror_root.exists() {
        fs::remove_dir_all(&mirror_root)
            .map_err(|error| format!("failed to remove {}: {error}", mirror_root.display()))?;
    }
    fs::create_dir_all(&mirror_root)
        .map_err(|error| format!("failed to create {}: {error}", mirror_root.display()))?;

    copy_file(&workspace_manifest, &mirror_root.join("Cargo.toml"))?;

    let workspace_lock = workspace_root.join("Cargo.lock");
    if workspace_lock.exists() {
        copy_file(&workspace_lock, &mirror_root.join("Cargo.lock"))?;
    }

    for relative in existing_packages {
        let package_root = workspace_root.join(relative);
        let mirror_package_root = mirror_root.join(relative);
        fs::create_dir_all(&mirror_package_root).map_err(|error| {
            format!("failed to create {}: {error}", mirror_package_root.display())
        })?;

        copy_file(
            &package_root.join("Cargo.toml"),
            &mirror_package_root.join("Cargo.toml"),
        )?;

        let src_dir = package_root.join("src");
        if src_dir.exists() {
            copy_dir_recursive(&src_dir, &mirror_package_root.join("src"))?;
        }

        let tests_dir = package_root.join("tests");
        if tests_dir.exists() {
            copy_dir_recursive(&tests_dir, &mirror_package_root.join("tests"))?;
        }

        let build_rs = package_root.join("build.rs");
        if build_rs.exists() {
            copy_file(&build_rs, &mirror_package_root.join("build.rs"))?;
        }
    }

    state
        .stdout
        .push_str(&format!("workspace mirror ok: {}\n", mirror_root.display()));

    Ok(())
}
````

````rs
fn copy_dir_recursive(source: &Path, destination: &Path) -> Result<(), String> {
    fs::create_dir_all(destination)
        .map_err(|error| format!("failed to create {}: {error}", destination.display()))?;
    for entry in fs::read_dir(source)
        .map_err(|error| format!("failed to read {}: {error}", source.display()))?
    {
        let entry = entry.map_err(|error| format!("failed to read directory entry: {error}"))?;
        let path = entry.path();
        let target = destination.join(entry.file_name());
        if path.is_dir() {
            copy_dir_recursive(&path, &target)?;
        } else {
            copy_file(&path, &target)?;
        }
    }
    Ok(())
}
````



````rs
fn copy_file(source: &Path, destination: &Path) -> Result<(), String> {
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create {}: {error}", parent.display()))?;
    }
    fs::copy(source, destination).map_err(|error| {
        format!(
            "failed to copy {} to {}: {error}",
            source.display(),
            destination.display()
        )
    })?;
    Ok(())
}
````
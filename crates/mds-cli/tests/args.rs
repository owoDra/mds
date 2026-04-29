use std::path::PathBuf;

use mds_cli::args::parse_args_from;
use mds_core::{
    AgentKitCategory, AiTarget, BuildMode, Command, DoctorFormat, PythonTool, RustTool,
    TypeScriptTool,
};

#[test]
fn parses_build_dry_run() {
    let request = parse_args_from(
        PathBuf::from("/repo"),
        ["build", "--dry-run", "--package", "pkg", "--verbose"].map(String::from),
    )
    .unwrap();
    assert_eq!(request.cwd, PathBuf::from("/repo"));
    assert_eq!(request.package, Some(PathBuf::from("pkg")));
    assert!(request.verbose);
    assert!(matches!(
        request.command,
        Command::Build {
            mode: BuildMode::DryRun
        }
    ));
}

#[test]
fn rejects_dry_run_for_check() {
    let error = parse_args_from(
        PathBuf::from("/repo"),
        ["check", "--dry-run"].map(String::from),
    )
    .unwrap_err();
    assert!(error.contains("--dry-run"));
}

#[test]
fn parses_post_mvp_commands() {
    let lint = parse_args_from(
        PathBuf::from("/repo"),
        ["lint", "--fix", "--check", "--package", "pkg"].map(String::from),
    )
    .unwrap();
    assert!(matches!(
        lint.command,
        Command::Lint {
            fix: true,
            check: true
        }
    ));

    let doctor = parse_args_from(
        PathBuf::from("/repo"),
        ["doctor", "--format", "json"].map(String::from),
    )
    .unwrap();
    assert!(matches!(
        doctor.command,
        Command::Doctor {
            format: DoctorFormat::Json
        }
    ));

    let sync = parse_args_from(
        PathBuf::from("/repo"),
        ["package", "sync", "--check"].map(String::from),
    )
    .unwrap();
    assert!(matches!(sync.command, Command::PackageSync { check: true }));
}

#[test]
fn parses_init_command() {
    let request = parse_args_from(
        PathBuf::from("/repo"),
        [
            "init",
            "--ai",
            "--target",
            "claude-code,opencode",
            "--categories",
            "instructions,commands",
            "--yes",
            "--force",
            "--install-project-deps",
            "--install-toolchains",
            "--install-ai-cli",
            "--ts-tools",
            "biome,jest",
            "--py-tools",
            "ruff,black,unittest",
            "--rs-tools",
            "rustfmt,nextest",
        ]
        .map(String::from),
    )
    .unwrap();
    match request.command {
        Command::Init { options } => {
            assert!(options.ai_only);
            assert!(options.yes);
            assert!(options.force);
            assert!(options.install_project_deps);
            assert!(options.install_toolchains);
            assert!(options.install_ai_cli);
            assert_eq!(
                options.targets,
                vec![AiTarget::ClaudeCode, AiTarget::Opencode]
            );
            assert_eq!(
                options.categories,
                vec![AgentKitCategory::Instructions, AgentKitCategory::Commands]
            );
            assert_eq!(
                options.ts_tools,
                vec![TypeScriptTool::Biome, TypeScriptTool::Jest]
            );
            assert_eq!(
                options.py_tools,
                vec![PythonTool::Ruff, PythonTool::Black, PythonTool::Unittest]
            );
            assert_eq!(options.rs_tools, vec![RustTool::Rustfmt, RustTool::Nextest]);
        }
        other => panic!("unexpected command: {other:?}"),
    }
}

#[test]
fn rejects_conflicting_init_tool_choices() {
    let error = parse_args_from(
        PathBuf::from("/repo"),
        ["init", "--ts-tools", "vitest,jest"].map(String::from),
    )
    .unwrap_err();
    assert!(error.contains("vitest and jest"));
}

#[test]
fn parses_release_check_command() {
    let request = parse_args_from(
        PathBuf::from("/repo"),
        ["release", "check", "--manifest", "dist/release.mds.toml"].map(String::from),
    )
    .unwrap();
    match request.command {
        Command::ReleaseCheck { options } => {
            assert_eq!(options.manifest, PathBuf::from("dist/release.mds.toml"));
        }
        other => panic!("unexpected command: {other:?}"),
    }
}

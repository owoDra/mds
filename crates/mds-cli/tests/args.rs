use std::path::PathBuf;

use mds_cli::args::parse_args_from;
use mds_core::{BuildMode, Command, DoctorFormat};

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

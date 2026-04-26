use std::path::PathBuf;

use mds_cli::args::parse_args_from;
use mds_core::{BuildMode, Command};

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

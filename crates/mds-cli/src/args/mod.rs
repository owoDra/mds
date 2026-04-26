use std::path::PathBuf;

use mds_core::{BuildMode, CliRequest, Command, DoctorFormat};

pub fn parse_args(cwd: PathBuf) -> Result<CliRequest, String> {
    parse_args_from(cwd, std::env::args().skip(1))
}

pub fn parse_args_from<I>(cwd: PathBuf, args: I) -> Result<CliRequest, String>
where
    I: IntoIterator<Item = String>,
{
    let mut args = args.into_iter();
    let Some(command_name) = args.next() else {
        return Err("missing command".to_string());
    };

    let mut package = None;
    let mut verbose = false;
    let mut dry_run = false;
    let mut fix = false;
    let mut check = false;
    let mut format = DoctorFormat::Text;
    let mut package_subcommand = None;
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--package" => {
                let Some(path) = args.next() else {
                    return Err("--package requires a path".to_string());
                };
                package = Some(PathBuf::from(path));
            }
            "--verbose" => verbose = true,
            "--dry-run" => dry_run = true,
            "--fix" => fix = true,
            "--check" => check = true,
            "--format" => {
                let Some(value) = args.next() else {
                    return Err("--format requires a value".to_string());
                };
                format = match value.as_str() {
                    "text" => DoctorFormat::Text,
                    "json" => DoctorFormat::Json,
                    _ => return Err("--format must be text or json".to_string()),
                };
            }
            "sync" if command_name == "package" && package_subcommand.is_none() => {
                package_subcommand = Some(arg);
            }
            _ => return Err(format!("unknown option `{arg}`")),
        }
    }

    let command = match command_name.as_str() {
        "check" => {
            if dry_run {
                return Err("--dry-run is only valid for build".to_string());
            }
            Command::Check
        }
        "build" => Command::Build {
            mode: if dry_run {
                BuildMode::DryRun
            } else {
                BuildMode::Write
            },
        },
        "lint" => {
            if dry_run {
                return Err("--dry-run is only valid for build".to_string());
            }
            if check && !fix {
                return Err("--check is only valid with lint --fix or package sync".to_string());
            }
            Command::Lint { fix, check }
        }
        "test" => {
            if dry_run || fix || check {
                return Err("test only accepts --package and --verbose".to_string());
            }
            Command::Test
        }
        "doctor" => {
            if dry_run || fix || check {
                return Err("doctor only accepts --package, --verbose, and --format".to_string());
            }
            Command::Doctor { format }
        }
        "package" => {
            if dry_run || fix {
                return Err(
                    "package sync only accepts --package, --verbose, and --check".to_string(),
                );
            }
            match package_subcommand.as_deref() {
                Some("sync") => Command::PackageSync { check },
                _ => return Err("package requires subcommand sync".to_string()),
            }
        }
        _ => return Err(format!("unknown command `{command_name}`")),
    };

    Ok(CliRequest {
        cwd,
        package,
        verbose,
        command,
    })
}

pub fn print_usage() {
    eprintln!("usage: mds check [--package <path>] [--verbose]");
    eprintln!("       mds build [--package <path>] [--dry-run] [--verbose]");
    eprintln!("       mds lint [--package <path>] [--fix [--check]] [--verbose]");
    eprintln!("       mds test [--package <path>] [--verbose]");
    eprintln!("       mds doctor [--package <path>] [--format text|json] [--verbose]");
    eprintln!("       mds package sync [--package <path>] [--check] [--verbose]");
}

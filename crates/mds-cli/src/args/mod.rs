use std::path::PathBuf;

use mds_core::{BuildMode, CliRequest, Command};

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
}

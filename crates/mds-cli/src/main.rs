use std::path::PathBuf;

use mds_core::{execute, BuildMode, CliRequest, Command};

fn main() {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(error) => {
            eprintln!("internal error: failed to read current directory: {error}");
            std::process::exit(3);
        }
    };

    let request = match parse_args(cwd) {
        Ok(request) => request,
        Err(message) => {
            eprintln!("usage error: {message}");
            print_usage();
            std::process::exit(2);
        }
    };

    let result = execute(request);
    if !result.stdout.is_empty() {
        print!("{}", result.stdout);
    }
    if !result.stderr.is_empty() {
        eprint!("{}", result.stderr);
    }
    std::process::exit(result.exit_code);
}

fn parse_args(cwd: PathBuf) -> Result<CliRequest, String> {
    parse_args_from(cwd, std::env::args().skip(1))
}

fn parse_args_from<I>(cwd: PathBuf, args: I) -> Result<CliRequest, String>
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

fn print_usage() {
    eprintln!("usage: mds check [--package <path>] [--verbose]");
    eprintln!("       mds build [--package <path>] [--dry-run] [--verbose]");
}

#[cfg(test)]
mod tests {
    use super::*;

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
}

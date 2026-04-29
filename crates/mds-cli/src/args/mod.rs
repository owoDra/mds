use std::path::PathBuf;

use mds_core::{
    AgentKitCategory, AiTarget, BuildMode, CliRequest, Command, DoctorFormat, InitOptions,
    ReleaseQualityOptions,
};

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
    let mut release_subcommand = None;
    let mut init_options = InitOptions::default();
    let mut init_targets: Option<Vec<AiTarget>> = None;
    let mut init_categories: Option<Vec<AgentKitCategory>> = None;
    let mut release_manifest = None;
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
            "--ai" if command_name == "init" => init_options.ai_only = true,
            "--yes" if command_name == "init" => init_options.yes = true,
            "--force" if command_name == "init" => init_options.force = true,
            "--target" if command_name == "init" => {
                let Some(value) = args.next() else {
                    return Err("--target requires a value".to_string());
                };
                init_targets = Some(parse_targets(&value)?);
            }
            "--categories" if command_name == "init" => {
                let Some(value) = args.next() else {
                    return Err("--categories requires a value".to_string());
                };
                init_categories = Some(parse_categories(&value)?);
            }
            "--install-project-deps" if command_name == "init" => {
                init_options.install_project_deps = true;
            }
            "--install-toolchains" if command_name == "init" => {
                init_options.install_toolchains = true;
            }
            "--install-ai-cli" if command_name == "init" => {
                init_options.install_ai_cli = true;
            }
            "--manifest" if command_name == "release" => {
                let Some(path) = args.next() else {
                    return Err("--manifest requires a path".to_string());
                };
                release_manifest = Some(PathBuf::from(path));
            }
            "sync" if command_name == "package" && package_subcommand.is_none() => {
                package_subcommand = Some(arg);
            }
            "check" if command_name == "release" && release_subcommand.is_none() => {
                release_subcommand = Some(arg);
            }
            _ => return Err(format!("unknown option `{arg}`")),
        }
    }
    if let Some(targets) = init_targets {
        init_options.targets = targets;
    }
    if let Some(categories) = init_categories {
        init_options.categories = categories;
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
        "init" => {
            if dry_run || fix || check || !matches!(format, DoctorFormat::Text) {
                return Err("init only accepts init-specific options and --package".to_string());
            }
            Command::Init {
                options: init_options,
            }
        }
        "release" => {
            if dry_run || fix || check || !matches!(format, DoctorFormat::Text) {
                return Err("release check only accepts --manifest and --verbose".to_string());
            }
            match release_subcommand.as_deref() {
                Some("check") => Command::ReleaseCheck {
                    options: ReleaseQualityOptions {
                        manifest: release_manifest
                            .unwrap_or_else(|| PathBuf::from("release.mds.toml")),
                    },
                },
                _ => return Err("release requires subcommand check".to_string()),
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

fn parse_targets(value: &str) -> Result<Vec<AiTarget>, String> {
    if value == "all" {
        return Ok(AiTarget::all().to_vec());
    }
    value
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(|part| {
            AiTarget::parse(part).ok_or_else(|| {
                format!(
                    "unknown AI target `{part}`; expected all, claude-code, codex-cli, opencode, or github-copilot-cli"
                )
            })
        })
        .collect()
}

fn parse_categories(value: &str) -> Result<Vec<AgentKitCategory>, String> {
    if value == "all" {
        return Ok(AgentKitCategory::all().to_vec());
    }
    value
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
        .map(|part| {
            AgentKitCategory::parse(part).ok_or_else(|| {
                format!(
                    "unknown init category `{part}`; expected all, instructions, skills, commands, workflows, or docs"
                )
            })
        })
        .collect()
}

pub fn print_usage() {
    eprintln!("usage: mds check [--package <path>] [--verbose]");
    eprintln!("       mds build [--package <path>] [--dry-run] [--verbose]");
    eprintln!("       mds lint [--package <path>] [--fix [--check]] [--verbose]");
    eprintln!("       mds test [--package <path>] [--verbose]");
    eprintln!("       mds doctor [--package <path>] [--format text|json] [--verbose]");
    eprintln!("       mds package sync [--package <path>] [--check] [--verbose]");
    eprintln!("       mds init [--package <path>] [--ai] [--target <list>] [--categories <list>] [--yes] [--force] [--install-project-deps] [--install-toolchains] [--install-ai-cli] [--verbose]");
    eprintln!("       mds release check [--manifest <path>] [--verbose]");
}

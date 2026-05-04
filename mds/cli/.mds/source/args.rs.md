# src/args.rs

## Purpose

Migrated implementation source for `src/args.rs`.

## Contract

- Preserve the behavior of the pre-migration Rust source.
- This file is synchronized into `.build/rust/mds/cli/src/args.rs`.

## Imports

| From | Target | Symbols | Via | Summary | Reference |
| --- | --- | --- | --- | --- | --- |
| builtin | std::path | PathBuf | - | - | - |
| external | mds_core | AgentKitCategory | - | - | [../../../core/.mds/source/lib.rs.md#source](../../../core/.mds/source/lib.rs.md#source) |
| external | mds_core | AiTarget | - | - | [../../../core/.mds/source/lib.rs.md#source](../../../core/.mds/source/lib.rs.md#source) |
| external | mds_core | BuildMode | - | - | [../../../core/.mds/source/lib.rs.md#source](../../../core/.mds/source/lib.rs.md#source) |
| external | mds_core | CliRequest | - | - | [../../../core/.mds/source/lib.rs.md#source](../../../core/.mds/source/lib.rs.md#source) |
| external | mds_core | Command | - | - | [../../../core/.mds/source/lib.rs.md#source](../../../core/.mds/source/lib.rs.md#source) |
| external | mds_core | DoctorFormat | - | - | [../../../core/.mds/source/lib.rs.md#source](../../../core/.mds/source/lib.rs.md#source) |
| external | mds_core | InitOptions | - | - | [../../../core/.mds/source/lib.rs.md#source](../../../core/.mds/source/lib.rs.md#source) |
| external | mds_core | LabelPreset | - | - | [../../../core/.mds/source/lib.rs.md#source](../../../core/.mds/source/lib.rs.md#source) |
| external | mds_core | NewOptions | - | - | [../../../core/.mds/source/lib.rs.md#source](../../../core/.mds/source/lib.rs.md#source) |
| external | mds_core | PythonTool | - | - | [../../../core/.mds/source/lib.rs.md#source](../../../core/.mds/source/lib.rs.md#source) |
| external | mds_core | RustTool | - | - | [../../../core/.mds/source/lib.rs.md#source](../../../core/.mds/source/lib.rs.md#source) |
| external | mds_core | TypeScriptTool | - | - | [../../../core/.mds/source/lib.rs.md#source](../../../core/.mds/source/lib.rs.md#source) |


## Source


````rs
pub fn parse_args(cwd: PathBuf) -> Result<CliRequest, String> {
    parse_args_from(cwd, std::env::args().skip(1))
}
````

````rs
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
    let mut init_options = InitOptions::default();
    let mut init_targets: Option<Vec<AiTarget>> = None;
    let mut init_categories: Option<Vec<AgentKitCategory>> = None;
    let mut new_name: Option<String> = None;
    let mut new_force = false;
    let mut update_version: Option<String> = None;
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
            "--labels" if command_name == "init" => {
                let Some(value) = args.next() else {
                    return Err("--labels requires a value (en or ja)".to_string());
                };
                init_options.label_preset = LabelPreset::parse(&value)
                    .ok_or_else(|| format!("unknown label preset `{value}`; expected en or ja"))?;
            }
            "--ts-tools" if command_name == "init" => {
                let Some(value) = args.next() else {
                    return Err("--ts-tools requires a value".to_string());
                };
                init_options.ts_tools = parse_ts_tools(&value)?;
            }
            "--py-tools" if command_name == "init" => {
                let Some(value) = args.next() else {
                    return Err("--py-tools requires a value".to_string());
                };
                init_options.py_tools = parse_py_tools(&value)?;
            }
            "--rs-tools" if command_name == "init" => {
                let Some(value) = args.next() else {
                    return Err("--rs-tools requires a value".to_string());
                };
                init_options.rs_tools = parse_rs_tools(&value)?;
            }
            "sync" if command_name == "package" && package_subcommand.is_none() => {
                package_subcommand = Some(arg);
            }
            "--force" if command_name == "new" => {
                new_force = true;
            }
            "--version" if command_name == "update" => {
                let Some(value) = args.next() else {
                    return Err("--version requires a value".to_string());
                };
                update_version = Some(value);
            }
            _ if command_name == "new" && !arg.starts_with('-') && new_name.is_none() => {
                new_name = Some(arg);
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
            return Err("`mds check` was removed; use `mds lint` for structure validation, `mds typecheck` for type checks, and `mds test` for tests".to_string())
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
        "typecheck" => {
            if dry_run || fix || check {
                return Err("typecheck only accepts --package and --verbose".to_string());
            }
            Command::Typecheck
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
        "new" => {
            if dry_run || fix || check || !matches!(format, DoctorFormat::Text) {
                return Err(
                    "new only accepts <name>, --package, --force, and --verbose".to_string()
                );
            }
            let name = new_name.ok_or_else(|| {
                "new requires a file name (e.g. `mds new greet.ts.md`)".to_string()
            })?;
            Command::New {
                options: NewOptions {
                    name,
                    force: new_force,
                },
            }
        }
        "update" => {
            // mds update [--version X.Y.Z]
            Command::Update {
                version: update_version,
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
````

````rs
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
````

````rs
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
                    "unknown init category `{part}`; expected all, instructions, skills, or commands"
                )
            })
        })
        .collect()
}
````

````rs
fn parse_ts_tools(value: &str) -> Result<Vec<TypeScriptTool>, String> {
    if value == "default" {
        return Ok(TypeScriptTool::defaults().to_vec());
    }
    if value == "none" {
        return Ok(Vec::new());
    }
    let tools = parse_tool_list(
        value,
        TypeScriptTool::parse,
        "TypeScript",
        "eslint, prettier, biome, vitest, jest, default, or none",
    )?;
    if tools.contains(&TypeScriptTool::Vitest) && tools.contains(&TypeScriptTool::Jest) {
        return Err("--ts-tools cannot select both vitest and jest".to_string());
    }
    Ok(tools)
}
````

````rs
fn parse_py_tools(value: &str) -> Result<Vec<PythonTool>, String> {
    if value == "default" {
        return Ok(PythonTool::defaults().to_vec());
    }
    if value == "none" {
        return Ok(Vec::new());
    }
    let tools = parse_tool_list(
        value,
        PythonTool::parse,
        "Python",
        "ruff, black, pytest, unittest, default, or none",
    )?;
    if tools.contains(&PythonTool::Pytest) && tools.contains(&PythonTool::Unittest) {
        return Err("--py-tools cannot select both pytest and unittest".to_string());
    }
    Ok(tools)
}
````

````rs
fn parse_rs_tools(value: &str) -> Result<Vec<RustTool>, String> {
    if value == "default" {
        return Ok(RustTool::defaults().to_vec());
    }
    if value == "none" {
        return Ok(Vec::new());
    }
    let tools = parse_tool_list(
        value,
        RustTool::parse,
        "Rust",
        "rustfmt, clippy, cargo-test, nextest, default, or none",
    )?;
    if tools.contains(&RustTool::CargoTest) && tools.contains(&RustTool::Nextest) {
        return Err("--rs-tools cannot select both cargo-test and nextest".to_string());
    }
    Ok(tools)
}
````

````rs
fn parse_tool_list<T>(
    value: &str,
    parse: fn(&str) -> Option<T>,
    label: &str,
    expected: &str,
) -> Result<Vec<T>, String>
where
    T: Copy + Eq,
{
    let mut tools = Vec::new();
    for part in value
        .split(',')
        .map(str::trim)
        .filter(|part| !part.is_empty())
    {
        if part == "default" || part == "none" {
            return Err(format!(
                "{label} tool preset `{part}` must be used by itself"
            ));
        }
        let tool = parse(part)
            .ok_or_else(|| format!("unknown {label} tool `{part}`; expected {expected}"))?;
        if !tools.contains(&tool) {
            tools.push(tool);
        }
    }
    Ok(tools)
}
````



````rs
pub fn print_usage() {
    eprintln!();
    eprintln!("mds — Markdown-driven code generation toolchain");
    eprintln!();
    eprintln!("Commands:");
    eprintln!(
        "  mds init                                  Interactive project setup (wizard mode)"
    );
    eprintln!("  mds init [options] --yes                  Non-interactive project setup");
    eprintln!("  mds new <name.lang.md>                    Create new implementation Markdown");
    eprintln!("  mds build [--package <path>] [--dry-run]  Generate derived code");
    eprintln!("  mds typecheck [--package <path>]          Run type checks on code blocks");
    eprintln!("  mds lint [--package <path>] [--fix]       Run linters on code blocks");
    eprintln!("  mds test [--package <path>]               Run tests from code blocks");
    eprintln!("  mds doctor [--package <path>]             Diagnose environment");
    eprintln!("  mds package sync [--package <path>]       Sync package index.md");
    eprintln!();
    eprintln!("Init options:");
    eprintln!("  --ai                      AI agent kit only (skip project files)");
    eprintln!("  --target <list>           AI targets: all, claude-code, codex-cli, opencode, github-copilot-cli");
    eprintln!(
        "  --categories <list>       Agent kit categories: all, instructions, skills, commands"
    );
    eprintln!("  --ts-tools <list>         TypeScript tools: eslint, prettier, biome, vitest, jest, default, none");
    eprintln!(
        "  --py-tools <list>         Python tools: ruff, black, pytest, unittest, default, none"
    );
    eprintln!("  --rs-tools <list>         Rust tools: rustfmt, clippy, cargo-test, nextest, default, none");
    eprintln!("  --labels <preset>         Section label language: en (default), ja (Japanese)");
    eprintln!("  --yes                     Execute without confirmation");
    eprintln!("  --force                   Overwrite non-managed files");
    eprintln!("  --install-project-deps    Run npm install / cargo fetch / uv sync");
    eprintln!("  --install-toolchains      Check required toolchains");
    eprintln!("  --install-ai-cli          Check AI CLI tools");
    eprintln!();
    eprintln!("Global options:");
    eprintln!("  --package <path>          Target package directory");
    eprintln!("  --verbose                 Show detailed output");
    eprintln!("  --help, -h                Show this help message");
    eprintln!("  --version, -V             Show version");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  mds init                                      # Interactive wizard");
    eprintln!("  mds init --package ./my-pkg --yes              # Quick setup with defaults");
    eprintln!("  mds new greet.ts.md                            # New TypeScript implementation");
    eprintln!("  mds new utils/helper.py.md --package ./my-pkg  # New Python implementation");
    eprintln!("  mds build --package ./my-pkg --dry-run         # Preview generation");
    eprintln!("  mds build --package ./my-pkg                   # Generate code");
    eprintln!();
    eprintln!("Documentation: https://github.com/owox/mds");
}
````

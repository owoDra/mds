# src/main.rs

## Purpose

Migrated implementation source for `src/main.rs`.

## Contract

- Preserve the behavior of the pre-migration Rust source.
- This file is synchronized into `.build/rust/mds/cli/src/main.rs`.

## Imports

| From | Target | Symbols | Via | Summary | Reference |
| --- | --- | --- | --- | --- | --- |
| external | mds_cli::args | parse_args | - | - | [args.rs.md#source](args.rs.md#source) |
| external | mds_cli::args | print_usage | - | - | [args.rs.md#source](args.rs.md#source) |
| external | mds_cli::wizard | run_interactive_init | - | - | [wizard.rs.md#source](wizard.rs.md#source) |
| external | mds_core | execute | - | - | [../../../core/.mds/source/lib.rs.md#source](../../../core/.mds/source/lib.rs.md#source) |
| external | mds_core | CliRequest | - | - | [../../../core/.mds/source/lib.rs.md#source](../../../core/.mds/source/lib.rs.md#source) |
| external | mds_core | Command | - | - | [../../../core/.mds/source/lib.rs.md#source](../../../core/.mds/source/lib.rs.md#source) |
| builtin | std::io | IsTerminal | - | - | - |
| builtin | std::process | Command as ProcessCommand | - | - | - |


## Source


````rs
const VERSION: &str = env!("CARGO_PKG_VERSION");
````

````rs
fn main() -> std::process::ExitCode {
    run()
}

fn run() -> std::process::ExitCode {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(error) => {
            eprintln!("internal error: failed to read current directory: {error}");
            return exit_code(3);
        }
    };

    let args: Vec<String> = std::env::args().skip(1).collect();

    // Handle --help and --version before any other processing
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        print_usage();
        return exit_code(0);
    }
    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!("mds {VERSION}");
        return exit_code(0);
    }

    // Interactive wizard: `mds init` with no additional flags
    if args.len() == 1 && args[0] == "init"
        || args.len() == 3 && args[0] == "init" && args[1] == "--package"
    {
        let package = if args.len() == 3 {
            Some(std::path::PathBuf::from(&args[2]))
        } else {
            None
        };

        match run_interactive_init(&cwd, package.as_deref()) {
            Ok(options) => {
                let request = CliRequest {
                    cwd,
                    package,
                    verbose: false,
                    command: Command::Init { options },
                };
                let result = execute(request);
                print_cli_output(&result.stdout, &result.stderr);
                return exit_code(result.exit_code);
            }
            Err(message) => {
                eprintln!("{message}");
                return exit_code(2);
            }
        }
    }

    let request = match parse_args(cwd) {
        Ok(request) => request,
        Err(message) => {
            eprintln!("error: {message}");
            eprintln!();
            if message.contains("missing command") {
                eprintln!("hint: Run `mds init` to set up a new project interactively.");
                eprintln!("      Run `mds lint --package <path>` to validate an existing project.");
            } else if message.contains("unknown command") {
                eprintln!("hint: Available commands: init, build, typecheck, lint, test, doctor, package sync");
            } else if message.contains("unknown option") {
                eprintln!("hint: Use --verbose for detailed output. Run `mds` without arguments for full usage.");
            }
            print_usage();
            return exit_code(2);
        }
    };

    // Handle update command directly (it replaces the current binary)
    if let Command::Update { ref version } = request.command {
        return run_self_update(version.as_deref());
    }

    let result = execute(request);
    print_cli_output(&result.stdout, &result.stderr);
    exit_code(result.exit_code)
}
````

````rs
fn run_self_update(version: Option<&str>) -> std::process::ExitCode {
    let repo = "owo-x-project/owox-mds";
    let target_version = match version {
        Some(v) => v.to_string(),
        None => {
            eprintln!("Checking for latest version...");
            match fetch_latest_version(repo) {
                Some(v) => v,
                None => {
                    eprintln!("error: failed to fetch latest version from GitHub");
                    return exit_code(1);
                }
            }
        }
    };

    if target_version == VERSION {
        println!("mds is already at version {VERSION}");
        return exit_code(0);
    }

    println!("Updating mds from {VERSION} to {target_version}...");

    let install_script = format!("https://raw.githubusercontent.com/{repo}/main/install.sh");

    let status = ProcessCommand::new("sh")
        .arg("-c")
        .arg(format!(
            "curl -fsSL {install_script} | sh -s -- --version {target_version}"
        ))
        .status();

    match status {
        Ok(s) if s.success() => {
            println!("Successfully updated to mds {target_version}");
            exit_code(0)
        }
        Ok(s) => {
            eprintln!("Update failed with exit code: {}", s.code().unwrap_or(1));
            exit_code(1)
        }
        Err(e) => {
            eprintln!("error: failed to run update: {e}");
            eprintln!("hint: You can manually update with:");
            eprintln!("  curl -fsSL https://raw.githubusercontent.com/{repo}/main/install.sh | sh");
            exit_code(1)
        }
    }
}
````

````rs
fn print_cli_output(stdout: &str, stderr: &str) {
    let stdout_color = std::io::stdout().is_terminal() && use_color();
    let stderr_color = std::io::stderr().is_terminal() && use_color();
    if !stdout.is_empty() {
        print!("{}", colorize_output(stdout, stdout_color));
    }
    if !stderr.is_empty() {
        eprint!("{}", colorize_output(stderr, stderr_color));
    }
}

fn use_color() -> bool {
    std::env::var_os("NO_COLOR").is_none()
}

fn colorize_output(output: &str, enabled: bool) -> String {
    if !enabled {
        return output.to_string();
    }
    let mut rendered = String::new();
    for chunk in output.split_inclusive('\n') {
        rendered.push_str(&colorize_line(chunk));
    }
    if !output.ends_with('\n') && !output.is_empty() {
        let tail = output.lines().last().unwrap_or_default();
        if rendered.is_empty() {
            rendered.push_str(&colorize_line(tail));
        }
    }
    rendered
}

fn colorize_line(line: &str) -> String {
    let trimmed = line.trim_end_matches('\n');
    let suffix = if line.ends_with('\n') { "\n" } else { "" };
    let prefix = if trimmed.starts_with("error:") {
        "\x1b[31m"
    } else if trimmed.starts_with("warning:") {
        "\x1b[33m"
    } else if trimmed.starts_with("hint:") {
        "\x1b[36m"
    } else if trimmed.ends_with(" ok") || trimmed.contains(" ok:") {
        "\x1b[32m"
    } else {
        ""
    };
    if prefix.is_empty() {
        return line.to_string();
    }
    format!("{prefix}{trimmed}\x1b[0m{suffix}")
}

fn exit_code(code: i32) -> std::process::ExitCode {
    std::process::ExitCode::from(code.clamp(0, 255) as u8)
}
````



````rs
fn fetch_latest_version(repo: &str) -> Option<String> {
    let output = ProcessCommand::new("curl")
        .args([
            "-fsSL",
            &format!("https://api.github.com/repos/{repo}/releases/latest"),
        ])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let body = String::from_utf8(output.stdout).ok()?;
    // Simple JSON parsing for "tag_name": "vX.Y.Z"
    let tag_start = body.find("\"tag_name\"")?;
    let value_start = body[tag_start..].find('"').and_then(|i| {
        let rest = &body[tag_start + i + 1..];
        rest.find('"').map(|j| tag_start + i + 1 + j + 1)
    })?;
    let value_end = body[value_start..].find('"')?;
    let tag = &body[value_start..value_start + value_end];
    Some(tag.strip_prefix('v').unwrap_or(tag).to_string())
}
````

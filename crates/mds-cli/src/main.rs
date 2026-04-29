use mds_cli::args::{parse_args, print_usage};
use mds_cli::wizard::run_interactive_init;
use mds_core::{execute, CliRequest, Command};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let cwd = match std::env::current_dir() {
        Ok(cwd) => cwd,
        Err(error) => {
            eprintln!("internal error: failed to read current directory: {error}");
            std::process::exit(3);
        }
    };

    let args: Vec<String> = std::env::args().skip(1).collect();

    // Handle --help and --version before any other processing
    if args.is_empty() || args.iter().any(|a| a == "--help" || a == "-h") {
        print_usage();
        std::process::exit(0);
    }
    if args.iter().any(|a| a == "--version" || a == "-V") {
        println!("mds {VERSION}");
        std::process::exit(0);
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

        match run_interactive_init() {
            Ok(options) => {
                let request = CliRequest {
                    cwd,
                    package,
                    verbose: false,
                    command: Command::Init { options },
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
            Err(message) => {
                eprintln!("{message}");
                std::process::exit(2);
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
                eprintln!(
                    "      Run `mds check --package <path>` to validate an existing project."
                );
            } else if message.contains("unknown command") {
                eprintln!("hint: Available commands: init, check, build, lint, test, doctor, package sync, release check");
            } else if message.contains("unknown option") {
                eprintln!("hint: Use --verbose for detailed output. Run `mds` without arguments for full usage.");
            }
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

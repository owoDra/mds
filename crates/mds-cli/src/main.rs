use mds_cli::args::{parse_args, print_usage};
use mds_core::execute;

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

use minigrep::Config;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    // Saving the argument values in variables
    let config = Config::build(&args).unwrap_or_else(|error| {
        eprintln!("Problem parsing arguments: {error}");
        process::exit(1);
    });

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

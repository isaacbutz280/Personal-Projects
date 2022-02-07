use slides::Config;
use std::{env, process};

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    // Possible improvement: Add confirmation here?

    if let Err(e) = slides::run(& config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    }
}

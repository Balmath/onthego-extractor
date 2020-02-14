use std::env;
use std::process;

use onthego_extractor::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("{}", err);
        process::exit(1);
    });

    if let Err(err) = onthego_extractor::run(config) {
        println!("Execution error: {}", err);
        process::exit(1);
    }
}


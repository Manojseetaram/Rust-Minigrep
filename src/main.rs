use std::env;
use std::fs;
use std::process;

use minigrep::{Config, run}; 

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing argument: {err}");
        process::exit(1);
    });

    println!("Searching for: {}", config.query);
    println!("In file: {}", config.file_path);

    let contents = fs::read_to_string(&config.file_path)
        .expect("should have been a file");

    println!("This is a poem.txt file:\n{}", contents);

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

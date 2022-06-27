extern crate rep;

use std::{env, process};

use rep::Config;

fn main() {
    // unwrap_or_else will pass the inner value of Err to the closure ( |err| )
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = rep::run(config) {
        eprintln!("error: {}", e);

        process::exit(1);
    }
}
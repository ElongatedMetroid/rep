extern crate rep;

use std::{env, process};

use rep::Config;

fn main() {
    // the args function returns an iterator to the command line arguments
    // we can use the collect function on an iterator to turn the iterators
    // values into collection such as in this example a vector    
    let args: Vec<String> = env::args().collect();
    
    // unwrap_or_else will pass the inner value of Err to the closure ( |err| )
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(e) = rep::run(config) {
        println!("error: {}", e);

        process::exit(1);
    }
}
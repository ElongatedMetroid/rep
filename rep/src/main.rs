use std::{env, process};
use std::fs::File;
use std::io::prelude::*;

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

    println!("Querying for '{}' in file '{}'", config.query, config.filename);

    let mut f = File::open(config.filename)
        .expect("File not found!");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("Error in reading the file!");

    println!("File contains:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments provided");
        }

        // make a copy of the data for the Config instance to own
        let query = args[1].clone();
        let filename = args[2].clone();
    
        Ok( Config {query, filename} )
    }
}


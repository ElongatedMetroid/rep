use std::fs::File;
use std::io::prelude::*;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments provided");
        }

        // make a copy of the data for the Config instance to own
        let query = args[1].clone();
        let filename = args[2].clone();
    
        Ok( Config {query, filename} )
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // the '?' operator will return the error value from the current function
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }

    Ok(())
}

// the vector returned will contain string slices that reference slices of the parameter contents
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();
    
    // the lines method returns an iterator to each line
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!( 
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }
}
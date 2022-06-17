use std::fs::File;
use std::env;
use std::io::prelude::*;
use std::error::Error;

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments provided");
        }

        // make a copy of the data for the Config instance to own
        let query = args[1].clone();
        let filename = args[2].clone();
    
        // returns true is the environment variable does not exist
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok( Config {query, filename, case_sensitive} )
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    // the '?' operator will return the error value from the current function
    let mut f = File::open(config.filename)?;

    let mut contents = String::new();
    f.read_to_string(&mut contents)?;

    // let results equal the search results depending on configuration
    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
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

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // lowercase the query string
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        // check whether each line converted to lowercase contatains the lowercase query
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
} 

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!( 
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
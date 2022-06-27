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
    pub fn new(mut args: std::env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(q) => q,
            None => return Err("No query provided!"),
        };

        let filename = match args.next() {
            Some(f) => f,
            None => return Err("No filename provided!"),
        };
    
        // returns true if the environment variable does exist
        let mut case_sensitive = env::var("CASE_SENSITIVE").is_ok();

        if args.len() > 3 {
            case_sensitive = match args.next() {
                Some(c) => c,
                None => String::new(),
            }.contains("c");
        }

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
    contents.lines()
        // go through each line of the contents and keep only the lines that contain the query
        // (keep only the lines where line.contains(query) returns true)
        .filter(|line| line.contains(query))
        .collect()
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // lowercase the query string
    let query = query.to_lowercase();

    contents.lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
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
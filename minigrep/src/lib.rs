use std::env;
use std::error::Error;
use std::fs;

// Make struct and members public so they can be used elsewhere
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

// Implement the parser function as a Config::build() function instead.
// The Error variant of the Result enum is a reference a string with a static lifetime.
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("Not enough arguments");
        }

        // First value in the vector will the name of our binary. Matches behavior of C programs. Lets programs use the name by which they were evoked in their execution.
        let query = args[1].clone();
        let file_path = args[2].clone();

        // The .is_ok() returns true if the Result enum is Ok and False if it is Err.
        // Doing this, we're checking whether or not environment variable is set, not
        // what the value of the environment variable is.
        // This call serves to unwrap the Result enum.
        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

// Extract logic to separate function.
// Box<dyn Error> specifies that the function will return a type that implements the Error trait.
// It's a way of specifying that an error will be returned, without specifying which error.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    let results = if config.ignore_case {
        search_case_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };

    for line in results {
        println!("{line}");
    }

    Ok(())
}

// The results is a vector of string slices, the string from which the slice
// is taken must be valid for the slice to be. Therefor, the result of search
// is defined to have the same lifetime as contents.
// If we sliced from a string in memory, we could use a static lifetime instead
// since the string would not be invalid once contents go out of scope.
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results: Vec<&str> = Vec::new();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
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

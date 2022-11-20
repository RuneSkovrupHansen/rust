use std::error::Error;
use std::fs;

// Make struct and members public so they can be used elsewhere
pub struct Config {
    pub query: String,
    pub file_path: String,
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

        Ok(Config { query, file_path })
    }
}

// Extract logic to separate function.
// Box<dyn Error> specifies that the function will return a type that implements the Error trait.
// It's a way of specifying that an error will be returned, without specifying which error.
pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    println!("Text:\n{}", contents);
    Ok(())
}

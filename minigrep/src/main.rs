use std::env;
use std::fs;

fn main() {
    // https://doc.rust-lang.org/std/env/index.html
    // std::env::args() returns an iterator of a process, yielding a String for each argument.

    let args: Vec<String> = env::args().collect();

    // First value in the vector will the name of our binary. Matches behavior of C programs. Lets programs use the name by which they were evoked in their execution.

    let query = &args[1];
    let file_path = &args[2];

    println!("Searching for {} in file {}", query, file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read file");

    println!("Text:\n{}", contents);
}

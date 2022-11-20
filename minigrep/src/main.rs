use std::env;
use std::process;

// Pull struct into scope
use minigrep::Config;

fn main() {
    // https://doc.rust-lang.org/std/env/index.html
    // std::env::args() returns an iterator of a process, yielding a String for each argument.

    let args: Vec<String> = env::args().collect();

    // Use of unwrap_or_else to handle Results return, unpacks the result if Ok variant,
    // if the variant is Error it is passed to function body.
    // The function unwrap_or_else is defined on the Results enum in the standard library.
    // |err| passes the error to the closure, i.e. anonymous function so that it can be used.
    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem passing arguments: {err}");
        // process::exit stops the program immediately like panic but does not print additional info like panic does.
        process::exit(1);
    });

    // Use of if-let to capture the error variant of the Result enum. We do not care about the okay variant
    // since it returns () and just specifies that everything works.
    // if-let is in this case similar to unwrap_or_else, we're just discarding the okay case.

    // Call run function from library
    if let Err(e) = minigrep::run(config) {
        println!("Application error: {e}");
        process::exit(1);
    }
}

use std::env;
use std::process;

// Pull struct into scope
use minigrep::Config;

fn main() {
    // https://doc.rust-lang.org/std/env/index.html
    // std::env::args() returns an iterator of a process, yielding a String for each argument.

    // unwrap_or_else takes closure for Err variant.
    let config: Config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem passing arguments: {err}");
        // process::exit stops the program immediately like panic but does not print additional info like panic does.
        process::exit(1);
    });

    // Use of if-let to capture the error variant of the Result enum. We do not care about the okay variant
    // since it returns () and just specifies that everything works.
    // if-let is in this case similar to unwrap_or_else, we're just discarding the okay case.

    // Call run function from library
    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

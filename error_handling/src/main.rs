use std::fs::File;
use std::io;
use std::io::Read;

fn main() {
    let filename = "hello.txt";

    // file: Result<File::open, std::io::Error>
    let greetings_file_result = File::open(filename);

    // Result enum provides a way for a call to tell us whether it succeeded or failed, and provide os either the return or error information.
    // If successful, Result will be an instance of Ok, and if it fails, it will be an instance of Err.

    let greetings_file = match greetings_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            io::ErrorKind::NotFound => match File::create(filename) {
                Ok(fc) => fc,
                Err(e) => panic!("Error creating file: {:?}", e),
            },
            other_error => {
                panic!("Error opening file: {:?}", other_error)
            }
        },
    };

    // unwrap() and expect(), shortcuts to panic on a Result enum. Use expect in production code.

    let username = read_username_from_file(filename);
    match username {
        Ok(username) => println!("username: {}", username),
        Err(error) => panic!("Error reading username: {:?}", error),
    }
}

fn read_username_from_file(filename: &str) -> Result<String, io::Error> {
    let mut username = String::new();
    File::open(filename)?.read_to_string(&mut username)?;
    Ok(username)
}

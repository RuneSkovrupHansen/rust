fn main() {
    // Ownership
    let string_literal: &str = "Hello world!";
    let mut string: String = String::from(string_literal);

    string.push_str(" It's great to meet you!");
    println!("{}", string);

    // Transfer ownership to function.
    // _take_ownership(string);

    // move occurs because `string` has type `String`, which does not implement the `Copy` trait
    // println!("{}", string);

    string = take_ownership_and_return(string);
    println!("{}", string);

    let new_string: String = String::from("My new string");
    proclaim(&new_string);

    let _my_slice = &new_string[..2];

    let first_word: &str = first_word(string_literal);

    println!("{}", first_word);
}

fn _take_ownership(some_string: String) -> String {
    some_string
}

fn take_ownership_and_return(some_string: String) -> String {
    some_string
}

fn proclaim(s: &String) -> () {
    println!("{}!", s);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

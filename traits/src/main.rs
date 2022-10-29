use traits::{Email, MediumPost, Summary};

fn print_summary(item: impl Summary) {
    println!("{}", item.summarize());
}

fn main() {
    let post = MediumPost {
        title: String::from("Rust, the next big thing?"),
        subtitle: String::from("An overview of Rust's place in the programming ecosystem"),
        text: String::from("Bla bla..."),
        author: String::from("Rune Skovrup"),
    };

    let subject = String::from("About Rust");
    let text = String::from("I would like to discuss the potential use case for Rust ...");
    let sender = String::from("Rune Skovrup");

    let email = Email {
        subject,
        text,
        sender,
    };

    println!("Post summary: {}", post.summarize());
    println!("Email summary: {}", email.summarize());

    print_summary(email);
}

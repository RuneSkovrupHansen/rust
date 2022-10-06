fn main() {
    let mut user_1 = User {
        active: true,
        username: String::from("username"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user_1.email = String::from("nooby@example.com");

    let user_2 = build_user(
        String::from("RuneSkovrupHansen"),
        String::from("runeskovrup@gmail.com"),
    );

    // Struct update, will make user_2 unusable because of = like assignment
    let user_3 = User {
        username: String::from("RuneSHansen"),
        ..user_2
    };
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(username: String, email: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

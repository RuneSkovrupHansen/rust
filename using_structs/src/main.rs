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

    // Struct update, will make user_2 unusable because of = like assignment which is caused by using heap data types.
    let user_3 = User {
        username: String::from("RuneSHansen"),
        ..user_2
    };

    let rect1: Rectangle = Rectangle {
        height: 10,
        width: 20,
    };

    // Debug printing, requires #[derive(Debug)]
    println!("rect1: {:?}", rect1);
    println!("rect1: {:#?}", rect1);

    // dbg is an expression, takes ownership and returns.
    dbg!(&rect1);

    let rect1: Rectangle = Rectangle {
        height: 50,
        width: 50,
    };

    let rect2: Rectangle = Rectangle {
        height: 30,
        width: 30,
    };

    let rect3: Rectangle = Rectangle {
        height: 10,
        width: 10,
    };

    println!("rect1 can hold rect2: {}", rect1.can_hold(&rect2));
    println!("rect3 can hold rect3: {}", rect2.can_hold(&rect3));
    println!("rect3 can hold rect1: {}", rect3.can_hold(&rect1));
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

// Tuple Structs
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Unit-Like Structs
struct UnitLike;

#[derive(Debug)]
struct Rectangle {
    height: i32,
    width: i32,
}

impl Rectangle {
    fn can_hold(&self, rectangle: &Rectangle) -> bool {
        return self.height >= rectangle.height && self.width >= rectangle.width;
    }
}

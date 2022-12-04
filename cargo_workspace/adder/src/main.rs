use add_one;
use add_two;

fn main() {
    let mut number = 0;
    println!("number, {}", number);

    number = add_one::add_one(number);
    println!("number, {}", number);

    number = add_two::add_two(number);
    println!("number, {}", number);
}

fn main() {
    let fib: u32 = get_fibonacci_number(4);

    println!("The fibonacci number is: {fib}");
}

fn get_fibonacci_number(number: u32) -> u32 {
    let mut value: u32 = 0;
    let mut temp: u32 = 1;

    for _ in 0..number - 1 {
        value += temp;
        temp = value;
    }

    return value;
}

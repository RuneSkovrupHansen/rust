fn main() {
    let fib: u32 = get_fibonacci_number(3);
    println!("The fibonacci number is: {fib}");
}

fn get_fibonacci_number(number: u32) -> u32 {
    return match number {
        0..=1 => number,
        _ => {
            let mut n1: u32 = 0;
            let mut n2: u32 = 1;
            let mut value: u32 = 0;

            for _ in 0..=(number - 2) {
                value = n2 + n1;
                n1 = n2;
                n2 = value;
            }

            value
        }
    };
}

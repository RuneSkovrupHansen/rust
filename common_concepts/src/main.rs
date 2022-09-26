fn main() {
    // Variables

    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // Type must be declared for const, limited number of compiler operations
    const THREE_HOURS_IN_SECONDS: i32 = 60 * 60 * 3;

    // Shadowing is a nice way to avoid having to come up with new variable names
    {
        let x = 500;
        println!("The value of x in the inner scope is: {x}");
        // x = 200; // Shadow is immutable, this will not compile
    }
    println!("The value of x is: {x}");

    // Data types

    // Scalar types, a single value in one type
    // Compound type, multiple values into one type

    let mut x = 1_000_000;

    let c: char = 'R'; // Use of single quotes for char
}

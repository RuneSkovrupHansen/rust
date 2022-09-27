fn main() {
    // Data types.

    // Compound data types can group multiple values into one type.
    // Both arrays and tuples are fixed size.

    let tuple: (char, u32, f64) = ('R', 24, 193.0);
    let (x, _y, _z) = tuple; // Prefixing with _ to indicate that values are unused
    println!("The value of x is: {x}");

    let _array: [i32; 4] = [5; 4]; // [5, 5, 5, 5]
    let array: [i32; 4] = [1, 2, 3, 4];

    let first = array[0];
    let second = array[1];

    println!("The value of first is: {first}, the value of second is: {second}")

    // Functions.
}

fn main() {
    box_pointer();
}

fn box_pointer() {
    // Store i32 on the heap, the pointer of type box is stored on the stack.
    let b = Box::new(5);
    println!("b = {}", b);
}
// Both the pointer b and the data it points to goes out of scope and is deallocated.

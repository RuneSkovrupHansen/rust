# rust
Repository for experimenting with the Rust programming language

# Hello, world!

rustc - The Rust compiler
rustup - The Rust toolchain tool

Main, fn main() {}, is special, always the first code that runs in every executable Rust program.

rustfmt - Standard Rust formatting tool

Using "!", ex. name!(), means we're calling a macro and not a function. Different rules.

 Rust is an ahead-of-time compiled language, meaning you can compile a program and give the executable to someone else, and they can run it even without having Rust installed.

# Hello, cargo!

Build system and package manager, common to use for Rust.

`cargo new <name>` creates new package, including setup.

TOML (Tom’s Obvious, Minimal Language) format, Cargo’s configuration format.

In Rust, packages of code are referred to as crates.

Cargo expects certain structure, src files in src/, etc.

`cargo build`
`cargo run`
`cargo check` - Checks compilation, does not produce executable, a lot faster then build.

When your project is finally ready for release, you can use cargo build --release to compile it with optimizations. This command will create an executable in target/release instead of target/debug.


# Ownership

## Stack

All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead.

## Heap

Heap allocation, the memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a pointer, which is the address of that location.
Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, but when you want the actual data, you must follow the pointer.

Pushing to the stack is faster than allocating on the heap because the allocator never has to search for a place to store new data; that location is always at the top of the stack.

## Ownership

* Each value in Rust has an owner.
* There can only be one owner at a time.
* When the owner goes out of scope, the value will be dropped.

## Memory and allocation

The basic data types are stored on the stack, trivial to add and pop off the stack. Trivial to make new copies.

String literals have a separate type, str, not String, which is more advanced. String stores data on the heap, therefore it can store an amount of text that is unknown at compile time.

String can be formed from string literal using `String::from(<literal>)`.

String can be mutated, if the variable itself is declared as mutable.

Literals immutability affords it significant speed and efficiency.

The String type requires us to dynamically allocate memory on the heap during runtime. Unknown at compile time.

To work with heap allocated types we need two things:

* Request memory from the memory allocator at runtime.
* Return memory to allocator when we're done with it.

We request memory at runtime using String::from() for example. Universal for all programming languages to manually request memory on the heap.

In many languages the second step is done using a garbage collector (GC), which keeps track of unused memory and cleans up memory that isn’t being used anymore, and we don’t need to think about it. In languages without a GC we need to manually allocate and free memory. Error prone. Double free memory issue.

Rust takes a different path: the memory is automatically returned once the variable that owns it goes out of scope.

Natural to return memory when the variable using memory goes out of scope. Rust calls special function `drop`when variables go out of scope.

Can get complicated.

For heap allocated variables we copy the data for the pointer, and not the object when binding multiple variables. The object is the same. However, rust also invalidates the first reference to the object.

Copying only the points is in some languages called a *shallow copy*, whereas copying the data is called a *deep copy*. However, in Rust, since we're copying the pointer and invalidating any existing reference, it is called a *move*.

This is used to prevent double free memory errors.

Rust never automatically creates deep copies of data. Common to use `<object>.clone()` to make a deep copy.

Talking about the data we can talk about the stack and the heap data. The stack data is the pointer that points into the heap, whereas the heap data is the actual data allocated on the heap. A shallow copy will only copy stack data, whereas a deep copy will also copy the heap data.

For data types such as integers their size is known and live entirely on the stack. Therefor copies do not invalidate existing references. *Copy* trait can be placed on types stored on the stack to provide this behavior. Trivially copied, still valid after assignment to another variable.

Variables which do not need heap allocation will have the Copy trait.

## Ownership and functions

The mechanics of passing a value to a function are similar to those when assigning a value to a variable.

Move for heap data and copy for stack data (with Copy annotation).

Ownership can be given or returned from a function, if the function returns a value. Ownership is given if a variables comes into scope inside of the function and is then returned.

The ownership of a variable follows the same pattern every time: assigning a value to another variable moves it. When a variable that includes data on the heap goes out of scope, the value will be cleaned up by drop unless ownership of the data has been moved to another variable.

## References and Borrowing

* At any given time, you can have either one mutable reference or any number of immutable references.
* References must always be valid.

References and borrowing is useful instead of having to deal with moves when passing data to functions.

A reference is like a pointer in that it’s an address we can follow to access the data stored at that address; that data is owned by some other variable. Unlike a pointer, a reference is guaranteed to point to a valid value of a particular type for the life of that reference.

A reference is annotated with &. A reference to an object points to the object.

References do not take ownership, when references go out of scope the variable is not dropped.

Referencing is called borrowing in Rust terms.

By default, references are immutable. We must pass a mutable reference `&mut` for the referenced object to be mutable. Note that the original object must also be mutable, otherwise the types are mismatched.

Mutable references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value.

Allows for mutation but in a very controlled fashion, prevents data races.

It's possible to have multiple references to an object, however, there can only ever be one mutable reference to an object. There cannot be a mutable reference to an object, if there are still immutable references in use.

Example error:

"cannot borrow `s` as mutable because it is also borrowed as immutable"

Rust prevents dangling references, i.e. references to things which no longer exists. Done at compile time.

Example of trying to return a reference from a function to an object which goes out of scope with the function. To solve this, instead of returning a reference, the value should be returned, which will result in a move of the value.

## The Slice Type

Slices let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.

A slice refers to a portion of a collection.

We create slices using a range within brackets by specifying [starting_index..ending_index], where starting_index is the first position in the slice and ending_index is one more than the last position in the slice.

Internally, slice stores starting position and length of the slice, which is ending_index-starting_index.

Ranges syntax for slices:

* 0..2 == ..2
* 2.. - From index 2 and to the end of the collection
* .. - The entire collection.

When defining a slice, we must use the reference an object. In other words, slice is borrowing from a collection.

A string slice has the type `str`. It's commonly seen in it's borrowed form `&str`.

Slice creates an immutable reference to an object. This has compile time implications for ensuring that the object cannot be modified later, since creating a mutable reference is not possible, if the immutable reference is still active.


String literals are slices. Since the sting literals are stored in the binary, and the variables are referring to the string.

`&str` a reference to a string literal, which is stored in the binary.

A reference to String, `&String` can be passed to a function accepting `&str`. This flexibility takes advantages of *deref coercions*.


# Using Structs to Structure Related Data
 



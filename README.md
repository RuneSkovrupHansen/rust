# rust
Repository for experimenting with the Rust programming language


# Misc notes

Final line of a block is called the tail, it is what the block evaluates to, as the block itself is an expression. This is also true for functions.

A good way to write Rust is to make it impossible to represent invalid states, this allows Rust type system to come into effect at compile-time and forces the programmer to handle all possible cases at compile-time.

Ranges are lazily computed at runtime, does not store all values in memory.


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
 
User defined data structure to hold multiple related values. Named values to add meaning and flexibility.
Lifetime specifiers are required for fields of a struct which is not owned by that struct. For example a *&str* is not owned by the struct but is in the binary. A *String* by contrast, is a heap structure that is owned by the struct.

Accessing fields of a borrowed struct instance does not move the field values, which is why you often see borrows of structs.

## Derived Traits

The println! macro can do many kinds of formatting, and by default, the curly brackets tell println! to use formatting known as Display: output intended for direct end user consumption. 

Putting the specifier :? inside the curly brackets tells println! we want to use an output format called Debug. The Debug trait enables us to print our struct in a way that is useful for developers so we can see its value while we’re debugging our code.

Methods are defined within the context of a struct. First parameter is always self.

`impl <struct> { ... }`

Dot notation is called "method syntax" in relation to calling functions.

&self is alias for &Self, reference to type of Struct.

Self is always borrowed, and can be borrowed with or without mutability. Use of `&mut self` to mutably borrow.

Very rare to actually take ownership by only using `self`.

Common to have methods with the same name as fields as getters. Visibility modifiers are available for Structs.

## Automatic referencing and dereferencing

In C and C++, two different operators are used for calling methods: you use . if you’re calling a method on the object directly and -> if you’re calling the method on a pointer to the object and need to dereference the pointer first. In other words, if object is a pointer, object->something() is similar to (*object).something().

In Rust, the dot notation can always be used. Rust automatically adds in the required modifiers.

This automatic referencing behavior works because methods have a clear receiver—the type of self.

The self in methods are very important for how it works, it seems.

## Associated Functions

Functions inside an `impl` block are associated functions, since they're associated with the type named after the impl.

Associated functions can be defined without having self, because they don't need an instance of the type to work with. Example, String::from().

The Self keywords in the return type and in the body of the function are aliases for the type that appears after the impl keyword. So for an associated function without self as a parameter, Self can be referred to, to get the associated type.

## Misc

After *defining* a struct we can create an *instance* of it.

Use of the dot-notation to access values of the struct, and if the instance if mutable, modify them.

Mutability applies to the entire structure.

Use of *field init shorthand* to easily initialize structs. Parameter names not required if the variables passed have the same name.

Struct update syntax uses uses = like assignment, meaning that move is used if there are any heap data copied.


# Enums and Pattern Matching

Enumerations, aka. enums. Makes it possible to define a type by enumerating its possible variants.

Where a struct provides a way to group related fields and data, an enum provides a way of saying a value is one of a possible set of values.

Useful to have variants of a type, and being able to list or *enumerate* all possible variants of that type. The type is fundamentally the same, but with variants.

Example of IP address versions, v4 and v6.

When defined, an enum is a custom data type that we can use elsewhere in our code.

Variants namespaced under identifier, which is the name of the enum:

`enumName::variant`

When defining enums with type, the name of each enum variant that we define also becomes a function that constructs an instance of the enum.


Variants of an enum can take a different type and number of parameters. Can also take custom types. Whatever is passed to the enum is passed to the initializer of the type which the parameter(s) is.

The enum parameters can also be named.

Example:

```
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
```

This enum would be difficult to implement with a struct. Would require four different classes, each of which are very different. Hard to allow a function to accept all of them.

Similarly to structs, it's possible to define methods on enums. Use of Self (&self commonly), to reference parameters.


## Option

Enum defined by standard library. Encodes scenario in which a value could be something or nothing.

Rust does not have a null feature. Null is a value that means  means there is no value there. In languages with null, variables can always be in one of two states: null or not-null.

Expressing this in terms of the type system means the compiler can check whether all cases are handled.

The problem is not the concept, i.e. having something that can contain a value or contain nothing, it's the implementation in where it's a state separate to the type system, so it is pervasive and difficult to handle.

The enum `Option<T>` encodes this idea of a value being present or absent.

Included in prelude, automatically included. Also variants. `Some<T>` and `None`.

Having the option type is useful for handling the null scenario. It makes it possible for the compiler for force us to handle the case. This is commonly done by forcing us to convert a type of `Option<T>` to `T`, making it explicit, rather than just assuming that it has a value, which can lead to null errors.

Use of match to handle different variants.

## match

match, a control flow construct called match that allows you to compare a value against a series of patterns and then execute code based on which pattern matches.

Patterns can be many things. Power comes from possibilities of patterns, and compilers ability to check that every case is handled.

match is an expression, the final expression in an arm is returned as the overall match expression.

Another useful feature of match arms is that they can bind to the parts of the values that match the pattern. This is how we can extract values out of enum variants. 

Essentially, we're able to extract the values / paramters of an enum in a match statement so that they can be used in the expression of the arm. Example:

```
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

Example with Option:

```
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

Matching against an enum is a common pattern. Match against enum, bind enum data inside the expression and execute code based on it.

Matches are forced to be exhaustive.

Use of "_" as a catch-all. Matches everything.

Use of "()" unit, to do nothing.

## if let

Choosing between match and if let depends on what you’re doing in your particular situation and whether gaining conciseness is an appropriate trade-off for losing exhaustive checking.

Used to only check a single arm of a match statement without the boilerplate of match.

In other words, you can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.


# Packages, Crates and Modules

## Packages and Crates

Crates, smallest amount of code Rust compiler considered at any time. Even calling rustc to compile a single file is considered a crate.

Two types, binary / library.

Binary crates are compiled into executables, must have a main() function. Library crates do not have a main() function.

A crate commonly refers to a library, and is synonymous with a library.

The crate root is a source file that the Rust compiler starts from and makes up the root module of your crate.


A package is a bundle of one or more crates that provides a set of functionality. A package contains a Cargo.toml file that describes how to build those crates. It's the *Cargo.toml* file which makes a directory a package.

A package can contain as many binary crates as you like, but at most only one library crate. A package must contain at least one crate, whether that’s a library or binary crate.

Example of Cargo, has both a binary crate with the tool, and a library with the functionality for the tool.

Cargo convention:
* *src/main.rs* is the binary crate of the package
* *src/lib.rs* is the library crate of the package

Called crate roots.

Both the binary and library crate has the same name as the package. These files are passed to rustc when cargo build is called.

## Defining Modules to Control Scope and Privacy

Modules are declared within the root crate files of the package, i.e. *main.rs* or *lib.rs*. Modules are private by default, can be made public using `pub mod`, as opposed to just `mod`.

Modules and submodules.

Package example:

https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html#modules-cheat-sheet

Modules lets us organize parts for reuse, also makes it possible to hide or expose functionality.

## Paths for Referring to an Item in the Module Tree

To forms:

* Absolute path
  * External crate, absolute path begins with name
  * Current crate, starts with literal 'crate'
    * 'create' = '/', for fs paths
* Relative path
  * Starts from current module, uses self, super or an identifier in current module

Path identifiers separated by double colons.

In Rust, all items are private to parent modules by default.

Making a module public does not make all of its members public. Members must be explicitly made public.

### Best practices

For modules with both a library and a binary, the binary should be as small as possible, just enough to call the library, so that the functionality is inside the library, which can be shared.

The module tree should be defined in src/lib.rs.

By putting the module tree in src/lib.rs, the binary is also a client of the library, and can refer to it using the module name.

### Relative paths and super

`super` = `..`.

Used to refer to paths relative to the parent.

### Structs and Enums

Structs and enums can also be made public.

The members of a struct must have their access modifier specified individually. A public struct does not have public members.
The variants of a public enum are all public.


## Bringing Paths into Scope with the use Keyword

The keyword `use` can be used to bring paths into scope.

Similar to a symbolic link, symlink, for the filesystem.

Use only applies to the scope in which it is defined.

The idiomatic way = following the conventions.

The idiomatic way to bring functions into scope is to include the parent of the item and then make the call clearly specifying the parent. This clearly specifies where the item from, making it clear that it is not in the local scope.

The idiomatic way to bring structs and enums into scope is to specify the full path including the struct or enum.

No reason that this is the idiomatic way, that's just the common way to write Rust.

The keyword `as` can be used to rename an imported path.

## Re-exporting Names with pub use

Use of `pub use` allows imports of the module to also use the path brought into scope.

Called re-export.

## Using External Packages

Adding external packages as dependencies in the Cargo.toml file makes the package available in the project.

`std` is shipped with Rust, but still needs to be brought into scope with use to use its functions.

### Nesting Packages

Packages can be nested, example of multiple imports from std.

`use std::{cmp::Ordering, io};`

`use std::io::{self, Write};`

In this example `self` refers to `std::io::`

### Glob Operator

Glob operator can be used to bring all public items defined in a path into scope, example:

`use std::collections::*;`

## Separating Modules into Different Files

Modules can be defined in separate files.

The module will still need to be specified in the root, i.e. src/main.rs or src/lib.rs, but can be limited to `mod <mod_name>`.

The compiler will know to look for the definition of the module elsewhere.

Declaring a module using `mod` loads the location of the file into the compiler. All calls refer to that code, it is not an include statement, i.e. it is not actually moved into our code.

Old and new style, the old style is using a .mod file. New style is better.


# Common Collections

Standard library which includes a very useful data structures called collections.

Stores multiple values. Data is stored on the heap - dynamic size.

Vector, string, hashmap.

## Vectors

`Vec<T>`, one or more values in a single data structure. Dynamic. Values must be of the same type.

## Strings

A difficult data structure.

Only one string in core language, string slice, `str`, commonly seen as `&str`. A reference to some UTF-8 encoded string data stored elsewhere. String literals are stored in the programs binary, and are therefor string slices.

The `String` type, which is provided by Rust’s standard library rather than coded into the core language, is a growable, mutable, owned, UTF-8 encoded string type.

Both UTF-8, both used.

Strings are implemented as a wrapper for a vector with some extra functionality.

Some calls take ownership, and others do not.

See link for a nice explanation of ownership with string addition. [Link](https://doc.rust-lang.org/book/ch08-02-strings.html#concatenation-with-the--operator-or-the-format-macro)

Rust does not support indexing into string to get individual characters. Letters which use more than 1 byte to encode a character causes issues.

Rust requires you to be more specific, slicing is possible. A slice of a referenced `String` turns into `&str`.


## Hash Maps







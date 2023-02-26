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

The last of our common collections is the hash map. The type HashMap<K, V> stores a mapping of keys of type K to values of type V.


# Error Handling

Recoverable and Un-recoverable errors.

For recoverable errors use of the `Result<T, E>` and for un-recoverable use of panic! macro, which stops execution.

## Panic! (At The Disco)

To panic, call code which panics, or call the panic macro.

Panic prints a failure message, unwind, clean up stack and quit. Via environment variable you can have Rust display call stack when a panic occurs.

Unwinding and walking backwards takes a lot of effort, an alternative is aborting, which lets a program end without cleaning up. Memory program was using will need to be cleaned up by operating system.

The program binary will be smaller if it is allows to abort rather than unwinding.

Aborting enabled by adding `panic = 'abort'` to appropriate [profile] sections if Cargo.toml file.

```
[profile.release]
panic = 'abort'
```

## Recoverable Errors with Result

Propagating errors is common. Return is an Result enum with type `<T, E>`.

Shortcut to propagating errors is `?`. In case of an `Ok` the program will continue, in case of an `Err`, the `Err` will be returned from the whole function as if `return` had been used.

`?` goes through the From trait defined in the standard library to convert error type received into the error type defined in the return type of the current function. Ensures that any error can be converted to the error defined return type.

To return custom error types, we need to define the `From trait` on the the new error type. `impl From<io::Error> for OurError`.

`?` can also be used with Option types. Will return None early. The function in which `?` is used must also return an Option. It is not possible to use `?` on an Option and a Result in the same function since the containing / parent function cannot then return a correct type.

The `?` can be though of, as to extract the correct value, since calls can be chained onto it.

Main can return `Result<(), E>`.

## To panic! or not to panic!

Returning Result instead of panicking provides options for calling code.

unwrap() and expect() can be used when you have more information than the compiler, i.e. you know that some case can never occur.

Panic when continuing becomes insecure.

Panic when the contract of a function is broken, e.x. when behavior is undefined given a specific set of inputs.

Having the Option type removes the need to handle cases where there is nothing, i.e. `None`. With this, and Rust's type system, you can always be sure that you have `Some` of the value you specified.

Create types, `struct`'s, to represent the valid states, this ensures that you only work on valid states.

https://doc.rust-lang.org/book/ch09-03-to-panic-or-not-to-panic.html

Note the for impl, if `&self` is not passed, then it is not called as a dot (.) function, but rather on the namespace, i.e. Namespace::Funciton(). Example with new() function.


# Generic Types, Traits and Lifetimes

## Generics

Runtime performance is not affected by use of generics. Compiler performs monomorphization to create instances of all used variants of a generics so that the compiler actually uses an implementation with a concrete type.

Possible to have implementations (use of impl) with specific generics. Generics must be specified after impl, e.x. `impl<T, U>`.

## Traits

Traits define shared behavior in an abstract way. 

Trait bounds can be used to specify generic types that has a certain behavior, i.e. to limit generics to types which have certain behaviors.

Behavior consists of the methods we can call on that type. Same behavior, same methods can be called on it.

Similar to interfaces.

Example:

```
pub trait Summary {
    fn summarize(&self) -> String;
}
```

The trait Summary is made public so that crates depending on this crate can make use of this trait too. Note that function references the type which implements the trait using (&self).

Any type that has Summary trait will be forced to implement the summarize function for the Summary trait by the compiler.


Implementing a trait for a struct:

```
pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```

Traits must be brought into scope before they can be used.

When implementing traits, either the trait or the type which it is implemented on, is local to the crate - it's not possible to implement external traits on external types.

This restriction is part of a property called coherence, and more specifically the orphan rule, so named because the parent type is not present. This rule ensures that other people’s code can’t break your code and vice versa. Without the rule, two crates could implement the same trait for the same type, and Rust wouldn’t know which implementation to use.

### Traits as Parameters

Traits can be specified as part of parameters, which can specify a type that implements that trait.

Example:

```
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

Longer form *trait bound*:

```
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

Functionally, the code is the same.

Common to use the longer form for more complex cases, since it is more expressive, there are also some cases which cannot be covered by the short *impl Trait* syntax. Such two parameters of the same type.


Example for more several traits as parameters:

```
pub fn notify(item: &(impl Summary + Display)) {
```

*trait bound* syntax:

```
pub fn notify<T: Summary + Display>(item: &T) {
```

### Clearer Traits Bound with where Clauses

It can be difficult to read function signatures with many trait bounds. Example:

```
fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {
```

This can be simplified with the where clause. Example:

```
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
```

### Returning Types that Implement Traits

Example:

```
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```

### Using Trait Bounds to Conditionally Implement Methods

Using traits we can conditionally implement functions.

Example with generics:

```
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}
```

In the example, we're implementing a new() function for all types for Pair. However, we're also implementing the cmp_display for the struct Pair where the type T implements the traits *Display* and *PartialOrd*.

Implementations of a trait on any type that satisfies the trait bounds are called *blanket implementations* and are extensively used in the Rust standard library.

*blanket implementation* is when a function is defined for any type which satisfies the trait bound.

## Validating References with Lifetimes

Lifetimes ensure that references are valid as long as we need them to be.

Every reference in Rust has a lifetime, a scope for which that reference is valid.

Like types, lifetimes are usually inferred, but similarly, it is sometimes required for us to annotate the relationships using generic lifetime parameters to ensure the actual references used at runtime will definitely be valid.

Prevents dangling references, which leads to null values.

Borrow checker is used to check lifetimes. Checks scopes to determine whether all borrows are valid.

See example: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html

References must be alive, for the reference to be valid. The lifetime of the reference must span the scope of the reference.

Example with if/else and references, lifetime required to specify which reference is returned.

### Lifetime Annotation

Lifetime annotations don’t change how long any of the references live. Rather, they describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes. 

Just as functions can accept any type when the signature specifies a generic type parameter, functions can accept references with any lifetime by specifying a generic lifetime parameter.

Annotated using `&'<name>` for a reference. `'a` is the common name for the first reference.

Examples:

```
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```


### Function Signatures

Like for generic parameters, lifetimes must be specified inside angle brackets before function definition.

Example:

We want the signature to express the following constraint: the returned reference will be valid as long as both the parameters are valid. This is the relationship between lifetimes of the parameters and the return value. We’ll name the lifetime 'a and then add it to each reference.

```
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

In practice explanation:

When we pass concrete references to longest, the concrete lifetime that is substituted for 'a is the part of the scope of x that overlaps with the scope of y. In other words, the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and y. Because we’ve annotated the returned reference with the same lifetime parameter 'a, the returned reference will also be valid for the length of the smaller of the lifetimes of x and y.


Lifetimes only in signature, it's only part of the definition, part of the contract of the function, not the implementation.


Code which will not compile:

```
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}
```

For this to compile, string2 would have to be valid until the println! call since result borrows string2, which is no longer valid. The function specifies that the result is only valid as long as the two arguments are valid.


### Thinking in Terms of Lifetimes

When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters.

Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and return values of functions.


### Lifetimes in Struct definitions

If a struct holds a reference, it could need a lifetime annotation in the definition.

This annotation means an instance of Struct can’t outlive the reference it holds in its part field.


### Lifetime Elision

For some patterns of writing code the compiler is able to infer lifetimes, because the patterns are so similar and commonly used.

The patterns programmed into Rust’s analysis of references are called the lifetime elision rules.

Input and output lifetimes.

See source for rules: https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html

1. All parameters gets its own lifetime
2. If there is only one reference parameter, the output lifetime is assigned to that
3. If there is a &self reference, output lifetime is assigned self lifetime


### Lifetime Annotations in Methods Definitions

Lifetime names for struct fields always need to be declared after the impl keyword and then used after the struct’s name, because those lifetimes are part of the struct’s type.

```
impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
}
```


### Static Lifetime

One special lifetime we need to discuss is 'static, which denotes that the affected reference can live for the entire duration of the program.

String literals, str, have the 'static lifetime, since they live in the binary which is always available.


# Writing Automated Tests

## How to Write Tests

Rust provides attributes to help with testing.

At its simplest, a test in Rust is a function that’s annotated with the test attribute. Attributes are metadata about pieces of Rust code.

To change a function into a test function, add #[test] on the line before fn. When you run your tests with the cargo test command, Rust builds a test runner binary that runs the annotated functions and reports on whether each test function passes or fails.

Macros for assertions. `assert_eq!`.

Use of `cargo run` to run tests.

Ignored tests are tests which are only run on command. We can filter tests, to only run tests specified by name in run command.

Possible to have documentation tests.

* `assert!`
* `assert_eq!`
* `assert_ne!`

Assert parameters are left and right, not expected, actual.

`assert_eq!` and `assert_ne!` use == and != under the hood. Because of this, the values being compared must implement the PartialEq and Debug traits. Custom Structs and Enums must implement these to be able to be tested.

Usually this is very simple, and can be done by deriving the traits. `#[derive(PartialEq, Debug)]`.

Optional arguments to assertion as passed for format! macro. Used to add custom failure messages.

Example:

```
pub fn greeting(name: &str) -> String {
    String::from("Hello!")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
}
```

The additional arguments to the assert! macro will be printed if the assertion fails.


It's possible to check for panics with `should_panic` attribute. If added, the test only passes if it panics. We can add arguments to attribute to check that function panicked for the correct reasons.

Specified as a substring of the error message that is printed, some error messages can be dynamic, i.e. include variables, so a substring is required.


When returning result, a common way to check the return is `assert!(value.is_err())`.


## Controlling How Tests Are Run

`cargo test` compiles code in test mode and runs resulting test binary.

Arguments can be passed to cargo test and to the resulting binary. Separated by --.

`cargo test --help` vs. `cargo test -- --test`.

By default, test runs in parallel, therefore you should be careful about using shared resources in the tests. Number of threads can be specified with command, example:

`cargo test -- --test-thread=1`

By default, if a test passes Rust captures anything printed to stdout and does **not** show it. If the test fails, it will be shown in terminal with failure message.

All output can be shown using output --show-output, example:

`cargo test -- --show-output`


### Running a Subset of Tests by Name

Tests can be run by name by passing their name after `cargo test`, example:

`cargo test <name>`

Where name is the name of the function which is under test.

It's not possible to specify multiple tests by name, however, any tests whose name matches the passed argument.


### Ignoring Some Tests Unless Specifically Requested

The `ignore` attribute can be added ot ignore them unless otherwise specified.

Useful for resource / time intensive tests.

We can run all ignored tests only by using the --ignored option, example:

`cargo test -- --ignored`

To run all tests, including ignored:

`cargo test -- --include-ignored`


## Test Organization

Unit tests vs. integration tests.

Unit tests live with the code in src/, the convention is to create a module named *tests* in each file to contain the test function and to annotate module with *cfg(test)*


### Tests Module and #[cfg(test)]

The *#[cfg(test)]* annotation on tests module tells rust to compile and runt test code on when `cargo test` is run. Saves compile time.

Integration tests do not have this annotation, since they do not live with the code.

When a library is created, Cargo automatically adds this *tests* module. Attribute *cfg* stands for configuration and tells Rust that the following item should only be included given a certain configuration option.


### Testing Private Functions

Debate on whether this should be done. Rust allows private member testing.

Example:

```
pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
```

`internal_adder` is a private function, it is not marked with `pub`.

The `tests` module is just another module inside the parent module, it is a child module. The child module can use items in their ancestor modules. In the *tests* module we can bring all module's parent's items into scope with `use super::*`.


### Integration Tests

Integration tests test the public API of a library.

Integration tests live in *tests/* in the top level of a project directory, in a file named *integration_test.rs*.

Each file in *tests/* is a new crate, so we need to bring the library into scope using *use <crate>*.


Note that the test configuration annotation is not required since the tests do not live with the source code, and is therefore compiled separately.

*tests/* is treated specially, has it's own section when `cargo test` is run.

Each integration has it's own file, and each integration test has it's own section in the test results.

Integration tests can be specified by name to run a single test.


### Submodules in Integration Tests

Files in subdirectories of the tests directory don’t get compiled as separate crates or have sections in the test output.

Place `setup()` and common functions in *tests/common/mod.rs*. Creates a common module which will not appear. An alternative way of specifying a module.

Afterwards the common module can be used, like so:

```
use adder;

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, adder::add_two(2));
}
```

### Integration Tests for Binary Crates

Only libraries with *src/lib.rs* can have integration tests, since they are the only ones with a public API.

Logic should live in *lib.rs*, the binary should just call library functions in *main.rs* and in this case tests can be added.


# Minigrep Project

When running through cargo, i.e. `cargo run ...`, double dash can be specified to pass arguments to the binary that is being run rather than cargo.

`cargo run <cargo_arguments> -- <binary_arguments>`


## Accepting Command Line Arguments

Iterator overview, iterators produce a series of values, and we can call the collect method on an iterator to turn it into a collection, such as a vector, that contains all the elements the iterator produces.

Good practice to bring parent modules in to scope rather than functions themselves, since it is less ambiguous 


## Refactoring To Improve Modularity And Error Handling

How should functionality be split up between a binary and a library?

* Split your program into a main.rs and a lib.rs and move your program’s logic to lib.rs.
* As long as your command line parsing logic is small, it can remain in main.rs.
* When the command line parsing logic starts getting complicated, extract it from main.rs and move it to lib.rs.

Always move logic to lib.rs, parsing can stay if it is minor parsing.


Main should only be responsible for:

* Calling the command line parsing logic with the argument values
* Setting up any other configuration
* Calling a run function in lib.rs
* Handling the error if run returns an error


This separates the concern of running the library and handling the logic of the task. Main handles running the program and lib handles the logic of the program.

Since main cannot be tested, this pattern also ensures that it does not contain logic which cannot be tested. The library functions can be tested.


An example uses `clone()` to make a clone of a copy to avoid ownership issues with a passed reference. Example notes that this is inefficient and should be avoided for more advanced rust, since it takes up memory and computation costs. It's a trade-off between efficiency and simplicity.


## Environment Variables

Can be set while running using cargo:

`<ENV_VAR>=<VALUE> cargo run`


## Writing to Standard Error Instead of Standard Output

stdout, stderr.

With distinction it's possible to split the output.

`println!` can only output to stdout.

Use of > to redirect standard output, will overwrite. >> to append.


Standard library provides macro `eprintln!` to print to standard error stream.


# Closures and Iterators

Functional language features.

In functional languages, it's common to pass functions as arguments and return functions.


## Closures: Anonymous Functions that Capture Their Environment

Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions. You can create the closure in one place and then call the closure elsewhere to evaluate it in a different context. Unlike functions, closures can capture values from the scope in which they’re defined.

Example, `<T>.unwrap_or_else()` which takes a closure without arguments which returns a value T or a value simply a value T.


```
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }
```

Arguments to the closure are passed between vertical bars, ||. The body of the closure calls self.most_stocked().


Closures can capture the environment.


### Closure Type Inference and Annotation

Functions have specific type annotations. This is required since functions form an interface outwards. In contrast, closures are not part of the interface, they are not exposed, instead they are stored in variables.

Typically used in a narrow context where the compiler can infer the types of most variables. Types can still be explicitly added for clarity.


Example:

```
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    };
```

Function and Closure Syntax:

```
fn  add_one_v1   (x: u32) -> u32 { x + 1 }
let add_one_v2 = |x: u32| -> u32 { x + 1 };
let add_one_v3 = |x|             { x + 1 };
let add_one_v4 = |x|               x + 1  ;
```

Note that some of it is optional.


The compiler will infer one concrete value of each parameter and for their return value.

Closures can be assigned to variables, example:

```
let example_closure = |x| x;
let value = example_closure(5); // value = 5
```


### Capturing References or Moving Ownership

Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter: borrowing immutably, borrowing mutably, and taking ownership. The closure will decide which of these to use based on what the body of the function does with the captured values.

Three captures:

* Borrow immutably
* Borrow mutably
* Take ownership

Dependent on what is in the body of the closure.

If a closure borrows mutably, it is not possible to borrow immutably between the closure definition and its last usage, since there exists a mutable borrow.

The keyword *move* can be used before the parameter list to force the closure to take ownership. Useful when passing a closure to a new thread to move the data so it's owned by the new thread. Example:

```
use std::thread;

fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}
```

In this example we want to move ownership to the new thread, however, it's not actually required for the operation and the compiler will therefor only assign immutable borrow for the operation.

Note that what is captured in the closure is not explicitly stated like in C++ where it is placed inside square brackets [].


### Moving Captured Values Of of Closures with the Fn Traits

A closure body can do any of the following:

* Move a captured value out of the closure
* Mutate the captured value
* Neither move nor mutate the value
* Capture nothing from the environment to begin with.

The way a closure captures and handles values from the environment affects which traits the closure implements, and traits are how functions and structs can specify what kinds of closures they can use. Closures will automatically implement one, two, or all three of these Fn traits, in an additive fashion, depending on how the closure’s body handles the values.

Traits specify what closures are supported.

Three traits:

* FnOnce
* FnMut
* Fn

See source.


See example, https://doc.rust-lang.org/book/ch13-01-closures.html#moving-captured-values-out-of-closures-and-the-fn-traits

Implementation of unwrap_or_else including the Fn Trait FnOnce.

In the example, FnOnce is used to specify that we're being passed a closure, and that the closure must implement the FnOnce trait.

In this example we could also use something that implements the Fn Trait, for example invoking a <Class>::new() call.


Fn Traits are a bit difficult, but important.

It seems that the traits are "assigned" to the closure depending on how it's written. We need to write it such that we get the correct Fn Trait so it is compatible with whatever context we need to pass the closure to. THis forces us to write th closure in a way that makes sense for the context.


## Processing a Series of Items with Iterators

The iterator pattern allows you to perform some task on a sequence of items in turn. An iterator is responsible for the logic of iterating over each item and determining when the sequence has finished. When you use iterators, you don’t have to reimplement that logic yourself.

In Rust, iterators are lazy, meaning they have no effect until you call methods that consume the iterator to use it up.

Can be created on a vector by calling `.iter()`.


In a for loop, an iterator is implicitly created and consumed for a vector even if the `.iter()` method is not called.

Languages without iterators usually use indexing in a loop to go though a sequence of items. Iterators reduce boilerplate and potential errors.

It's possible to iterate over more structures than those which can be indexed into.


An iterator is a pattern which makes it easier to loop over a sequence of items. Reduces the boilerplate. Makes it possible to iterate over different items. Makes it possible to add additional logic, such ad consuming adaptors (sum() etc.)


### The `Iterator` Trait and the `next` Method

All iterators implement the `Iterator`trait defined in standard library.

The trait requires you to implement and type Item and the `next` method. Item is an associated type, and it is what is returned when iterating.

`next` method returns one item of the iterator at a time wrapped in `Some` and when iteration is over `None`. I.e. the `Option` enum.


An iterator needs to be mutable, since internal state is used keep track of current where it is in the sequence. An iterator is used up as it is iterated through, `next` consumes it.

Different iterator methods:

* iter - immutable references
* iter_mut - mutable references
* into_iter - takes ownership


### Methods that Consume the Iterator

Iterator trait has a number of default methods defined which uses the `next` method.

Example is `sum()` which can be called on an iterator with a default implementation, it consumes the iterator.

Methods which call next are called *consuming adaptors* since they use up the iterator.

It's probably possible to override the default behavior for custom iteration types.


### Methods that Produce Other Iterators

*Iterator adapters* are methods defined on the Iterator trait that don't consume the iterator. Instead, they produce different iterators by changing some aspect of the original iterator.

*adaptors* seems to methods defined on the Iterator trait, and then there are sub-types, such as *consuming* and *iterators*. 

Example is `map()` which takes a closure to call on each item as the items are iterated through. The `map()` method returns a new iterator that produces the modified items.

The iterators do no do anything by themselves - they are iterators, they need to be consumed.


`collect()` can be called to consume the iterators and return the values into a collection data type, for example a vector with the values.

Iterators can be chained, but remember that they are lazy, they must be called for their functionality to be invoked.


Very common to pass closures to iterator adaptors which capture environment, example `filter()` to filter an iterator.

Example,

`my_vector.into_iter().filter(|x| x > 10).collect()`

Will return a filtered version of my_vector with values greater than 10. Note that `into_iter()` is used to get ownership, meaning that my_vector is "consumed".

filter takes a closure that produces a boolean.


### Comparing Performance: Loops vs. Iterators

There is no overhead using the iterators abstraction. Iterators get compiled down to the same code that a for-loop does.

"Zero-cost abstraction". You can't code it faster by yourself.

"Unrolling" The compiler unrolls a loop to produce the code for each iteration of the loop to make it faster.


# Cargo and Crates.io

## Customize Builds with Release Profiles

In Rust, release profiles are predefined and customizable profiles with different configurations that allow a programmer to have more control over various options for compiling code. Each profile is configured independently of the others.

Cargo has two main profiles

* dev - `cargo build`, optimized for development
* release - `cargo build --release`, optimized for releases

Profiles are added to Cargo.toml file. *[profile.<name>]*

Cargo has default settings used for profiles where a value is not specified. *opt-level* controls optimization, 0-3. 


Example:

```
[profile.dev]
opt-level = 1
```


## Documentation

Three slashes, `///`, is a documentation comment. Used to generate HTML documentation.

Comment is placed before the item that is commented.

Supports Markdown notation for formatting the text.

Several common sections such as *Examples*, *Panics*, etc.


`//!` - Adds documentation to item containing the comments rather than the item following the comment

### Cargo

Cargo can build the documentation with `cargo doc`, which calls the rust tool `rustdoc` distributed with rust. Puts the generated HTML documentation in target/doc.

Build and open docs:

`cargo doc --open`


Cargo can run the code written in documentation as a way to test that the documentation is up to date.

`Doc-tests <crate>`

Will run code written under "Example"


### API Documentation

Internal structure will normally dictate the outwards facing API of a crate, however, this can be changed by re-exporting. Doing this, a public item is made public in another location making it possible to change the way in which it is accessed.

See example, https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html

re-exporting is done using `pub use <path>`. This will re-export an item at the top of the crate. This is done in the crate.

It is usually also here that documentation using `//!` for the crate lives.

The re-exports will be shown in the documentation, making it clear where their source documentation can be found.


### Account Creation

https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#setting-up-a-cratesio-account


### Create Metadata

Added in the [package] section of the crates *Cargo.toml* file.

Crates.io is a first-come-first-serve name basis.

Crate names are defined under [package] section with *name* attribute.

Multiple licenses possible using OR.


### crates.io

Publishes are permanent, cannot be deleted. This ensures dependencies on crates.io can never break.

Newer versions can be published.

`cargo yank` can be used to prevent project using versions as dependencies in the future. This can be undone.

`cargo yank --vers 1.0.1 (--undo)`


## Cargo workspaces

A package can consist of a binary crate and a library crate. In other words, a package is the binary and or the library. However, as a project develops, the library crate can grow too large, and should be split into multiple library crates.

Cargo workspaces can be used for this. A feature that can manage multiple related packages that are develop in tandem.

A workspace is a set of packages that share the same Cargo.lock and output directory.


Several ways to structure a workspace.


See example for structure, not difficult.

https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html


When building, there is only a single target workspace at the top of the workspace.

Crates are not assumed to depend on each other, dependencies in crate level Cargo.toml file must be added.


In top-level we can specify which binary crate to dun using -p argument.

`cargo run -p <crate>`.


The same is true for testing.

`cargo test -p <crate>`


### Depending on External Packages in a Workspace

External dependencies can be added at both workspace level and crate level. Both of those are saved in the Cargo.lock as a single version.

All crates should use the same version of an external library to ensure compatibility.

It is not possible to use a dependency declared in one crate in another, to use it, it must be declared in the using crate.


### Publishing a Workspace

Each carate in a workspace must be published separately.


## Installing Binaries with `cargo install`

Useful to install packages shared on crates.io.

Can only install packages with a binary crate.


## Extending Cargo with Custom Commands

Binaries in $PATH names cargo-<name> can be invoked with `cargo <name>`. Installed binaries also extend cargo.


# Smart Pointers

A pointer is a general concept for a variable that contains an address in memory. This address refers to, or "points at", some other data.

Most common pointer in Rust is a reference, indicated by the & symbol, borrow the value they point to. No special abilities - no overhead.


Smart points are data structures that act like a pointer, but also have additional metadata and capabilities. Not unique to Rust, originated in C++.


Smart pointers in many cases own the data they point to, as opposed to references.

Both `String` and `Vec<T>` are smart pointers. They own some data and allow you to manipulate it. They also have metadata and extra capabilities or guarantees.


Smart pointers are usually implemented using structs. Unlike an ordinary struct, smart pointers implement the `Deref` and `Drop` traits. 

Deref, allows an instance of the struct to behave like a reference. Drop, customized code that runs when an instance of the smart pointer goes out of scope.


## `Box<T>` - Allocating Values on the Heap

The most straightforward smart pointer is a box, whose type is written Box<T>. Boxes allow you to store data on the heap rather than the stack.

The data is called heap data.


Use cases:

* Type whose size cannot be known at compile time, and you want to use a value of that type in a context that requires exact size
* Large amount of data which must transfer ownership but not be copied
* When you want to own a value and you care only that it’s a type that implements a particular trait rather than being of a specific type


### Enabling Recursive Types with Boxes

A value of recursive type can have another value of the same type as part of itself. Recursive types pose an issue because at compile time Rust needs to know how much space a type takes up. However, the nesting of values of recursive types could theoretically continue infinitely, so Rust can’t know how much space the value needs. Because boxes have a known size, we can enable recursive types by inserting a box in the recursive type definition.

Essentially, since a box is just a pointer (stored on the stack) which points to data (stored on the heap) is can be used to create types with an "unknown size".

*cons list* example. Common recursive type, not common in Rust, but good basis for other recursive structures.


Construction function (cons). From Lisp language. Nested pairs of values. Last item is pared with Nil - canonical name for base case of a recursive function.


Recursive enum using a Cons struct:

```
enum List {
    Cons(i32, List),
    Nil,
}

```

With recursion, Rust is unable to calculate how much a List object could take up.

An instance of the List enum could have an infinite size. To instantiate it we need to use *indirection* by pointing to it with a pointer.

```
enum List {
    Cons(i32, Box<List>),
    Nil,
}
```

Makes it possible to compute the maximum stack memory a List enum instance can take up.

The Enum must haver a base case, in this case that is the `Nil` variant. It's just a name of the variant which does not store any data.


Boxes provide *indirection* and *heap allocation*. No other special properties. No performance overhead. Useful for *Cons lists* and similar cases.


`Box<T>` implements the `Deref` trait, which allows values to be treated like references. `Drop` trait ensures that heap data and pointer is cleaned up.


## `Deref` Trait, Treating Smart Pointers Like Regular References

Defining the `Deref` trait allows the use of * to dereference an object.

It provides the compiler to take objects which are references and de-reference them to a & reference by itself. Normally this would be done by the programmer using the & operator. But with the trait defined the compiler is able to do it itself.


Without the Deref trait, the compiler can only dereference "&"-references. The deref method gives the compiler the ability to take a value of any type that implements Deref and call the deref method to get a & reference that it knows how to dereference.


### Implicit Deref Coercion with Functions and Methods

Deref coercion converts a reference to a type that implements the Deref trait into a reference to another type. For example, deref coercion can convert &String to &str because String implements the Deref trait such that it returns &str.

Removes the need to use as many * and & for explicit conversion.


Example:

```
fn hello(name: &str) {
    println!("Hello, {name}!");
}
```

```
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m);
}
```


We're passing a reference to MyBox to the `hello()` function, this is possible since the class MyBox implements a deref which passes a reference to the type <T> which it stores. See full example in source.

Without Deref coercion:

```
fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&(*m)[..]);
}
```


The Deref trait is what makes the coercions possible, since the Rust compiler can check if a method to perform the coercion exists.


### Deref Coercion with Mutability.

Possible to overwrite the * operator on mutable references as well.


## Running Code on Cleanup with the Drop Trait

The Drop trait defines what happens when a value goes of out scope. Can be defined on any type, and can be used to release resources like files or network connections.

Almost always defined when implementing a smart pointer. For example Box<T> deallocates the space on the heap that the box points to.

In other languages the programmer must manually clean up, which can cause memory issues. Implementing the Drop trait makes it possible to ensure that clean-up code is run every time an object goes out of scope.


The Drop trait requires you to implement the `drop()` method, that takes a single mutable reference to self.

Drop trait is included in the prelude, does not need to be brought into scope.


See source for example.


### Dropping a Value Early with std::mem::drop

Values can be dropped earlier using the drop function can be useful in certain cases.

Rust can refer to the drop method as a "destructor", which is a general term for a function that cleans up an instance. Rust has several, and drop is one of them.


It's not possible to call drop function explicitly, i.e. `instance.drop()`.

The instance can instead be dropped using `drop(instance)`. This drops the instance early.


## Rc<T>, the Reference Counted Smart Pointer

Sometimes a single value can have multiple owners, for example a node in a graph is owned by all edges pointing towards that node.

Multiple ownership is explicitly enabled by using the type Rc<T>.

Abbreviation for *reference counting*


The smart pointer keeps tract of the number of reference to a value to determine whether or not the value is still in use, when there are no more references to a value, it can be cleaned up without any references becoming invalid.

Used when we want to allocate some data on the heap for multiple parts of our program to read, and we can't determine at compile time which part will finish using the data last. If we knew that, we could simply make that part of the code the owner, and pass references to the other parts of the code.

Only for single-threaded scenarios.


When using Rc<T>, the `Rc::clone(&instance)` has to be called to create a reference and increment the counter for the number of references.


Can be brought into scope with `use std::rc::Rc;`

`instance.clone()` makes a full clone, `Rc::clone(&instance)` only gets a reference and increments the counter, it's faster to make.


Can only be used to provide read-only access. We need RefCell<T> for mutability.


## RefCell<T> and the interior Mutability Pattern

Interior mutability is a design pattern that allows you to mutate data even when there are immutable references to that data. Normally disallowed due to borrowing rules.

Use of *unsafe* code inside data structure to bend Rust's rules.

The *unsafe* code is wrapped in a safe API and the outer type is still immutable. We still have to comply with some borrowing rules.


### Enforcing Borrowing Rules at Runtime with RefCell<T>

RefCell<T> represents a single ownership over the data that it holds.


With references and Box<T>, the borrowing rules' invariants are enforced at *compile time*. With RefCell<T> these invariants are enforced at *runtime*.

An error with references produces a compile error, an error with RefCell results in a panic and exit at runtime.


Checking at compile-time is better for performance and is safer, however the compile-time checks will prevent usage of some memory-safe operations. Default references and compile-time checking, however it is possible to opt-in to the other behavior.

Some problems cannot be checked at detected at compile-time, such as the Halting Problem.


Only for single-threaded use cases.


Overview:

* Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
* Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
* Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.


### Interior Mutability: A Mutable Borrow to an Immutable Value

See source for example.

Rules for borrowing still hold, but are checked at runtime instead.


### Mutable Data with Multiple Owners

Common to combine Rc<T> holding a RefCell<T> to allow the pattern.


## Reference Cycles Can Leak Memory

It is possible to create reference cycles with RefCell<T> and Rc<T> which can lead to memory leakage. This is since the reference cycle can keep referenced counted by the Rc<T> which are never dropped because of the cyclic dependencies.

Weak references Rc::downgrade, does not result in shared ownership. Can be used to prevent references cycles.

strong_count / weak_count, to track different kinds pof references.

Weak references must call upgrade method to get actual value, returns Option<> to handle case where referenced object i dropped.


Tree & node example in source.


# Fearless Concurrency

Concurrent programming - different parts of a program execute independently

Parallel programming - different parts of a program execute at the same time

Historically, both have been difficult and error prone.


In Rust, many concurrency errors can be checked at compile-time because of the type system and ownership system.

*Fearless concurrency*

For now, concurrent = concurrent and/or parallel.


Other languages usually have tradeoffs between level of efficiency, or only support some functionality for concurrency.


## Using Threads to Run Code Simultaneously

In most current operating systems, a program's code is run in a *process*, and the operating system will mange multiple processes at once. This is why we're killing processes on Windows and Linux.

A program can have independent parts, the features that are run independently are called threads.

* Process
  * Thread
  * (...)
* (...)


Using multiple threads can improve performance, also adds complexity.

Threads run simultaneously, no guarantee about order in which parts of your code on different threads will run.

Can lead to problems:

* Race conditions where threads are accessing data or resources in an inconsistent order
* Deadlocks, where two threads are waiting for each other, preventing both threads from continuing
* Bugs that only happen in certain situations are hard to reproduce and fix reliably

Programming languages implement threads in different ways. Many operating systems provide an api which the language can call for creating new threads. The Rust standard library uses a 1:1 model of thread implementation - on language thread per language thread.


To create a thread:

`thread::spawn(<closure>);`


Sleeping:

`thread::sleep(Duration::from_millis(1));`


When the main thread of a Rust program completes, all spawned threads are shut down, whether they have finished running or not. 

Calls to sleep force a thread to stop its execution for a short duration, allowing a different thread to run.


### Waiting for All Threads to Finish Using join Handles

Saving the return of `thread::spawn()` as a handle we can call `join()` to ensure that the thread closes properly.

`join()` called on a JoinHandler will wait for its thread to finish. Blocking call.

The placement of a join call determines when a thread waits, and if work actually happens concurrently.


### Using move Closures with Threads

Common to use *move* keyword with closures passed to thread::spawn to transfer ownership of variables passed in the closure so that the thread can actually modify them.

Sometimes a call on a thread only needs to borrow a value, however because the lifetime cannot be guaranteed, ownership must be moved.


```
use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}
```


## Using Message Passing to Transfer Data Between Threads

Go language docs:

“Do not communicate by sharing memory; instead, share memory by communicating.”


Rust standard library provides an implementation of *channels*, a general concepts by which data is sent from one thread to another.

One transmitter and one receiver, a channel is closed if either transmitter or receiver half is dropped.


See source example.


mpsc::channel()

multiple producer single consumer.

Returns tuple, (tx, rx), tx - transmitter, rx - receiver.


recv / try_recv:

* recv - blocking
* try_recv - non blocking
  * Result<T, E> Ok with value for message, Err for not received


### Channels and Ownership Transference

Ownership rules are important for concurrency to prevent problems.

When sent, ownership is transferred.


Example of using a receiver as an iterator. Iteration ends when channel is closed:

```
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```


### Creating Multiple Producers by Cloning the Transmitter

tx can cloned to create multiple transmitters.


## Shared-State Concurrency

Mutex - Mutual Exclusion

A mutex allows only one thread to access some data at any given time. A mutex guards data via it's locking system, keeps track of who has access. You ask for a lock on a mutex for access.

Acquire lock before data can be used, release lock after data has been used.


Mutexes is used to allow shared memory. The alternative is sharing data by communicating using channels as in the prev. section.


### Mutex<T>

let m = Mutex::new(5);

To access the data inside of the mutex we need to acquire the lock for the mutex.

After a lock has been acquired we can treat the return value as a mutable reference to the data inside of the mutex.

Type system enforces that we get a lock before accessing value. In this example the type of m would be Mutex<i32>.

Smart pointers are used in the implementation, it's automatically a scoped lock.


Use of Arc<T> over Rc<T> to have multiple ownership of a mutex between threads. Arc<T> as a performance penalty but ensures that the count is correct between threads.


Similarity with RefCell<T> / Rc<T> and Mutex<T> / Arc<T> in terms of providing internal mutability and multiple owners.


## Extensible Concurrency with the Sync and Send Traits

Not a lot of concurrency features in the Rust language, most is implemented in the standard library, other libraries for concurrency as well.

There are some though, std::marker traits *Sync* and *Send*.


### std::marker Send Trait

The Send marker trait indicates that ownership of values of the type implementing Send can be transferred between threads. Almost every Rust type is Send.

Rc<T> is an exception. Arc<T> can be used instead but incurs a concurrency-safety penalty.


### Sync

The Sync marker trait indicates that it is safe for the type implementing Sync to be referenced from multiple threads.

If an immutable reference is safe to send, then a type can have Sync.

Manually implementing either is unsafe.


### Summary

SoTA concurrency is provided in crates. Tokio is a popular crate.

This is because not a lot of how Rust handles concurrency is in the language itself.


# Object-Oriented Programming Features of Rust

## Characteristics of Object Oriented Programming

Rust can do objects, structs and enums.

Rust can do encapsulation, pub option for fields and methods on structs.

Rust cannot do inheritance.

Rust can do polymorphism using traits.


Inheritance has fallen out of favor, has a lot of negative side effects with it. Polymorphism is still commonly used to substitute types, this can still be done with Rust.


## Using Trait Objects That Allow for Values of Different Types

Example of GUI with draw functionality. Inheritance would have a common component class with a Draw method which could be overwritten, Rust would use Traits.

Defining a vector that takes a trait object, objects which define a trait.

A trait object points to both an instance of a type implementing our specified trait and a table used to look up trait methods on that type at runtime.

To create trait object we specify a pointer, ex, `&` reference or `Box<T>`, then use `dyn` keyword, then specify relevant keyword.

<point> dyn <trait>


Doing this, the compiler wil guarantee that only objects defining the trait can be used.


Traits are defined separately form structs and enums in Rust. They cannot define members, only method which must be implemented.


Draw trait:

```
pub trait Draw {
    fn draw(&self);
}

```

Vector of objects implementing Draw:

```
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}
```


Implementing a method on the Screen struct to draw all components:

```
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
```

**Trait objects** and **generic type parameter with trait bounds**.

Trait objects are not the same as generic type parameter with trait bounds. A trait object is dynamic, meaning that multiple different types can be added to the vector, whereas for a generic type parameter with a trait bound, the objects could not be mixed. Generic type parameter with a trait bound is defined with a `where` clause.

In cases where only a single type is used, generic type parameter with trait bound is preferred. They are monomorphized at compile-time, substitution for actual types.

Implementing Draw for a button, not self reference as argument to have access to the button:

```
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}
```

We don't need to actual types, only that it provides a certain interface.

Duck typing, if it walks like a duck and quacks like a duck it must be a duck.


### Dispatching

Trait objects perform dynamic dispatch.

Generic parameter with trait bounds perform static dispatch because of monomorphization at compile-time.

With dynamic, Rust must at runtime perform lookup to check the pointer for which method to call. This is increased overhead, so it is less efficient.

It's a tradeoff. Trait objects has more flexibility and can be freely swapped.


## Implementing an Object-Oriented Design Pattern

Example. Making invalid states impossible to represent using the type system.


# Patterns and Matching

Patterns are a special syntax in Rust for matching against the structure of types.

Patterns in conjunction with `match` expression provides strong flow control of a program.


## All The Places Patterns Can Be Used

Match expressions are defined by `match` keyword, a value to match on, and one or more match arms which consists of a patterns and an expression to run if the value matches the arms pattern. Syntax:

```
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```

`match` expressions need to be exhaustive. The expression will **always** match.

_ matches anything, commonly used as a last arm to ignore all other values than the ones listed.


### Conditional `if let` Expressions

`if let` expressions a shorter way to write the equivalent of a match that only matches one case. Optionally, `if let` can have a corresponding else containing code to run if the pattern in the `if let` doesn’t match.

`if let` can be used to add a match expression to an if statement. Can also be used with else.

Common to use in `if-else` flow management to match on a single pattern and create a variable to use in the body of that flow.

Example:

```
fn main() {
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();

    if let Some(color) = favorite_color {
        println!("Using your favorite color, {color}, as the background");
    } else if is_tuesday {
        println!("Tuesday is green day!");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("Using purple as the background color");
        } else {
            println!("Using orange as the background color");
        }
    } else {
        println!("Using blue as the background color");
    }
}
```

In this example, if the pattern `Some(color)` matches the variable `favorite_color`, the body of the match-arm is executed with access to the `color` variable

Shadowed variables can be introduced by `if let`, they are new variables.

It's not possible to do complex logic inside of the `if let`.


Using `if let` the compiler does not check exhaustively.


### While `let` Conditional Loops

Similar in construction to `if let,` the `while let` conditional loop allows a while loop to run for as long as a pattern continues to match.

Let is used to match a pattern.

```
fn main() {
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }
}
```

While stack.pop() continue to match the pattern Some(top) loop, and provide access to variable as `top`.

`let` provides pattern matching, and access to the match through a variable. Commonly the variable is shadowed so that the name remains the same.

The loop will continue to provide popped off elements until the `.pop()` method return None.


### `for` Loops

In a `for` loop, the value after `for` is a pattern.

Example:

```
fn main() {
    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }
}
```

The `for` loop will continue to match the pattern of breaking elements into a tuple and providing access to the elements.


### `let` Statements

The most variable assignment is actually a pattern.

`let x = 5;`
`let PATTERN = EXPRESSION;`

The variable is just a very simple pattern.

x is a pattern that means “bind what matches here to the variable x.” Because the name x is the whole pattern, this pattern effectively means “bind everything to the variable x, whatever the value is.”

The let statements allows us to bind to variables using patterns.


Example of a simple pattern which does not work:

`let (x, y) = (1, 2, 3);`

The pattern does not match and the compiler will throw an error.


### Function Parameters

Function parameters can also use patterns. Example of a pattern being used to destruct a tuple into individual points:

```
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("Current location: ({}, {})", x, y);
}

fn main() {
    let point = (3, 5);
    print_coordinates(&point);
}
```

Can also be used in function closures etc.


## Refutability: Whether a Pattern Might Fail to Match

Two forms of patterns, refutable and irrefutable.

An example would be x in the statement let x = 5; because x matches anything and therefore cannot fail to match. Patterns that can fail to match for some possible value are refutable. An example would be Some(x) in the expression if let Some(x) = a_value because if the value in the a_value variable is None rather than Some, the Some(x) pattern will not match.

Function parameters, let statements, and for loops can only accept irrefutable patterns, because the program cannot do anything meaningful when values don’t match. The if let and while let expressions accept refutable and irrefutable patterns, but the compiler warns against irrefutable patterns because by definition they’re intended to handle possible failure: the functionality of a conditional is in its ability to perform differently depending on success or failure.

In general, not something that should be worried about, but worth to know about for error handling.

You either need to change the pattern or the construct matching the pattern. For example to pass an irrefutable pattern to a for loop.


`let Some(x) = option_value;` will not compile since `let` requires an irrefutable pattern.


Similarly, the following will fail:

```
fn main() {
    if let x = 5 {
        println!("{}", x);
    };
}
```

Because the `if let` requires a refutable pattern - it does not make sense to have an if else otherwise.

`if let` flow control can have a single irrefutable pattern for matching all `other`.


## Pattern Syntax

https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html

Matching with `match` against variables requires a special syntax since the variable name will be interpreted as a shadows variable and will bind anything matching.

`|` can be used to have multiple patterns inside of a `match` expression.

`..=` to specify ranges to match for a single arm. Convenient. Chars also work.

Can be used to destruct values, for example tuples to individual values.


### Structs

Very common to shadow variable names.

Example of not shadowing names and pulling structure members into values:

```
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}
```

Partial Struct matching:

```
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}
```

The pattern partially describes the structure of the Struct for the match.

`match` expression will stop at the first arm that matches. Use specific arms first.

Notice the use of curly braces for matching when arguments are named.


### Enums

Example of destructing an Enum and it's different variants.

Notice that the variants both have named and unnamed variables and the syntax is different for destructing.

```
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);

    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }
}
```

It is still possible to destruct unnamed variables into named variables, the syntax just uses normal parenthesis.

Struct-like Enums are the ones with named variables.


### Destructing Nested Elements

Destructing can be extended to nested elements.

https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#destructuring-nested-structs-and-enums


### Mix And Matching

Matching can be mixed and matched.

`let ((feet, inches), Point { x, y }) = ((3, 10), Point { x: 3, y: -10 });`


### Ignoring Values in a Pattern

All values or parts of a value can be ignored using `_`.

Values matched with _ cannot be used in the match expression.

Names can be prefixed with _ to prevent unused warnings. Cannot be done with patterns, makes no sense. Either ignore or don't bind them.


All following values can be ignored using `..`.

```
fn main() {
    struct Point {
        x: i32,
        y: i32,
        z: i32,
    }

    let origin = Point { x: 0, y: 0, z: 0 };

    match origin {
        Point { x, .. } => println!("x is {}", x),
    }
}
```

Also works between variables we're interested in.


### Extra Conditionals with Match Guards

A match guard is an additional if condition, specified after the pattern in a match arm, that must also match for that arm to be chosen. Match guards are useful for expressing more complex ideas than a pattern alone allows.

Use of `if` inside pattern.

```
fn main() {
    let num = Some(4);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }
}
```

Compiler cannot check exhaustiveness with match guards so we need an additional arm as a catch-all.

Guard precedence: `(4 | 5 | 6) if y => ...`, i.e. separate to other part of pattern.

### @ Bindings

Used to both test ranges and capture a variable.

Using @ lets us test a value and save it in a variable within one pattern.


# Advanced Features

## Unsafe Rust

Compiler is conservative. A computer system is unsafe, some unsafe operations required to perform certain actions.

Unsafe superpowers:

* Dereference a raw pointer
* Call an unsafe function or method
* Access or modify a mutable static variable
* Implement an unsafe trait
* Access fields of unions

Allows us to move around memory safety. Other guard-rails are still in place.


Common to wrap unsafe code in a safe abstraction to prevent leakage of the `unsafe` keyword. Some standard library functions are unsafe wrapped in safe API.


### Dereferencing a Raw Pointer

Immutable or mutable raw pointers `*const T` and `*mut T`.

The asterisk isn’t the dereference operator; it’s part of the type name. In the context of raw pointers, immutable means that the pointer can’t be directly assigned to after being dereferenced.


Raw pointers:

* Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
* Aren’t guaranteed to point to valid memory
* Are allowed to be null
* Don’t implement any automatic cleanup


```
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
```

Creating a pointer does no harm; it’s only when we try to access the value that it points at that we might end up dealing with an invalid value.

Useful for interfacing with C code nad more.


### Calling an Unsafe Function or Method

Example of split_at_mut(), which returns two slices of a mutable vector. Compiler cant figure out, but it is safe.

See source.


### Using `extern` Functions to Call External Code

Sometimes, your Rust code might need to interact with code written in another language. For this, Rust has the keyword extern that facilitates the creation and use of a Foreign Function Interface (FFI). An FFI is a way for a programming language to define functions and enable a different (foreign) programming language to call those functions.

```
extern "C" {
    fn abs(input: i32) -> i32;
}

fn main() {
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }
}
```

Within the extern "C" block, we list the names and signatures of external functions from another language we want to call. The "C" part defines which application binary interface (ABI) the external function uses: the ABI defines how to call the function at the assembly level. The "C" ABI is the most common and follows the C programming language’s ABI.


`extern` can also be used to create functions which are callable from other languages in Rust.

See source.

Note use of `#[no_mangle]` annotation on the function. Mangling is a process when a compiler changes the name we've given a function to a different name that contains more info for other parts of the compilation process to consume, but is less human readable.

For Rust functions to be callable by other languages, we can't have mangling.

Mangling different from language compilers.


### Accessing or Modifying a Mutable Static Variable

In Rust, global variables = static variables.

Problematic with ownership rules in Rust.

```
static HELLO_WORLD: &str = "Hello, world!";

fn main() {
    println!("name is: {}", HELLO_WORLD);
}
```

Static variables can be mutable, but it's unsafe to change and read them. Static variables have a `'static` lifetime.

Use SCREAMING_SNAKE_CASE by default.


### Implementing an Unsafe Trait

We can use unsafe to implement an unsafe trait. A trait is unsafe when at least one of its methods has some invariant that the compiler can’t verify.


### Accessing Fields of a Union.

The final action that works only with unsafe is accessing fields of a union. A union is similar to a struct, but only one declared field is used in a particular instance at one time. Unions are primarily used to interface with unions in C code.


## Advanced Traits

### Specifying Placeholder Types in Trait Definitions with Associated Types

```
pub trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

Similar to generics for Traits, but locks in a single implementation for a type. Reduces the type requirements when writing implementations.


### Default Generic Type Parameters and Operator Overloading

Note possible to create new operators, but existing can be overloaded. Common to overload + operator.

`std::ops::<trait>` contains the traits.

```
use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {
    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}
```


Default generic type implementation:

```
trait Add<Rhs=Self> {
    type Output;

    fn add(self, rhs: Rhs) -> Self::Output;
}

```

Rhs is a stand-in, when the type is not defined, it defaults to Rhs, which is set to self, i.e. the same type as Add is defined on.

To add Millimeters and Meters, we specify impl Add<Meters> to set the value of the Rhs type parameter instead of using the default of Self.

```
use std::ops::Add;

struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}
```


Usage:

You’ll use default type parameters in two main ways:

    * To extend a type without breaking existing code
    * To allow customization in specific cases most users won’t need


### Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name

Traits and structs can have the same function names. They can be fully specified for disambiguation 

```
trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}
```

```
fn main() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}
```

Defaults to the Struct implementation.

Full syntax also required for struct call with no reference to self.

`<Type as Trait>::function(receiver_if_method, next_arg, ...);`


### Using Supertraits to Require One Trait’s Functionality Within Another Trait

`trait OutlinePrint: fmt::Display`

The Trait OutlinePrint requires the trait fmt::Display, i.e. can only be implemented for types which implement Display.

Useful when a Trait requires / builds upon functionality provided by another Trait.


### Using the Newtype Pattern to Implement External Traits on External Types

Wrapping other types in Structs to make it possible to define Traits on them, since it's only possible to define Traits on locally defined types.

No impact on runtime performance.

Example:

```
use std::fmt;

struct Wrapper(Vec<String>);

impl fmt::Display for Wrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
    let w = Wrapper(vec![String::from("hello"), String::from("world")]);
    println!("w = {}", w);
}
```

`self.0` is used to access wrapped object.


## Advanced Types

### Using the Newtype Pattern for Type Safety and Abstraction

Common to create new types to wrap types for abstraction. Example of wrapping u32 in a Days struct, or `HashMap<i32, String>` in a Persons struct. Can also limit the public interface of the underlying type.


### Creating Type Synonyms with Type Aliases


```
type Kilometers = i32;

let x: i32 = 5;
let y: Kilometers = 5;

println!("x + y = {}", x + y);
```

A synonym, will be used as the same type.

Since the type can be substituted, we don't get compile time checks that we didn't accidentally use a random value with the same type.

Most commonly used to reduce overhead for long argument and return types.

Common to reduce repetition with Result returns. Can be reduced to `Result<T>`.

Alias definition:

`type Result<T> = std::result::Result<T, std::io::Error>;`


### The Never Type that Never Returns

! = empty type. No values.

Used when a function will never return.

Panic! macro and loops have this type in functions. Can be used to annotate that a function will panic or loop forever.


### Dynamically Sized Types and the Sized Trait

For some types it is not possible to know the size at compile-time, str and string slice example.

"..this is the way in which dynamically sized types are used in Rust: they have an extra bit of metadata that stores the size of the dynamic information. The golden rule of dynamically sized types is that we must always put values of dynamically sized types behind a pointer of some kind."

The dynamic size is hidden behind a type that is statically sized. The string-slice `&str` contains this information.

DST = Dynamically Sized Type


## Advanced Function and Closures

### Function Pointers

Functions can be passed to functions.

The type `fn` is a function pointer.

Example:

```
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

fn main() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);
}
```

Closure passing has a different syntax.

Functions can be passed in place of a closure, a function pointer, with type `fn` implements the traits `Fn` `FnMut` `FnOnce`, which are the traits for closures.

Sometimes we only want to accept functions and not closures, for example when interfacing with C code. 


### Returning Closures

To return a closure we need to use `Box::new()` since it's not possible to know at compile time how much it will take to store the closure.

```
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}
```


## Macros

### Overview

The term macro refers to a family of features in Rust: declarative macros with macro_rules! and three kinds of procedural macros:

* Custom #[derive] macros that specify code added with the derive attribute used on structs and enums
* Attribute-like macros that define custom attributes usable on any item
* Function-like macros that look like function calls but operate on the tokens specified as their argument

Fundamentally, macros are a way of writing code that writes other code, which is known as metaprogramming.

`println!` and `vec!` expand to more code than was written. It's a way to reduce the amount of code that has to be written and maintained.

Macros can take a variable number of parameters. Macros are expanded before the compiler interprets the meaning of the code, so a macro can for example implement a trait on a given type.

Macros are harder to write, since it's Rust that writes more Rust. Difficult to maintain and understand.

Macros must be brought into scope or defined before they can be used, cannot be any place like functions.


### Declarative Macros with `macro_rules!`for General Metaprogramming

The most widely used form of macros in Rust is the declarative macro. These are also sometimes referred to as “macros by example,” “macro_rules! macros,” or just plain “macros".

Can be compared to a match expression. The value passed is the literal rust code, the patterns are compared with the structure of the source code.

To define a macro you use hte `macro_rules!` construct.


Example of `vec!` macro for initializing vectors.

`let v: Vec<u32> = vec![1, 2, 3];`


Simplified definition of `vec!`:

```
#[macro_export]
macro_rules! vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}
```

The #[macro_export] annotation indicates that this macro should be made available whenever the crate in which the macro is defined is brought into scope. Without this annotation, the macro can’t be brought into scope.

Name is specified without ! for the definition. The curly braces denote the body of the definition.

See source. https://doc.rust-lang.org/book/ch19-06-macros.html


Arms to match the code, complex macros can have more than one arm.

In the matching it's possible to capture specific elements and generate code based on the elements, for example the number of items passed to `vec!` results in more push() calls.

The expression `$x:expr`, matches any Rust expression and gives the expression the name `$x`.

`$()` declare a variable in the macro system that will contain the Rust code matching the pattern. The dollar sign makes it clear this is a macro variable as opposed to a regular Rust variable. Next comes a set of parentheses that captures values that match the pattern within the parentheses for use in the replacement code.


### Procedural Macros for Generating Code from Attributes

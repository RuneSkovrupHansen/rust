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


### Improving Our I/O Project




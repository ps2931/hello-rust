//////////////////////////////////////////////////
// Managing Large Projects with Packages & Modules
//////////////////////////////////////////////////

// Reference: https://www.sheshbabu.com/posts/rust-module-system/
// https://aloso.github.io/2021/03/28/module-system.html

// As you write large programs, organizing your code will become increasingly important.
// As a project grows, you should organize code by splitting it into multiple modules
// and then multiple files.

// A package can contain multiple binary crates and optionally one library crate.
// As a package grows, you can extract parts into separate crates that become external dependencies.

//////////////////////
// Packages and Crates
//////////////////////
/*
A crate is the smallest amount of code that the Rust compiler considers at a time.
Even if you run rustc rather than cargo and pass a single source code file, the compiler
considers that file to be a crate.

A crate can come in one of two forms: a binary crate or a library crate. Binary crates
are programs you can compile to an executable that you can run, such as a command-line
program or a server. Each must have a function called main that defines what happens when
the executable runs.

Library crates don’t have a main function, and they don’t compile to an executable.
Instead, they define functionality intended to be shared with multiple projects.
For example, the rand crate we used earlier.

The crate root is a source file that the Rust compiler starts from and makes up
the root module of your crate.

A package is a bundle of one or more crates that provides a set of functionality.
A package contains a Cargo.toml file that describes how to build those crates.

Cargo follows a convention that src/main.rs is the crate root of a binary crate
with the same name as the package.

Likewise, Cargo knows that if the package directory contains src/lib.rs, the package
contains a library crate with the same name as the package, and src/lib.rs is its crate
root.

Cargo passes the crate root files to rustc to build the library or binary.

*/

/*
   Defining Modules
    - When compiling a crate, the compiler first looks in the crate root file
    (usually src/lib.rs for a library crate or src/main.rs for a binary crate)
    for code to compile.

    - In the crate root file, you can declare new modules; say, you declare a
    `garden` module with `mod garden`; the compiler will look for the module's code
    in these places:
        - in the file src/garden.rs
        - in the file src/garden/mod.rs

    - In any file other than the crate root, you can declare submodules. For e.g
    you might declare `mod vegetables`; in src/garden.rs. The compiler will look for
    the submodule's code within the directory named for the parent module in these places:
        - in the file src/garden/vegetables.rs
        - in the file src/garden/vegetables/mod.rs

    - You can refer to code in a module from anywhere else in that same crate. For e.g
    a Cabbage type in the garden/vegetables module would be found at crate::garden::vegetables::Cabbage.

    - Code within a module is private from its parent modules by default. Use `pub` keyword
    to make a module public.

    - Within a scope, the `use` keyword creates shortcuts to items to reduce repetition of long paths.

    Example:

    backyard
    ├── Cargo.lock
    ├── Cargo.toml
    └── src
        ├── garden
        │       └── vegetables.rs
        ├── garden.rs
        └── main.rs

    Here we create a binary crate named backyard that illustrates these rules.
    The crate’s directory, also named backyard.

    The crate root file in this case is src/main.rs, and it contains:

    ```
    use crate::garden::vegetables::Asparagus;

    pub mod garden;

    fn main() {
        let plant = Asparagus {};
        println!("I'm growing {:?}!", plant);
    }
    ```

    The `pub mod garden;` line tells the compiler to include the code it finds in
    src/garden.rs, which is:

    pub mod vegetables;

    Here, `pub mod vegetables;` means the code in src/garden/vegetables.rs is included too.
    That code is:

    #[derive(Debug)]
    pub struct Cabbage {}


*/

pub fn run() {
    println!("Read the commens in the file. No code");
}

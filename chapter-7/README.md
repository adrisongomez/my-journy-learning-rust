# Chapter 7: Managing Growing project with Packages, Crates and Modules.

## Overview

Basically on this chapter, we will see how Cargo and rust works to create
and mantain large application with multiple module.

We are mainly understand, how to deal with:
- Packages
- Crates
- Modules and use
- Paths


## 7.1 Packages and Crates (No code)

A `crate` is define as the smallest amount of code that the Rust compiler
considers at a time. Crates can contain modules, and the modules may be
defined in other files that get compiled with the crate.

A crate can come in one of two forms: a *binary crate* or *library crate*

A binary create is just a program that you can compile to an executable 
you can run, such as a command-line program or a server. Each must have
a function called `main` that defines what happens when the executable runs...

So far we just write binary crates.

A library create doesn't have a main function and tehy don't compile into a
executable, Instead it can be used to define functionality. Like the `rand`
package that we install on one of the program is just bring us the functionality
that we need, but itself it's not compile into a executable.

The crate root is a source file that the rust compilers start from and make up
the root module of your create.

Now, a `package` is a bundle of one or more crates that provides a set of 
fuctionality. A package contains a `Cargo.toml` file that descibe how to
build the crate.

A package can contain as many binary crates as we like. But at most only one 
library crate, A package must contain at least one crate, whether that's
library or binary or binary crate.

- `src/lib.rs` contain a library crate.
- `src/bin/` to store executable binary crates.
- `src/main.rs` our root crate, by default on cargoe.


## 7.2 Defining Module to Control Scope and Privacy

### Rules to create a Module

- Start from the crate root. The compile first looks in the crate root file
(usually the `src/lib.rs` or `src/main.rs`).
- Declaring modules. In the crate root file, you can declare new modules. e. g. `mod garden`;
    - Inline, within curly brackets that replace the semicolon following mod garden
    - In the file src/garden.rs
    - In the file src/garden/mod.rs
- Declaring submodules. In any file other than the crate you, you can declare submodules
  for example, you might declare mod vegatables; in src/garden.rs. The compiler will look 
  for the submodule code within the directory named for the parent module in these places: 
    - Inline, directly following mod vegetables, within curly brackets instead of the semicolon
    - In the file `src/garden/vegatables.rs`
    - In the file `src/garden/vegatables/mod.rs`
- Paths to code in modules: Once a module is part of your crate, you can refer to code in 
  that module from anywhere else in that same crate, as long as the privay rules allow 
  using the path code. For example an `Asparagus` type in the garden vegetables module would
  be found at `crate::garden::vegetables::Asparagus`.
- Private vs public: Code within a module is private from its parent modules by default.
  To make a module public, declare it with `pub mod` instead of `mod`. to make items within
  a public module public as well, use `pub` before their delcarations.
` The `use` keyword. Within a scope, the `use` keyword creates shortcuts to items to reduce
  repetition of long paths. In any scope that can refer to `crate::garden::vegetables::Asparagus`,
  you can create a shortcuts with use `crate::garden::vegetables::Asparagus;` and from then on your
  only need to write Asparagus to make use of that type in the scope.

Let's start playing with it.

So `cargo new backyard`

Let's create the following files
- `src/garden.rs`
- `src/garden/vegetables.rs`

Now we write the following into the files.

```rust
// pwd: `./src/garden.rs`
pub mod vegetables;

// pwd `./src/garden/vegatables.rs`
#[derive(Debug)]
struct Asparagus{}

// pwd `./src/main.rs`

use crate::garden::vegetables::Asparagus;

pub mod garden; // This line tell the compiler to include the code in src/garden.rs

fn main() {
    let vegetable = Asparagus {};
    dbg!(vegetable);
}
```

I think these explain themselve... so fun!!

Another example `cargo new restaurant --lib` <- lib?

it just create the project with library create of the root. so `src/lib.rs`.

```rust
// pwd "./src/lib.rs"
mod a {
    mod ab {
        fn abc() {}
        fn abd() {}
    } 
    
    mod ac {
        fn acd() {}
        fn ace() {}
    }
}
```

## 7.3 Paths for referring to an item in the Module Tree

So there is three way to access functionality from an external module:

using relative path: -> crate_name::next_crate::module_name::functionality.
using super to access the parent: -> super::crate_name::module_name::functionality
and absolute: crate::crate_name::module_name::functionality.

simple...

## 7.4 Bringing paths into Scope with the `use` keyword

So we use `use` if we want to bring a module into scope and we can assign alias using
the `as` keyword as typescript too... and we can use {self, module_name} to bring an
specific module to scope and the whole module too with self.

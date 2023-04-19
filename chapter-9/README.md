# Chapter 9: Error Handling

So on Rust, they groups error on two category:
- Recoverable Error: Like File not found and then we show the error to
the user and also can retry the action.
- Unrecoverable Error: Like try to get an element from array but out of
range. So the program immidealty shut down.


Rust doesn't have exceptions?! mmmm intersting...

Result<T, E> -> Recoverable Error
panic! -> To stop execution whe the program encounters an Unrecoverable
error.


## Chapter 9.1: Unrecoverable Error with `panic!`

There are two ways to cause a panic in practice: by taking an action 
that causes our code to panic (such as accessing an array past the end)
or by explicitly calling the `panic!` macro.

Via an environment variable, you can also have Rust display the call 
stack when a panic occurs to make it easier to track down the source 
of the panic.

```rust
fn main() {
    panic!("crash and burn");
}
```

This is how you can call a panic! from your code.

**note** add `export RUST_BACKTRACE=1` to your shell to see call stack
and better track down the panic! call

## Chapter 9.2 Recoverable Errors with Result

Most errors aren't serious enough to require the program to stop entirely.
Sometimes, when a function fails, it's for a reason that you can easily
interpret and respond to. For example, if you try to open a file and that
operation fails because the file doesn't exist, you might want to create 
the file instead of terminating the process.

Recall from "Handling potencial failure with `Result`" in Chapter 3 that
the `Result` enum is defined as having Two variants, Ok and Err, as follows:

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

T and E are just generics for T -> The `return` type  and E -> the `error`
type.



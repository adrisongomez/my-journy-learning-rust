# Chapter 3: Building a guessing game.

## Keynotes

```rust
use std::io; // looks like the to import or call packages/modules on rust
```

How to read from the stdin. the `.expect()` looks weird but insterting.
```rust
let mut word = String::new()
io::stdin() // this return an instance of Stdin which is a type I can use to handle the stdin
    .read_line(&mut word) // This read the stadin and append tha value into mutable reference
    // By default reference are immutable, so that's we need to used &mut to specify that this
    // reference can be mutable. this method return a Result instance which basically like a
    // promise but a better promise. The value that should hold this Result is set of byte
    // return it by the stdin
    .expect("Failed to read from the stdin"); // This method provided an error message is
    // something while resolving the Result.
`

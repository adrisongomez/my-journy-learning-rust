# Chapter 2: Learning the Syntaxt and General Overview

## 2.1: Variables and Mutability

### Variables

This is how you create `your_goddamn` variable

```rust
let your_goddamn = "peace"; // <- The goddamn ';' is back.
```

Variables, in rust, are by default inmutable (unless you explicit mark it).

```rust
let your_goddamn = "peace"; 
//  ^^^^^^^^^^^^ Value is immutable, so not changes allow

your_goddamn = "fuck"; 
// ❌ Goddamn it! ...Compile Error... Immutable variables can't change it's value
```

This is made based on the idea of in somes programs where variables are not
expected to changes, changing a variable could derived to unpredicted behaviour,
so setting variables as immutable by default could help the compiler to find
those bug before the code would be ship to production.

if you want to mutate the value of variable on runtime, variable must be add
the keyword `mut` on runtime.

```rust
let mut your_goddamn = "peace";
//      ^^^^^^^^^^^^ Value is mutable, it can be change

your_goddamn = "fuck";
// ✅ This is allowed! Good boy.
```

### Constant

Constant in Rust are defined using the `const` keyword. Constant value are
normaly using uppercase and undercore like `MS_IN_AN_HOUR`.

```rust
const MS_IN_AN_HOUR: u32 = 60 * 60 * 1000;
//    ^Constant Name ^Type Anotation 😯 ^ Value could be decompose and compile can compute this value or just set the value
//                                       for readability could keep like this because for future supporter could be more easy
//                                       to understand.

// OR we can maked more weird

const MS_IN_AN_SEC: u32 = 1000;
const SEC_IN_AN_MIN: u32 = 60;
const MIN_IN_AN_HOUR: u32 = 60;

const MS_IN_AN_HOUR: u32 = MIN_IN_AN_HOUR * SEC_IN_AN_MIN * MS_IN_AN_SEC;

// More clear and probably too verbose but it work 🤓
```

### Shadowing

Shadowing is a term used when you declare variable again in the scope and different
scope which can allow us to change the value of a variable.

```rust
let x = 25; // Initial value
let x = x + 5; // 30
{
    let x = x + 5; // 35
    // This value is only store within this scope and we exits this scope the variable is going to be
    // revert to previos
}
let x = x + 5; // 35
```

`mut` and Shadowing is not the same thing. Because making the mutation without `let`
on a immutable variable will throw a compile error. Basically, this raise the sense of
the awareness of the changes that you are making to variables.

Shadowing allow us to change data type of variable.

```rust
let x = 25
let x = "25"
```

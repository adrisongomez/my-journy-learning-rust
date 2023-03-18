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

## 2.2 Data Types

Rust is statically language, althought the compile can infer the datatype
sometime we must typed the variable when the result of function in uncertain.

For example parsing functions

```rust
let x: u32 = "42".parse().expect("Not to be a number") // Copy from the docs easy to understand example must be copy as it is.
```

### Scalars

The four primary type in rust are intergers, floating-point numbers, characters and booleans.

### Integers

The integers are defined as:

| signed | unsinged |
| ------ | -------- |
|   i8   |    u8    |
|  i16   |   u16    |
|  i32   |   u32    |
|  i64   |   u64    |
| i128   |  u128    |
| isize  |  usize   |

`isize` and `usize` takes amount of bits from the OS (e. g. 32bit OS -> u32 or 64bit OS -> u64)

One curiouse thing that is mention on the books, how Rust handle integer overflowing which
the case when the value that is intent to store overflow the data type maximun.

If it happen while running on `cargo run` or running the program after compile with `cargo build`
it would throw a panic call exit.

If it happen when running the compile binary by the `cargo build --release`, the program is going
perform a process call [two's complements](https://en.wikipedia.org/wiki/Two%27s_complement) which
made the program to not throw a error but set different value which are not the intended.

*For example*:

When you assigned 256 to a `u8` variable, it would overflow the variable because it passing
the maximun value of the data type 255. Rust would have differnt behaviour deppend of the
enviroment where this happen.

On binarie it would be like this:

```Bash
1111 1111 # which is 255
1 0000 0000 # which is 256 and it happen the overflow for u8 which are this first 8 bits
0000 0000 # so 256 is interpreted as 0.
```

**Note**: Remember this behaviour is not desired on production code because it can derived to
unpredictable behaviour, rust just provided a mecanism to no throw an exception on prod, so we
need to take care on the kind of data_type that we use. By default all number not explicit
defined are set as u32 so u32 it's good and enought.

```rust
let x: u8 = 256  // x=0 and throw a panic call, while running on `cargo run`
// would throw a panic call on development run but `cargo build --release`
// it make two's complements process. making this varialbe eq to 0. x=0
let x:u8 = 257 // x=1
let x:u8 = 258 // x=2
```

### Floating Point

Supported Floating points types are `f32` and `f64` which are based on the standard
[IEEE-754](https://en.wikipedia.org/wiki/IEEE_754). **TODO FOR ME: READ WHEN I HAVE SPAER TIME**

### Numeric Operators

Rust supports the following numerics operators, + - * / %

### Boolean

Rust boolean value are: `true` and `false` which are repsenting a bit value `0 -> false` and `1 -> true`

### Character

Rust charater type is a 8-bit char which make it possible to store additionally normal ASCII value, japanese,
Chinise and Emoji. But not all the unicode value are defined as `char`.

```rust
let x: char = 't' // This how you defined a char with single quote.
let x = 't' // implicit declaration
```

### Compounds

Compund types are define as data type which allow to store or group multiple value in one type. Rust provide two primitve for that: `tuples` and `arrays`

#### Tuples

Tupe = store multiple data type.

Tuples are define using the following syntax.

```rust
let x: (u32, f64, char) = (200, 3.543, 'f');
//     ^ Parentesis and all the desired types separed by commas
```

You can access the data of tuple in different ways.

```rust
let x: (u32, f64, char) = (200, 3.543, 'f');

let (a, b, c) = x; // this is call destructuring
let x = x.0; // this is a direct access. I personally like this approach
```

Also you can define a empty tuple by using just `()` up to this point don't know how is being used. Probably I'll comeback and add some extra comments here to not leave anyone wondering.


#### Array

Array = store a single data type and a fixed amount of them

Array are define using the following syntax:

```rust
let x: [u32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
//     ^ data type ';' length of the array
```

The only way to access a value is the traidicional way `x[index]`.

When index is out of bound throw a panic call and exists the program.

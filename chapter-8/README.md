# Chapter 8: Common Collections

Until now, we know diffrent ways to store multiple type of data in a single
variable. From array and tuple to enums and structs. Pretty powerful stuff,
right? But they lack that you need to define their at head of time. But what
if you want to store data in a dynamic way?. So to do that we need to data
struture which allow of to grow them on runtime.

In this chapter, we are going cover some of most common data structure that 
allow to store data in a collections.

 - Vector
 - String
 - HashMap

Rust has more collections in its standard library like BinaryTree and LinkedLisst
check the Rust documentation. To see all collections.


## Chapter 8.1: Storing Lists of Values with Vectors

`Vector<T>` or better known as a vector. Vectors allow you to store more
than one value in a single data structure that puts all the values next to each
other in memory. Vectors can only store values of the same type. They are useful
when you have a list of items, such as the lines of text in a file or the prices 
of items in a shopping cart.

```rust 

let v: Vect<i32> = Vec::new();

```

This is the way to instatiate a vector in rust. Note that this is in the global
scope so this datastructure comes in the prelude of Rust.

```rust
let v = vec![1,2,3];
```

Also you can use the `vec!` macro.

```rust
v.push(5);
v.push(6);
v.push(7);
v.push(8)
```

To add element to the `vector` we can use the `push` method.

```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {third}");

let third: Option<&i32> = v.get(2);
match third {
    Some(third) => println!("The third element is {third}"),
    None => println!("There is no third element."),
}
```

So here we are reading from the vector `v`, we can use the `[]` and a number 
which index of the vector (similar to array). Remmber borrow checker would 
invalidate the array if we not use the `&` so we have a reading reference
to the element on position 2. And the second methods we use the method
`.get(#)` so this return an Option type so we need to make sure if the data 
exists or not so we can use the `if let` to check is data is None or Some, or 
just use the match expression.

... Future adrison is comming here ... So it looks like that it actually you
can use `if let` as I thought. let me show you.

```rust
// My first idea...

let v = [1, 2, 3, 4, 5];

let value: Option::<i32> = v.get(4);

if let None = value {
    // do something
}

println!("{value}"); 
// This won't work because we are in the same scope even thought we make the for the None
// posiblities... 

// The way that I implement..

let v = [1, 2, 3, 4, 5];

let mut temp_variable = 0; // its important that should be mutable, if not when the next block
// get out scope the shallowing would return temp_variable to its default value.

if let Some(x) = v.get(5) {
    temp_variable = v.clone(); // cloning because I set the mutable to be i32... So I should change it to &i32 to have 
    // a read reference.
}

// do something


// This way work because we are using a anoter temp_variable to hold our data instead of using the same variable. 
// the downside if that we need to define an default value... if you see in onther POV, this isn't a const becuase 
// it make our code less prompt to errors.
```

Now that I show that, we can get back ...

Why we are using the `get` method. Because the vector is dynamic set of data
we don't actually its length on compile time so, rust can't provide error if
we are requesting an object that out of band, so it won't throw an error, 
before compile, Because it could be true or not if you ask for the element in x
position, that element could exists. But what happen if not so we would have
 error on run time so to avoid that we use the `.get`.


 Also another important thing about `vector` if you a `mut vector` this would apply 
 the borrow checker to it and its elements so if you pull an element from the vector
 and then mutate the vector, your value its invalidate unless you copy the data instead
 of just have a reference... tradeoff memory or write code that cover this case. This is a good 
 point to discuss on the future...  


If you want to store multiple data type, you can define an enums with all of your data type and
you have a dynamic vector which accept more than one data type.

```rust
   enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
```

An last, as the same with all the variables... when a vector get out of scope itself and its childrens
got invalidate as well. so you know don't fuck your self by not understanding well borrow checker...

Until now, borrow checker seems a little bit anoying but is not anoying bad instead anoying good. Because
the way to solve an issue seems more secure...


## 8.2 Storing UTF-8 Encoded Text with Strings

Rust has only one string type in the core language, which is the string slice `str` that is usually
seen in its borrowed form `&str`. In chapter 5, we talked about string slices, which are referencest
to some UTC-8 encpded string data stored elsewhere.

To be honest, this topic is pretty insterting but it's not relevant to me, so I'm not going to explain 
anything...

Just see the example in [./understanding-string/](./understanding-string)

## 8.3 HasMap por fin!!!!!!


A HashMap is a data structure that map a key -> which it's normally created by a hashing function <-
with a specefic value. HashMap are useful when you want to look up data not by using an index, as
you can with vector. but by using a key that can be of any type. For Example, in a game, you could
keep track of each team's score in a hash map in which each key is a  team's name and the value
are each tema's score. Given a team name, you can retrive its score.

Just see the example in [./understanding-hash-map/](./understanding-hash-map)



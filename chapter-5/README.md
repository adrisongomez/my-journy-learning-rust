# Chapter 5: Structs

A `struct or *structure*, is a custom data type that lets you package together and name
multiple related values that make up a meaningful group.

## Chapter 5.1: Defining and Instantiating Structs

On rust, the way structure data is using Struct as golang. So to create a simple struct you can use the following 
syntax.

```rust
struct Foo {
    field1: String,
    field2: u32,
}

// OR

struct Bar(u32,String,Boolean);

// OR even

struct Foobar;
```

Super wierd the last way. So structs and typle are too related because it allow us to compound multiple data type
but allow us to create type which can add behaviour using methods and create more convinient way to structure data.

To instanciate a Struct use the following.

```rust
let instance = Foo {
    field1: String::from("field-1"),
    field2: 32,
};

let struct_tuple = Bar(32,String::from("fee"), true);

let weird_struct = Foobar{};

// you can even use destructuring

let instance2 = Foo {
    field2: 65,
    ..instance,
};
```

Keep in mind that the concept of borrowing and owning is still valid for struct so when you pass a struct without using 
a reference to a function rust will complain if you try to used the struct again after the function call. And also field
could get invalided if you are using reference within a struct. Rembember there is a way to make it work, but the book
hasn't tought me yet.

## Chapter 5.2: Trying struct 

We create a program to calculate the area of rectangle. [example-struct](/chapter-5/example-struct/)

## Chapter 5.3: Add methods and associated functions.

So methods are just functions attach to the instance of struct. You can access the property of the object and do actions
To create a methods used the folowing.

```rust 
struct FizzBuzz {}

impl Fizz {
    fn new() -> Self {
        Self{}
    }
    fn buzz(&self) -> String {
        return String::from("Buzz")
    }

    fn fizz(&self) -> String {
        return String::from("Buzz")
    }

    fn fizzbuzz(&self) -> String {
        return String::from("FizzBuzz")
    }

    fn playgame(instance: &FizzBuzz) {
        instance.fizz();
        instance.buzz();
        instance.fizzbuzz();
    }
}
```

Here happen a lot, but I'm too tired to explain. Just said we can create static methods using the `fn method_name () {}`
here is not called static methods because this isn't oop, they are called associated functions you can access using the 
`::`. And to access real methods (the ones who has the `&self` <- which is ref of the object) you can use the `.` not matter 
is the object or a reference of the object. Sorry c this thing `->` is just for returning types...

So basically that said... Too tired to write. See you later my friend

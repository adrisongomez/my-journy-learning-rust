# 4 Understanding Ownership

## 4.1 What is Ownership?

Every programming languague has their owns way to handle memory.

The way of rust handle the memory is really special and it deeply
related to a term used a lot call the `ownership`.

### `stack` and `heap`

In many programming languague there are two term used in memory management which
are fundamental to understand.

The `stack` is a part of the memory where data get store in order as they get,
and they leave the memory following the pattern of FILO (*FIRST IN, 
LAST OUT*) a good analogy of stack is pile of plate, as plate are entering
to the stack the first one are push_back and only the last are the one who
leave first the pile. It's the same with the memory, the `stack` allocated
memory following this principle where the first data input are more deep on
the stack and when stack is being clear it's clear from top to bottom. This
make totally sense, because it make more accessible the most recenlty data
in the stack instead of the most deeper level.

One Important think about of the data store in that stack, it is their size (The 
amount of bytes which this data is occupying on memory) must be known, in 
other words every element has fixed value, (this doesn't mean that if we have variables
x occupying 6 bytes and y occupying 8 bytes are not valid entry to the stack, they are; what 
fixed value try to said in this context is the value of x or y length won't change
on their life-cycel until they get drop from the stack). 

On the other hand, the `heap` is another part of the memory that would be used
to store any other kind of data that their size is unknwon a compile time, the heap
is less organize than the `stack` and slower to read too.

The process to store in the `heap` it's a little more complex than the `stack`. Here the
memory allocator needs to figure out how much memory need to reserve for this data and also
need to figure where to store data (find a section of memory where to store the desired value),
mark that section as being used and return a address to that memory region.

When your code calls a function, the values passed into the function (including, potentially,
pointers to data on the heap) and the function’s local variables get pushed onto the stack.
When the function is over, those values get popped off the stack.

Keeping track of what parts of code are using what data on the heap, minimizing the amount
of duplicate data on the heap, and cleaning up unused data on the heap so you don’t run out
of space are all problems that ownership addresses.

**Note** Here is a [reference](https://doc.rust-lang.org/1.22.0/book/first-edition/the-stack-and-the-heap.html) if you want learn more about memory management

### Rules of Ownership:

- Each value in Rust has an owner.
- There can only be one owner at a time.
- When the owner goes out of scope, the value will be drop.

Try to remmember this things, because it's really important to keep mind when the mind blowing
section comes because this rule are basically the whole fundation of how Rust behave on some
situation.


### Variables Scope

In Rust, variables have a range where they are valid. Let's explore some of them.

`scope block`. a variable is valid within the scope is being used but
after the scope has been closed the value is drop from the stack.

Example:

```rust
let s = "world";
{
    let s = "hello";
    // s valid within this scope so you can use s within this block of scope
    println!("{s}"); // output: hello 
} // drop it's called on this block an any variable created is drop from the heap 
// of the stack

// So this line work only because we declare s too on this scope too, but if we comment the first line,
// we couldn't compile this program, because s has being invalidated because it's out of this println!.
println!("{s}"); // output: world 
```

On this program we are using too entry to the stack:

```rust
let  s = "world";
{
    let s = "hello";
}
```

Both of this variables assigment are valid entry to the stack because their value is fixed
because we are using the string literals `"world"` and `hello` which return the type `&str`
which made them immutable value so the compile knows that this value wont change can be allocated
on the stack instead of the heap. But what happen if our data is uncertain so, the compiler can't
put that this value on the `stack` because it doesn't know the size of the data unless if
a scallar type (e. g. `u32`, it know that is going to be 32 bit so it size never going to change 
the value it can change but the size of bit if the same).

Using the `String` type we can used to start storing value on the heap.

``` rust
let mut s = String::from("hello");// String is struct and ::  is an operator which allow us to access
// function within a namespace. <- I don't know what it is this yet...
s.push_str(", world!"); // here we are mutating our s variable appending a new string to it.
println!("{s}") // output: hello, world
```

So it this case `s` is storing it's value on the heap, becuase is using the `String` type which can
be mutated. This is because how rust interpret string literals and `String`. string literals are fixed 
value so it's now it value but `String` are growable value so it value can't be determited at compile 
time, so we can't know how much memory can take, so basically it will send to the heap...

**Note** It's you want to a more deeply explation of the difference of `str` and `String` take
a look on this [article]("https://www.ameyalokare.com/rust/2017/10/12/rust-str-vs-String.html").

Now, that we have seen a example using both the `stack` and `heap` let's take a looks a little be of 
the onwership in action.

```rust
let x = 50; // allocated 50 in memory and return it's reference or pointer to x...
let y = x; // read the x value and copy its value and allocated in a new section of the memory... 
// In nutshell, x and y have the same value but they are not sharing the same memory.
```

So as we see, `x` and `y` are being store on the stack but they are not sharing the same memory address.
That's totally make sense, because if we remember the rule of ownership (*There can only be one owner at a time*).
The question should be why?. Well the answer it's quite simple, everytime a blocked scope is exited, rust call
`drop` on every piece of memory used on that blocked, so if we have variable sharing the same memory address,
we would calling drop to the same address multiple time, so to avoid this, rust bring two concept first 
its `copy/clone` and the `move`.

The first term `copy` we have seen it in action where we assign `let y = x;` this make a copy of x value into y.

So that's mean we can do the following.

```rust
let x = 50;
let y = x;
println!("{} - {}", x, y) // output: 50 - 50
```
`x` owns 50 and `y` owns another 50 which is a copy of x.

Before, we start to talk of the second term, let's modify the example a little with `String`

```rust
let a = String::from("hello");
let b = a;
```

In this code we are doing the same process of the previous. First we are initializing a variable call `a` with
a value which is a `String` and then, we assign `a` to `b`, so `b` is a copy of `a`. Let's add the last line.

```rust
let a = String::from("hello");
let b = a;
println!("{} - {}", a, b) // compile error -> you can't not longer `x` after `y`
```

If you are using [rust_anaylizer](https://rust-analyzer.github.io) probably it tell you that you have an error
on your code and the reason is because when assign a the value of `a` into `b`. But, Why? it work before, why 
not now? the respond is rely in how data is being store on the `heap`, reference of how to get this data back 
and how the `String` or any data type which are not a scallar get clone.

How `heap` data got store and referenced? So when a new entry of the heap is created a new entry on stack is 
created as well. This stack entry is composed by 3 elements, first a `pointer` to the data store on the heap, 
a `length` which is the size of our data in bytes and the `capacities` which is the total amount of memory 
allow on that section of the heap. This made the process to read from the heap less complex because we can pull 
this reference from the stack get to our data faster because have a pointer which pointing our data on 
the `heap`.

This made totally sense to me...

But what happen when we do `let b = a;`, we are actually creating a new entry on the stack which is right 
according of the rule of ownership, but also we are making a mistake because on that copy we are also taking 
the reference of the memory on the heap, so we unintentionally violating one of the rule because, in nutshell,
`a` and `b` are pointing to the same memory address, they are owning the same data. To bring more clearness to 
understand this we need to bring two more concept which are `deep copy` and `shallow copy`.

- A `deep copy` is when the value of variable is being deeply copy; This mean that not matter how many level are 
on your data structure all the value are being duplicated and we are not using the same memory address. We are
not sharing the same value on the memory. Every one owns its copy.

- A `shallow copy` the data is being access throught reference, so two variables are sharing reference to the 
same memory address. Every one is pointing to the same thing.

So `deep copy` it could be related to an anology of creating a new bird house for every birth bird on your yark,
you are creating a new space for every birth bird where you can point where this bird live. On the other hand,
`shallow copy` is just have a single bird house where you can point where all the birth bird would live, any 
change that house would affected to all the bird which contracts which the `deeply copy` is you modify the 
house B it only going to affect house B.

So back in our example, so what happening here is that we are making a shallow copy of `a` so we are coping 
the heap entry and it's reference too, so when the drop get call at the end of this block, it's going to drop
the smae memory twice, so to avoid this rust alert you of this mistake... So why did rust not alert before? 
Well, because rust allow you to make this `move` call... which the other term that I was missing.

`move` means pass the ownership of a value from a variable to another variable, rust allow this to happen but
with the only condition that the previous variable got never being or reference afterwork. Rust basically
invalidated the previous variable so when the deallocation happen this variable got never call. But what 
happen if I want to use both variable. So we need to perform a `deep copy`. 

Let fix our example:

```rust
let a = String::from("hello");
let b = a.clone();
println!("{} - {}", a, b) // compile error -> you can't not longer `x` after `y`
```

So now our code it's working...

So why because the first example. Why using intergers throw a error?, Well this is basically rust on scallar
type the `deep copy` is the default behaviour on assigment. so we don't need to explicit called. 
Why? don't know yet.

Another intersting about `move` it's that it happen on function call...

```rust
fn do_something(a: String){
  println!("{a}");
}

fn main() {
    let b = String::from("hi");
    do_something(b); // <- move the onwership of b to the function so a got invalitated

    println!("{b}") // throw a compile error
}
```

So as we see in the sample argument of a function move the onwership of varaible to the function which made
our `b` variable invalidated which make us impossible to use `b` after the call of the function.

One solution to this is making our `do_something` return their arguments back.


```rust
fn do_something(a: String, b: u8) -> (String, u8){
  println!("{a}-{b}");
  (a, b)
}

fn main() {
    let c = String::from("hi");
    let d = 5;
    let (c,d) = do_something(c, d); // return the tuple and we destructuring to get the value and use
    // the shallowing to keep using the variable name.
    println!("{c}-{d}") // output: hi-5
}
```

which make it really weird because that's mean I need to return my data back if I going to used later on the
block... weird.

Rust has something fix this need too...


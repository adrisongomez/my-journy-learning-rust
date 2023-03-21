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

### Rules of Ownership

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

## 4.2 Borrowing and Reference

Let's recap our previous error...

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

We can't access this variable `b` because it's invalitated by moving its ownership to the `do_something`
function. At the end of the previous section I told you that rust have something to fix this issue. So this
thing is the `reference` and `borrowing` which are basically, in nutshell, different way to *borrow* a value
ownership of the data from a variable.

First let's first fix this issue...

```rust
fn do_something(a: &String){
  println!("{a}");
}

fn main() {
    let b = String::from("hi");
    do_something(&b); // <- move the onwership of b to the function so a got invalitated

    println!("{b}") // throw a compile error
}
```

So here are appending a `&` to the argument on the function call and on the type definition which means That
now instead of *moving* the onwership we are *borrowing* the ownership by passing a reference to the data.
Similiar heap example, reference are just simple a stack entry which point to another memory addess but the
defirrents betweens them it's that reference only point to entry within the stack so we are not actualling pointing
to the heap and when they are being *drop* their pointer is not getting drop as well, because their a not owning
the value only a reference to it, they are borrowing it.

So now we have to kind of reference:

- `immutable`: which are read-only reference and we can have multiple variable of that.
- `mutable`: which are write reference and you can only one at the time.

You may ask why mutable reference have are limited to one at time, they are like this because it solve an issues
that many programming languague have call data race. Similar to race condition, this error happen when a mutations
are happen to the same time drive to unpredictable behaviour.

For example:

You have a string store on memory `Hello, world` and you have multiple writer sending data simultaneosly. This
really make a lot of unpredictable behaviour every writer are writing at the same time and we cannot sure That
all changes are being apply after the mutation were apply. So to avoid this Rust don't allow this by just macking
sure that only one writer is allow.

To create a immutable referecen just enter `&s` and to create a mutable reference `&mut s`.

### Dangling references

What happen if intead of return a value (move the ownership from a value to another variable), I would like to
just pass the reference (borrow the value) in a function. As far as, we know, we can't deduce that this could
drive to an error which it's if we are returning just the reference to value initialize on the function that
value would *drop* when the function get out of scope. That's call the dangling reference.

Rust know that shit could happen and it would throw a compile error to let you know you need to fix this.

For now, we can return the value but rust has another tool that can help us to fix this issues... But

it's not explain on this section damn!

Well it is what it is...

### The rules of references

- At any given time, you can have either one mutable reference or any number of immutable references.
- References must always be valid.

## 4.3 The Slyce Type

So let define this case, we have a program which take a string as argument and we want to return the first
word (set of char separated with `space`) and if the string doesn't have mutliple word should return the
entire string. So how we can create this on rust? because we haven't quite don't know how to split a string.

A soluction could be returning the ending position an din dong... it solve because we can use this to said
when first word end...

```rust
fn first_word_1(s: &String) -> usize {
    let bytes = s.as_bytes(); // this return an array of bytes.
    // convert every element to it bytes eq.

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // b' ' is a byte literal syntaxt which return the binary representation
            // of the chan
            return i;
        }
    }
    
    s.len()
}

fn _implementation_1() {
    let mut s = String::from("hello, world");
    let first_word_ending = first_word_1(&s); // so our function is returning only a the index of were the first
    // word end.
    s.clear(); // this reset the string so now s is = ""
    
    // so as we loose the data on S we don't actually have the first_word so we are making our
    // depend on s. S should not change to our code make sense... it works but it's the right
    // behaviour
    println!("{}", first_word_ending)

}
```

Here is our code, as you see, we get the data by calling bunch o methods and using new things like the
bynary string literal (!SWEET) and also we notice, althought we solve our issue, we encounter another issue
which rust can't quite catch, because it's more a us bug than a rust issue...

When we find the last_position of our word, we are tied to the string state... What do I mean with that?
Well basically, in the case of string got modify we are screwed, we can't 100% that the last digit of the
first word is in that postion so that means that our variables are strictly dependen. And this is not bad,
maybe is what we want... but the issue rust can't tell of if we are wrong or not... maybe we will be having
some issues later, because we need to design a way to invalidated our previous index value to get synced.

How can we solve this? Rust has another alternative that would be nice to explore.

So to solve this we are going to use a `Slice` or in this a `string slice`.

A Slice is just reference to set of data on memory or partially set of data on memory.

What does this mean?

Well when a slice is being created in its stack entry it's going to have a pointer to some place
of the memory (similar to the reference when we were talking about borrowing) but also store a length value
that it's going to represent the amount of memory would read after that pointer. which in this case:
the pointer is the start of our string and the length is how many position would take to get the
first word...

Let's ilustrate this...

```AL
Here a ilustration of the memory

                  v Here is when our first word end. 
--------------------------------------------
| H | E | L | L | O |  | W | O | R | L | D |   <- This our string on the heap
--------------------------------------------
  0   1   2   3   4  5   6   7   8   9  10     <- This represent the Address or pointer in memory
--------------------------------------------
  ^
  Here it is the first letter
  


So our slice reference to our this data would be something like this

{
    pointer: 0, <- Which is the position or address of our first letter
    length: 5 <- Which the step that we need to walk to get to the end of 
                 the first word or end of the slice.
}
```

I think I already explain myself well. so let's cut to point.

By doing this we still are actually reling on the data that is store on variable `s` but
because we are using `string slice`. Rust can correct us when our `first_word` variable got
invalidated by mutating the `s` variable. Let's see the code.

``` rust
fn first_word_2(s: &String) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i] // This create a new entry of the stack which contians a pointer on the
            // first position of string or starting point and a len which contains how many bytes
            // are conforming this string so if we have on the head this
            // h e l l o space w o r l d.
            // ^       *  n=5
            // This a our starting point and our len is 5.
        }
    }
    &s[..]
}

fn _implementation_3() {
    let mut s = String::from("hello, world");
    let first_word = first_word_2(&s); // So here we have our string slice but still tie to 's' variable because
    // s is link to the actual heap entry, with `first_word` we only have a reference to the first
    // letter and len which mean, we are going to read until 5 postion after our start.
    // So if we modify the s variable which mean change the data in the heap we are not sure that
    // our variable first_word have the data that we want...
    // Rust knows about this it let us know with a compile time error or diagnostic is you are
    // using rust_analyzer.
    // s.clear(); // uncommented this to see the actual error.
    println!("{}", first_word);
    s.clear();

}
```

And this behaviour make totally sense because our `first_word` is not actualling the data on the heap
remmber the rule of ownership *There can only be one owner at a time* and we are not moving the ownership
we are just creating a new kind of reference which it's going to rely on s. you can spot it on the return
type of the function `&str` we are just getting a reference or a string slice...

But what can we do if I want to have first_word not bind or how I called strictly related? Well, just created
a new `String`, this would created a new entry on the heap and also on the stack (keep in mind that...) and
now `s` can be modify without invalidated `first_word` but at the cost of more space on memory being occupy.
Here is my solution.

```rust
fn _implementation_2() {
    let mut s = String::from("hello, world");
    let first_word = String::from(first_word_2(&s));
    // Here we are solving the error explain above by creating a new entry on the heap for our
    // first_word, so the string_slice that we are getting is being take as String::from a now we
    // have a indepent entry on the heap where our first_word could look for the data without relay
    // on s to be sync. Now first_word and s are not stritly related.
    s.clear();

    println!("{}", first_word)
}

```

*Note*: My initial idea was to create a copy by calling the `&str.clone()` but what got clone it, was the
reference not the actual data.

You can create more king of slice... I'll just leave an example of `&[i32]` which a type of slice related
to an array of `i32`.

```rust
let a = [1,2,3,4,5];
let slice = &a[1..3];
// It's the same idea of the string slice. Instead of point to the heap, we are pointing to the
// stack and we have a length propery with n positions take by our array slice.

assert_eq!(slice, &[2, 3]); // assert is two value are the same

```

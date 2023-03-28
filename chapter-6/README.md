# Chapter 6: Enums and Pattern matching

## 6.1 Defining an enums.

Structs are being a really awesome tool to group data in a common data type that we can use all over place
in our code base. Now, we can create different data type based using the structs... But what happen when you 
need to group all this data type and the most important create different side effects based on their data
type. Rust a powerful tool that can help us on that. That is using `enums` or enumerate. 

`Enums` are just data type which allow us to group differents type of data in single data type that we can
create side effects based on the `variant` value on it.

To understand better this, the book show us a example of `IpAddress` struct which store all the information
about an ip addresss. Currently IP address support two version which are v4 and v6. If we take the pure Struct
way we can make our code work creating an `IpAddressV4` and `IpAddressV6`. But this only works when we know
ahead of time which ip version we would have, but what happen if we don't have... You might thinking maybe,
just create and `IpAddress` now v4 and v6 are represented on the same data type but what if you want to create
different behavior depend of the v4 (Insert reference of our previous project with the Area of `Rectangle `, what if 
we want to create an `compute_figure_area` how can we create different side of effect (compute the area) in different 
figure if every formula is different) that's what enums and pattern matching are here for...

So let's first see the `enums` syntaxt.

```rust
enum IpKind {
    V4,
    v6,
}

// OR

enum IpKindWithData{
    V4(u8, u8, u8, u8), // You can store value on the variant too..
    V6(String),
}

// =================FIGURES==================

enum Shapes {
    RECTANGLE (Rectangle), // You can also reference others data type.
    TRIANGLE (Triangle),
    Circle (Circle),
    SQUARE (u8),
}

let value = IpKind::V4;

let value_with_data = IpKindWithData::V6(String::from("::1"));
```

As we see here, we can create a `enum` just by `enum Name` and define every variant UPPERCASE and we can even
assign value to variants extra points for it. And to access a variant we just need to use the enum names and
`::` operator to access the value under the namespace. So as enum are also trait as DataType you can used within
your structs too.

```rust
struct IpAddress {
    kind: IpKindWithData,
}
```

Pretty cool right... But what do you think if I tell you that enums can also attach methods as well.


```rust
impl IpKindWithData {
    fn get_data(&self) {
        &self
    }
}
```

Awesome...

Rust also a lot of built-in struct that we can use from the standard library. One of them is the `Option`...

Why it's so special. it because if basically the only data type that we can define is variable could be holding 
data or not... I know WTF... Rust doesn't have a null definition so you can't explicit said that a variable 
value is null. But we know that the true null is something with must have on our code base, because we can't be
100% sure if we got data, specially when we are reading for external sources. Why take this decision...

Basically it avoid us to make error on our code because haven't check if a value exists or not... So the focus 
of rust is type safe... so they decide to remove it... And they create this sort of helper enum call Option,

So basically Option have two variants `None` and `Some<T>` where T is a generic so it cant be anything...
So None mean that a value is null and Some that a value could be None, after we make the check our data change
from Some<T> to T, because we are certain that we have the data on our variable.

``` rust
let value = Some<Figure>(value_figure);
let none_value: Option<Figure> = None;

value.do_something(); // would fail because we haven't check if our value hold data
```


## The `match` expression.


So let's do this quick.

HERE how you define a match
```rust

enum Enumerador {
    VARIANTE_1,
    VARIANTE_2,
}


// ...

let value = Enumerador::VARIANTE_1;

// ...

match value {
    Enumerador::VARIANTE_1 => {
        // do something
    }

    Enumerador::VARIANT_2 => {
        // do something else,
    }

    _ => {
        // default behaviour

        // I think this shouldn't define only when you have are using enums because when you add
        // a new variant on your code the compiler would tell you that you need to implement the side 
        // effect on every match function where your enums is being used... sound more work, but it make
        // your code more safe and prevent bugs to happens.

        // This only be used if you are using numbers or string on your match expression.
    }
}

```

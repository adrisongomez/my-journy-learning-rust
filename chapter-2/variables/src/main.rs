const MS_IN_AN_SEC: u32 = 1000;
const SEC_IN_AN_MIN: u32 = 60;
const MIN_IN_AN_HOUR: u32 = 60;

const MS_IN_AN_HOUR: u32 = MIN_IN_AN_HOUR * SEC_IN_AN_MIN * MS_IN_AN_SEC;

fn main() {
    const THIS_IS_A_CONSTANT: &str = "THIS IS A CONSTANT, LOVE ME PLEASE <3!";
    let mut x = 25;
    println!("x: {x}");
    x = 5;
    println!("x: {x}");
    println!("This is how ms I can hold an erection {MS_IN_AN_HOUR}.");
    println!("Secret Message: {THIS_IS_A_CONSTANT}");

    // Shadowing
    let x = 25;
    println!("{x}");
    //     ^ the '!' is added because println is a macro don't know how, why and what is a macro, but it works..

    let x = x + 5;
    println!("{x}");

    {
        let x = x + 5;
        println!("{x}");
    }

    println!("{x}");

    let x = "here it change value";

    println!("{x}");

    let mut y = 25;
    // y = "here it change value";
    // Commented because compile would throw an error because mut can change data type.
    // y = 5;
    // Rust compiler is smart enought to suggest me to remove the mut from the variables if the
    // value change before I get implemented the variable
    println!("{y}");
    y = 5;
    println!("{y}");

}

// Really fun learning this...

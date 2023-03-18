fn main() {
    let number = 3;
    if number <= 3 {
        println!("condition was true"); // this is call as arm
    } else {
        println!("condition was false")
    }

    // if number { // this should be always a boolean
    //     println!("must throw an error");
    // }

    if number == 1 {
        println!("it's a trap");
    } else if number == 2 {
        println!("run bitch, ruuuun!!!");
    } else if number == 3 {
        println!("you look that... that was brave!!!");
    } else if number == 4 {
        println!("oh shit! here we going again");
    } else if number == 5 {
        println!("Never give you up, never gonna let you down");
    } else {
        println!("hello world, nothing happen here.");
    }

    let x = if number < 3 { 'c' } else { 'a' };
    // let x = if number <3 { 'c' } else { 4 };
    // this is not allow because both arm must be the same data type
    println!("{x}");

    let mut i = 0;
    loop {
        // this is a infinite loop
        println!("again!");
        i += 1;
        if i == 5 {
            break; // this how you can stop it
        } else {
            continue; // this not neccesary but I just put it here to show that it can be used too.
        }
    }

    i = 0;
    let b = loop {
        i += 1;

        if i == 6 {
            break i * 2; // you can't use return...
        }
    };

    let mut count = 0;
    'outer: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'outer;
            }
            remaining -= 1;
        }
        count += 1
    }

    println!("{b}");


    let mut number = 3;
    while number != 0 {
        println!("{number}");
        number -= 1;
    }

    // let number = 3;
    // while number != 0 {
    //     let number = number - 1;
    //     println!("{number}");
    // }
    // This is valid rust and can give send you to a infinite loop because you are 
    // redeclaring a new  variable in a new scope and the first variable never get reassigned.

    let a = [2, 4, 8, 16, 32];
    let mut index = 0;

    while index < a.len()  {
        println!("the value is {}", a[index]);
        index += 1;
    }

    for x in a {
        println!("the value is {x}");
    }

    for number in (1..4).rev() {
        println!("{number}!")
    }
     
}

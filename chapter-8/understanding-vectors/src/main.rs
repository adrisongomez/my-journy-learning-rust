

fn main() {
    let v = vec![1,2,3,4,5,6];

    let value = v.get(5);
    match value {
        None => println!("The value is none"),
        Some(v) => println!("The value is {v}")
    }


    let mut v = vec![1,2,3,4,5,6];
    // let first = &v[0]; 
    // borrow the value from the vector but get invalidated when vector get mutate
    let _first = &v[0].clone();
    // This work because we are copying the value so we don't have actually reference
    // to the previous element. we have a value. But if the vector changes this 
    // variables wont get invalidated
    v.push(6);
    let first = &v[0]; 
    // now is borrowing properly, so first is not invalidated.

    println!("The first element is: {first}");

    // a weird experiment
    let mut temp = &0;
    if let Some(x) = v.get(0) {
        temp = x;
    }
    // v.push(5); // <- So the compile can detect and modifying the array invalidated the reference
    // when v is mutated.
    println!("{temp}");

    for i in &v {
        println!("{i}");
    }

    for i in &mut v {
        *i += 50;
    }

    for i in &v {
        println!("{i}");
    }



}

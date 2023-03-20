
fn print_message(container: &String) {
    println!("This is your message: {}", container)
}

fn add_message(container: &mut String) {
    container.push_str("Hello world!");
}

fn main() {
    let mut s = String::new();

    add_message(&mut s); // we are passing mutable reference to the function so the ownership is not
    // move
    print_message(&s); // we are passsign a immutable reference to the function but the ownership is
    // not move

    let r1 = &s;
    let r2 = &s;

    println!("{r1}-{r2}");

    let r3 = &mut s;
    // let r4 = &mut s; // this is not allow

    println!("{r3}");
    let r4 = &mut s;
    println!("{r4}");
}

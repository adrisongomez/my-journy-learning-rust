use std::fmt::Display;

fn print_message<T: Display>(message: &T) {
    println!("This is your message {message}")
}

fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    0
}


fn main() {
    print_message(&"Hello world!");
    print_message(&16);
    let s = 125;
    print_message(&s);
}

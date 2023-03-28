fn main() {
    let a = Some::<u8>(10);
    let b = 5;

    if let Some(x) = a {
        let y = x + b;
        println!("{y}");
    }
}

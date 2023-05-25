
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let a = String::from("Hello, world!"); 
    let r;

    {
        let b = String::from("Hello, world!");
        r = longest(a.as_str(), &b.as_str());
        println!("The longest string is {}", r);
    }

}

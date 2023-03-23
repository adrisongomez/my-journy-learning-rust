
// https://stackoverflow.com/questions/46388386/what-exactly-does-derivedebug-mean-in-rust#:~:text=%23%5B...%5D%20is%20an,Would%20the%20real%20%7B%3A%3F%7D
// Basically the compiler implement some methods and traits for you on the struct.
#[derive(Debug)] // Outer atributes 
struct Rectangle {
    width:      f64,
    height:     f64,
}

fn compute_area(rectangle: &Rectangle) -> f64 {
    rectangle.width * rectangle.height
}

fn main() {
    
    let rectangle = Rectangle{
        width: 24.16,
        height: 40.32
    };

    println!("The are of the rectangle is: {}", compute_area(&rectangle));

    println!("{:?}", rectangle)
}

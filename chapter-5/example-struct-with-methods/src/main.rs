
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    fn new (width: f64, height: f64) -> Self { 
        // This is called as assoicated function
        Self{
            width,
            height
        }
    }
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

impl Rectangle {
    fn can_hold(&self, other_rectangel: &Self) -> bool {
        //                              ^ remember the ownership laws \___(=___=)___/
        other_rectangel.area() < self.area()
    }
}

fn main() {
    let rectangle = Rectangle::new(128.22, 0xff32 as f64);
    println!("Calculate Are in Rectangle: {} cm", rectangle.area());
    dbg!("{:?}",&rectangle);

    let rectangle_2 = Rectangle::new(12.0, 23.0);

    if rectangle.can_hold(&rectangle_2) {
        println!("Rectangle two can fit on rectangle 1: {:?}", rectangle_2)
    }
}

use std::ops::Add;

fn main() {
    let c1 = Counter { value: 0 };
    let c2 = Counter { value: 5 };
    println!("Hello, world!, {}", (c1 + c2).sum::<u32>());
    run()
}

struct Counter {
    value: u32,
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.value += 1;
        if self.value >= 10 {
            return None;
        }
        Some(self.value)
    }
}

impl Add for Counter {
    type Output = Counter;

    fn add(self, rhs: Self) -> Self::Output {
        Counter {
            value: self.value + rhs.value,
        }
    }
}

trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn run() {
    let person = Human;
    Pilot::fly(&person);
    Wizard::fly(&person);
    person.fly();
}


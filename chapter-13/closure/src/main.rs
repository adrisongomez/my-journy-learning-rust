use std::thread;

fn main() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_preference = Some(ShirtColor::Red);

    let giveaway1 = store.giveaway(user_preference);
    println!(
        "The user with user with preference {:?} gets {:?}",
        user_preference, giveaway1
    );

    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // Moving ownership
    thread::spawn(move || println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor {
        user_preference.unwrap_or_else(|| self.most_stocked())
    }

    fn most_stocked(&self) -> ShirtColor {
        let mut num_red = 0;
        let mut num_blue = 0;
        for x in self.shirts[..].into_iter() {
            match x {
                ShirtColor::Blue => num_blue += 1,
                ShirtColor::Red => num_red += 1,
            }
        }
        if num_blue < num_red {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

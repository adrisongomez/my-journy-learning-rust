use crate::garden::vegetables::Asparagus;

pub mod garden; // This line tell the compiler to include the code in src/garden.rs

fn main() {
    let vegetable = Asparagus {};
    dbg!(vegetable);
}

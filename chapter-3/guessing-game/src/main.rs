use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number:u32 = rand::thread_rng().gen_range(1..=100);
    let mut attempts = 0;

    while attempts < 10 {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin() // this return an instance of Stdin which is a type I can use to handle the stdin
            .read_line(&mut guess)
            // This read the stadin and append tha value into mutable reference
            // By default reference are immutable, so that's we need to used &mut to specify that this
            // reference can be mutable. this method return a Result instance which basically like a
            // promise but a better promise. The value that should hold this Result is set of byte
            // return it by the stdin
            .expect("Failed to read from the stdin");
            // This method provided an error message is something while resolving the Result.
        
        let guess: u32 = guess
            .trim()
            .parse()
            .expect("Please type a number!");

        println!("You guessed: {guess}");

        match secret_number.cmp(&guess) {
            Ordering::Less => println!("Too big!"),
            Ordering::Greater => println!("Too small!"),
            Ordering::Equal => {
                println!("You win");
                break;
            }

        }
        attempts += 1;
    }
}

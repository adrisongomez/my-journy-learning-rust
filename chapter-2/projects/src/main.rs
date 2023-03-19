mod temperature_converter;

use std::io::{self, stdin};

// enum MenuOption {
//     TemperatureConverter,
// }

fn main() -> Result<(), io::Error> {
    println!("Hello, world!");
    let mut opt: String = String::new();
    
    while opt != "0" {
        print_main_menu();
        let result = stdin().read_line(&mut opt);
        println!("{}", result.is_ok())
    }
    Ok(())
}

fn print_main_menu() {
    println!("Main Menu");
}

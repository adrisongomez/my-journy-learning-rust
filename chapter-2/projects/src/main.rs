pub mod base;
pub mod fibonnacci_secuense;
mod temperature_converter;
pub mod twelve_days;
mod utils;

use std::io::stdin;

use base::base::App;
use fibonnacci_secuense::fibonnacci_secuense::FibonnacciGenerator;
use temperature_converter::temperature_converter::TemperatureConverter;
use twelve_days::twelve_days_son::TwelveDaysGenerator;

fn main() {
    let mut opt: u8 = 5;
    let temperature_app = TemperatureConverter {
        menu_name: "Temperature Converter.".to_string(),
        menu_number: 1,
    };
    let fibonnacci_secuense = FibonnacciGenerator {
        option: 2,
        name: "Fibonnaci Generator".to_string(),
    };

    let twelve_days = TwelveDaysGenerator {
        name: "Twelve Christma generator".to_string(),
        option: 3,
    };

    while opt != 0 {
        let mut current_opti = String::from("");
        print_main_menu();
        stdin()
            .read_line(&mut current_opti)
            .expect("User didn't input option");
        opt = current_opti
            .trim() // Remove whitespace and breakline... on stdin when the user submit the
            // input a newline is added at the end so this would prevent us to faill
            // in the parsing to number
            .parse::<u8>()
            .expect("User Input must be an valid option (0-255)");
        println!("Selected the Option: {}", opt);
        match opt {
            1 => temperature_app.run(),
            2 => fibonnacci_secuense.run(),
            3 => twelve_days.run(),
            _ => {
                println!("Option not valid or not implemented, please use one from the list");
            }
        }
    }
}

fn print_main_menu() {
    println!("Main Menu");
    println!("[1] Convert Temparture.");
    println!("[2] Generate Fibonacci Secuense.");
    println!("[3] Sing \"The Twelve Days of Christmas\"");
    println!("[0] Exit");
    println!("Enter the desired app:");
}

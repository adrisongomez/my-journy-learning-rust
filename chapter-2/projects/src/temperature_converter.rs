pub mod temperature_converter {

    use std::io::stdin;

    use crate::base::base::App;
    use crate::utils::utils;

    pub struct TemperatureConverter {
        pub menu_name: String,
        pub menu_number: u8,
    }

    impl App for TemperatureConverter {
        fn get_title(&self) -> String {
            self.menu_name.clone()
        }

        fn get_option_number(&self) -> String {
            format!("{}", self.menu_number)
        }

        fn run(&self) -> () {
            println!("Welcome to the temprature converter app");
            loop {
                let mut opt = String::new();
                print_menu_options();
                stdin().read_line(&mut opt).expect("User input not valid");
                let opt: u8 = opt
                    .trim()
                    .parse()
                    .expect("User input not valid, must be number");

                match opt {
                    1 => execute_convertion(ctof),
                    2 => execute_convertion(ftoc),
                    3 => execute_convertion(ktoc),
                    4 => execute_convertion(ctok),
                    0 => println!("Bye, exiting the temperature converter"),
                    _ => println!("Option not valid, please enter valid option"),
                }
            }
        }
    }

    fn execute_convertion(formula_fn: fn(f32) -> f32) {
        println!("Please enter the base temperature");
        let value: f32 = utils::read_from_stdin()
            .trim()
            .parse()
            .expect("Value is not decimal");

        print_result(formula_fn(value));
    }

    fn print_menu_options() {
        println!("Temperature Converter menu");
        println!("[1] From Celcius to Fahrenheit");
        println!("[2] From Fahrenheit to Celcius");
        println!("[3] From Kelvin to Celcius");
        println!("[4] From Celcius to Kelvin");
        println!("[0] Exit");
        println!("Enter desired option:");
    }

    // Convert temperature from Celcius to Fahrenheit
    fn ctof(value: f32) -> f32 {
        (value * 1.8) + 32.0
    }

    // Convert temperature from Fahrenheit to Celcius
    fn ftoc(temp_in_fahrenheit: f32) -> f32 {
        (temp_in_fahrenheit - 32.0) * 9.0 / 5.0
    }

    // Convert temperature from kelvin to celcius
    fn ktoc(temp_in_kelvin: f32) -> f32 {
        temp_in_kelvin - 273.15
    }

    // Convert temperature from celcius to kelvin
    fn ctok(temp_in_celcius: f32) -> f32 {
        temp_in_celcius + 273.15
    }

    fn print_result(value: f32) {
        println!("The response is {}", value);
    }
}

pub mod fibonnacci_secuense {
    use crate::base::base::App;
    use crate::utils::utils::read_from_stdin;

    pub struct FibonnacciGenerator {
        pub name: String,
        pub option: u8,
    }

    impl App for FibonnacciGenerator {
        fn get_title(&self) -> String {
            self.name.clone()
        }

        fn get_option_number(&self) -> String {
            format!("{}", self.option)
        }

        fn run(&self) {
            println!("Enter the n fibonnacci number(0-255):");
            let n: u8 = read_from_stdin().trim().parse().expect("Bad Input");
            let response = generate_fibonnaci(n);
            println!("The Fibonnaci secuence is: {}", response);
            println!("Bye, exiting fibonnacci secuense generator")
        }
    }

    fn generate_fibonnaci(n: u8) -> String {
        let mut ant_prev: u128 = 0;
        let mut prev: u128 = 1;
        let mut result = String::new();
        let first_entry = format!("{}, {}", ant_prev, prev);
        result.push_str(&first_entry[..]);

        for _ in (1..=n).enumerate().into_iter() {
            let new_value = prev + ant_prev;
            // swapt elements
            ant_prev = prev;
            prev = new_value;

            let new_entry = format!(", {}", new_value);
            result.push_str(&new_entry[..]);
        }

        result
    }
}

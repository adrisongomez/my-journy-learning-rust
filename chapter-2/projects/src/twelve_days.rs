pub mod twelve_days_son {

    use crate::base::base::App;

    pub struct TwelveDaysGenerator {
        pub name: String,
        pub option: u8,
    }

    impl App for TwelveDaysGenerator {
        fn get_title(&self) -> String {
            self.name.clone()
        }

        fn get_option_number(&self) -> String {
            format!("{}", self.option)
        }

        fn run(&self) -> () {
            let mut song = String::new();
            let mut acc = String::new();
            let header = "My true love gave to me:";
            let bottom = "A partridge in a pear tree.";
            for day in 1u8..=12u8 {
                let day_string = get_day(&day);
                let rest = get_section(&day);
                let first_line = format!("On the {} day of Christmas", day_string.to_lowercase());
                if day != 1 {
                    acc = format!("{day_string} {rest}\n{acc}");
                }
                song = format!("{song}\n{first_line}\n{header}\n{acc}{bottom}\n");
            }

            println!("{}", song);
        }
    }
    fn get_day(day_number: &u8) -> String {
        match day_number {
            1 => return "First".to_string(),
            2 => return "Two".to_string(),
            3 => return "Three".to_string(),
            4 => return "Four".to_string(),
            5 => return "Five".to_string(),
            6 => return "Six".to_string(),
            7 => return "Seven".to_string(),
            8 => return "Eight".to_string(),
            9 => return "Nine".to_string(),
            10 => return "Ten".to_string(),
            11 => return "Eleven".to_string(),
            12 => return "Twelve".to_string(),
            _ => return "".to_string(),
        }
    }

    fn get_section(day_number: &u8) -> String {
        match day_number {
            2 => return "turtle doves and".to_string(),
            3 => return "french hens".to_string(),
            4 => return "calling birds".to_string(),
            5 => return "golden rings".to_string(),
            6 => return "geese a-laying".to_string(),
            7 => return "swans a-swimming".to_string(),
            8 => return "maids a-milking".to_string(),
            9 => return "ladies dancing".to_string(),
            10 => return "lords a-leaping".to_string(),
            11 => return "pipers piping".to_string(),
            12 => return "drummers drumming".to_string(),
            _ => return "".to_string(),
        }
    }
}

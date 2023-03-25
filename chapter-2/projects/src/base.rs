pub mod base {
    pub trait App {
        fn get_title(&self) -> String;
        fn get_option_number(&self) -> String;
        fn run(&self) -> ();
        fn get_menu_entry(&self) -> String {
            format!("{}", self.get_option_number())
        }
    }
}

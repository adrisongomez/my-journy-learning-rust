pub mod utils {
    use std::io::stdin;

    pub fn read_from_stdin() -> String {
        let mut value = String::new();
        stdin()
            .read_line(&mut value)
            .expect("Error while reading input value");
        value
    }
}

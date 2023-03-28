
enum Test {
    Variant1,
    Variant2,
}

fn handle_variant(value: Test) {
    match value {
        Test::Variant1 => {
            println!("Variant 1")
        },
        Test::Variant2 => {
            println!("Variant 2")
        }
    }
}

fn main() {
    let value = Test::Variant1;
    let value_2 = Test::Variant2;

    handle_variant(value);
    handle_variant(value_2);

    match 8 {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        4 => println!("4"),
        5 => println!("5"),
        _ => println!("Other value"),
    }

    match 9 {
        1 => println!("1"),
        2 => println!("2"),
        others => println!("Not supported value: {}", others)
    }
}

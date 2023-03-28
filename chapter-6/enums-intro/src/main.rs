struct _IpAddress1 {
    address: String,
    kind: String, // v4 or v6
}

// Create our enum

enum _IpAddressKind1 {
    V4,
    V6,
}

struct _IpAddress2 {
    address: String,
    kind: _IpAddressKind1,
}

// Extend enum creation

enum _IpAddressKind2 {
    V4(u8, u8, u8, u8),
    V6(String),
} // I don't need the struct now.

// let's add a method

impl _IpAddressKind2 {
    fn _return_value(&self) -> &_IpAddressKind2 {
        self
    }
}

fn main() {
    let value_1 = Some::<u8>(35);
    let none_value: Option<u8> = None;
    let value_2 = 10;

    // SPOILER
    match value_1 {
        Some(value_1) => {
            println!("{}", value_1 + value_2)
        }
        None => {
            println!("Value 1 is None")
        }
    }
    // SPOILER 2
    match none_value {
        Some(none_value) => {
            println!("{}", none_value + value_2)
        }
        None => {
            println!("Value None is None")
        }
    }
}

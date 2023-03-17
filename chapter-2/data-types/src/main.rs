fn main() {
    // Scalars
    let _x: u32;
    let _x: f64;
    let _x: char;
    let _x: bool;

    // Unsigned Integer
    let _x: u8 = 255;
    let _x: u16 = 0;
    let _x: u32 = 0;
    let _x: u64 = 0;
    let _x: u128 = 0;
    let _x: usize = 0; // <- this used depend of the arch of the OS 32bit -> 32

    // Signed Integer
    let _x: i8 = 127;
    let _x: i16 = 0;
    let _x: i32 = 0;
    let _x: i64 = 0;
    let _x: i128 = 0;
    let _x: isize = 0; // <- this used depend of the arch of the OS 32bit -> 32

    // let x: u8 = 256;
    // will produce an overflow so run this code and would thorw a panic call on development run
    // but in production it restart the variable to 256=0, 257=1
    // This is because u8 only allow you to start 8 bits which limits is 255 and when it's
    // overflow, Rust make a wrapping round proccess which make the number be 0 again so
    // pass from 1111 1111 to 0000 0000 in bynary.
    println!("{_x}");

    let x = 255u8; // <- Another way to define a type declaration but the type definition happen on
                   // value it call type suffix.
    println!("{x}");

    let x = 1_000_000; // _ can be used a sepator to make big numbers more readable.

    println!("{x}");

    // Integer Literals
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_1111; // it allow _ as separator on binary too
    let binary_only_u8 = b'A';

    println!("{decimal} {hex} {octal} {binary} {binary_only_u8}");

    // <============================================================)
    // Floating-point numbers

    let x = 6.45; // by default are set to be f64
    println!("{x}");
    let x: f32 = 6.32;
    println!("{x}");

    // |============================================================|
    // Numeric Operators

    let sum = 3 + 5;
    let difference = 5 - 3;
    let product = 6 * 4;
    let quotient = 55.43 / 66.34;
    let truncated = -5 / 3; // -1
    let remainder = 5 % 3;

    println!(
        "{sum} {difference} {product} {}, {}, {}",
        quotient, truncated, remainder
    );

    // |===========================================================|
    // Boolean
    
    let _true_value = true;
    let _false_value: bool = false;

    // |==========================================================|
    // Character
    let _charc = 'a';
    let _charc_expl: char = 'a';
    let _emoji = '😻';

    // compounts
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    let (x, y, z) = tup;

    let five_hundred = tup.0;
    // let no_exists = tup.4; <- it detect that I can't access this because it's not exists

    let _y: () = (); // this is call a unit is empty tuple which is being used to represent an
    
    // empty value or an empty return type
    
    println!("{y} {x} {z} {five_hundred}");

    let array: [u32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    // let array: [u32; 5] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]; -> Almost knows when I violate this
    // array size...
    
    let first_value = array[0];
    println!("{first_value}");
    let _fill_with_ones = [1; 10];
    let first_value = _fill_with_ones[0];
    println!("{first_value}");

    // let value = array[10] <- will call panic and exists the program, because the value is out of
    // band

}

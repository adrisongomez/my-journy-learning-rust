


// This is the main function which is the first function that rust looks to execute the program.
fn main() {
    print_labeled_measurement(6, 'h');

    let _x = 3;
    // this is a statement so it doesn't return value so it can't be assaign to a
    // variable. I mean the entireo let _x = 3 can't be assaign
    let y = {
        let x = 3;
        x+1 // this is a expression because it doesn't have a return value to y
    }; 
    println!(" the value of y is: {y}");
    let value = function_return_implicit();
    println!("{value}");
    let value = function_return_explicit();
    println!("{value}");
    print(add_one(5));
}

fn print(x: i32) {
    println!("{x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn function_return_implicit() -> i32 {
    53
}

fn function_return_explicit() -> i32 {
    return 53;
}

fn add_one(x: i32) -> i32 {
    x+1
}

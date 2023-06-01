fn main() {
    let mut num = 5;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    let r3 = &mut num as *mut i32;
    unsafe {
        println!("r1 is: {}", *r1);
        *r3 = 7;
        println!("r2 is: {}", *r2);
        *r2 = 10;
        println!("r3 is: {}", *r3);
        dangerous();
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    // This function apply unsafe rust within it
    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}


static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

unsafe fn dangerous() {
    println!("This is INSANE!")
}
extern "C" {
    fn abs(input: i32) -> i32;
}

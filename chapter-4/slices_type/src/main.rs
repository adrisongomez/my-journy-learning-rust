fn main() {
    _implementation_1();
    _implementation_2();
    _implementation_3();
    _implementation_4();

    let a = [1,2,3,4,5];
    let slice = &a[1..3];
    // It's the same idea of the string slice. Instead of point to the heap, we are pointing to the
    // stack and we have a length propery with n positions take by our array slice.

    assert_eq!(slice, &[2, 3]); // assert is two value are the same
}

fn first_word_1(s: &String) -> usize {
    let bytes = s.as_bytes(); // this return an array of bytes.
    // convert every element to it bytes eq.

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' { // b' ' is a byte literal syntaxt which return the binary representation
            // of the chan
            return i;
        }
    }
    
    s.len()
}

fn first_word_2(s: &String) -> &str {
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i] // This create a new entry of the stack which contians a pointer on the
            // first position of string or starting point and a len which contains how many bytes
            // are conforming this string so if we have on the head this
            // h e l l o space w o r l d.
            // ^       *  n=5
            // This a our starting point and our len is 5.
        }
    }
    &s[..]
}


fn first_word_3(s: &str) -> &str {
    // Using the type  ^   &str make this function to accept both String and string slice, due to a
    // thing call deref coercions, don't know what it is but seems pretty complex... maybe not...
    for (i, &item) in s.as_bytes().iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}


fn _implementation_1() {
    let mut s = String::from("hello, world");
    let first_word_ending = first_word_1(&s); // so our function is returning only a the index of were the first
    // word end.
    s.clear(); // this reset the string so now s is = ""
    
    // so as we loose the data on S we don't actually have the first_word so we are making our
    // depend on s. S should not change to our code make sense... it works but it's the right
    // behaviour
    println!("{}", first_word_ending)

}

fn _implementation_2() {
    let mut s = String::from("hello, world");
    let first_word = String::from(first_word_2(&s));
    // Here we are solving the error explain above by creating a new entry on the heap for our
    // first_word, so the string_slice that we are getting is being take as String::from a now we
    // have a indepent entry on the heap where our first_word could look for the data without relay
    // on s to be sync. Now first_word and s are not stritly related.
    s.clear();

    println!("{}", first_word)
}

fn _implementation_3() {
    let mut s = String::from("hello, world");
    let first_word = first_word_2(&s); // So here we have our string slice but still tie to 's' variable because
    // s is link to the actual heap entry, with `first_word` we only have a reference to the first
    // letter and len which mean, we are going to read until 5 postion after our start.
    // So if we modify the s variable which mean change the data in the heap we are not sure that
    // our variable first_word have the data that we want...
    // Rust knows about this it let us know with a compile time error or diagnostic is you are
    // using rust_analyzer.
    // s.clear(); // uncommented this to see the actual error.
    println!("{}", first_word);
    s.clear();

}

fn _implementation_4() {
    let s = String::from("hello, world");
    let _first_word = first_word_3(&s);
    let _first_word = first_word_3(&s[..]);

    let _first_word = first_word_3("hello, world");
    let _first_word = first_word_3(&"hello, world"[0..6]);

    println!("{}", _first_word);
}

fn main() {
    let string_literal = "Hello World"; 
    // This is a borrowed version of the scalar str of rust -> &str
    // let )string_literal = *"Hello World"; // Compilation Time error...
    // This is the scalar str of rust -> &str
    
    let _growable_string = String::new();
    // This is also a string but it's using the std::String which is 
    // growable, mutable owned, UTF-8 string type.
    //
    // String -> It's a implementation of Vec<u8>
    //

    let _from_string_literal = string_literal.to_string();
    // Here i'm creating a String from a string literal or `&str`
    //
    let _from_string_string = String::from(string_literal);
    let _hello = String::from("السلام عليكم");
    let _hello = String::from("Dobrý den");
    let _hello = String::from("Hello");
    let _hello = String::from("שָׁלוֹם");
    let _hello = String::from("नमस्ते");
    let _hello = String::from("こんにちは");
    let _hello = String::from("안녕하세요");
    let _hello = String::from("你好");
    let _hello = String::from("Olá");
    let _hello = String::from("Здравствуйте");
    let _hello = String::from("Hola");
    // All of this are valid string.

    let mut s = String::from("foo");
    s.push_str("bar");// push or concat two string...
    println!("{s}");

    s.push('l'); // you can push a char using the `.push`

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let _s3 = s1 + &s2;

    // This is because + <- operator call a method `add` which for string is 
    // fn add(self, s: &str) -> String {
    //

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let _s = format!("{s1}-{s2}-{s3}");

    // other form to create another way to concat strings.
    //
    let _s1 = String::from("hello");
    // let h = _s1[0];
    //
    // This is really complex to explain so let say you can't do that because some 
    // char are not actually a 1-1 relation with a bytes. Becaus we are using UTF-8
    // so value are define with too Bytes... so if you want the first letter it could 
    // 2 Bytes instead of 1, so the only way that you implment this is using the string
    // string slice
    let _h = &_s1[0..0]; //here we are getting a string slice and its the first letter
    //
    
    for c in "Зд".chars() {
        println!("{c}");
    }
}

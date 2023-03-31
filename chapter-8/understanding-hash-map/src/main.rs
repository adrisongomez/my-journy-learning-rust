use std::collections::HashMap;
// So we need to bring the HashMap into scope from the std::collection

fn main() {

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    dbg!(&scores);

    let team_name = String::from("Blue");
    //                                            And this is to get the value and is None just
    //                                          V retun 0 
    let score = scores.get(&team_name).copied().unwrap_or(0);
    //                 ^ Get the value ^ instead of using the actual value just copied the value
    //                   Option<T>       Option<T>

    println!("{team_name} score:{score}");


    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    scores.insert(
        "Eagles".to_string(),
        10 // This is being copy
    );
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(
        field_name,
        field_value // This is moved into the map, because not implement the Copy trait
    ); 

    // filed_value is invalidated;
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25); // This would overwrite previous value

    println!("{:?}", scores);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);

    scores
        .entry(String::from("Yellow")) // This return a type Entry which is tuple of the key and
        // mutable reference to the value
        .or_insert(50);
        // This return a mutable reference to the value and its the value is not defined insert the
        // value on the argument
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

}

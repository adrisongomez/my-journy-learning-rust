fn main() {
    let s = "hello"; // this allocatted this data in the stack
    // "hello" this is a string literal
    {
        let s = "hello world"; // this created a new entry on the stack
        
        // Any change of s would be reflect to s within this scope not the pointer of the s on main
        // function
        
        println!("{s}"); // hello world
    } // s value get drop and s is now "hello"
    println!("{s}");
    
    let mut s = String::from("hello"); // drop previous value and asign a new instance of String
    // which is more complex data type which are going to store data on the heap and assign the
    // reference to our s variable, now we can start iteracting with the heap using our mutable
    // string.
    //
    s.push_str(", world!"); // here we push_str append new data into the head in  this case the ",
    // world!" and mutated the data of our s variable.
    
    println!("{s}");


    let _x = 56; // Here we are allocatting 56 on the stack because is an scallar and assign a ref
    // to x which point to the memory on the stack.
    let _y = _x; // Here we are copy the value of _x into a new entry of the stack.
    // So this two variable are not sharing the same reference or memory space but they have the
    // same value

    let _s1 = String::from("hi"); // Here we are allocating _s1 into the heap... this heap entry is
    // devided into 3 part, which the pointer which is ref that point of the memory space holding
    // the data; the len how many unit of space are being used and capacity how many memory
    // units/slot are being reserved on the heap to this varable
    let _s2 = _s1; // Here we are coping the heap entry so we are using the same pointer to the
    // heap.

    {
        let s1 = String::from("h1"); // Here we are allocating on  the heap s1
        let s2 = s1.clone(); // Here are cloning the value of s1 into s2
        // This made a new entry of the heap which means that s2 is not pointing the same memory
        // space on the heap, which means s1 and s2 doesn't share the same memory address


        // Rust allow this but only if s1 is not being call anymore. Why?
        // Because when the scope close if s2 and s1 are being used when 'drop' function get call
        // it's going to call drop to the same memory slot... so a bug will happen, rust compiler
        // now this and throw an error when this happen try to uncommented this line and run
        // `cargo check`
        // let s2 = s1;
        // move is the action that is happening we are moving the entry from s1 and s2 and s1 get
        // invalidated

        println!("{s1} {s2}");
    }

    {
        let here = String::from("here");

        fn do_something_with_here(here: String) {
            println!("{here}");
        } 

        do_something_with_here(here); // by doing this we are moving the ownership of this variable
        // into the scope of `do_something_with_here` function so we can't not use here anymore
        // becasue it got invalidated so the only way that this can work if we create a new copy of
        // the `here` value so we just passing a new reference and not our original `here`
        // variable
        //
        // This is similar to shallow copy but it's not...
        
        // println!("{here}") // this would throw a compile error
        {
            let here = String::from("here");
            do_something_with_here(here.clone()); // we are making a copy of  here so we are not
            // sending the actual reference of here into the function so here is not invalidated in
            // the scope.

            println!("{here}");
        }

        
    }

    {
        let x = 50;
        let y = &x;

        println!("{}", x+*y)
    }

    {
        let a = String::from("hello");
        let b = a.clone();
        println!("{} - {}", a, b);
    }

}

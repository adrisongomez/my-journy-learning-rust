use std::thread;
use std::time::Duration;


fn main() {
    let new_thread = thread::spawn(|| {
        for i in 1..10 {
            println!("Inside the thread {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    new_thread.join().unwrap();
    for i in 1..5 {
        println!("Inside the main {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    let vector: Vec<String> = vec![];

    let new_thread = thread::spawn( move || {
            println!("Here is your vector dummy: {:?}", vector);
        }
    );

    new_thread.join().unwrap();

}

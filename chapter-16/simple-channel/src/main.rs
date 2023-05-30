use std::sync::mpsc;
use std::time::Duration;
use std::thread;

fn main() {
    let (transmiter, receiber) = mpsc::channel();
    thread::spawn(move ||{
        transmiter.send("Hi! Rubber Duck").unwrap();
    });

    let message = receiber.recv().unwrap();
    println!("Here is your message: {}", message);

    let (transmiter, receiber) = mpsc::channel();
    thread::spawn(move || {
        let v = vec![
            String::from("First"),
            String::from("Second"),
            String::from("Thrith")
        ];

        for message in v {
            transmiter.send(message).unwrap();
            thread::sleep(Duration::from_millis(1));
        }

    });

    for (i, recieved) in receiber.iter().enumerate() {
        println!("Here is my msg #{}: {}", i+1, recieved);
    }
}

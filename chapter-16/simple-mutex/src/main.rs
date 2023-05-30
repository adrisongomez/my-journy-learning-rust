use std::sync::{Mutex, Arc};
use std::thread;

fn main() {
    let my_mutex = Mutex::new(5);

    {
        let value = my_mutex.lock().unwrap();
        println!("{}", value)
    }

    println!("{:?}", my_mutex);

    let m = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let mutex_thread = Arc::clone(&m);
        let handle = thread::spawn(move || {
            let mut n = mutex_thread.lock().unwrap();
            *n += 1;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!(" Final value: {} ", *m.lock().unwrap())
}

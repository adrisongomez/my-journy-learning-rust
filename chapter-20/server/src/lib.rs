use std::collections::HashMap;
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

type HandlerFunction = fn(&mut TcpStream);
pub struct Server {
    port: String,
    routes: HashMap<String, HandlerFunction>,
    thread_pool: ThreadPool,
}

impl Server {
    pub fn start(port: &str, n: Option<usize>) -> Server {
        println!("Server started on {port}");
        let n: usize = if let Some(n) = n { n } else { 5 };
        Server {
            port: port.to_string(),
            routes: HashMap::new(),
            thread_pool: ThreadPool::new(n),
        }
    }

    pub fn set_handler(&mut self, path: String, handler: HandlerFunction) {
        self.routes.insert(path, handler);
    }

    pub fn listen_and_serve(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Start listening...");
        let listener =
            TcpListener::bind(format!("127.0.0.1:{}", self.port)).expect("Error binding");
        for incomming_conn in listener.incoming() {
            let mut new_conn = incomming_conn.expect("Error accepting the connection");
            let mut buffer = String::new();
            BufReader::new(&mut new_conn)
                .read_to_string(&mut buffer)
                .expect("Error reading the request");
            let request = HTTPRequest::new(buffer.as_str());
            println!("Receive {}: {}", request.method, request.path);
            self.handle_request(request, new_conn)
        }
        Ok(())
    }
    fn handle_request(&self, request: HTTPRequest, mut new_conn: TcpStream) {
        if let Some(handle) = self.routes.get(&request.path) {
            handle(&mut new_conn);
            // self.thread_pool.execute(move || {
            //     let handle = *handle.as_ref();
            //     handle(new_conn);
            // .   new_conn.flush().unwrap();
            // });

            new_conn.flush().unwrap();
            
        } else {
            println!("404 not found");
            // self.thread_pool.execute(move || {
            let contents =
                fs::read_to_string("./assets/404.html").expect("Error reading the 404 page");
            let length = contents.len();
            let response = format!(
                "HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\n\r\n{}",
                length, contents
            );

            new_conn
                .write_all(response.as_bytes())
                .expect("Error writing the response");
            // });
        }
    }
}

struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

type WorkerReceiver = Arc<Mutex<mpsc::Receiver<Job>>>;

impl ThreadPool {
    fn new(n: usize) -> ThreadPool {
        assert!(n > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let workers: Vec<Worker> = (0..n)
            .into_iter()
            .map(move |x| Worker::new(x, receiver.clone()))
            .collect();
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, task: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(task);
        if let Some(ref sender) = self.sender {
            sender
                .send(job)
                .expect("Error trying to send job to workers");
        }
    }
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());

        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread_handler.take() {
                thread.join().unwrap();
            }
        }
    }
}

struct Worker {
    id: usize,
    thread_handler: Option<thread::JoinHandle<()>>,
}

impl Worker {
    pub fn new(id: usize, receiver: WorkerReceiver) -> Worker {
        Worker {
            id,
            thread_handler: Some(thread::spawn(move || loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("Worker {id} got a job; executing...");
                job();
            })),
        }
    }
}

struct HTTPRequest {
    method: String,
    path: String,
}

impl HTTPRequest {
    fn new(raw_request: &str) -> HTTPRequest {
        let lines = raw_request
            .lines()
            .next()
            .unwrap()
            .split(" ")
            .collect::<Vec<&str>>();
        let method = lines[0].to_string();
        let path = lines[1].to_string();
        HTTPRequest { method, path }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_should_parse_http_request() {
        let raw_request =
            "GET / HTTP/1.1\nHost: localhost:8080\nUser-Agent: curl/7.64.1\nAccept: */*";
        let request = super::HTTPRequest::new(raw_request);
        assert_eq!(request.method, "GET");
        assert_eq!(request.path, "/");
    }
}

use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::thread;
use std::sync::{mpsc, Arc, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    id:usize,
    thread:thread::JoinHandle<()>,
}

pub struct ThreadPool {
    workers:Vec<Worker>,
    sender:mpsc::Sender<Job>,
    receiver: Arc<Mutex<mpsc::Receiver<Job>>>,
    max_threads:usize,
    active_count:Arc<AtomicUsize>,
}

impl Worker {
    fn new(id:usize,receiver:Arc<Mutex<mpsc::Receiver<Job>>>,active_count: Arc<AtomicUsize>) -> Worker {
        let thread = thread::spawn(move || {
            let active_count = Arc::clone(&active_count);
            loop {
                let job = receiver.lock().unwrap().recv().unwrap();
                active_count.fetch_add(1, Ordering::SeqCst);
                job();
                active_count.fetch_sub(1, Ordering::SeqCst);
            }
        });

        Worker{id,thread}
    }
}

impl ThreadPool {
    fn new(size:usize) -> ThreadPool {
        assert!(size>0);

        let (sender,receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let active_count = Arc::new(AtomicUsize::new(0));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver),Arc::clone(&active_count)));
        }

        ThreadPool { workers, sender, receiver, max_threads:(size*2), active_count }
    }

    fn execute<F>(&mut self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        if self.active_count.load(Ordering::SeqCst) >= self.workers.len() && self.workers.len() < self.max_threads {
            let receiver_clone = Arc::clone(&self.receiver);
            let active_clone = Arc::clone(&self.active_count);
            let id = self.workers.len();
            let new_worker = Worker::new(id, receiver_clone, active_clone);
            self.workers.push(new_worker);
        }
        
        self.sender.send(job).unwrap();
    }
}

fn handle_connection(mut stream:TcpStream){
    let mut buffer = [0;512];
    stream.read(&mut buffer).unwrap();
    
    let request = String::from_utf8_lossy(&buffer);
    
    let response = "HTTP/1.1 200 OK\r\nContent-Length: 12\r\n\r\nHello World!";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    let mut pool = ThreadPool::new(1);
    println!("Server launched to 127.0.0.1:8080");

    for stream in listener.incoming(){
        let stream = stream.unwrap();
        pool.execute(move || {
            handle_connection(stream);
        });
    }
}
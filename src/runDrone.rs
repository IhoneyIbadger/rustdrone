use std::{sync::{mpsc, Arc, Mutex}, net::{TcpListener, TcpStream}, thread};

struct Drone {
    switchedOn: boolean;
}

impl Drone {
    fn bootUp(&self) {
        self.switchedOn = true;
    }

    fn run(&self) {
        
    }
}

enum DroneState {
    SwitchedOff,
    Booting,
    Connecting,
    Running,
    Updating,
    Debugging,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new (size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));
 
        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            workers.push(Worker::new(id));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where 
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new (id: usize, receiver: mpsc::Receiver<Job>) -> Worker {
        let thread = thread::spawn(move || {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");

            job();
        });

        Worker { id, thread }
    }
}
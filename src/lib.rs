use std::{sync::{mpsc, Arc, Mutex}, thread};

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    // We wrap the sender in an Option so we can take it out when shutting down
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {
    // Create a new ThreadPool with the given number of threads.
    // Panics if you try to make zero thread
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0, "Can't have a pool of zero threads, sorry!");

        // Make a channel for sending jobs
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        // Pre-allocate our workers vector
        let mut workers = Vec::with_capacity(size);

        // Spin up the workers
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        if let Some(tx) = &self.sender {
            // If the pool hasn't been shut down yet, send the job.
            tx.send(job).expect("ThreadPool has been shut down");
        } else {
            // Otherwise, let the user know they've already torn us down.
            eprintln!("Cannot execute; ThreadPool is already shut down");
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        // close the sending side. Workers will see Err and exit.
        self.sender.take();
        
        // then join on all the worker threads so we don't leak.
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(handle) = worker.thread.take() {
                handle.join().expect("Worker thread panicked");
            }
        }
    }
}

struct Worker {
    id: usize,
    // Wrapped in Option so we can take() it in Drop
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        // Spawn the thread that will pull jobs off the channel.
        let thread = thread::spawn(move || loop {
            // Wait for a job to arrive (or for the channel to close).
            let msg = receiver
                .lock()
                .expect("Mutex poisoned")
                .recv();

            match msg {
                Ok(job) => {
                    // Got a job!
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Err(_) => {
                    // No more jobs — channel closed.
                    println!("Worker {} disconnecting; shutting down.", id);
                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

use std::{
    sync::{Arc, Mutex, mpsc},
    thread
};
// use `mpsc` to communicate between threads with sender-receiver channel
// use Arc-Mutex to allow multiple workers/threads own the receiver in a thread-safe way

// ====================================================
// Define structs `ThreadPool` and `Job`
// ====================================================

pub struct ThreadPool { // create a new struct named ThreadPool
    workers: Vec<Worker>, // This ThreadPool store a vector named `workers` containing a finite number of threads (Worker)
    sender: mpsc::Sender<Job>, // It also has the `sender` to assign `Job` to each worker
}

type Job = Box<dyn FnOnce() + Send + 'static>;
// This type `Job` represents the closures/tasks assigned to each worker by the ThreadPool
// Type `Job` implements `FnOnce()` that represents the closure,
// implements `Send` to allow it to be sent down the mpsc channel,
// and 'static to enables the job to live for the entire of the program.
// We wrap them inside a pointer `Box` because we don't know what task would be given (dynamically sized)

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool { // `new()` method to initialize the thread pool: `let pool = ThreadPool::new(...)`
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel(); // start the channel
        let receiver = Arc::new(Mutex::new(receiver)); // Wrap the `receiver` inside Arc-Mutex

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver))); // Create `size` number of workers
            // For each worker, we use `Arc::clone()` to bump the reference count to `receiver`,
            // so that multiple workers can own and access this `receiver`
        }

        ThreadPool { workers: workers, sender: sender}
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
        // Explained all the traits above
    {
        let job: Job = Box::new(f); // create `Job` from given task `f: F`
        self.sender.send(job).unwrap(); // pool.execute() -> send the jobs down the workers in the pool
    }
}

// ====================================================
// Define struct `Worker`
// ====================================================
/*
Why do we need a struct `Worker`?

When we run `thread::spawn(func)`,
we have to give a function for the
spawn threads to run on as soon as it is created.

However, in our project, the workers in our thread pool
have to wait for the code that we will send later.
The standard library's implementation does not include that.
=> We have to implement it manually.

Our `Worker` struct will store a `JoinHandle<()>`
and an `id` to distinguish between them.

The `Worker` also receive the `Receiver<Job>`,
from the ThreadPool's Sender.
*/

type WorkerThread = Arc<Mutex<mpsc::Receiver<Job>>>; // create type alias for short

struct Worker { // This `Worker` struct is not `pub`, only used internally.
    id: usize,
    thread: thread::JoinHandle<WorkerThread>,
}
// Why `thread::JoinHandle<WorkerThread>`?
// Recall the old lession when we call `thread::spawn(...)`,
// it returns a `JoinHandle<T>` type, where T is the type the closure returns.
// Here, our closure return a `receiver`,
// which is a type of Job wrapped in Arc and Mutext (alias WorkerThread)
//
// So when we call `thread::spawn(|| receiver)`,
// the returned value will have type `JoinHandle<WorkerThread>`

impl Worker {
    fn new(id: usize, receiver: WorkerThread) -> Worker {
        let thread = thread::spawn(move || { // use `move` to transfer the ownership of the `job` to the closure
            loop { // open a loop for the thread to ask the receiver for a job again and again (forever...)
                let job = receiver.lock().unwrap().recv().unwrap(); // ask the receiver for the lock and the job

                println!("Worker {id} got a job; executing.");

                job(); // when `pool.execute()` is called, the sender will send the job, then this thread will execute it.
            } // goes out of scope, release the lock for other threads
        });

        Worker { id: id, thread: thread } // return the initialized Worker instance
    }
}

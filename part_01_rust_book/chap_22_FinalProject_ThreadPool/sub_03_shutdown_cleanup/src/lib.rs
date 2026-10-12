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
    sender: Option<mpsc::Sender<Job>>, // It also has the `sender` to assign `Job` to each worker
    // We wrap the `sender` in `Option` to be able to use `Option::take` to move the sender out of the threadpool (for shutting down)
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

        ThreadPool { workers: workers, sender: Some(sender)} // wrap sender in `Some(sender)` to match `Option<mpsc::Sender<Job>>` type
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
        // Explained all the traits above
    {
        let job: Job = Box::new(f); // create `Job` from given task `f: F`
        self.sender.as_ref().unwrap().send(job).unwrap(); // pool.execute() -> send the jobs down the workers in the pool
        // In the old version, `self.sender` gives us `&Sender<Job>`, so `self.sender.send()` is `&Sender<Job>::send()`.
        //
        // But now, we have wrapped it inside Option, so `self.sender` gives us `&Option<Sender<Job>>`.
        // Thus, we cannot run `self.sender.send()` anymore because it would be `&Option<...>::send()`.
        // So, we run `self.sender.as_ref()` first to convert into `Option<&Sender<Job>>`,
        // then `self.sender.as_ref().unwrap()` to get the `&Sender<Job>`.
        // Only after that we can call `.send(job)` like the old version
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
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
    thread: Option<thread::JoinHandle<()>>,
}
// Why `Option<thread::JoinHandle<()>>`?
//
// First, about `JoinHandle<()>`.
// Recall the old lession when we call `let thread = thread::spawn(...)`,
// it returns a `JoinHandle<T>` type, so the type of `thread` is `JoinHandle<T>`,
// where T is the type the closure returns.
// Here, our closure return a unit type `()`,
// => the type of the handle is `JoinHandle<()>
//
// About wrapping the thread inside `Option<..>`,
// This allow us to call `worker.thread.take()`
// to take the onwership of the thread `JoinHandle()` from `Option<JoinHandle>`,
// and put a None value back there, beco,e `Option<None>`

impl Worker {
    fn new(id: usize, receiver: WorkerThread) -> Worker {
        let thread = thread::spawn(move || { // use `move` to transfer the ownership of the `job` to the closure
            loop { // open a loop for the thread to ask the receiver for a job again and again (forever...)
                let message = receiver.lock().unwrap().recv(); // ask the receiver for the lock
                // in the old code, we run `receiver.lock().unwrap().recv().unwrap()` to get the job from the Result<T,E>.
                // But now, we don't unwrap the received object anymore, keep it as Result<T,E> to use for the match

                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                        job(); // when `pool.execute()` is called, the sender will send the job, then this thread will execute it.
                    },
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break; // break the loop, shutdown
                    }
                }
            } // goes out of scope, release the lock for other threads
        });

        Worker { id: id, thread: Some(thread) } // return the initialized Worker instance
    }
}

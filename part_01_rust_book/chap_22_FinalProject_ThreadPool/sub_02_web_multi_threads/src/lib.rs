use std::thread;

pub struct ThreadPool { // create a new struct named ThreadPool
    threads: Vec<thread::JoinHandle<()>> // This ThreadPool store a vector named `threads` containing a finite number of threads
};
// Why `Vec<thread::JoinHandle<()>>?
// This `threads` vector stores the output `thread::spawn(...)`.
// Recall the old lession when we call `thread::spawn(...)`,
// it returns a `JoinHandle<T>` type, where T is the type the closure returns.
// Here, our closure returns nothing (unit struct `()`), so we let it as `JoinHandle<()>`

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool { // `new()` method to initialize the thread pool: `let pool = ThreadPool::new(...)`
        assert!(size > 0);
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        f()
    }
}

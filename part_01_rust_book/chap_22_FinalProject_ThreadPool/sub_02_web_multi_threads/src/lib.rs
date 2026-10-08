use std::thread;

pub struct ThreadPool; // create a new struct named ThreadPool

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool { // `new()` method to initialize the thread pool: `let pool = ThreadPool::new(...)`
        assert!(size > 0);
        ThreadPool
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        f()
    }
}

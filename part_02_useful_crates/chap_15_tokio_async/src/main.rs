fn main() {
    println!("Hello, world!");
}
/*
| File | Focus |
|---|---|
| **`02_time_select_cancellation.rs`** | Timers, `interval`, `timeout`, `select!`, task abortion, and what happens to unfinished work. |
| **`03_channels_shared_state.rs`** | Tokio’s bounded `mpsc`, `oneshot`, async mutexes, and semaphores. Explain how their behavior differs from the standard-library tools you already learned. |
| **`04_async_io.rs`** | A small local TCP client/server, buffered reading/writing, message boundaries, and a concise introduction to Tokio file operations. |
| **`05_task_management_blocking.rs`** | `JoinSet`, `spawn_blocking`, connecting Tokio with Rayon, and graceful shutdown in a practical example. |
 */

fn main() {
    println!("Hello, world!");
}
/*
| File | Contents |
|---|---|
| **`02_reductions_errors.rs`** | `sum`, `min`/`max`, `reduce`, and per-worker accumulation with `fold`; combining partial results; collecting `Result` values; `try_for_each`; short-circuiting and why other work may already be running. |
| **`03_thread_pools_performance.rs`** | Automatic global pool; custom `ThreadPoolBuilder`; worker counts; `pool.install`; reusing a pool; processing batches/blocks like your Python pool examples; measuring real CPU work and understanding overhead. |
| **`04_join_scope_spawn.rs`** | `rayon::join` for two computations; recursive work splitting; `rayon::scope` and scoped spawning; borrowing surrounding data; `rayon::spawn`, ownership, and explicit completion; differences from `thread::spawn` and its `JoinHandle`. |
| **`05_shared_state_channels.rs`** | Using standard `Mutex`, `Arc`, and atomics with Rayon; when borrowing eliminates the need for `Arc`; avoiding lock contention through local accumulation; sending results through standard `mpsc`; safe completion and channel closure; a practical compute-and-write-results example. |
 */

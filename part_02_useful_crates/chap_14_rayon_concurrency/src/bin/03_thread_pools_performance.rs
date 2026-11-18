//! Rayon: choose a thread pool, reuse workers, and measure CPU work.
//!
//! Setup: cargo add rayon@1.12
//! Run:   cargo run --release --bin 03_thread_pools_performance
//!
//! Use --release when interpreting timings. Debug-mode results are not a
//! useful basis for choosing a production worker count.
//! A worker is an OS thread, not a reserved CPU core or a separate process.
//! Many jobs share the pool; an input item does not get its own OS thread.
//!
//! References:
//! https://docs.rs/rayon/1.12.0/rayon/struct.ThreadPoolBuilder.html
//! https://docs.rs/rayon/1.12.0/rayon/struct.ThreadPool.html
//! https://docs.rs/rayon/1.12.0/rayon/iter/trait.IndexedParallelIterator.html
//!
//! This file runs bounded CPU calculations; it does not write files or sleep.
//! The timings are an illustration, not a rigorous benchmark or speedup promise.

use rayon::prelude::*;
use rayon::{ThreadPool, ThreadPoolBuildError, ThreadPoolBuilder};
use std::hint::black_box;
use std::time::{Duration, Instant};

fn main() -> Result<(), ThreadPoolBuildError> {
    println!();

    demo_global_pool();
    println!("\n============================\n");

    demo_custom_pool()?;
    println!("\n============================\n");

    demo_worker_identity()?;
    println!("\n============================\n");

    demo_pool_reuse()?;
    println!("\n============================\n");

    demo_nested_operations()?;
    println!("\n============================\n");

    demo_granularity()?;
    println!("\n============================\n");

    demo_global_configuration();
    println!("\n============================\n");

    demo_cpu_timing()?;
    println!("\n============================\n");

    demo_worker_counts()?;
    println!("\n============================\n");

    demo_batch_processing()?;

    Ok(())
}

// =====================================================
// 01. The global pool is available automatically
// =====================================================

fn demo_global_pool() {
    println!("01. The global pool is available automatically");

    let available = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(1);
    println!("Available parallelism estimate: {available}");
    println!("Rayon global workers: {}", rayon::current_num_threads());
    println!("Caller worker index: {:?}", rayon::current_thread_index()); // None here

    let squares: Vec<i32> = (1_i32..=5).into_par_iter().map(|n| n * n).collect();
    println!("Using the global pool: {squares:?}");
    // Calling current_num_threads outside a pool queries/initializes the global
    // pool. Otherwise, the first parallel operation can initialize it for you.
    // Inside a custom pool, current_num_threads reports THAT current pool instead.
    // available_parallelism is an OS-informed estimate, not a physical-core count.
    // Neither value guarantees that all workers run simultaneously.
}

// =====================================================
// 02. Build a custom pool and run work with install()
// =====================================================

fn demo_custom_pool() -> Result<(), ThreadPoolBuildError> {
    println!("02. Build a custom pool and run work with install()");

    let pool = ThreadPoolBuilder::new()
        .num_threads(2)
        .build()?;

    let squares: Vec<i32> = pool.install(|| {
        println!("Inside custom pool: {} workers", rayon::current_num_threads());
        (1_i32..=6).into_par_iter().map(|n| n * n).collect()
    });
    println!("Result returned to caller: {squares:?}");
    println!("Pool's own worker count: {}", pool.current_num_threads());
    // build() can fail, for example if OS thread creation fails; ? propagates it.
    // install runs the closure in this pool and returns its result synchronously.
    // Rayon operations inside the closure inherit this pool's execution context.
    // Creating a pool does not redirect unrelated parallel operations outside it.
    // install is not a spawn returning a task handle, nor a background prefetcher.
    Ok(())
}

// =====================================================
// 03. Worker names and indices are diagnostic information
// =====================================================

fn demo_worker_identity() -> Result<(), ThreadPoolBuildError> {
    println!("03. Worker names and indices are diagnostic information");

    let pool = ThreadPoolBuilder::new()
        .num_threads(2)
        .thread_name(|index| format!("rayon-demo-{index}"))
        .build()?;

    let records: Vec<String> = pool.install(|| {
        (0_usize..8).into_par_iter()
            .map(|item| {
                let thread = std::thread::current();
                format!("item={item}, worker={:?}, name={:?}",
                    rayon::current_thread_index(), thread.name())
            })
            .collect()
    });
    for record in records {
        println!("{record}");
    }
    // Print after collecting to keep reporting in input order.
    // Some/all tiny jobs may use the same worker. No round-robin distribution
    // or permanent input-to-worker assignment is promised.
    // Worker indices are local to a pool, not globally unique OS thread IDs.
    // Do not use worker indices as sample IDs or reproducible random seeds.
    Ok(())
}

// =====================================================
// 04. Reuse a pool across independent operations
// =====================================================

fn demo_pool_reuse() -> Result<(), ThreadPoolBuildError> {
    println!("04. Reuse a pool across independent operations");

    let pool = ThreadPoolBuilder::new().num_threads(2).build()?;
    let mut data = vec![7_i32, 2, 9, 1];

    pool.install(|| data.par_sort_unstable());
    let squares: Vec<i32> = pool.install(|| {
        data.par_iter().map(|&n| n * n).collect()
    });
    println!("Sorted input: {data:?}");
    println!("Mapped output: {squares:?}");
    // install can borrow surrounding data; no Arc is necessary for this use.
    // Its work completes before the next line uses that data.
    // Build the pool once, then pass &ThreadPool into reusable processing helpers.
    // Building a new pool for every element or batch adds startup/resource costs.
    // Several live custom pools each own workers: their thread counts add up.
    Ok(())
}

// =====================================================
// 05. Nested parallel operations share the current pool
// =====================================================

fn demo_nested_operations() -> Result<(), ThreadPoolBuildError> {
    println!("05. Nested parallel operations share the current pool");

    let pool = ThreadPoolBuilder::new().num_threads(2).build()?;
    let rows = vec![vec![1_i64, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
    let row_sums: Vec<i64> = pool.install(|| {
        rows.par_iter()
            .map(|row| row.par_iter().copied().sum())
            .collect()
    });
    println!("Nested row sums: {row_sums:?}"); // [6, 15, 24]
    // The inner par_iter reuses this same pool. It does not create two more
    // threads per row. Rayon splits work so other workers can take pending jobs.
    // This scheduling strategy is called work stealing; no manual queue is needed.
    // Nested parallelism is supported, but small inner workloads may be cheaper
    // with row.iter().copied().sum() while only the outer loop is parallel.
    // Other libraries may have their own pools (for example numerical kernels);
    // combining them can oversubscribe CPUs even though Rayon reuses its own pool.
    Ok(())
}

// =====================================================
// 06. Granularity: group useful work instead of tiny tasks
// =====================================================

fn demo_granularity() -> Result<(), ThreadPoolBuildError> {
    println!("06. Granularity: group useful work instead of tiny tasks");

    let pool = ThreadPoolBuilder::new().num_threads(2).build()?;
    let numbers: Vec<u64> = (0_u64..1_024).collect();
    let (fine, chunked) = pool.install(|| {
        let fine: Vec<u64> = numbers.par_iter()
            .with_min_len(128)
            .map(|&n| n * n)
            .collect();

        let chunked: Vec<u64> = numbers.par_chunks(128)
            .map(|chunk| chunk.iter().map(|&n| n * n).sum())
            .collect();
        (fine, chunked)
    });
    println!("Fine-grained output has {} values", fine.len());
    println!("Chunked output has {} partial sums", chunked.len());
    println!("Same total: {}", fine.iter().sum::<u64>() == chunked.iter().sum::<u64>());
    // with_min_len guides splitting of an indexed parallel iterator. It does not
    // fix worker count, assign jobs to workers, or change the output into chunks.
    // par_chunks changes the iterator's items to slices; our map then produces
    // one partial sum per slice. The two outputs intentionally have different shapes.
    // Chunk size is a tuning choice: too small adds overhead, too large can limit
    // load balancing. Start with Rayon's defaults before adding tuning controls.
    Ok(())
}

// =====================================================
// 07. Global configuration is a one-time application choice
// =====================================================

fn demo_global_configuration() {
    println!("07. Global configuration is a one-time application choice");

    println!("This program already initialized the global pool in section 01.");
    // In a DIFFERENT program, before any global Rayon use, you could write:
    // ThreadPoolBuilder::new().num_threads(4).build_global()?;
    // Do not put that call here: a running global pool cannot be reconfigured.
    // Libraries should generally avoid choosing a process-wide policy for callers;
    // accept a custom pool or use the caller's current/default Rayon context.

    println!();

    match ThreadPoolBuilder::new().num_threads(1).build_global() {
        Ok(()) => println!("Global pool initialized"),
        Err(error) => println!("Expected already-initialized error: {error}"),
    }
    // Alternatively configure the DEFAULT worker count before process startup:
    // RAYON_NUM_THREADS=4 cargo run --release --bin 03_thread_pools_performance
    // An explicit nonzero num_threads on a custom builder overrides that default.
    // num_threads(0) means automatic selection, not a pool with zero workers.
    // A custom pool does not change the already-configured global pool.
}

// =====================================================
// 08. Time real CPU work outside pool construction
// =====================================================

fn demo_cpu_timing() -> Result<(), ThreadPoolBuildError> {
    println!("08. Time real CPU work outside pool construction");

    let pool = ThreadPoolBuilder::new().num_threads(2).build()?;
    let inputs: Vec<u64> = (0_u64..8_192).collect();

    // Warm both paths before measurement. All results must agree.
    let expected = sequential_checksum(&inputs);
    assert_eq!(parallel_checksum(&pool, &inputs), expected);

    let (sequential, sequential_time) = measure_median(|| sequential_checksum(&inputs));
    let (parallel, parallel_time) = measure_median(|| parallel_checksum(&pool, &inputs));
    assert_eq!(sequential, parallel);

    println!("Checksum: {parallel}");
    println!("Sequential median of 3: {sequential_time:?}");
    println!("Parallel median of 3:   {parallel_time:?}");
    if parallel_time.as_secs_f64() > 0.0 {
        println!("Observed sequential/parallel ratio: {:.2}x",
            sequential_time.as_secs_f64() / parallel_time.as_secs_f64());
    }
    // Pool construction, input allocation, warm-up, and printing are outside
    // these timed regions. The parallel path DOES include install/scheduling costs.
    // black_box in the helpers discourages optimization from removing the work.
    // A ratio below 1 means this parallel run was slower. That is valid evidence.
    // Three repetitions are illustrative; thermal changes, CPU contention,
    // frequency scaling, cache effects, and fixed measurement order can bias them.
    // For serious benchmarking use a benchmark harness and many samples.
    Ok(())
}

// =====================================================
// 09. Compare worker counts without assuming more is faster
// =====================================================

fn demo_worker_counts() -> Result<(), ThreadPoolBuildError> {
    println!("09. Compare worker counts without assuming more is faster");

    let inputs: Vec<u64> = (0_u64..8_192).collect();
    let expected = sequential_checksum(&inputs);
    for workers in [1, 2, 4] {
        let pool = ThreadPoolBuilder::new().num_threads(workers).build()?;
        assert_eq!(parallel_checksum(&pool, &inputs), expected); // warm-up
        let (checksum, elapsed) = measure_median(|| parallel_checksum(&pool, &inputs));
        assert_eq!(checksum, expected);
        println!("workers={workers}, median of 3={elapsed:?}");
    }
    // Fixed small counts keep this demo bounded, even on a large machine.
    // A one-worker Rayon pool still incurs pool/iterator overhead: it is not
    // identical to the ordinary sequential iterator measured in section 08.
    // Four workers are allowed even if fewer CPUs are available; the OS schedules
    // them, but that does not create extra CPU capacity or guarantee acceleration.
    // Memory bandwidth, work imbalance, synchronization, and tiny workloads can
    // limit scaling. The best count depends on the workload and machine.
    // Reuse the chosen pool in the real application rather than rebuilding it
    // per request. This loop creates separate pools only to compare configurations.
    Ok(())
}

// =====================================================
// 10. Practical example: reuse a pool for batches of blocks
// =====================================================

fn demo_batch_processing() -> Result<(), ThreadPoolBuildError> {
    println!("10. Practical example: reuse a pool for batches of blocks");

    fn process_blocks(pool: &ThreadPool, blocks: &[Vec<u64>]) -> Vec<u64> {
        pool.install(|| {
            blocks.par_iter()
                .map(|block| block.iter().fold(0_u64, |sum, &value| {
                    sum.wrapping_add(cpu_work(value))
                }))
                .collect()
        })
    }

    let pool = ThreadPoolBuilder::new().num_threads(2).build()?;
    let batches = [
        vec![vec![1_u64, 2, 3], vec![4, 5]],
        vec![vec![6_u64, 7], vec![8, 9, 10]],
    ];
    for (index, blocks) in batches.iter().enumerate() {
        let checksums = process_blocks(&pool, blocks);
        println!("Batch {index}: {checksums:?}");
    }
    // One pool, two sequential batch calls; blocks WITHIN each batch run in parallel.
    // Results retain block order, like Python Pool.map, but use shared-memory threads.
    // process_blocks waits for this batch. It does not overlap the next batch
    // with a training loop or implement the dataloader's prefetch queue.
    // Later we will add explicit background production and bounded buffering.
    // Keep blocking I/O outside this CPU-work example; sleeping measures waiting,
    // not CPU throughput. Rayon is not a separate-process isolation mechanism.
    Ok(())
}

// =====================================================
// Helpers for sections 08-10
// =====================================================

fn cpu_work(mut value: u64) -> u64 {
    // A deterministic chain of integer operations, not a cryptographic hash.
    // Explicit wrapping arithmetic keeps overflow behavior identical in debug
    // and release builds. Each input's chain is sequential; inputs are independent.
    for _ in 0..256 {
        value ^= value >> 12;
        value ^= value << 25;
        value ^= value >> 27;
        value = value.wrapping_mul(2_685_821_657_736_338_717);
    }
    value
}

fn sequential_checksum(inputs: &[u64]) -> u64 {
    black_box(inputs).iter()
        .fold(0_u64, |sum, &value| sum.wrapping_add(cpu_work(value)))
}

fn parallel_checksum(pool: &ThreadPool, inputs: &[u64]) -> u64 {
    pool.install(|| {
        black_box(inputs).par_iter()
            .map(|&value| cpu_work(value))
            .reduce(|| 0_u64, u64::wrapping_add)
    })
    // Wrapping addition is associative, so regrouping gives the same checksum.
}

fn measure_median<F>(mut operation: F) -> (u64, Duration)
where
    F: FnMut() -> u64,
{
    let mut timings = [Duration::ZERO; 3];
    let mut result = 0;
    for timing in &mut timings {
        let start = Instant::now();
        result = black_box(operation());
        *timing = start.elapsed();
    }
    timings.sort_unstable();
    (result, timings[1])
    // The caller verifies results; the median reduces sensitivity to one outlier.
    // This assumes a deterministic operation returning the same result each run.
}

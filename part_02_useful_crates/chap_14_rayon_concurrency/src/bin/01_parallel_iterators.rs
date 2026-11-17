//! Rayon: process collections in parallel without manually managing workers.
//!
//! Setup (inside chap_14_rayon_concurrency):
//!     cargo add rayon@1.12
//! Run:
//!     cargo run --bin 01_parallel_iterators
//!
//! Rayon uses a pool of OS threads within ONE process. Independent CPU work
//! can run on different cores; Rayon is not Python-style multiprocessing.
//! You describe the computation; Rayon splits and schedules the work.
//! There is no async/.await here, and no manual worker loop or job queue.
//!
//! Tiny inputs keep examples readable; they do not demonstrate speedups.
//! Pool configuration and meaningful timing belong in file 03.
//! Reductions/error handling belong in file 02; tasks/shared state come later.
//!
//! References:
//! https://docs.rs/rayon/1.12.0/rayon/
//! https://docs.rs/rayon/1.12.0/rayon/iter/trait.ParallelIterator.html
//! https://docs.rs/rayon/1.12.0/rayon/iter/trait.IndexedParallelIterator.html
//! https://docs.rs/rayon/1.12.0/rayon/slice/index.html

use rayon::prelude::*;

fn main() {
    println!();

    demo_sequential_vs_parallel();
    println!("\n============================\n");

    demo_borrowing();
    println!("\n============================\n");

    demo_owned_inputs();
    println!("\n============================\n");

    demo_mutation();
    println!("\n============================\n");

    demo_filtering();
    println!("\n============================\n");

    demo_tuple_arguments();
    println!("\n============================\n");

    demo_zip_and_enumerate();
    println!("\n============================\n");

    demo_execution_order();
    println!("\n============================\n");

    demo_chunks();
    println!("\n============================\n");

    demo_sorting();
    println!("\n============================\n");

    demo_searching();
    println!("\n============================\n");

    demo_string_processing();
    println!("\n============================\n");

    demo_block_processing();
}

// =====================================================
// 01. From sequential map to parallel map
// =====================================================

fn demo_sequential_vs_parallel() {
    println!("01. From sequential map to parallel map");

    let numbers = vec![2_i32, 3, 8, 9];

    let sequential: Vec<i32> = numbers.iter()
        .map(|&n| n * n)
        .collect();

    let parallel: Vec<i32> = numbers.par_iter()
        .map(|&n| n * n)
        .collect();

    println!("Sequential: {sequential:?}");
    println!("Parallel:   {parallel:?}"); // [4, 9, 64, 81]
    println!("Same results: {}", sequential == parallel);
    // rayon::prelude::* brings the extension traits into scope.
    // .map(...) describes a transformation; .collect() drives the work and
    // waits for this whole operation to finish. No JoinHandle is necessary.
    // There is no promise of one task/thread per element or a speedup here.
    // Starting with a vector gives an ordered source; Vec collection retains
    // that logical order even when computations finish in another order.
}

// =====================================================
// 02. par_iter(): borrow elements without moving them
// =====================================================

fn demo_borrowing() {
    println!("02. par_iter(): borrow elements without moving them");

    let words = vec![String::from("Rust"), String::from("Rayon"), String::from("parallel")];
    let lengths: Vec<usize> = words.par_iter().map(|word| word.len()).collect();

    println!("Byte lengths: {lengths:?}"); // [4, 5, 8]
    println!("Original still usable: {words:?}");
    // Items are &String. Borrowing the input does not clone every String.
    // len() counts UTF-8 bytes, not necessarily user-perceived characters.

    let suffix = String::from("!");
    let decorated: Vec<String> = words.par_iter()
        .map(|word| format!("{word}{suffix}"))
        .collect();
    println!("Borrowed shared configuration: {decorated:?}");
    // This operation completes before collect returns, so closures can borrow
    // surrounding data. Neither move nor Arc is automatically required.
    // Shared borrowed data must be Sync; Rayon enforces the relevant bounds.
}

// =====================================================
// 03. into_par_iter(): take ownership of elements
// =====================================================

fn demo_owned_inputs() {
    println!("03. into_par_iter(): take ownership of elements");

    let words = vec![String::from("alpha"), String::from("beta")];
    let decorated: Vec<String> = words.into_par_iter()
        .map(|mut word| {
            word.push('!');
            word
        })
        .collect();

    println!("Moved and transformed: {decorated:?}");
    // Items are owned Strings, so the closure can mutate and return each one.
    // words has moved into the iterator and cannot be used afterward.
    // Moving values between workers requires Send; no process serialization
    // or Python-style pickling is involved.

    let cubes: Vec<i32> = (1_i32..=5).into_par_iter().map(|n| n.pow(3)).collect();
    println!("Parallel integer range: {cubes:?}"); // [1, 8, 27, 64, 125]
}

// =====================================================
// 04. par_iter_mut(): safely change separate elements
// =====================================================

fn demo_mutation() {
    println!("04. par_iter_mut(): safely change separate elements");

    let mut numbers = vec![1_i32, 2, 3, 4];
    numbers.par_iter_mut().for_each(|n| *n *= 10);
    println!("Changed in place: {numbers:?}"); // [10, 20, 30, 40]
    // Each closure receives an exclusive &mut i32 to a distinct element.
    // Rayon splits the mutable slice safely: no Mutex is needed for this.
    // .for_each() executes the side effects and returns (), not a new Vec.

    let mut destination = vec![0_i32; numbers.len()];
    destination.par_iter_mut().zip(numbers.par_iter())
        .for_each(|(out, &input)| *out = input + 1);
    println!("Separate output buffer: {destination:?}");
    // Avoid capturing one mutable Vec and calling push() from every closure.
    // Use map(...).collect() to create output, or give each worker a distinct
    // output element as above. Closures must support concurrent invocation.
}

// =====================================================
// 05. Filter and transform with a parallel pipeline
// =====================================================

fn demo_filtering() {
    println!("05. Filter and transform with a parallel pipeline");

    let numbers = vec![1_i32, 2, 3, 4, 5, 6];
    let squares: Vec<i32> = numbers.par_iter()
        .copied()
        .filter(|n| n % 2 == 0)
        .map(|n| n * n)
        .collect();
    println!("Squares of even numbers: {squares:?}"); // [4, 16, 36]
    // copied() changes &i32 into i32. filter's predicate borrows its item;
    // map then receives the retained i32 values.
    // Rayon fuses this pipeline rather than creating a Vec after every step.

    let labels = ["  Rust ", "", " Rayon", "   "];
    let cleaned: Vec<String> = labels.par_iter()
        .filter_map(|text| {
            let text = text.trim();
            if text.is_empty() { None } else { Some(text.to_owned()) }
        })
        .collect();
    println!("Nonblank labels: {cleaned:?}");
    // filter_map combines filtering and mapping via Option.
    // filter/filter_map retain relative order for this ordered source, but
    // lose IndexedParallelIterator: the output length is not known upfront.
    // To enumerate original positions, call enumerate BEFORE filtering.
}

// =====================================================
// 06. Tuple inputs: the equivalent of Python starmap
// =====================================================

fn demo_tuple_arguments() {
    println!("06. Tuple inputs: the equivalent of Python starmap");

    fn add(x: i32, y: i32) -> i32 {
        x + y
    }

    fn calculate(x: i32, y: i32, z: i32) -> i32 {
        x * y + z
    }

    let pairs = vec![(1, 2), (3, 4), (5, 6)];
    let sums: Vec<i32> = pairs.par_iter().map(|&(x, y)| add(x, y)).collect();
    println!("Two arguments: {sums:?}"); // [3, 7, 11]

    let triples = vec![(1, 2, 2), (2, 4, 3), (3, 6, 4)];
    let results: Vec<i32> = triples.into_par_iter()
        .map(|(x, y, z)| calculate(x, y, z))
        .collect();
    println!("Three arguments: {results:?}"); // [4, 11, 22]
    // Rust has no need for a separate starmap method: destructure the tuple.
    // par_iter yields references (hence &(x, y)); into_par_iter yields values.
    // The & pattern copies these i32 components; non-Copy fields need borrowing
    // or ownership-aware destructuring instead of copying them out of a reference.
}

// =====================================================
// 07. Pair inputs with zip; attach indices with enumerate
// =====================================================

fn demo_zip_and_enumerate() {
    println!("07. Pair inputs with zip; attach indices with enumerate");

    let left = [10_i32, 20, 30];
    let right = [1_i32, 2, 3];
    let sums: Vec<i32> = left.par_iter().zip(right.par_iter())
        .map(|(&a, &b)| a + b)
        .collect();
    println!("Paired sums: {sums:?}"); // [11, 22, 33]
    // zip stops at the shorter input, like ordinary Iterator::zip.
    // zip_eq is available when unequal lengths should be a programmer error
    // (it panics). Validate lengths explicitly for untrusted input.

    let labeled: Vec<String> = left.par_iter().enumerate()
        .map(|(index, value)| format!("item_{index}={value}"))
        .collect();
    println!("Labels: {labeled:?}");
    // enumerate gives logical input indices, not worker IDs or completion order.
    // zip/enumerate require indexed parallel iterators such as these slices.
}

// =====================================================
// 08. Execution order differs from collected result order
// =====================================================

fn demo_execution_order() {
    println!("08. Execution order differs from collected result order");

    let squares: Vec<usize> = (0_usize..6).into_par_iter()
        .map(|index| {
            println!("Computing input {index}");
            index * index
        })
        .collect();

    println!();

    println!("Ordered output: {squares:?}"); // [0, 1, 4, 9, 16, 25]
    // The Computing lines MAY appear in any order, including increasing order.
    // Never rely on task start/completion order or assume all threads are used.
    // This Vec still follows the range's logical order.
    // collect waits for the operation; background batch prefetching is a separate
    // design that we will introduce in the dataloader example later.
    // Printing here illustrates ordering; it is a poor CPU performance benchmark.
}

// =====================================================
// 09. Process non-overlapping slices with par_chunks
// =====================================================

fn demo_chunks() {
    println!("09. Process non-overlapping slices with par_chunks");

    let numbers = [1_i32, 2, 3, 4, 5, 6, 7];
    let reversed: Vec<Vec<i32>> = numbers.par_chunks(3)
        .map(|chunk| chunk.iter().rev().copied().collect())
        .collect();
    println!("Reversed chunks: {reversed:?}"); // [[3, 2, 1], [6, 5, 4], [7]]
    // par_chunks(3) yields borrowed slices; the last can be shorter than 3.
    // chunk.iter() inside the closure is sequential; chunks are processed in parallel.
    // A chunk is a unit of data, not a permanently assigned worker/thread.

    let mut values = [1_i32, 2, 3, 4, 5, 6, 7];
    values.par_chunks_mut(3).for_each(|chunk| chunk.reverse());
    println!("Reversed chunks in place: {values:?}"); // [3, 2, 1, 6, 5, 4, 7]
    // Mutable chunks do not overlap, allowing safe parallel changes.
    // Chunk sizes must be greater than zero. These APIs do not provide a
    // background prefetch queue or implement a complete dataloader by themselves.
}

// =====================================================
// 10. Parallel sorting: stable, unstable, and custom keys
// =====================================================

fn demo_sorting() {
    println!("10. Parallel sorting: stable, unstable, and custom keys");

    let mut numbers = vec![5, 1, 4, 1, 3];
    numbers.par_sort();
    println!("Ascending: {numbers:?}");

    numbers.par_sort_unstable_by(|a, b| b.cmp(a));
    println!("Descending: {numbers:?}");
    // Sorting is eager, changes the slice in place, and returns ().
    // Stable sorting retains relative order among equal keys; unstable sorting
    // need not, but is often faster and does not allocate an auxiliary buffer.

    let mut people = vec![("Alice", 30), ("Bob", 20), ("Carol", 30)];
    people.par_sort_by_key(|person| person.1);
    println!("Stable sort by age: {people:?}");
    // Bob first; Alice remains before Carol because both have age 30.

    let mut readings = vec![3.5_f64, f64::NAN, -1.0, 2.0];
    readings.par_sort_by(|a, b| a.total_cmp(b));
    println!("Floats with an explicit total order: {readings:?}");
    // f64 lacks Ord because NaN is unordered under ordinary comparisons.
    // total_cmp gives a total ordering including NaNs; decide whether your
    // application instead wants to reject or filter NaNs before sorting.
}

// =====================================================
// 11. Parallel searches: any match versus first match
// =====================================================

fn demo_searching() {
    println!("11. Parallel searches: any match versus first match");

    let numbers = [4_i32, 9, 12, 15, 18];
    let any = numbers.par_iter().copied().find_any(|n| n % 3 == 0);
    let first = numbers.par_iter().copied().find_first(|n| n % 3 == 0);
    println!("Any multiple of 3: {any:?}");
    println!("First in input order: {first:?}"); // Some(9)
    println!("Has a negative: {}", numbers.par_iter().any(|&n| n < 0));
    println!("All positive: {}", numbers.par_iter().all(|&n| n > 0));
    // find_any can return any matching item; it does not promise input order.
    // find_first respects logical order and can require more coordination.
    // Searching may stop early, but other tasks may already be running.
    // Keep predicates free of side effects that require every item to be visited.
}

// =====================================================
// 12. Process lines of text in parallel
// =====================================================

fn demo_string_processing() {
    println!("12. Process lines of text in parallel");

    let text = "  alpha  \nbeta\n\n gamma ";
    let labels: Vec<String> = text.par_lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(str::to_uppercase)
        .collect();
    println!("Cleaned labels: {labels:?}"); // ["ALPHA", "BETA", "GAMMA"]
    // par_lines borrows lines from an already available string.
    // It does not read a file asynchronously or create a streaming file reader.
    // Parallel string iteration is also provided by rayon::prelude::*.
    // Do not assume every parallel iterator is indexed or supports enumerate.
}

// =====================================================
// 13. Practical example: transform blocks like Pool.map
// =====================================================

fn demo_block_processing() {
    println!("13. Practical example: transform blocks like Pool.map");

    fn reverse_block(block: &[i32]) -> Vec<i32> {
        block.iter().rev().copied().collect()
    }

    let blocks = vec![vec![1, 2, 3], vec![10, 20, 30, 40], vec![100, 200]];
    let outputs: Vec<Vec<i32>> = blocks.par_iter()
        .map(|block| reverse_block(block))
        .collect();

    println!("Inputs:  {blocks:?}");
    println!("Outputs: {outputs:?}"); // [[3, 2, 1], [40, 30, 20, 10], [200, 100]]
    // This plays the role of pool.map(reverse_block, blocks) in your Python file.
    // Rayon uses threads sharing process memory, not separate worker processes.
    // Blocks can be borrowed; no pickling, queue loop, or stop sentinel is needed.
    // Unlike the mixed Python examples, this Vec<Vec<i32>> has one element type.
    // Generic functions or enums can support other deliberately chosen types.

    println!();

    println!("Use map + collect for returned values; for_each for side effects.");
    println!("Borrow with par_iter; move with into_par_iter; mutate with par_iter_mut.");
    // Prefer independent CPU work. Avoid long blocking reads, sleeps, or channel
    // waits inside pool work; these occupy workers rather than accelerate computation.
    // Parallelism adds overhead; small operations may be faster sequentially.
    // The next files cover combining results, errors, and controlling the pool.
}

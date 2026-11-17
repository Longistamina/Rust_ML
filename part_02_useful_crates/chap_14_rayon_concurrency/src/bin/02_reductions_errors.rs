//! Rayon: combine parallel results and handle fallible operations.
//!
//! Setup: cargo add rayon@1.12
//! Run:   cargo run --bin 02_reductions_errors
//!
//! File 01 transforms collections; this file combines their results.
//! A reduction merges independently computed partial values into one result.
//! A parallel fold creates local accumulators, which usually need a final merge.
//! These operations complete before returning; no async/.await is involved.
//!
//! References:
//! https://docs.rs/rayon/1.12.0/rayon/iter/trait.ParallelIterator.html
//! https://docs.rs/rayon/1.12.0/rayon/iter/trait.IndexedParallelIterator.html
//!
//! Small data makes the examples easy to inspect, not a speed benchmark.
//! Arithmetic examples use deliberately small values unless checking overflow.

use rayon::prelude::*;
use std::collections::BTreeMap;
use std::num::ParseIntError;

fn main() {
    println!();

    demo_builtin_reductions();
    println!("\n============================\n");

    demo_reduce();
    println!("\n============================\n");

    demo_reduce_with();
    println!("\n============================\n");

    demo_associativity();
    println!("\n============================\n");

    demo_fold_and_merge();
    println!("\n============================\n");

    demo_frequency_table();
    println!("\n============================\n");

    demo_collect_result();
    println!("\n============================\n");

    demo_keep_all_results();
    println!("\n============================\n");

    demo_try_for_each();
    println!("\n============================\n");

    demo_try_reduce();
    println!("\n============================\n");

    demo_batch_statistics();
}

// =====================================================
// 01. Built-in reductions and empty inputs
// =====================================================

fn demo_builtin_reductions() {
    println!("01. Built-in reductions and empty inputs");

    let numbers = [2_i64, 3, 4, 5];
    let sum: i64 = numbers.par_iter().copied().sum();
    let product: i64 = numbers.par_iter().copied().product();
    let minimum = numbers.par_iter().copied().min();
    let maximum = numbers.par_iter().copied().max();
    let evens = numbers.par_iter().filter(|&&n| n % 2 == 0).count();

    println!("sum={sum}, product={product}"); // 14, 120
    println!("min={minimum:?}, max={maximum:?}, even count={evens}");
    // sum/product need a result type; min/max return Option because input can be empty.
    // copied() makes min/max return Option<i64> rather than Option<&i64>.

    let empty: [i64; 0] = [];
    println!("Empty sum: {}", empty.par_iter().copied().sum::<i64>()); // 0
    println!("Empty product: {}", empty.par_iter().copied().product::<i64>()); // 1
    println!("Empty minimum: {:?}", empty.par_iter().copied().min()); // None
    // The empty sum/product use their operations' identity elements.
    // These APIs do not automatically check integer overflow; choose types
    // and bounds carefully, or use checked arithmetic as shown later.
}

// =====================================================
// 02. reduce(): combine values using an identity
// =====================================================

fn demo_reduce() {
    println!("02. reduce(): combine values using an identity");

    let numbers = [1_i64, 2, 3, 4, 5];
    let squares = numbers.par_iter()
        .map(|&n| n * n)
        .reduce(|| 0_i64, |left, right| left + right);
    println!("Sum of squares: {squares}"); // 55
    // || 0 constructs a neutral initial value; it may be called MANY times.
    // Rayon computes partial sums and merges them with the same operation.
    // reduce consumes and returns the iterator's Item type: here i64.
    // For this particular operation, .sum::<i64>() is shorter.

    let product = numbers.par_iter().copied()
        .reduce(|| 1_i64, |left, right| left * right);
    println!("Product: {product}"); // 120
    // Use 1 for multiplication: 1*x=x. An initial value such as 10 for addition
    // would be counted repeatedly, not just added once to the final answer.
    // Add a one-time offset AFTER reducing with the correct identity.
}

// =====================================================
// 03. reduce_with(): combine without supplying an identity
// =====================================================

fn demo_reduce_with() {
    println!("03. reduce_with(): combine without supplying an identity");

    let numbers = [12_i32, 5, 30, 8];
    let largest = numbers.par_iter().copied().reduce_with(i32::max);
    println!("Largest: {largest:?}"); // Some(30)

    let empty: [i32; 0] = [];
    println!("Empty input: {:?}", empty.par_iter().copied().reduce_with(i32::max));
    // No identity needed: existing items seed the partial reductions.
    // The result is Option<Item>: None for empty input, Some(value) otherwise.
    // .max() is clearer for this simple case; reduce_with supports custom merges.
}

// =====================================================
// 04. Grouping matters: associativity and floating point
// =====================================================

fn demo_associativity() {
    println!("04. Grouping matters: associativity and floating point");

    let a = 10_i32;
    let b = 3_i32;
    let c = 2_i32;
    println!("Subtraction, left grouping: {}", (a - b) - c); // 5
    println!("Subtraction, other grouping: {}", a - (b - c)); // 9
    // Rayon may group reduction operations differently as work is split.
    // A valid associative operation satisfies (a op b) op c = a op (b op c).
    // Subtraction/division generally fail that rule: do not use them directly
    // as parallel reductions when you mean a sequential left-to-right fold.

    println!();

    let x = 1e16_f64;
    let y = -1e16_f64;
    let z = 1.0_f64;
    println!("Floating addition, left grouping: {}", (x + y) + z); // 1
    println!("Floating addition, other grouping: {}", x + (y + z)); // 0
    let total: f64 = [x, y, z].par_iter().copied().sum();
    println!("Parallel floating sum: {total}");
    // Mathematical addition is associative; finite-precision floating addition
    // is not. Do not promise bit-for-bit equality with a sequential sum or
    // a fixed answer across different execution configurations.
    // Ordered collection does not imply left-to-right evaluation of a reduction.
}

// =====================================================
// 05. fold() builds local accumulators; reduce() merges them
// =====================================================

fn demo_fold_and_merge() {
    println!("05. fold() builds local accumulators; reduce() merges them");

    let words = ["Rust", "makes", "parallel", "work", "expressive"];
    let (word_count, byte_count) = words.par_iter()
        .fold(
            || (0_usize, 0_usize),
            |(count, bytes), word| (count + 1, bytes + word.len()),
        )
        .reduce(
            || (0, 0),
            |left, right| (left.0 + right.0, left.1 + right.1),
        );
    println!("words={word_count}, UTF-8 bytes={byte_count}"); // 5, 31
    // fold consumes input items (&str references here) into a DIFFERENT type:
    // a (count, bytes) tuple. reduce merges tuples with tuples.
    // Unlike standard Iterator::fold, Rayon fold returns a parallel iterator
    // of partial accumulators, not one final accumulator.
    // Local means per subdivision of the work, NOT exactly one per OS thread.
    // The number of accumulators and initialization calls is not fixed.
    // No shared counter or mutex is necessary.
}

// =====================================================
// 06. Frequency table: local maps instead of one shared lock
// =====================================================

fn demo_frequency_table() {
    println!("06. Frequency table: local maps instead of one shared lock");

    let words = ["rust", "rayon", "rust", "threads", "rayon", "rust"];
    let counts: BTreeMap<&str, usize> = words.par_iter().copied()
        .fold(BTreeMap::new, |mut local, word| {
            *local.entry(word).or_insert(0) += 1;
            local
        })
        .reduce(BTreeMap::new, |mut left, right| {
            for (word, count) in right {
                *left.entry(word).or_insert(0) += count;
            }
            left
        });
    println!("Counts: {counts:?}"); // rayon=2, rust=3, threads=1
    // Each local map is exclusively owned by its fold; merges consume maps.
    // BTreeMap keeps printed keys sorted for an easy-to-read demo.
    // HashMap is another option when sorted keys are unnecessary.
    // No Arc<Mutex<Map>> or counter crate is needed for this pattern.
    // Simply collecting duplicate (word, 1) pairs into a map would overwrite
    // entries; it would NOT add the counts.
}

// =====================================================
// 07. Collect Result<Vec<T>, E>: fail when any item fails
// =====================================================

fn demo_collect_result() {
    println!("07. Collect Result<Vec<T>, E>: fail when any item fails");

    let valid = ["10", "20", "30"];
    let parsed: Result<Vec<i32>, ParseIntError> = valid.par_iter()
        .map(|text| text.parse::<i32>())
        .collect();
    println!("All valid: {parsed:?}"); // Ok([10, 20, 30])

    println!();

    let mixed = ["10", "oops", "30", "also wrong"];
    let parsed: Result<Vec<i32>, ParseIntError> = mixed.par_iter()
        .map(|text| text.parse::<i32>())
        .collect();
    match parsed {
        Ok(values) => println!("Values: {values:?}"),
        Err(error) => println!("Expected parse error: {error}"),
    }
    // One Err makes the overall result Err; no successful partial Vec is returned.
    // Rayon tries to stop unnecessary work, but some work may already be running.
    // With multiple failures, which error is returned is unspecified: it need not
    // be the first invalid item in input order.
    // No automatic retry or rollback happens. Prefer pure transformations here.
}

// =====================================================
// 08. Collect Vec<Result<T, E>> to retain every outcome
// =====================================================

fn demo_keep_all_results() {
    println!("08. Collect Vec<Result<T, E>> to retain every outcome");

    let inputs = ["10", "oops", "30", "bad"];
    let outcomes: Vec<Result<i32, ParseIntError>> = inputs.par_iter()
        .map(|text| text.parse::<i32>())
        .collect();

    println!("All outcomes: {outcomes:?}");

    println!();

    // Process the collected results sequentially for reproducible reporting.
    for (index, outcome) in outcomes.iter().enumerate() {
        match outcome {
            Ok(value) => println!("row {index}: {value}"),
            Err(error) => println!("row {index}, input {:?}: {error}", inputs[index]),
        }
    }
    // The outer Vec means parse errors are data: every item is processed unless
    // a panic or another exceptional failure interrupts the operation.
    // The ordered source preserves row correspondence in this collected Vec.
    // Result<Vec<T>, E> = an all-or-error result.
    // Vec<Result<T, E>> = a complete per-item report.
    // Silently using filter_map(|text| text.parse().ok()) would discard failures
    // and lose row correspondence; do that only if discarding is your policy.
}

// =====================================================
// 09. try_for_each(): run fallible work without collecting
// =====================================================

fn demo_try_for_each() {
    println!("09. try_for_each(): run fallible work without collecting");

    fn validate(n: i32) -> Result<(), &'static str> {
        if n < 0 { Err("negative values are forbidden") } else { Ok(()) }
    }

    let valid = [1, 2, 3];
    let result: Result<(), &'static str> = valid.par_iter()
        .try_for_each(|&n| validate(n));
    println!("Valid input: {result:?}"); // Ok(())

    println!();

    let invalid = [1, -2, 3, -4];
    let result: Result<(), &'static str> = invalid.par_iter()
        .try_for_each(|&n| validate(n));
    println!("Expected validation failure: {result:?}");
    // Returns Ok(()) if all operations succeed, otherwise an encountered Err.
    // There is no returned collection: use map(...).collect() when you need values.
    // If used to write files or mutate buffers, earlier side effects survive an
    // error, and other in-flight work may finish. This is NOT a transaction.
    // Do not assume exactly how many items have run when failure is reported.
}

// =====================================================
// 10. try_reduce(): merge fallible results and check overflow
// =====================================================

fn demo_try_reduce() {
    println!("10. try_reduce(): merge fallible results and check overflow");

    fn parse_and_sum(inputs: &[&str]) -> Result<u64, String> {
        inputs.par_iter()
            .map(|text| text.parse::<u64>()
                .map_err(|error| format!("invalid integer {text:?}: {error}")))
            .try_reduce(
                || 0_u64,
                |left, right| left.checked_add(right)
                    .ok_or_else(|| String::from("sum exceeds u64::MAX")),
            )
    }

    println!("Valid total: {:?}", parse_and_sum(&["10", "20", "30"])); // Ok(60)
    println!("Empty total: {:?}", parse_and_sum(&[])); // Ok(0)

    println!();

    println!("Expected parse failure: {:?}", parse_and_sum(&["10", "wrong", "30"]));

    println!();

    println!("Expected overflow: {:?}", parse_and_sum(&["18446744073709551615", "1"]));
    // map produces Result<u64, String>; try_reduce unwraps successful values
    // for the merge closure and propagates errors from either stage.
    // The identity closure returns the success value 0, not Ok(0).
    // Nonnegative addition is useful here: if the exact total fits in u64,
    // every partial total fits too. Otherwise checked_add reports overflow.
    // With signed numbers, intermediate overflow can depend on grouping even
    // when the final mathematical sum fits. Choose a suitable accumulation type.
    // As with other fallible parallel operations, error selection is unspecified.
}

// =====================================================
// 11. Practical example: validate and summarize batch values
// =====================================================

fn demo_batch_statistics() {
    println!("11. Practical example: validate and summarize batch values");

    #[derive(Debug)]
    struct Stats {
        count: u64,
        sum: u64,
        min: u32,
        max: u32,
    }

    fn summarize(inputs: &[&str]) -> Result<Option<Stats>, String> {
        inputs.par_iter()
            .map(|text| {
                let value = text.parse::<u32>()
                    .map_err(|error| format!("invalid measurement {text:?}: {error}"))?;
                Ok(Stats { count: 1, sum: u64::from(value), min: value, max: value })
            })
            .try_reduce_with(|left, right| {
                Ok(Stats {
                    count: left.count.checked_add(right.count)
                        .ok_or_else(|| String::from("measurement count overflow"))?,
                    sum: left.sum.checked_add(right.sum)
                        .ok_or_else(|| String::from("measurement sum overflow"))?,
                    min: left.min.min(right.min),
                    max: left.max.max(right.max),
                })
            })
            .transpose()
    }

    match summarize(&["10", "20", "30", "40"]) {
        Ok(Some(stats)) => {
            let mean = stats.sum as f64 / stats.count as f64;
            println!("count={}, sum={}, min={}, max={}, mean={mean}",
                stats.count, stats.sum, stats.min, stats.max);
        }
        Ok(None) => println!("No measurements"),
        Err(error) => println!("Invalid batch: {error}"),
    }
    println!("Empty batch: {:?}", summarize(&[])); // Ok(None)

    println!();

    println!("Expected invalid batch: {:?}", summarize(&["10", "oops", "30"]));
    // try_reduce_with needs no identity; it returns Option<Result<Stats, String>>.
    // transpose changes that into Result<Option<Stats>, String>:
    // empty input -> Ok(None), success -> Ok(Some(stats)), any error -> Err(...).
    // Min/max can start from actual data instead of invented sentinel values.
    // All counts/sums stay integer until the final approximate floating mean.
    // For very large integers, conversion to f64 can lose precision.
    // Result handles anticipated failures; it does not catch a panic. Rayon
    // propagates panics from these operations, and side effects are not rolled back.
    // Next: pool configuration, worker counts, and performance measurement.
}

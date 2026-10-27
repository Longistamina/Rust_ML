/*
indicatif: terminal progress bars and spinners.

1. Basic progress bar
2. Manual position, initial progress, and changing totals
3. Iterator adapters
4. Unknown totals and spinners
5. Styles, templates, and spinner frames
6. Dynamic messages and metadata
7. Elapsed time, throughput, and ETA
8. Finish, clear, or abandon
9. Print without disrupting progress
10. Multiple / nested progress bars
11. Shared progress across threads
12. Byte streams and batch processing
13. Visibility and refresh frequency

Setup:
    cargo add indicatif@0.18

Save as src/bin/01_indicatif.rs and run:
    cargo run --bin 01_indicatif

No [[bin]] entry is needed for src/bin/.
Only indicatif and the standard library are used.

Demos use short sleeps to make progress visible; remove them for real work.
Bars normally draw to stderr and hide when it isn't a terminal.
Run in a terminal to see animation; redirected output may contain no bars.
This isn't an automatic HTML notebook-widget replacement for tqdm.auto.

Rayon integration is reserved for the later Rayon chapter.
Reference: https://docs.rs/indicatif/0.18/indicatif/
*/

use indicatif::{
    MultiProgress, ProgressBar, ProgressDrawTarget, ProgressIterator,
    ProgressStyle,
};
use std::error::Error;
use std::io::{self, Cursor, Read};
use std::thread;
use std::time::Duration;

// Shared helpers keep individual demos short.

fn pause() {
    thread::sleep(Duration::from_millis(20));
}

fn standard_style() -> ProgressStyle {
    ProgressStyle::with_template(
        "{prefix:>10} [{bar:24.cyan/blue}] {pos}/{len} {msg}",
    )
    .expect("valid literal template")
    .progress_chars("=>-")
}

fn make_bar(total: u64, label: &str) -> ProgressBar {
    let bar = ProgressBar::new(total);
    bar.set_style(standard_style());
    bar.set_prefix(label.to_owned());
    bar
}

// ============================================================================
// 1. Basic progress bar
// ============================================================================

fn demo_basic() {
    let bar = make_bar(20, "Basic");

    for _ in 0..20 {
        pause(); // perform work first
        bar.inc(1); // then record completed work
    }

    bar.finish_with_message("done");

    // new(total) sets the amount of work.
    // inc(n) adds completed units; n needn't be 1.
    // Progress uses integer u64 units, not fractional positions.
    // Explicitly finish bars before printing section separators.
}

// ============================================================================
// 2. Manual position, initial progress, and changing totals
// ============================================================================

fn demo_manual() {
    let bar = make_bar(100, "Manual");
    bar.set_position(40); // resume at 40/100

    for completed in [50, 60, 70, 80, 90, 100] {
        pause();
        bar.set_position(completed);
    }
    bar.finish_with_message("resumed task complete");

    let bar = make_bar(3, "Discovered");

    for _ in 0..2 {
        pause();
        bar.inc(1);
    }

    bar.set_length(5); // discovered 3 additional tasks
    for _ in 0..3 {
        pause();
        bar.inc(1);
    }
    bar.finish_with_message("all five done");

    // inc(n) is relative; set_position(n) is absolute.
    // For 75% progress, use position 75 with total 100.
    // Choose integer units fine enough for the precision you need.
    // Changing totals / resuming can affect rate and ETA estimates.
}

// ============================================================================
// 3. Iterator adapters: like tqdm(items)
// ============================================================================

fn demo_iterators() {
    for _ in (0..12).progress() {
        pause();
    }

    println!();

    let bar = make_bar(6, "Iterator");
    for value in (1..=6).progress_with(bar.clone()) {
        pause();
        bar.set_message(format!("value={value}"));
    }
    bar.finish_with_message("iterator complete");

    // progress() infers length from an ExactSizeIterator.
    // progress_with(bar) uses a bar you configured yourself.
    // Adapter progress measures items yielded; it doesn't verify that the
    // processing inside the loop succeeded.

    let filtered = (0..20).filter(|n| n % 2 == 0);

    for _ in filtered.progress_count(10) {
        pause();
    }

    // filter generally removes ExactSizeIterator support.
    // progress_count(total) supplies the count explicitly.
    // Don't invent a total if it is genuinely unknown; use a spinner.
    // On an early break / error, don't mark incomplete work as successful.
}

// ============================================================================
// 4. Unknown totals and spinners
// ============================================================================

fn demo_spinner() {
    let spinner = ProgressBar::new_spinner();
    spinner.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} {elapsed_precise} {msg}",
        )
        .expect("valid literal template"),
    );

    spinner.set_message("discovering input...");
    spinner.enable_steady_tick(Duration::from_millis(80));

    for item in 1..=15 {
        pause();
        spinner.set_message(format!("discovered {item} items"));
    }

    spinner.finish_with_message("discovery complete");

    // steady_tick animates even while work isn't updating the bar.
    // Alternatively, call tick() manually as your work advances.
    // With no total, a meaningful completion percentage / ETA isn't available.
    // A spinner is an activity indicator, not proof that work is progressing.
}

// ============================================================================
// 5. Custom styles and spinner frames
// ============================================================================

fn demo_styles() {
    let bar = ProgressBar::new(15);
    bar.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{wide_bar:.cyan/blue}] {percent:>3}% {msg}",
        )
        .expect("valid literal template")
        .progress_chars("#>-")
        .tick_strings(&["-", "\\", "|", "/", "done"]),
    );

    bar.set_message("custom appearance");

    for _ in 0..15 {
        pause();
        bar.inc(1);
    }
    bar.finish_with_message("styled task complete");

    // Templates use indicatif placeholders, not Rust variable interpolation.
    // {bar:24} fixes width; {wide_bar} uses available terminal space.
    // progress_chars: completed portion, current head, remaining portion.
    // tick_strings supplies spinner frames; the final entry is the finish frame.
    // Use ASCII frames when your terminal's Unicode rendering is unsuitable.
    // Invalid templates return an error; expect() here is for fixed literals.
}

// ============================================================================
// 6. Dynamic messages: task name, loss, accuracy, etc.
// ============================================================================

fn demo_messages() {
    let bar = make_bar(10, "Training");

    for step in 1..=10 {
        pause();

        let loss = 1.0 / step as f64;
        let accuracy = step as f64 / 10.0;

        bar.set_message(format!(
            "loss={loss:.3} accuracy={accuracy:.0}%",
            accuracy = accuracy * 100.0,
        ));
        bar.inc(1);
    }

    bar.finish_with_message("training demo complete");

    // set_prefix labels the task; set_message supplies changing metadata.
    // The template must contain {prefix} / {msg} to display them.
    // indicatif's default bar template doesn't show either field.
    // For generated text, pass an owned String, as format! does here.
}

// ============================================================================
// 7. Timing, rate, and ETA
// ============================================================================

fn demo_timing() {
    let bar = ProgressBar::new(20);
    bar.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:20.green/white} \
             {pos}/{len} {per_sec} ETA {eta_precise}",
        )
        .expect("valid literal template"),
    );

    for _ in 0..20 {
        pause();
        bar.inc(1);
    }

    bar.finish();

    // Estimates need observations; early ETA can fluctuate.
    // {per_sec} measures your chosen units per second.
    // For byte progress, use {bytes_per_sec}, {bytes}, {total_bytes}.
    // reset_eta() clears the rate/ETA estimator when a new phase has
    // substantially different work characteristics.
}

// ============================================================================
// 8. Finish, clear, or abandon
// ============================================================================

fn demo_completion() {
    let completed = make_bar(3, "Complete");
    for _ in 0..3 {
        pause();
        completed.inc(1);
    }
    completed.finish_with_message("success");

    let temporary = make_bar(3, "Temporary");
    for _ in 0..3 {
        pause();
        temporary.inc(1);
    }
    temporary.finish_and_clear(); // remove the finished display

    let interrupted = make_bar(10, "Stopped");
    interrupted.inc(3);
    pause();
    interrupted.abandon_with_message("stopped after 3 items");

    // finish() marks a bounded bar complete; it isn't a validation step.
    // Don't call it to disguise an error or unfinished work.
    // abandon leaves the current position instead of claiming completion.
    // These calls manage display state, not cancellation of the actual task.
}

// ============================================================================
// 9. Print messages while a bar is active
// ============================================================================

fn demo_logging() {
    let bar = make_bar(12, "Logging");

    for step in 1..=12 {
        pause();
        bar.inc(1);

        if step % 4 == 0 {
            bar.println(format!("checkpoint after {step} items"));
        }
    }

    bar.suspend(|| {
        eprintln!("Temporarily suspended rendering for ordinary output.");
    });

    bar.finish_with_message("logging demo complete");

    // bar.println cooperates with progress rendering.
    // suspend hides the bar while the closure runs, then restores it.
    // Don't put long-running work inside suspend.
    // Progress-aware messages may also disappear when rendering is hidden;
    // important persistent logs need their own logging/output policy.
}

// ============================================================================
// 10. Multiple / nested progress bars
// ============================================================================

fn demo_multiple() {
    let multi = MultiProgress::new();

    // Add bars before doing anything that might draw them.
    let outer = multi.add(ProgressBar::new(3));
    outer.set_style(standard_style());
    outer.set_prefix("Files");

    for file in 1..=3 {
        let inner = multi.add(ProgressBar::new(5));
        inner.set_style(standard_style());
        inner.set_prefix(format!("File {file}"));

        for _ in 0..5 {
            pause();
            inner.inc(1);
        }

        inner.finish_and_clear();
        multi.remove(&inner);
        outer.inc(1);
    }

    outer.finish_with_message("all files complete");

    // MultiProgress coordinates terminal drawing; it doesn't create threads.
    // For logging with several bars, use multi.println / multi.suspend.
    // Keep handles alive until the associated work has finished.
}

// ============================================================================
// 11. Share one progress bar across threads
// ============================================================================

fn demo_threads() {
    let bar = make_bar(40, "Threads");

    thread::scope(|scope| {
        for _ in 0..4 {
            let worker_bar = bar.clone();

            scope.spawn(move || {
                for _ in 0..10 {
                    pause();
                    worker_bar.inc(1);
                }
            });
        }
    }); // scoped threads have finished before returning

    bar.finish_with_message("40 items completed");

    // Cloning ProgressBar shares its state; it doesn't create another bar.
    // It is Send + Sync and handles synchronized updates internally.
    // Finish once after all workers complete, not independently in each worker.
    // This demo's workers don't fail; real worker errors must be collected.
    // No Rayon dependency is needed here.
}

// ============================================================================
// 12. Byte streams and batch processing
// ============================================================================

fn demo_bytes_batches() -> io::Result<()> {
    // A memory-backed stream keeps this guide independent of external files.
    let data = vec![0u8; 64 * 1024];
    let bar = ProgressBar::new(data.len() as u64);

    bar.set_style(
        ProgressStyle::with_template(
            "{bar:20.cyan/blue} {bytes}/{total_bytes} \
             {bytes_per_sec} ETA {eta}",
        )
        .expect("valid literal template"),
    );

    let mut input = Cursor::new(data);
    let mut buffer = [0u8; 4096];

    let result: io::Result<()> = (|| {
        loop {
            let read = input.read(&mut buffer)?;
            if read == 0 {
                break;
            }

            // Process buffer[..read] here.
            pause();
            bar.inc(read as u64); // actual bytes read, not buffer capacity
        }
        Ok(())
    })();

    match result {
        Ok(()) => bar.finish_with_message("stream read"),
        Err(error) => {
            bar.abandon_with_message("read failed");
            return Err(error);
        }
    }

    // For a real file:
    // let file = std::fs::File::open("input.bin")?;
    // let total = file.metadata()?.len();
    // Use that File as the Read source and total as the bar length.
    // This measures bytes read, not successful processing or durable writes.

    let items: Vec<_> = (0..23).collect();
    let batch_size = 5usize;
    let total_batches = items.len().div_ceil(batch_size);
    let bar = make_bar(total_batches as u64, "Batches");

    for batch in items.chunks(batch_size) {
        pause();
        bar.set_message(format!("{} items", batch.len()));
        bar.inc(1);
    }

    bar.finish_with_message("five batches complete");

    // Progress units here are BATCHES, not individual items.
    // Last batch has three items; ceiling division includes it.
    Ok(())
}

// ============================================================================
// 13. Visibility and refresh frequency
// ============================================================================

fn demo_rendering() {
    let enabled = false;
    let bar = if enabled {
        make_bar(5, "Visible")
    } else {
        ProgressBar::hidden()
    };

    for _ in 0..5 {
        bar.inc(1);
    }
    bar.finish_and_clear();

    let throttled = make_bar(10, "Refresh");
    throttled.set_draw_target(ProgressDrawTarget::stderr_with_hz(10));

    for _ in 0..10 {
        pause();
        throttled.inc(1);
    }
    throttled.finish_with_message("limited drawing frequency");

    // Hidden bars let the same processing code run without progress output.
    // Draw frequency limits rendering, not how often position can be updated.
    // For very fast loops, larger inc(n) batches can reduce update overhead.
    // Steady spinner ticks and draw throttling are different mechanisms.
    // The default stderr target hides bars in non-terminal environments.
    // Output may therefore be absent in redirected logs or some IDE consoles.
}

// ============ //
//    main()    //
// ============ //

fn main() -> Result<(), Box<dyn Error>> {
    println!();

    demo_basic();
    println!("\n============================\n");

    demo_manual();
    println!("\n============================\n");

    demo_iterators();
    println!("\n============================\n");

    demo_spinner();
    println!("\n============================\n");

    demo_styles();
    println!("\n============================\n");

    demo_messages();
    println!("\n============================\n");

    demo_timing();
    println!("\n============================\n");

    demo_completion();
    println!("\n============================\n");

    demo_logging();
    println!("\n============================\n");

    demo_multiple();
    println!("\n============================\n");

    demo_threads();
    println!("\n============================\n");

    demo_bytes_batches()?;
    println!("\n============================\n");

    demo_rendering();

    Ok(())
}

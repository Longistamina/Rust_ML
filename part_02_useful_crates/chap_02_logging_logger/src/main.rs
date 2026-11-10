/*
log + flexi_logger: convenient console and file logging.

1. Initialize once; keep the handle alive
2. Emit messages at the five standard levels
3. Change level filters temporarily
4. Filter individual targets / modules
5. Format messages and include useful context
6. Console threshold versus the primary file output
7. Log errors without confusing logging with error handling
8. Avoid unnecessary expensive work
9. File rotation and retention
10. Flush, shut down, and inspect the files

Setup:
    cargo add log@0.4
    cargo add flexi_logger@0.31.10

Save as src/bin/01_log_flexi_logger.rs and run:
    cargo run --bin 01_log_flexi_logger
    RUST_LOG=trace cargo run --bin 01_log_flexi_logger
    RUST_LOG='info,parser=debug' cargo run --bin 01_log_flexi_logger

Default features include colored terminal formatting.
No env_logger, colored, or chrono dependency is needed in your Cargo.toml.
This guide writes logs into its own new temporary directory and prints its path.
Files are LEFT THERE for review; remove that directory when finished.

Python logger.info(...) -> Rust info!(...) after one-time initialization.
log supplies the macros; flexi_logger handles destinations, filters and formatting.
One global log implementation can be installed per process. Never initialize
again in every demo/library function; use the returned handle for reconfiguration.

Official docs:
https://docs.rs/log/latest/log/
https://docs.rs/flexi_logger/latest/flexi_logger/
https://docs.rs/flexi_logger/latest/flexi_logger/struct.LoggerHandle.html
*/

use flexi_logger::{
    AdaptiveFormat, Cleanup, Criterion, DeferredNow, Duplicate, FileSpec,
    FlexiLoggerError, Logger, LoggerHandle, Naming, WriteMode,
};
use log::{debug, error, info, log_enabled, trace, warn, Level, Record};
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

type DemoResult = Result<(), Box<dyn std::error::Error>>;

fn main() -> DemoResult {
    println!();

    println!("01. Initialize logging once");
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let directory = std::env::temp_dir()
        .join(format!("rust_logging_{}_{stamp}", std::process::id()));
    fs::create_dir(&directory)?;
    let mut handle = init_logging(&directory)?;
    println!("Log directory: {}", directory.display());
    info!("Logger initialized; default filter is debug unless RUST_LOG overrides it");
    // Keep handle alive through all logging work, including worker threads.
    // '_handle' is also fine if no reconfiguration is needed; '_' drops it at once.
    println!("\n============================\n");

    demo_levels();
    println!("\n============================\n");

    demo_thresholds(&mut handle)?;
    println!("\n============================\n");

    demo_targets(&mut handle)?;
    println!("\n============================\n");

    demo_message_formatting();
    println!("\n============================\n");

    demo_console_threshold(&mut handle)?;
    println!("\n============================\n");

    demo_errors(&mut handle)?;
    println!("\n============================\n");

    demo_expensive_work(&mut handle)?;
    println!("\n============================\n");

    demo_rotation(&mut handle)?;
    println!("\n============================\n");

    // Shut down even if file inspection subsequently fails.
    println!("10. Flush, shut down, and review logs");
    handle.flush();
    handle.shutdown();
    show_log_files(&directory)?;
    println!("\nLogs retained at: {}", directory.display());
    // Do not emit more log messages after shutdown. println! still works.
    Ok(())
}

// One reusable setup function: this is the configuration to adapt for your app.
fn init_logging(directory: &Path) -> Result<LoggerHandle, FlexiLoggerError> {
    Logger::try_with_env_or_str("debug")?
        .log_to_file(FileSpec::default()
            .directory(directory)
            .basename("application")
            .suffix("log")
            .suppress_timestamp())
        .duplicate_to_stderr(Duplicate::All)
        .format_for_files(plain_file_format)
        .adaptive_format_for_stderr(AdaptiveFormat::Detailed)
        .write_mode(WriteMode::Direct)
        .rotate(Criterion::Size(2_048), Naming::Numbers, Cleanup::KeepLogFiles(3))
        .cleanup_in_background_thread(false)
        .start()
    // try_with_env_or_str: use RUST_LOG if set, otherwise the supplied fallback.
    // log_to_file selects the main output; duplicate_to_stderr keeps console logs.
    // Console: detailed format, colored on a terminal, plain when redirected.
    // Files: our own plain formatter, without ANSI styling.
    // Direct writes simplify these short demos; buffered/async modes also exist.
    // 2 KiB rotation + 3 archives is deliberately tiny for demonstration.
    // Synchronous cleanup makes file inspection predictable in this guide.
    // Console-only alternative: omit log_to_file/duplicate/rotate and use
    // .log_to_stderr().adaptive_format_for_stderr(AdaptiveFormat::Detailed).
}

// A Loguru-inspired FILE layout. Console formatting is configured separately.
fn plain_file_format(out: &mut dyn Write, now: &mut DeferredNow, record: &Record<'_>) -> io::Result<()> {
    write!(out, "{} | {:<5} | {}:{} [{}] - {}",
        now.format("%Y-%m-%d %H:%M:%S%.3f %:z"),
        record.level(),
        record.module_path().unwrap_or("unknown"),
        record.line().unwrap_or(0),
        record.target(),
        record.args(),
    )
    // The writer adds the record's newline; the formatter needn't add one.
    // DeferredNow supplies the timestamp; no direct chrono import is needed.
    // Include the UTC offset so a local timestamp is less ambiguous.
    // Never invoke info!/debug!/etc. from inside a logging formatter.
}

// 02. Basic logging calls
// ------------------------
fn demo_levels() {
    println!("02. Five levels, from detailed tracing to errors");

    trace!("TRACE: individual parser step");
    debug!("DEBUG: intermediate value is {}", 42);
    info!("INFO: input loaded successfully");
    warn!("WARN: optional configuration missing; using defaults");
    error!("ERROR: demonstration of an operation failure");
    // Threshold order: error < warn < info < debug < trace (increasing verbosity).
    // debug enables debug/info/warn/error, but not trace.
    // These messages follow RUST_LOG or the fallback set in init_logging.
    // log has no built-in SUCCESS or CRITICAL levels: use an appropriate level
    // plus a clear message. Logging error! does not exit, panic, or return Err.
}

// 03. Compare thresholds in ONE process without reinitializing the logger
// ------------------------------------------------------------------------
fn demo_thresholds(handle: &mut LoggerHandle) -> Result<(), FlexiLoggerError> {
    println!("03. Change level thresholds temporarily");

    for level in ["off", "error", "warn", "info", "debug", "trace"] {
        println!("\nThreshold: {level}");
        handle.parse_and_push_temp_spec(level)?;
        trace!("trace example");
        debug!("debug example");
        info!("info example");
        warn!("warning example");
        error!("error example");
        handle.pop_temp_spec();
    }
    // Each temporary spec replaces the active one, then pop restores it.
    // The original RUST_LOG/fallback is restored after the demo.
    // parse_new_spec("info")? replaces it permanently instead.
    // This is process-wide configuration, not an independent per-function logger.
    Ok(())
}

// 04. Targets normally default to the Rust module path
// -----------------------------------------------------
fn demo_targets(handle: &mut LoggerHandle) -> Result<(), FlexiLoggerError> {
    println!("04. Module/target filtering");

    handle.parse_and_push_temp_spec("warn,parser=debug,network=info")?;
    debug!(target: "parser", "Parser details are enabled");
    info!(target: "network", "Connection established");
    debug!(target: "network", "Hidden: network allows info and above");
    info!(target: "other", "Hidden: default threshold is warn");
    warn!(target: "other", "Visible with the default warn threshold");
    handle.pop_temp_spec();
    // target: overrides a record's filtering target, not its source file/module.
    // Without target:, the module path (often binary_name::module) is used.
    // Example in a real app: RUST_LOG='warn,my_app::parser=debug'.
    // A module/target is the closest ordinary analogue to getLogger(__name__).
    Ok(())
}

// 05. Rust formatting and contextual information
// ------------------------------------------------
fn demo_message_formatting() {
    println!("05. Formatted messages and record metadata");

    let filename = "input.csv";
    let rows = 120;
    info!("Loaded {rows} rows from {filename}");
    debug!("First three IDs: {:?}", [10, 20, 30]);
    info!("Training loss={:.3}, accuracy={:.1}%", 0.12345, 97.25);
    // Formatting resembles println!; no intermediate format!(...) is needed.
    // The formatter adds time, level, module, line, and target to every record.
    // Variable text in the message is not automatically structured JSON data.
    // For this setup, no need to add colored styling to individual messages:
    // the console formatter handles it and file records remain plain text.
}

// 06. Global filtering runs BEFORE console duplication filtering
// ----------------------------------------------------------------
fn demo_console_threshold(handle: &mut LoggerHandle) -> Result<(), FlexiLoggerError> {
    println!("06. Detailed files, quieter console");

    handle.parse_and_push_temp_spec("debug")?;
    handle.adapt_duplication_to_stderr(Duplicate::Info)?;
    debug!("FILE ONLY: diagnostic detail");
    info!("FILE + CONSOLE: normal progress");
    warn!("FILE + CONSOLE: warning");
    handle.adapt_duplication_to_stderr(Duplicate::All)?;
    handle.pop_temp_spec();
    // Main file receives everything allowed by the active spec (debug here).
    // Duplicate::Info adds a second threshold for console copies.
    // Console duplication cannot recover messages blocked by the main spec.
    // This simple setup's direction differs from Python's default debug console
    // plus logger.add(file, level='INFO'). Per-writer filters/named writers can
    // implement independent routing; they are beyond this introductory setup.
    Ok(())
}

// 07. Logging describes an error; Result controls what the program does next
// --------------------------------------------------------------------------
fn demo_errors(handle: &mut LoggerHandle) -> Result<(), FlexiLoggerError> {
    println!("07. Log an expected failure and continue");

    handle.parse_and_push_temp_spec("info")?;
    info!("About to parse a worker count");

    println!();

    let input = "many";
    match input.parse::<u16>() {
        Ok(workers) => info!("Configured {workers} workers"),
        Err(cause) => error!("Invalid worker count {input:?}: {cause}"),
    }
    info!("The demo continues after logging the error");
    handle.pop_temp_spec();
    // Use return Err(...), ?, a fallback, or retry when your application needs it.
    // Prefer logging at the boundary that handles/reports an error; logging at
    // every propagation layer can produce several records for one failure.
    Ok(())
}

// 08. Logging arguments can be skipped when their level is disabled
// ------------------------------------------------------------------
fn demo_expensive_work(handle: &mut LoggerHandle) -> Result<(), FlexiLoggerError> {
    println!("08. Expensive diagnostics and side effects");

    handle.parse_and_push_temp_spec("info")?;
    if log_enabled!(Level::Debug) {
        let diagnostic = (0..100).map(|n| n.to_string()).collect::<Vec<_>>().join(",");
        debug!("Detailed state: {diagnostic}");
    }
    info!("Debug-only computation was skipped");
    handle.pop_temp_spec();
    // Guard expensive preparation done OUTSIDE a logging macro with log_enabled!.
    // Expressions directly inside a disabled log macro need not be evaluated.
    // Never rely on a log argument to perform required work or mutate app state.
    // Use log_enabled!(target: "parser", Level::Debug) for an explicit target.
    Ok(())
}

// 09. Demonstrate size-based rotation without flooding the console
// -----------------------------------------------------------------
fn demo_rotation(handle: &mut LoggerHandle) -> Result<(), FlexiLoggerError> {
    println!("09. Rotate small files and retain three archives");

    handle.parse_and_push_temp_spec("info")?;
    handle.adapt_duplication_to_stderr(Duplicate::None)?;
    let payload = "x".repeat(300);
    for index in 0..30 {
        info!("Rotation sample {index:02}: {payload}");
    }
    handle.flush();
    handle.adapt_duplication_to_stderr(Duplicate::All)?;
    handle.pop_temp_spec();
    println!("Wrote 30 sample records to files only.");
    // Criterion::Size uses BYTES; records can push a file beyond the threshold.
    // Naming::Numbers names archives; the active file has a CURRENT infix.
    // Cleanup::KeepLogFiles(3) keeps 3 rotated files PLUS the current file.
    // Older demo records may already have been deleted by the retention policy.
    // Production example resembling rotation='1 MB', retention='10 days':
    // .rotate(Criterion::Size(1_000_000), Naming::Numbers, Cleanup::KeepForDays(10))
    // KeepForDays is an age policy, not 'keep 10 files'. Cleanup happens while
    // the logger runs; it is not a background service after your program exits.
    // This guide uses synchronous cleanup so inspection has no cleanup race.
    Ok(())
}

// 10. The final section is called after the logger is shut down in main
// ---------------------------------------------------------------------
fn show_log_files(directory: &Path) -> io::Result<()> {
    let mut paths: Vec<PathBuf> = fs::read_dir(directory)?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<_, _>>()?;
    paths.sort();
    for path in &paths {
        println!("{}: {} bytes", path.file_name().unwrap_or_default().to_string_lossy(),
            fs::metadata(path)?.len());
    }
    // Print a short preview, rather than all the generated rotation payloads.
    if let Some(path) = paths.first() {
        let text = fs::read_to_string(path)?;
        println!("\nFirst retained file, first line:");
        if let Some(line) = text.lines().next() {
            println!("{line}");
        }
    }
    // flush/shutdown return (), not Result. File-output failures are reported
    // through flexi_logger's error channel; info! itself does not return a Result.
    // These files are retained for you to inspect, not removed by this guide.
    Ok(())
}

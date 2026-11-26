//! Self-spawning: one executable, two operating modes.
//!
//! Save as src/bin/04_self_spawning.rs.
//! Run: cargo run --bin 04_self_spawning
//! No external crates or external command-line programs needed.
//!
//! Normal invocation -> main -> parent demo -> spawn this executable.
//! Worker invocation -> main -> worker branch -> return.
//! Each process has its own main(), memory, and process ID.
//!
//! References:
//! https://doc.rust-lang.org/std/env/fn.current_exe.html
//! https://doc.rust-lang.org/std/process/struct.Command.html

use std::env;
use std::error::Error;
use std::io;
use std::process::{Command, ExitCode};

fn main() -> Result<ExitCode, Box<dyn Error>> {
    println!();

    // BOTH parent and child enter here. Arguments decide which path to take.
    // Keep dispatch above the tutorial demos so a worker doesn't spawn workers
    // recursively. The worker flag is our own protocol, not a Rust feature.
    let mut args = env::args_os().skip(1); // Skip the executable name.
    if args.next().as_deref() == Some(std::ffi::OsStr::new("--worker")) {
        let name = args.next().ok_or_else(|| {
            io::Error::new(io::ErrorKind::InvalidInput, "--worker needs a name")
        })?;
        println!("Worker PID: {}", std::process::id());
        println!("Hello, {}!", name.to_string_lossy());
        return Ok(ExitCode::SUCCESS); // Essential: don't run the parent demo.
    }

    demo_self_spawning()?;
    println!("\n============================\n");

    demo_worker_failure()?;

    Ok(ExitCode::SUCCESS)
}

// =====================================================
// 01. Launch this executable in worker mode
// =====================================================

fn demo_self_spawning() -> io::Result<()> {
    println!("01. Launch this executable in worker mode");

    let executable = env::current_exe()?;
    println!("Parent PID: {}", std::process::id());
    println!("Executable: {}", executable.display());

    let output = Command::new(&executable)
        .args(["--worker", "Rust learner"])
        .output()?;

    println!("Worker status: {}", output.status);
    print!("Captured worker stdout:\n{}", String::from_utf8_lossy(&output.stdout));
    // current_exe returns the currently running compiled binary, not this .rs
    // source file or Cargo. Even under cargo run, it launches the binary directly.
    // The worker enters main again, recognizes --worker, prints, and returns.
    // output waits for that separate process and captures its output.
    // Its first blank line comes from the initial println!() in main.
    // "Rust learner" stays ONE argument despite its space; no shell is involved.
    // Separate process memory means ordinary Rust variables aren't shared.
    // Pass small inputs through arguments; use stdin/pipes/files for larger data.
    Ok(())
}

// =====================================================
// 02. Worker errors belong to the worker process
// =====================================================

fn demo_worker_failure() -> io::Result<()> {
    println!("02. Worker errors belong to the worker process");

    println!();

    let output = Command::new(env::current_exe()?)
        .arg("--worker") // Deliberately omit the required name.
        .output()?;

    println!("Worker successful: {}", output.status.success());
    println!("Worker status: {}", output.status);
    print!("Worker stderr: {}", String::from_utf8_lossy(&output.stderr));
    // The worker's main returns Err, so Rust reports it on stderr and exits with
    // a failure status. That Err does NOT propagate into the parent's memory.
    // The parent's output()? still succeeds: launching and collecting worked.
    // The parent must inspect output.status to detect worker-reported failure.
    // Returning from main allows normal local cleanup, unlike process::exit.
    //
    // This pattern suits subprocess tests and worker modes. It isn't required
    // for ordinary Command usage. A flag is mode selection, not authentication.
    // current_exe isn't a security guarantee: don't use its returned path as
    // proof of executable identity in privileged or security-sensitive code.
    Ok(())
}

//! std::process::Child: observe, wait, stop, and clean up processes.
//!
//! Save as src/bin/03_child_lifecycle.rs.
//! Run: cargo run --bin 03_child_lifecycle
//! No external Rust crates. Linux examples use printf, cat, and sleep.
//! Every demo builds its commands directly; there are no helper functions.
//!
//! Command configures a launch; Child represents that particular process.
//! Dropping Child does NOT automatically kill it or wait for its completion.
//! Keep ownership until completion has been observed or cleanup attempted.
//! Sleeps here model waiting external programs, not CPU benchmarks.
//!
//! Reference: https://doc.rust-lang.org/std/process/struct.Child.html

use std::error::Error;
use std::io::{self, Write};
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn Error>> {
    println!();

    demo_spawn_wait()?;
    println!("\n============================\n");

    demo_try_wait()?;
    println!("\n============================\n");

    demo_wait_with_output()?;
    println!("\n============================\n");

    demo_cooperative_eof()?;
    println!("\n============================\n");

    demo_kill_wait()?;
    println!("\n============================\n");

    demo_manual_timeout()?;
    println!("\n============================\n");

    demo_timeout_capture()?;
    println!("\n============================\n");

    demo_concurrent_children()?;
    println!("\n============================\n");

    demo_cleanup_after_error()?;

    Ok(())
}

// =====================================================
// 01. spawn, id, and wait: own one child's lifecycle
// =====================================================

fn demo_spawn_wait() -> io::Result<()> {
    println!("01. spawn, id, and wait: own one child's lifecycle");

    let mut child = Command::new("sleep").arg("0.1").spawn()?;
    println!("Started PID {}", child.id());
    println!("The parent can perform other work before waiting.");

    let status = child.wait()?;
    println!("Completion: {status}");
    println!("Repeated wait: {}", child.wait()?);
    // sleep 0.1 waits approximately one tenth of a second, then exits.
    // wait blocks the calling thread until completion. A repeated wait returns
    // the recorded status; it doesn't start or wait for a different process.
    // On Unix, observing completion also releases the terminated child's process
    // bookkeeping ('reaping'). Dropping a handle alone doesn't do this.
    // PID values can be reused later; don't treat an old PID as a permanent ID.
    Ok(())
}

// =====================================================
// 02. try_wait: check completion without blocking
// =====================================================

fn demo_try_wait() -> io::Result<()> {
    println!("02. try_wait: check completion without blocking");

    let mut child = Command::new("sleep").arg("0.15").spawn()?;
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                println!("Completion observed: {status}");
                break;
            }
            Ok(None) => {
                println!("No exit status yet; parent can do other work.");
                thread::sleep(Duration::from_millis(50));
            }
            Err(error) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(error);
            }
        }
    }
    // Result<Option<ExitStatus>> has three cases: finished, not finished, error.
    // None is not failure. Scheduling can make even the first check return Some.
    // Sleep between checks to avoid a CPU-burning busy loop. try_wait itself
    // doesn't block, but thread::sleep does: this remains synchronous Rust.
    // Some(status) already observes completion; another wait isn't necessary.
    // Unlike wait, try_wait does not close a stdin handle stored inside Child.
    Ok(())
}

// =====================================================
// 03. wait_with_output: finish and collect captured bytes
// =====================================================

fn demo_wait_with_output() -> io::Result<()> {
    println!("03. wait_with_output: finish and collect captured bytes");

    let child = Command::new("printf")
        .args(["%s\n", "finished computation", "result=42"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    println!("Started PID {}", child.id());

    let output = child.wait_with_output()?;
    println!("Status: {}", output.status);
    print!("Output:\n{}", String::from_utf8_lossy(&output.stdout));
    // Unlike wait(&mut self), wait_with_output(self) CONSUMES the Child handle.
    // You can't use child afterward. It drains the captured streams while waiting.
    // spawn inherits streams by default, so request piped output before spawning.
    // output() from file 01 is the convenient launch-and-collect version.
    // Don't wait() first on a child whose undrained output pipes could fill.
    // Capturing retains bytes in memory; stream large output as in file 02.
    Ok(())
}

// =====================================================
// 04. Cooperative completion: close stdin to send EOF
// =====================================================

fn demo_cooperative_eof() -> io::Result<()> {
    println!("04. Cooperative completion: close stdin to send EOF");

    let mut child = Command::new("cat")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;
    let mut input = child.stdin.take().expect("piped stdin");
    let write_result = input.write_all(b"finish after reading this\n");

    drop(input); // cat reads EOF and exits normally after copying the input.
    let output_result = child.wait_with_output();
    write_result?;
    let output = output_result?;
    println!("Status: {}", output.status);
    println!("Copied: {:?}", String::from_utf8_lossy(&output.stdout));
    // This is cooperative because cat's input protocol defines EOF as completion.
    // It is NOT a universal 'stop' message for every program. Other tools may
    // require a command, socket message, or another documented shutdown protocol.
    // flush doesn't close stdin. wait closes stdin still stored in Child, but
    // cannot close a handle you moved out with take; you must drop that handle.
    // The small fixed input here avoids the large-input issue covered in file 02.
    Ok(())
}

// =====================================================
// 05. kill followed by wait: terminate and observe completion
// =====================================================

fn demo_kill_wait() -> io::Result<()> {
    println!("05. kill followed by wait: terminate and observe completion");

    // cat waits for input while we keep its piped stdin open inside Child.
    let mut child = Command::new("cat")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()?;
    println!("Stopping PID {}", child.id());

    println!();

    let kill_result = child.kill();
    let wait_result = child.wait(); // Attempt cleanup even if kill returned Err.
    kill_result?;
    let status = wait_result?;
    println!("After termination: {status}; numeric code={:?}", status.code());

    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        println!("Unix termination signal: {:?}", status.signal());
    }
    // kill requests forced termination; wait observes completion afterward.
    // On Unix, kill uses SIGKILL: the child can't perform graceful cleanup.
    // Signal termination may have code() == None, rather than a numeric code.
    // std's stable portable Child API doesn't offer arbitrary signal sending.
    // OS-specific APIs/crates are needed for signals such as Unix SIGTERM.
    // kill targets this process, not an entire descendant process tree.
    // Launching a tool directly avoids an extra shell process in between.
    Ok(())
}

// =====================================================
// 06. Manual timeout: deadline, try_wait, kill, and wait
// =====================================================

fn demo_manual_timeout() -> io::Result<()> {
    println!("06. Manual timeout: deadline, try_wait, kill, and wait");

    let mut child = Command::new("cat")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()?;
    let deadline = Instant::now() + Duration::from_millis(150);

    println!();

    let (timed_out, status) = loop {
        match child.try_wait() {
            Ok(Some(status)) => break (false, status),
            Ok(None) => {}
            Err(error) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(error);
            }
        }
        if Instant::now() >= deadline {
            let kill_result = child.kill();
            let wait_result = child.wait();
            kill_result?;
            break (true, wait_result?);
        }
        thread::sleep(Duration::from_millis(20));
    };
    println!("Deadline policy triggered: {timed_out}; status: {status}");
    // cat can't complete normally while its stdin stays open and no data arrives.
    // Stable std Child has no wait_timeout method. This loop implements a policy.
    // Instant is a monotonic clock for elapsed time; wall-clock time isn't needed.
    // Polling/scheduling and termination add delay: 150 ms is not a hard bound.
    // A process can exit between try_wait and kill, so 'timed out' means our
    // deadline policy triggered, not proof of the exact instant the child exited.
    // Without kill/another shutdown action, a timeout only stops YOUR waiting.
    Ok(())
}

// =====================================================
// 07. Timeout with capture: drain output while polling
// =====================================================

fn demo_timeout_capture() -> io::Result<()> {
    println!("07. Timeout with capture: drain output while polling");

    let mut child = Command::new("cat")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let mut stdout = child.stdout.take().expect("piped stdout");
    let mut stderr = child.stderr.take().expect("piped stderr");
    let out_reader = thread::spawn(move || {
        let mut bytes = Vec::new();
        use std::io::Read;
        stdout.read_to_end(&mut bytes).map(|_| bytes)
    });
    let err_reader = thread::spawn(move || {
        let mut bytes = Vec::new();
        use std::io::Read;
        stderr.read_to_end(&mut bytes).map(|_| bytes)
    });
    let deadline = Instant::now() + Duration::from_millis(100);

    println!();

    // Store errors until the readers have been joined, rather than returning
    // immediately and abandoning their handles. No helper function is involved.
    let completion = loop {
        match child.try_wait() {
            Ok(Some(status)) => break Ok(status),
            Ok(None) if Instant::now() < deadline => {
                thread::sleep(Duration::from_millis(20));
            }
            Ok(None) => {
                let kill_result = child.kill();
                let wait_result = child.wait();
                break kill_result.and(wait_result);
            }
            Err(error) => {
                let _ = child.kill();
                let _ = child.wait();
                break Err(error);
            }
        }
    };
    let out_result = out_reader.join()
        .map_err(|_| io::Error::other("stdout reader panicked"));
    let err_result = err_reader.join()
        .map_err(|_| io::Error::other("stderr reader panicked"));
    let status = completion?;
    let stdout = out_result??;
    let stderr = err_result??;
    println!("Completion: {status}; stdout={} bytes, stderr={} bytes", stdout.len(), stderr.len());
    // Each ?? handles the thread join result, then the reader's I/O result.
    // Both readers drain concurrently, preventing full output pipes from blocking
    // the child while the parent polls. This cat receives no input, so both are empty.
    // Reader buffers can still grow without bound: use files/streaming when needed.
    // This isn't a universal hard-timeout runner: inherited pipe handles held by
    // descendants can delay EOF, and readers then keep waiting after this child exits.
    Ok(())
}

// =====================================================
// 08. Concurrent children: spawn all, then wait for each
// =====================================================

fn demo_concurrent_children() -> io::Result<()> {
    println!("08. Concurrent children: spawn all, then wait for each");

    let started = Instant::now();
    let mut children: Vec<Child> = Vec::new();
    for delay in ["0.15", "0.05", "0.10"] {
        match Command::new("sleep").arg(delay).spawn() {
            Ok(child) => {
                println!("Started sleep {delay}: PID {}", child.id());
                children.push(child);
            }
            Err(error) => {
                for child in &mut children {
                    let _ = child.kill();
                    let _ = child.wait();
                }
                return Err(error);
            }
        }
    }

    let mut first_error = None;
    for child in &mut children {
        match child.wait() {
            Ok(status) => println!("Observed PID {}: {status}", child.id()),
            Err(error) => {
                let _ = child.kill();
                let _ = child.wait();
                first_error.get_or_insert(error);
            }
        }
    }
    println!("Elapsed: {:?}", started.elapsed());
    if let Some(error) = first_error {
        return Err(error);
    }
    // All processes were launched before the waiting loop: they run concurrently.
    // Waiting in vector order affects when WE report results, not when they run.
    // This is not completion order, and it doesn't require one Rust thread per child.
    // For many jobs, limit how many children are active instead of spawning all.
    // These children emit no output; arbitrary piped-output children need concurrent
    // draining. Waiting sequentially with undrained pipes can otherwise deadlock.
    Ok(())
}

// =====================================================
// 09. Error paths: clean up children already started
// =====================================================

fn demo_cleanup_after_error() -> io::Result<()> {
    println!("09. Error paths: clean up children already started");

    let mut first = Command::new("cat")
        .stdin(Stdio::piped())
        .stdout(Stdio::null())
        .spawn()?;

    println!();

    let second_result = Command::new("/this/path/does/not/exist/demo-program").spawn();
    match second_result {
        Err(error) => {
            println!("Expected second launch error: {error}");
        }
        Ok(mut unexpected) => {
            let kill_result = unexpected.kill();
            let wait_result = unexpected.wait();
            println!("Unexpected second child cleanup: kill={kill_result:?}, wait={wait_result:?}");
        }
    }

    let kill_result = first.kill();
    let wait_result = first.wait();
    println!("First child cleanup: kill={kill_result:?}, wait={wait_result:?}");
    kill_result?;
    wait_result?;
    // Writing second.spawn()? immediately after first.spawn()? would return early
    // on failure, dropping first's handle without terminating/waiting for it.
    // Plan cleanup before using ? on later operations that could fail.
    // let _ = ... in earlier error branches is best-effort cleanup that preserves
    // the original error; production tools may also need to report cleanup errors.
    // Forced termination can fail; no example can guarantee cleanup after every
    // OS error or parent crash. Process groups/job objects are separate OS topics.
    // Tokio users should use tokio::process and an explicit cancellation policy;
    // dropping an async waiting future alone is not a general process-stop command.
    Ok(())
}

//! std::process: stdin, stdout, stderr, redirection, and pipelines.
//!
//! No external crates. Save as src/bin/02_stdio_pipelines.rs.
//! Run: cargo run --bin 02_stdio_pipelines
//!
//! stdin = input read by a program; stdout = normal output;
//! stderr = diagnostics. Each stream is configured independently.
//! The parent WRITES ChildStdin and READS ChildStdout / ChildStderr.
//!
//! Linux examples: printf, cat, wc, tr, and sh must be available on PATH.
//! Every demo constructs its commands directly: no self-launching or helpers.
//! Fixed shell scripts are used only when a demo needs two output streams or
//! a deliberate exit code. Ordinary input/output pipes use Stdio directly.
//! One demo creates a unique temporary directory, then removes its own files.
//!
//! References:
//! https://doc.rust-lang.org/std/process/struct.Stdio.html
//! https://doc.rust-lang.org/std/process/struct.Child.html
//! https://doc.rust-lang.org/std/io/trait.BufRead.html

use std::env;
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::process::{Command, Stdio};
use std::thread;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() -> Result<(), Box<dyn Error>> {
    println!();

    demo_inherit()?;
    println!("\n============================\n");

    demo_null()?;
    println!("\n============================\n");

    demo_capture()?;
    println!("\n============================\n");

    demo_stdin_eof()?;
    println!("\n============================\n");

    demo_large_input()?;
    println!("\n============================\n");

    demo_file_redirection()?;
    println!("\n============================\n");

    demo_read_lines()?;
    println!("\n============================\n");

    demo_drain_both()?;
    println!("\n============================\n");

    demo_pipeline()?;
    println!("\n============================\n");

    demo_pipeline_failure()?;

    Ok(())
}

// =====================================================
// 01. inherit(): connect directly to the parent's streams
// =====================================================

fn demo_inherit() -> io::Result<()> {
    println!("01. inherit(): connect directly to the parent's streams");

    let status = Command::new("sh")
        .args(["-c", "printf 'normal output\\n'; printf 'diagnostic output\\n' >&2"])
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()?;
    println!("Status: {status}");
    // The fixed script runs two printf commands; >&2 directs the second to
    // stderr. The shell is used only to generate both streams for this demo.
    // inherit means the same destination as the parent, which could itself be
    // a terminal, file, or pipe. It does not necessarily mean a terminal.
    // spawn/status inherit all three streams by default. We set them explicitly
    // here, and use null stdin so this tutorial never waits for keyboard input.
    Ok(())
}

// =====================================================
// 02. null(): discard output or provide immediate EOF
// =====================================================

fn demo_null() -> io::Result<()> {
    println!("02. null(): discard output or provide immediate EOF");

    let status = Command::new("sh")
        .args(["-c", "printf 'normal output\\n'; printf 'diagnostic output\\n' >&2"])
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()?;
    println!("Silent child status: {status}");

    let output = Command::new("wc").arg("-c")
        .stdin(Stdio::null())
        .output()?;
    println!("Empty input: {}", String::from_utf8_lossy(&output.stdout).trim());
    // wc -c reads stdin and prints the byte count: 0 here.
    // null stdout/stderr discard bytes. null stdin reads as EOF immediately.
    // This is portable: no hard-coded /dev/null or Windows NUL path needed.
    Ok(())
}

// =====================================================
// 03. output(): capture both streams, or configure just one
// =====================================================

fn demo_capture() -> io::Result<()> {
    println!("03. output(): capture both streams, or configure just one");

    let output = Command::new("sh")
        .args(["-c", "printf 'normal output\\n'; printf 'diagnostic output\\n' >&2"]).output()?;
    println!("stdout: {:?}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {:?}", String::from_utf8_lossy(&output.stderr));
    println!("Status: {}", output.status);
    // output defaults: stdin closed, stdout/stderr piped and collected as bytes.
    // It drains both output pipes while waiting. Capturing still uses memory
    // proportional to the output size; it isn't an unlimited logging solution.

    println!();

    let output = Command::new("sh")
        .args(["-c", "printf 'normal output\\n'; printf 'diagnostic output\\n' >&2"])
        .stderr(Stdio::inherit())
        .output()?;
    println!("Captured stdout: {:?}", String::from_utf8_lossy(&output.stdout));
    println!("Captured stderr length: {}", output.stderr.len()); // 0
    // Explicit stream settings override output's defaults. stderr was displayed
    // directly, so Output.stderr is empty. stdout/stderr have no combined order
    // when captured separately; printing them later cannot recover that order.
    Ok(())
}

// =====================================================
// 04. piped stdin: write input, then close it to send EOF
// =====================================================

fn demo_stdin_eof() -> io::Result<()> {
    println!("04. piped stdin: write input, then close it to send EOF");

    let mut child = Command::new("wc").arg("-c")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    let mut input = child.stdin.take().expect("stdin was configured as piped");
    let write_result = input.write_all(b"Rust\nPython\n");
    drop(input); // Close the pipe: wc can finish counting and exit.
    let output_result = child.wait_with_output();
    write_result?;
    let output = output_result?;
    println!("{}", String::from_utf8_lossy(&output.stdout).trim()); // 12 bytes
    println!("Status: {}", output.status);
    // take moves the handle out of Option and leaves None in child.stdin.
    // flush only pushes buffered data; it DOES NOT close a pipe or send EOF.
    // wait/wait_with_output close stdin still stored in Child, but cannot close
    // a handle you took out. Keeping input alive while waiting could hang here.
    // wc -c counts input bytes, then prints the total at EOF.
    // For arbitrary programs or large data, use concurrent writing and reading.
    Ok(())
}

// =====================================================
// 05. Large input: write while output is being drained
// =====================================================

fn demo_large_input() -> io::Result<()> {
    println!("05. Large input: write while output is being drained");

    let payload = vec![b'x'; 512 * 1024];
    let mut child = Command::new("cat")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let mut input = child.stdin.take().expect("piped stdin");

    let writer = thread::spawn(move || input.write_all(&payload));
    let output_result = child.wait_with_output();
    let writer_result = writer.join()
        .map_err(|_| io::Error::other("stdin writer panicked"));
    let output = output_result?;
    writer_result??;
    println!("Echoed bytes: {}", output.stdout.len());
    println!("Status: {}", output.status);
    // cat copies its stdin bytes to stdout, without adding a newline.
    // The writer owns input and drops it when the closure finishes: EOF.
    // join returns a thread Result containing the write_all I/O Result;
    // the two ? operators above unwrap those two distinct error layers.
    // Meanwhile wait_with_output drains stdout/stderr. Writing all input first
    // could deadlock: parent waits for free stdin space, child for stdout space.
    // This is the std equivalent of the I/O coordination behind communicate().
    // We still collect output in memory here; streaming is demonstrated below.
    Ok(())
}

// =====================================================
// 06. Files: input, overwrite, append, and separate stderr
// =====================================================

fn demo_file_redirection() -> io::Result<()> {
    println!("06. Files: input, overwrite, append, and separate stderr");

    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)
        .map_err(io::Error::other)?.as_nanos();
    let directory = env::temp_dir().join(format!("rust_stdio_{}_{stamp}", std::process::id()));
    fs::create_dir(&directory)?; // Fail instead of reusing an existing directory.

    let input_path = directory.join("input.txt");
    let output_path = directory.join("output.txt");
    let error_path = directory.join("stderr.txt");
    fs::write(&input_path, "from a file\n")?;

    let status = Command::new("cat")
        .stdin(File::open(&input_path)?)
        .stdout(File::create(&output_path)?)
        .stderr(File::create(&error_path)?)
        .status()?;
    println!("Redirected child status: {status}");
    // File::create truncates an existing file (like shell >).
    // File::open is read-only (like shell <). File converts into Stdio.

    let append_file = OpenOptions::new().create(true).append(true).open(&output_path)?;
    let status = Command::new("sh")
        .args(["-c", "printf 'normal output\\n'; printf 'diagnostic output\\n' >&2"])
        .stdout(append_file)
        .stderr(Stdio::null())
        .status()?;
    println!("Append child status: {status}");
    println!("File contents: {:?}", fs::read_to_string(&output_path)?);
    println!("stderr file size: {}", fs::metadata(&error_path)?.len());
    // append(true) is like >>. File destinations avoid capturing all output
    // in parent memory. stderr can have an entirely different destination.
    fs::remove_dir_all(&directory)?;
    // If an earlier operation fails, the uniquely named directory may remain
    // in the system temporary folder for inspection.
    Ok(())
}

// =====================================================
// 07. Read stdout incrementally with BufReader::lines
// =====================================================

fn demo_read_lines() -> io::Result<()> {
    println!("07. Read stdout incrementally with BufReader::lines");

    let mut child = Command::new("printf").args(["%s\n", "rust", "python", "pipelines"])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;
    let stdout = child.stdout.take().expect("piped stdout");

    for line in BufReader::new(stdout).lines() {
        match line {
            Ok(text) => println!("Received line: {text:?}"),
            Err(error) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(error);
            }
        }
    }
    println!("Status: {}", child.wait()?);
    // lines removes newline endings and requires UTF-8 text. It waits until a
    // newline or EOF; it is unsuitable for binary data or \r-only progress bars.
    // Use Read::read / io::copy for bytes. Parent buffering cannot force a child
    // to flush; programs sometimes buffer more when stdout is a pipe.
    // printf emits three lines and exits quickly. This loop processes lines as
    // they become available, but these short lines may all arrive together.
    Ok(())
}

// =====================================================
// 08. Drain stdout and stderr concurrently
// =====================================================

fn demo_drain_both() -> io::Result<()> {
    println!("08. Drain stdout and stderr concurrently");

    let mut child = Command::new("sh")
        .args(["-c", "i=0; while [ \"$i\" -lt 4096 ]; do printf 'stdout message\\n'; printf 'stderr message\\n' >&2; i=$((i + 1)); done"])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let mut stdout = child.stdout.take().expect("piped stdout");
    let mut stderr = child.stderr.take().expect("piped stderr");

    let out_reader = thread::spawn(move || io::copy(&mut stdout, &mut io::sink()));
    let err_reader = thread::spawn(move || io::copy(&mut stderr, &mut io::sink()));
    let status_result = child.wait();
    let out_result = out_reader.join()
        .map_err(|_| io::Error::other("stdout reader panicked"));
    let err_result = err_reader.join()
        .map_err(|_| io::Error::other("stderr reader panicked"));
    println!("Status: {}", status_result?);
    println!("stdout bytes: {}, stderr bytes: {}", out_result??, err_result??);
    // As in demo 05, each ?? handles the thread result and then its I/O result.
    // The fixed sh script repeats stdout/stderr messages 4096 times. sh's >&2
    // routes that printf to stderr; the arithmetic and loop are shell syntax.
    // Each reader streams into a sink, counting bytes without retaining them.
    // Real readers could stream to separate files or process data incrementally.
    // Reading stdout to EOF BEFORE reading stderr can deadlock if stderr fills.
    // Waiting BEFORE draining either pipe has the same problem. Merely taking
    // the handles is insufficient: some thread must actually keep reading them.
    Ok(())
}

// =====================================================
// 09. Pipeline: producer stdout becomes consumer stdin
// =====================================================

fn demo_pipeline() -> io::Result<()> {
    println!("09. Pipeline: producer stdout becomes consumer stdin");

    let mut producer = Command::new("printf").args(["%s\n", "rust", "python", "pipelines"])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()?;
    let pipe = producer.stdout.take().expect("piped producer stdout");

    // Prepare the consumer without retaining its configured Command afterward:
    // the builder owns a pipe read handle too, until it is dropped.
    let consumer_result = Command::new("tr")
        .args(["a-z", "A-Z"])
        .stdin(Stdio::from(pipe))
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn();
    let consumer = match consumer_result {
        Ok(child) => child,
        Err(error) => {
            let _ = producer.kill();
            let _ = producer.wait();
            return Err(error);
        }
    };

    let output_result = consumer.wait_with_output();
    let producer_status = producer.wait();
    let output = output_result?;
    println!("Consumer output: {:?}", String::from_utf8_lossy(&output.stdout));
    println!("Producer: {}; consumer: {}", producer_status?, output.status);
    // Like `producer | consumer`, but no shell parses anything. The OS moves
    // bytes through the pipe; parent doesn't have to relay them manually.
    // printf supplies three lines; tr translates lowercase ASCII to uppercase.
    // Both children run concurrently. Waiting for producer before STARTING the
    // consumer could deadlock once the pipe fills. Check BOTH exit statuses.
    Ok(())
}

// =====================================================
// 10. A successful consumer does not prove pipeline success
// =====================================================

fn demo_pipeline_failure() -> io::Result<()> {
    println!("10. A successful consumer does not prove pipeline success");

    // Expected failure case, separated from the section title.
    println!();

    let mut producer = Command::new("sh")
        .args(["-c", "printf 'producer failed deliberately\\n' >&2; exit 7"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;
    let pipe = producer.stdout.take().expect("piped producer stdout");
    let output_result = Command::new("wc").arg("-c")
        .stdin(Stdio::from(pipe))
        .output();
    // This producer emits only a tiny fixed diagnostic, so waiting to drain its
    // stderr here is safe. For arbitrary producers, drain stderr concurrently.
    let producer_result = producer.wait_with_output();
    let output = output_result?;
    let upstream = producer_result?;
    println!("Producer: {}; consumer: {}", upstream.status, output.status);
    println!("Diagnostic: {}", String::from_utf8_lossy(&upstream.stderr).trim());
    println!("Whole pipeline successful: {}", upstream.status.success() && output.status.success());
    // Here sh intentionally exits 7, while wc successfully counts zero bytes.
    // std does not apply shell pipefail rules or turn nonzero statuses into
    // io::Error. Your application decides which stage statuses are acceptable.
    // Early consumer exit may also make producer writes fail (BrokenPipe, or
    // platform/signal-dependent behavior). Don't assume one portable exit code.
    Ok(())
}


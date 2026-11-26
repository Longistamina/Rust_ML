//! std::process::Command: straightforward examples using Linux commands.
//!
//! Save as src/bin/01_command_basics.rs.
//! Run: cargo run --bin 01_command_basics
//! No external Rust crates. Examples use common Linux programs:
//! printf, pwd, printenv, env, true, false, and sh.
//! The minimal-environment example uses the Linux path /usr/bin/env.
//! These command names/options are not intended as Windows examples.
//!
//! Command::new(...) builds a configuration. It does not start anything yet.
//! status(): launch, wait, return ExitStatus; normally inherit terminal I/O.
//! output(): launch, wait, return status plus captured stdout/stderr bytes.
//! spawn(): launch and return a Child handle; wait explicitly later.
//!
//! Every demo contains its complete command. There is no self-launching setup,
//! child-mode dispatcher, or shared command-construction helper.
//!
//! Reference: https://doc.rust-lang.org/std/process/struct.Command.html

use std::env;
use std::error::Error;
use std::io;
use std::process::Command;

fn main() -> Result<(), Box<dyn Error>> {
    println!();

    demo_status()?;
    println!("\n============================\n");

    demo_output()?;
    println!("\n============================\n");

    demo_arguments()?;
    println!("\n============================\n");

    demo_exit_status()?;
    println!("\n============================\n");

    demo_launch_error();
    println!("\n============================\n");

    demo_decode_output()?;
    println!("\n============================\n");

    demo_child_directory()?;
    println!("\n============================\n");

    demo_child_environment()?;
    println!("\n============================\n");

    demo_builder_reuse()?;
    println!("\n============================\n");

    demo_explicit_shell()?;
    println!("\n============================\n");

    demo_spawn_preview()?;

    Ok(())
}

// =====================================================
// 01. status(): run a program and wait for completion
// =====================================================

fn demo_status() -> io::Result<()> {
    println!("01. status(): run a program and wait for completion");

    let status = Command::new("printf")
        .arg("Hello from a separate process!\n")
        .status()?;

    println!("Returned status: {status}");
    println!("Successful: {}", status.success());
    // Runs the external printf program, not Rust's println! macro.
    // Child output goes directly to the parent's stdout destination.
    // status() blocks until completion and returns no captured output bytes.
    // Python analogy: subprocess.run(["printf", "Hello...\n"]).
    // An executable name without a path normally uses PATH lookup.
    Ok(())
}

// =====================================================
// 02. output(): capture stdout and stderr
// =====================================================

fn demo_output() -> io::Result<()> {
    println!("02. output(): capture stdout and stderr");

    let output = Command::new("printf")
        .args(["%s\n", "Rust", "Python"])
        .output()?;

    println!("Status: {}", output.status);
    println!("stdout bytes: {:?}", output.stdout);
    println!("stdout text: {}", String::from_utf8_lossy(&output.stdout));
    println!("stderr text: {:?}", String::from_utf8_lossy(&output.stderr));
    // printf's %s prints a string; \n prints a newline. The format repeats for
    // the remaining arguments, producing two lines. stderr is empty here.
    // Output contains status, stdout: Vec<u8>, stderr: Vec<u8>.
    // Captured output is not displayed until WE print it.
    // Python analogy: subprocess.run(args, capture_output=True).
    // output() closes stdin by default and collects both output streams.
    // It buffers captured output in memory; streaming follows in file 02.
    Ok(())
}

// =====================================================
// 03. arg and args: pass separate argument values
// =====================================================

fn demo_arguments() -> io::Result<()> {
    println!("03. arg and args: pass separate argument values");

    let output = Command::new("printf")
        .arg("<%s>\n")
        .arg("file with spaces.txt")
        .args(["hello world", "*.rs", "$HOME", "A|B"])
        .output()?;

    print!("{}", String::from_utf8_lossy(&output.stdout));
    // Each value is ONE argument. Spaces stay inside their argument.
    // No shell expands *.rs or $HOME, and A|B is text, not a pipeline.
    // Do not add extra quote characters around "file with spaces.txt".
    // Command::new("printf hello") would search for that entire executable name.
    // Use Command::new("printf").arg("hello") instead.
    // Rust passes arguments; the target program still interprets its own options
    // and syntax. Here printf interprets the FIRST argument as a format string,
    // so variable text belongs in the following %s arguments.
    Ok(())
}

// =====================================================
// 04. Exit status: a child can run successfully and report failure
// =====================================================

fn demo_exit_status() -> io::Result<()> {
    println!("04. Exit status: a child can run successfully and report failure");

    let status = Command::new("true").status()?;
    println!("true: code={:?}, success={}", status.code(), status.success());
    // The true program exits with status 0 without printing anything.

    println!();

    let status = Command::new("false").status()?;
    println!("false: code={:?}, success={}", status.code(), status.success());
    if !status.success() {
        println!("The program ran, but returned an unsuccessful exit status.");
    }
    // false deliberately exits unsuccessfully. The ? did NOT return an error:
    // status() returned Ok(ExitStatus). Check success() yourself.
    // std Command has no check=True switch like Python subprocess.run.
    // code() is Option<i32>: Unix signal termination can produce None.
    // Nonzero exit meanings depend on the program; inspect its documentation.
    Ok(())
}

// =====================================================
// 05. Launch errors: the program could not be started
// =====================================================

fn demo_launch_error() {
    println!("05. Launch errors: the program could not be started");

    println!();

    match Command::new("/this/path/does/not/exist/demo-program").output() {
        Ok(output) => println!("Unexpected launched process: {}", output.status),
        Err(error) => println!("Expected launch error: {:?}: {error}", error.kind()),
    }
    // Here there is no child exit status: starting the program failed.
    // Other causes include missing permissions or an invalid current_dir.
    // Compare with false above: that program DID start and returned a status.
}

// =====================================================
// 06. Decode captured bytes explicitly
// =====================================================

fn demo_decode_output() -> Result<(), Box<dyn Error>> {
    println!("06. Decode captured bytes explicitly");

    let output = Command::new("printf")
        .args(["%s\n", "Rust", "Python"])
        .output()?;
    let text = String::from_utf8(output.stdout)?;

    println!("Strict UTF-8: {text:?}");
    for line in text.lines() {
        println!("Parsed line: {line}");
    }
    // This reads lines AFTER the child finishes, not as live output arrives.
    // Command does not automatically decode output using the system locale.

    println!();

    let output = Command::new("printf")
        .arg("\\377") // printf interprets this octal escape as byte 0xFF.
        .output()?;
    println!("Raw bytes: {:?}", output.stdout);
    println!("Lossy UTF-8: {:?}", String::from_utf8_lossy(&output.stdout));
    match String::from_utf8(output.stdout) {
        Ok(text) => println!("Unexpected valid UTF-8: {text:?}"),
        Err(error) => println!("Expected decoding error: {error}"),
    }
    // Lossy decoding replaces invalid sequences. Keep Vec<u8> when exact bytes
    // matter. Paths/arguments can use Path/OsStr instead of forcing UTF-8 too.
    Ok(())
}

// =====================================================
// 07. current_dir: choose the child's working directory
// =====================================================

fn demo_child_directory() -> io::Result<()> {
    println!("07. current_dir: choose the child's working directory");

    let parent_directory = env::current_dir()?;
    let requested = env::temp_dir().canonicalize()?;

    let output = Command::new("pwd")
        .arg("-P")
        .current_dir(&requested)
        .output()?;

    println!("Requested child directory: {}", requested.display());
    print!("pwd reported: {}", String::from_utf8_lossy(&output.stdout));
    println!("Parent directory: {}", env::current_dir()?.display());
    println!("Parent unchanged: {}", parent_directory == env::current_dir()?);
    // pwd -P prints the physical working directory of the child.
    // current_dir does not change the parent's directory.
    // Relative file arguments normally resolve from the child's directory.
    // A relative EXECUTABLE PATH plus current_dir has platform-specific
    // ambiguity; resolve such executable paths to absolute paths beforehand.
    Ok(())
}

// =====================================================
// 08. env, envs, env_remove, and env_clear
// =====================================================

fn demo_child_environment() -> io::Result<()> {
    println!("08. env, envs, env_remove, and env_clear");

    let before = env::var_os("COMMAND_DEMO_VALUE");
    let output = Command::new("printenv")
        .args(["COMMAND_DEMO_VALUE", "COMMAND_DEMO_MODE"])
        .env("COMMAND_DEMO_VALUE", "child only")
        .envs([("COMMAND_DEMO_MODE", "learning")])
        .output()?;
    print!("Child values:\n{}", String::from_utf8_lossy(&output.stdout));
    println!("Parent unchanged: {}", before == env::var_os("COMMAND_DEMO_VALUE"));
    // Children normally inherit the environment. env/envs add or override keys.
    // envs does not replace the whole environment like Python run(env=mapping).
    // printenv is the external Linux program that prints selected variables.

    println!();

    let output = Command::new("printenv")
        .arg("COMMAND_DEMO_VALUE")
        .env("COMMAND_DEMO_VALUE", "temporary override")
        .env_remove("COMMAND_DEMO_VALUE")
        .output()?;
    println!("Removed variable: stdout={:?}, status={}",
        String::from_utf8_lossy(&output.stdout), output.status);
    // printenv returns a nonzero status when the requested variable is absent.
    // env_remove also prevents inheritance of that variable from the parent.

    println!();

    let output = Command::new("/usr/bin/env")
        .env_clear()
        .env("COMMAND_DEMO_VALUE", "minimal environment")
        .output()?;
    print!("Entire minimal environment:\n{}", String::from_utf8_lossy(&output.stdout));
    // env prints the environment. With env_clear, only our explicit key remains.
    // Use an absolute executable path here since PATH inheritance is disabled.
    // This changes only the child; no env::set_var/remove_var is needed.
    Ok(())
}

// =====================================================
// 09. Inspect and reuse a Command builder
// =====================================================

fn demo_builder_reuse() -> io::Result<()> {
    println!("09. Inspect and reuse a Command builder");

    let mut command = Command::new("printf");
    command.args(["<%s>\n", "first"]);

    println!("Program: {:?}", command.get_program());
    println!("Arguments: {:?}", command.get_args().collect::<Vec<_>>());
    let output = command.output()?;
    print!("First launch:\n{}", String::from_utf8_lossy(&output.stdout));

    command.arg("second");
    let output = command.output()?;
    print!("Second launch:\n{}", String::from_utf8_lossy(&output.stdout));
    // arg/args append, so the second child prints BOTH first and second.
    // The configuration survives each launch; build a new Command to start fresh.
    // Each output/status/spawn launches a new process. Inspection methods do not
    // produce a guaranteed shell-ready command string.
    Ok(())
}

// =====================================================
// 10. Shell syntax requires explicitly launching a shell
// =====================================================

fn demo_explicit_shell() -> io::Result<()> {
    println!("10. Shell syntax requires explicitly launching a shell");

    let output = Command::new("sh")
        .args(["-c", "printf 'normal output\\n'; printf 'diagnostic output\\n' >&2"])
        .output()?;

    println!("stdout: {:?}", String::from_utf8_lossy(&output.stdout));
    println!("stderr: {:?}", String::from_utf8_lossy(&output.stderr));
    println!("Shell status: {}", output.status);
    // sh -c executes the following argument as a shell script. In that script,
    // ; separates commands, and >&2 sends the second printf's output to stderr.
    // Rust itself does not interpret these symbols. They work because sh does.
    // Use fixed scripts here; don't interpolate arbitrary user text into scripts.
    // For ordinary programs, pass arguments directly as in earlier sections.
    // File 02 shows pipes/redirection through Stdio without shell syntax.
    Ok(())
}

// =====================================================
// 11. spawn(): start now, wait explicitly later
// =====================================================

fn demo_spawn_preview() -> io::Result<()> {
    println!("11. spawn(): start now, wait explicitly later");

    let mut child = Command::new("printf")
        .args(["%s\n", "Hello from the spawned process!"])
        .spawn()?;

    println!("Child PID: {}", child.id());
    println!("The parent can perform other work here.");

    let status = child.wait()?;
    println!("Child completed: {status}");
    // spawn returns without waiting for completion. Child output may appear
    // before or after the parent's messages: scheduling order is not guaranteed.
    // Child is a process handle, not a Rust thread or Tokio task.
    // Dropping Child does not kill or wait for it; plan its completion explicitly.
    // These std waits block the calling thread. tokio::process offers async APIs.
    // Polling, timeouts, killing, and cleanup follow in file 03.
    Ok(())
}

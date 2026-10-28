/*
clap basics: define typed command-line arguments with the derive API.

1. Parser derive and supplied argument lists
2. Positional arguments
3. Required short / long options
4. Optional values and defaults
5. Boolean flags and verbosity counts
6. Repeated options and multiple values
7. Paths, spaces, and native OS strings
8. Enumerated choices with ValueEnum
9. Built-in numeric validation
10. Custom validation: positive finite dimensions
11. Hyphens, negative numbers, and --
12. Help, version, and parse errors

Setup:
    cargo add clap@4 --features derive

Save as src/bin/01_clap_basics.rs and run:
    cargo run --bin 01_clap_basics

No [[bin]] entry is needed for src/bin/.
Every demo uses try_parse_from with explicit argument lists, so all demos run
without requiring command-line options or exiting on expected mistakes.
The FIRST supplied argument is always the program name, not a user argument.
Each element is one argument token; these lists aren't shell command strings.

Actual CLI: Args::parse() reads std::env::args_os(), prints help / errors, and
exits when appropriate. Args::try_parse() reads real arguments but returns Result.
parse_from(...) is the exit-on-error counterpart of try_parse_from(...).
The next guide covers subcommands, argument relations, and a real calculator CLI.
Reference: https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html
*/

use clap::{ArgAction, CommandFactory, Parser, ValueEnum};
use std::fmt::Debug;
use std::path::PathBuf;

// Keep expected errors local: an invalid example must not stop later demos.
fn show<T: Debug>(result: Result<T, clap::Error>) {
    match result {
        Ok(args) => println!("{args:?}"),
        Err(error) => {
            println!("kind: {:?}", error.kind());
            print!("{error}");
        }
    }
}

// ============================================================================
// 1. Parser derive and argument lists
// ============================================================================

fn demo_parser() {
    #[derive(Debug, Parser)]
    #[command(name="greet", about="Greet a person")]
    struct Args {
        /// Name to greet
        #[arg(long)]
        name: String,
    }

    show(Args::try_parse_from(["greet", "--name", "Alice"]));
    // Args { name: "Alice" }

    println!();

    show(Args::try_parse_from(["greet"])); // missing required --name
    // String means required here. Option<String> makes the option optional.
    // /// doc comments become user-facing help descriptions.
    // #[command(...)] configures the program; #[arg(...)] configures one argument.
}

// ============================================================================
// 2. Positional arguments
// ============================================================================

fn demo_positionals() {
    #[derive(Debug, Parser)]
    #[command(name="copy-demo")]
    struct Args {
        /// Input file
        input: PathBuf,
        /// Optional destination
        destination: Option<PathBuf>,
    }

    show(Args::try_parse_from(["copy-demo", "input.csv", "output.csv"]));
    show(Args::try_parse_from(["copy-demo", "input.csv"])); // destination: None

    println!();

    show(Args::try_parse_from(["copy-demo"])); // input required
    // Without short / long, a field is positional. Order matters.
    // Keep required positional arguments before optional ones.
    // Parsing paths does not perform a copy or access the filesystem.
}

// ============================================================================
// 3. Required short / long options
// ============================================================================

fn demo_required_options() {
    #[derive(Debug, Parser)]
    #[command(name="rectangle")]
    struct Args {
        /// Rectangle length
        #[arg(short='l', long, value_name="NUMBER")]
        length: f64,
        /// Rectangle width
        #[arg(short='w', long, value_name="NUMBER")]
        width: f64,
    }

    show(Args::try_parse_from(["rectangle", "-l", "5", "--width", "3"]));
    show(Args::try_parse_from(["rectangle", "--width=3", "--length=5"]));

    println!();

    show(Args::try_parse_from(["rectangle", "--length", "5"])); // width missing
    // Named options can appear in either order. --option=value also works.
    // value_name changes the help placeholder, not the Rust field or type.
    // f64 conversion alone doesn't enforce positive / finite dimensions.
}

// ============================================================================
// 4. Optional values and defaults
// ============================================================================

fn demo_defaults() {
    #[derive(Debug, Parser)]
    #[command(name="settings")]
    struct Args {
        #[arg(short, long, value_name="FILE")]
        out: Option<PathBuf>,
        #[arg(long, default_value_t=4)] // `default_value_t` takes a Rust expression matching the field’s type
        workers: u16,
        #[arg(long, default_value="text")] // `default_value` takes a string, which clap parses into the field’s type
        format: String,
    }

    show(Args::try_parse_from(["settings"])); // out=None, workers=4, format="text"
    show(Args::try_parse_from(["settings", "-o", "result.txt", "--workers", "8"]));

    println!();

    show(Args::try_parse_from(["settings", "--out"])); // option supplied without its value
    // Optional OPTION doesn't mean its VALUE may be omitted once the flag appears.
    // Prefer Option<PathBuf> to using an empty string as a 'no output' sentinel.
    // default_value_t uses a typed value; default_value uses textual input.
}

// ============================================================================
// 5. Boolean flags and counted verbosity
// ============================================================================

fn demo_flags() {
    #[derive(Debug, Parser)]
    #[command(name="flags")]
    struct Args {
        #[arg(long)]
        dry_run: bool,
        #[arg(short='v', long, action=ArgAction::Count)]
        verbose: u8,
        #[arg(long="no-color", action=ArgAction::SetFalse)]
        color: bool,
    }

    show(Args::try_parse_from(["flags"])); // dry_run=false, verbose=0, color=true
    show(Args::try_parse_from(["flags", "--dry-run", "-vvv", "--no-color"]));
    // dry_run=true, verbose=3, color=false
    // bool defaults to a presence flag (SetTrue), like Python store_true.
    // SetFalse implements a disabling flag, like store_false.
    // Rust snake_case field dry_run becomes --dry-run by default.
}

// ============================================================================
// 6. Repeated options vs multiple values per occurrence
// ============================================================================

fn demo_multiple_values() {
    #[derive(Debug, Parser)]
    #[command(name="lists")]
    struct Args {
        #[arg(short='I', long="include")]
        include: Vec<PathBuf>,
        #[arg(long, num_args=1..)]
        sides: Vec<f64>,
        #[arg(long, value_delimiter=',')]
        tags: Vec<String>,
    }

    show(Args::try_parse_from([
        "lists", "-I", "src", "-I", "tests", "--sides", "5", "4", "3",
        "--tags", "fast,debug",
    ]));
    // include=["src","tests"], sides=[5.0,4.0,3.0], tags=["fast","debug"]

    show(Args::try_parse_from(["lists"])); // all Vecs empty

    println!();

    show(Args::try_parse_from(["lists", "--sides"])); // needs at least one value
    // Vec<T> normally appends one value per option occurrence.
    // num_args=1.. accepts several after one option (Python nargs='+').
    // Add required=true if --sides itself must appear, not merely have a value
    // when present. An unbounded value list stops at another recognized option.
}

// ============================================================================
// 7. Paths and spaces
// ============================================================================

fn demo_paths() {
    #[derive(Debug, Parser)]
    #[command(name="paths")]
    struct Args {
        #[arg(long)]
        input: PathBuf,
    }

    show(Args::try_parse_from(["paths", "--input", "my data/input.csv"]));
    // One array element preserves the space. Don't include shell quote characters.
    // At a terminal, quote the argument: --input 'my data/input.csv'.
    // PathBuf handles native paths, including names not representable as UTF-8.
    // Existence / permissions / file type are NOT checked by PathBuf parsing.
    // clap doesn't expand ~, environment variables, or glob patterns itself;
    // shell expansion depends on quoting and the shell used to launch the program.
}

// ============================================================================
// 8. Enumerated choices
// ============================================================================

fn demo_choices() {
    #[derive(Debug, Clone, Copy, ValueEnum)]
    enum OutputFormat { Text, Json, PrettyJson }

    #[derive(Debug, Parser)]
    #[command(name="choices")]
    struct Args {
        #[arg(long, value_enum, default_value="text")]
        format: OutputFormat,
    }

    show(Args::try_parse_from(["choices", "--format", "pretty-json"]));

    println!();

    show(Args::try_parse_from(["choices", "--format", "xml"])); // invalid choice
    // Like argparse choices, but the result is a Rust enum instead of a String.
    // ValueEnum exposes allowed values in generated help and diagnostics.
}

// ============================================================================
// 9. Built-in numeric validation
// ============================================================================

fn demo_numeric_validation() {
    #[derive(Debug, Parser)]
    #[command(name="workers")]
    struct Args {
        #[arg(long, default_value_t=4, value_parser=clap::value_parser!(u16).range(1..=256))]
        workers: u16,
    }

    for value in ["8", "0", "257", "70000", "many"] {
        show(Args::try_parse_from(["workers", "--workers", value]));
    }
    // 8 succeeds. Other inputs fail range checking, integer overflow, or conversion.
    // Range validation runs during parsing, before application work starts.
}

// ============================================================================
// 10. Custom validation: positive finite floats
// ============================================================================

fn positive_finite(text: &str) -> Result<f64, String> {
    let value: f64 = text.parse().map_err(|_| format!("{text:?} is not a number"))?;
    if !value.is_finite() || value <= 0.0 {
        return Err("expected a positive, finite number".into());
    }
    Ok(value)
}

fn demo_custom_validation() {
    #[derive(Debug, Parser)]
    #[command(name="dimension")]
    struct Args {
        #[arg(long, value_parser=positive_finite)]
        length: f64,
    }

    for value in ["5.5", "0", "NaN", "inf", "wrong"] {
        show(Args::try_parse_from(["dimension", "--length", value]));
    }
    // Custom parser returns a typed value or an explanatory error message.
    // Reuse positive_finite for length / width / sides in the calculator later.
    // Cross-field relationships and errors during actual work belong elsewhere.
}

// ============================================================================
// 11. Negative numbers and the -- separator
// ============================================================================

fn demo_hyphens() {
    #[derive(Debug, Parser)]
    #[command(name="hyphens")]
    struct Args {
        #[arg(long, allow_negative_numbers=true)]
        offset: Option<f64>,
        files: Vec<PathBuf>,
    }

    show(Args::try_parse_from(["hyphens", "--offset", "-2.5", "input.txt"]));
    show(Args::try_parse_from(["hyphens", "--", "--not-an-option.txt"]));
    // -- ends option parsing; following tokens are positional values.
    // This permits file names beginning with '-'. It doesn't fill a missing
    // required named option. Negative numeric values need an appropriate policy.
}

// ============================================================================
// 12. Help, version, and expected errors
// ============================================================================

fn demo_help_errors() {
    #[derive(Debug, Parser)]
    #[command(name="help-demo", version="1.0.0", about="Demonstrate CLI help",
        after_help="Examples:\n  help-demo --name Alice")]
    struct Args {
        /// Name to greet
        #[arg(short, long, value_name="PERSON")]
        name: String,
    }

    println!("{}", Args::command().render_long_help());
    show(Args::try_parse_from(["help-demo", "--version"])); // DisplayVersion
    show(Args::try_parse_from(["help-demo", "--help"])); // DisplayHelp, not invalid user input

    println!();

    show(Args::try_parse_from(["help-demo", "--unknown"])); // unknown argument
    // In a real CLI, Args::parse() prints help/version and exits successfully;
    // invalid input exits with an error status. try_parse_from returns these
    // outcomes for you to handle, which is why every demo here can continue.
    // Long option abbreviations aren't enabled by default, unlike argparse's default.
    // Prefer generated usage and clear value_name labels over backspace tricks.
}

// ============ //
//    main()    //
// ============ //

fn main() {
    println!();

    demo_parser();
    println!("\n============================\n");
    demo_positionals();
    println!("\n============================\n");
    demo_required_options();
    println!("\n============================\n");
    demo_defaults();
    println!("\n============================\n");
    demo_flags();
    println!("\n============================\n");
    demo_multiple_values();
    println!("\n============================\n");
    demo_paths();
    println!("\n============================\n");
    demo_choices();
    println!("\n============================\n");
    demo_numeric_validation();
    println!("\n============================\n");
    demo_custom_validation();
    println!("\n============================\n");
    demo_hyphens();
    println!("\n============================\n");
    demo_help_errors();
}

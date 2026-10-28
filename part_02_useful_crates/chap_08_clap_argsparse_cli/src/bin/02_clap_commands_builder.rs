// 02_clap_commands_builder.rs
// ===========================
// Setup (inside chap_08_clap_argsparse):
//     cargo add clap@4 --features derive
//
// Run every demo:
//     cargo run --bin 02_clap_commands_builder
// Run the real rectangle CLI instead:
//     cargo run --bin 02_clap_commands_builder -- --help
//     cargo run --bin 02_clap_commands_builder -- -l 5 -w 3
//     cargo run --bin 02_clap_commands_builder -- -l 5 -w 3 -o rectangle.txt
// The last command writes/overwrites the explicitly selected output file.
//
// Derive API: typed structs/enums; usually the easiest starting point.
// Builder API: Command + Arg + ArgMatches; useful for runtime configuration.
// Both describe the same underlying clap parser.
// Docs:
// https://docs.rs/clap/latest/clap/_derive/_tutorial/index.html
// https://docs.rs/clap/latest/clap/_tutorial/index.html

use clap::{Arg, ArgAction, ArgGroup, Args, Command, Parser, Subcommand};
use std::{fmt::Debug, path::PathBuf, process::ExitCode};

fn main() -> ExitCode {
    println!();

    // With arguments, act as the real calculator. Without them, run the guide.
    if std::env::args_os().nth(1).is_some() {
        let args = RectangleCli::parse();
        return match run_rectangle(args) {
            Ok(()) => ExitCode::SUCCESS,
            Err(error) => {
                eprintln!("Error: {error}");
                ExitCode::FAILURE
            }
        };
    }

    demo_subcommands();
    println!("\n============================\n");

    demo_optional_subcommand();
    println!("\n============================\n");

    demo_flatten_and_global();
    println!("\n============================\n");

    demo_conflicts_and_requires();
    println!("\n============================\n");

    demo_argument_group();
    println!("\n============================\n");

    demo_builder_arguments();
    println!("\n============================\n");

    demo_builder_subcommands();
    println!("\n============================\n");

    demo_derive_with_builder();
    println!("\n============================\n");

    demo_rectangle();

    ExitCode::SUCCESS
}

fn show<T: Debug>(result: Result<T, clap::Error>) {
    match result {
        Ok(args) => println!("{args:#?}"),
        Err(error) => println!("{:?}:\n{error}", error.kind()),
    }
}

// 01. Subcommands: one executable, several operations
// -------------------------------------------------
// This allows making subcommands:
// ```
// files inspect notes.txt
// files copy a.txt b.txt --overwrite
// ```
//
// Just like `cargo new project` or `cargo add deps`
//
// cargo new project
// │     │   └── positional argument: project name
// │     └────── subcommand
// └──────────── executable
fn demo_subcommands() {
    #[derive(Debug, Parser)]
    #[command(name="files")]
    struct Cli {
        #[command(subcommand)]
        command: Operations,
    }

    #[derive(Debug, Subcommand)]
    enum Operations {
        /// Inspect a file
        Inspect { path: PathBuf },
        /// Copy a file (the demo only prints the requested operation)
        Copy {
            source: PathBuf,
            destination: PathBuf,
            #[arg(long)]
            overwrite: bool,
        },
    }

    for argv in [
        vec!["files", "inspect", "notes.txt"], // files inspect notes.txt
        vec!["files", "copy", "a.txt", "b.txt", "--overwrite"], // files copy a.txt b.txt --overwrite
    ] {
        match Cli::try_parse_from(argv) {
            Ok(cli) => match cli.command {
                Operations::Inspect { path } => println!("Inspect {}", path.display()),
                Operations::Copy { source, destination, overwrite } => {
                    println!("Copy {} -> {}, overwrite={overwrite}",
                        source.display(), destination.display());
                }
            },
            Err(error) => println!("{error}"),
        }
    }

    println!();

    show(Cli::try_parse_from(["files"])); // missing required subcommand
    // An enum field requires a subcommand; Option<Enum> makes it optional.
}

// 02. Optional subcommands and generated subcommand help
// -----------------------------------------------------
fn demo_optional_subcommand() {
    #[derive(Debug, Parser)]
    #[command(name="app")]
    struct Cli {
        #[command(subcommand)]
        command: Option<Operations>,
    }

    #[derive(Debug, Subcommand)]
    enum Operations {
        /// Show application status
        Status,
    }

    show(Cli::try_parse_from(["app"])); // command: None
    show(Cli::try_parse_from(["app", "status"]));
    show(Cli::try_parse_from(["app", "help", "status"])); // DisplayHelp
    // Real programs match on None to choose their own default behavior.
}

// 03. Reusable argument structs with flatten; global options
// ---------------------------------------------------------
fn demo_flatten_and_global() {
    #[derive(Debug, Args)]
    struct Logging {
        #[arg(short, long, action=ArgAction::Count, global=true)]
        verbose: u8,
    }

    #[derive(Debug, Parser)]
    #[command(name="tool")]
    struct Cli {
        #[command(flatten)]
        logging: Logging,
        #[command(subcommand)]
        command: Operations,
    }

    #[derive(Debug, Subcommand)]
    enum Operations {
        Check { path: PathBuf },
    }

    show(Cli::try_parse_from(["tool", "-vv", "check", "src"]));
    show(Cli::try_parse_from(["tool", "check", "src", "-vv"]));
    // flatten inserts fields into this command, rather than adding a subcommand.
    // global=true lets this option also appear after the subcommand name.
    // The same Logging struct can be flattened into other CLI structs.
}

// 04. Relationships: conflicts_with and requires
// ----------------------------------------------
fn demo_conflicts_and_requires() {
    #[derive(Debug, Parser)]
    #[command(name="export")]
    struct Cli {
        #[arg(long, conflicts_with="quiet")]
        verbose: bool,
        #[arg(long)]
        quiet: bool,
        #[arg(long)]
        output: Option<PathBuf>,
        #[arg(long, requires="output")]
        append: bool,
    }

    show(Cli::try_parse_from(["export", "--verbose"]));
    show(Cli::try_parse_from(["export", "--append", "--output", "log.txt"]));

    println!();

    show(Cli::try_parse_from(["export", "--verbose", "--quiet"]));

    println!();

    show(Cli::try_parse_from(["export", "--append"]));
    // Relationships refer to argument IDs (derive: field names by default),
    // not CLI spellings such as "--output".
}

// 05. ArgGroup: exactly one input source
// --------------------------------------
fn demo_argument_group() {
    #[derive(Debug, Parser)]
    #[command(name="read")]
    #[command(group(
        ArgGroup::new("source")
            .required(true)
            .multiple(false)
            .args(["file", "stdin"])
    ))]
    struct Cli {
        #[arg(long)]
        file: Option<PathBuf>,
        #[arg(long)]
        stdin: bool,
    }

    show(Cli::try_parse_from(["read", "--file", "data.txt"]));
    show(Cli::try_parse_from(["read", "--stdin"]));

    println!();

    show(Cli::try_parse_from(["read"])); // neither source

    println!();

    show(Cli::try_parse_from(["read", "--file", "data.txt", "--stdin"]));
    // required(true): at least one; multiple(false): at most one.
    // An Args struct shares/reuses fields; ArgGroup adds selection constraints.
}

// 06. Builder API: definitions and typed extraction
// ------------------------------------------------
fn demo_builder_arguments() {
    let command = Command::new("build")
        .about("Builder API example")
        .arg(Arg::new("input").required(true).value_parser(clap::value_parser!(PathBuf)))
        .arg(Arg::new("workers")
            .short('j').long("workers")
            .default_value("4")
            .value_parser(clap::value_parser!(u16).range(1..=256)))
        .arg(Arg::new("verbose")
            .short('v').long("verbose").action(ArgAction::Count))
        .arg(Arg::new("dry_run").long("dry-run").action(ArgAction::SetTrue));

    let matches = command.clone()
        .try_get_matches_from(["build", "src", "-j", "8", "-vv", "--dry-run"])
        .expect("valid demo arguments");

    let input = matches.get_one::<PathBuf>("input").expect("required input");
    let workers = matches.get_one::<u16>("workers").expect("default provided");
    println!("input={}, workers={workers}, verbose={}, dry_run={}",
        input.display(), matches.get_count("verbose"), matches.get_flag("dry_run"));
    // Extraction uses the Arg ID, so get_flag("dry_run"), not "dry-run".
    // get_one::<T> must match the type produced by value_parser.
    // Unlike derive's typed expression default_value_t, this builder default
    // is text. The u16 value parser converts "4" to the actual field value.

    println!();

    show(command.try_get_matches_from(["build", "src", "--workers", "0"]));
    // try_get_matches_from consumes Command; clone it when reusing a definition.
    // get_matches() instead reads the real process arguments and handles exits.
}

// 07. Builder subcommands and dispatch
// ------------------------------------
fn demo_builder_subcommands() {
    let command = Command::new("cache")
        .subcommand_required(true)
        .subcommand(Command::new("list").about("List cached entries"))
        .subcommand(Command::new("clear")
            .about("Clear cached entries")
            .arg(Arg::new("force").long("force").action(ArgAction::SetTrue)));

    let matches = command.clone()
        .try_get_matches_from(["cache", "clear", "--force"])
        .expect("valid demo arguments");

    match matches.subcommand() {
        Some(("list", _)) => println!("List requested"),
        Some(("clear", args)) => println!("Clear requested, force={}", args.get_flag("force")),
        _ => unreachable!("a known subcommand is required"),
    }
    // The demo prints the request; it does not delete anything.

    println!();

    show(command.try_get_matches_from(["cache", "unknown"]));
}

// 08. Combining derive with builder customization
// -----------------------------------------------
fn demo_derive_with_builder() {
    use clap::{CommandFactory, FromArgMatches};

    #[derive(Debug, Parser)]
    #[command(name="hybrid")]
    struct Cli {
        #[arg(long, default_value_t=4)]
        workers: u16,
    }

    // Keep typed fields, but customize command metadata at runtime.
    let matches = Cli::command()
        .about("Command configured through the builder API")
        .try_get_matches_from(["hybrid", "--workers", "8"])
        .expect("valid demo arguments");

    show(Cli::from_arg_matches(&matches));
    // Prefer derive for ordinary fixed CLIs; use the builder when convenient.
}

// 09. Practical rectangle calculator (adapted from Python argparse)
// ----------------------------------------------------------------
#[derive(Debug, Parser)]
#[command(name="rectangle", version, about="Calculate rectangle perimeter and area")]
#[command(after_help="Example: rectangle -l 5 -w 3 -o rectangle.txt")]
struct RectangleCli {
    /// Positive, finite length
    #[arg(short='l', long, value_name="LENGTH", value_parser=positive_finite)]
    length: f64,

    /// Positive, finite width
    #[arg(short='w', long, value_name="WIDTH", value_parser=positive_finite)]
    width: f64,

    /// Save the results to this file (overwrites an existing file)
    #[arg(short='o', long="out", value_name="FILE")]
    output: Option<PathBuf>,
}

fn positive_finite(text: &str) -> Result<f64, String> {
    let value = text.parse::<f64>().map_err(|_| "expected a number".to_owned())?;
    if value.is_finite() && value > 0.0 {
        Ok(value)
    } else {
        Err("expected a positive, finite number".to_owned())
    }
}

fn run_rectangle(args: RectangleCli) -> Result<(), Box<dyn std::error::Error>> {
    let perimeter = 2.0 * (args.length + args.width);
    let area = args.length * args.width;
    if !perimeter.is_finite() || !area.is_finite() {
        return Err("dimensions are too large: calculation overflowed".into());
    }

    let result = format!(
        "Length: {}\nWidth: {}\nPerimeter: {perimeter}\nArea: {area}\n",
        args.length, args.width,
    );
    print!("{result}");

    if let Some(path) = args.output {
        // A filename need not have an extension. Let the filesystem report errors.
        std::fs::write(&path, &result)?;
        println!("Saved to {}", path.display());
    }
    Ok(())
}

fn demo_rectangle() {
    let args = RectangleCli::try_parse_from(["rectangle", "-l", "5", "-w", "3"])
        .expect("valid demo arguments");
    run_rectangle(args).expect("no file output requested");
    // This demo does not write a file. Use the real CLI with -o to try saving.
    // Parsing and application logic are separate, as in Python's main function.
    // The earlier guide covers bool flags and lists; this CLI keeps only the
    // rectangle-related arguments from the Python example.

    println!();

    show(RectangleCli::try_parse_from(["rectangle", "-l", "0", "-w", "3"]));

    println!();

    show(RectangleCli::try_parse_from(["rectangle", "-l", "5"])); // missing width
}

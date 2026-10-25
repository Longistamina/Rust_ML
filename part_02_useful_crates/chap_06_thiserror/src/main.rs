/*
thiserror: define error types using Rust's standard Error trait.

1. Error enums: unit, tuple, and named-field variants
2. Error structs and formatted fields
3. Automatic conversion with #[from] and propagation with ?
4. Attach context with #[source] and map_err
5. Transparent wrappers
6. Match variants to decide what to do
7. Inspect the source chain
8. Practical example: load and validate a worker count

Setup:
    cargo add thiserror@2

Save as src/bin/01_thiserror.rs and run:
    cargo run --bin 01_thiserror

No [[bin]] entry is needed for src/bin/.
No other dependencies or external files are required.
Expected errors are handled in demos, so the whole guide keeps running.

thiserror derives std::error::Error and Display from your declarations.
You still use ordinary Result, ?, match, and map_err to handle errors.
#[derive(Debug)] supplies debug output; #[error(...)] describes Display output.
thiserror doesn't automatically log errors, retry operations, or catch panics.
Reference: https://docs.rs/thiserror/latest/thiserror/
*/

use std::error::Error as StdError; // trait, used for source() and trait objects
use std::io::{self, Read};
use std::num::ParseIntError;
use thiserror::Error; // derive macro

// ============================================================================
// 1. Error enums
// ============================================================================

#[derive(Debug, Error)]
enum RequestError {
    #[error("request contains no items")]
    Empty,
    #[error("unknown item: {0}")]
    UnknownItem(String),
    #[error("requested {requested} items, but the limit is {limit}")]
    TooMany { requested: usize, limit: usize },
}

fn demo_enum() {
    let errors = [
        RequestError::Empty,
        RequestError::UnknownItem("sample-42".into()),
        RequestError::TooMany { requested: 12, limit: 10 },
    ];

    for error in errors {
        println!("Display: {error}");
        println!("Debug: {error:?}");
    }
    // {0} uses a tuple field; {requested} uses a named field.
    // Display is your explanatory message; Debug shows variants and stored data.
}

// ============================================================================
// 2. An error can be a struct too
// ============================================================================

#[derive(Debug, Error)]
#[error("value {value} is outside {min}..={max} for {field:?}")]
struct BoundsError {
    field: &'static str,
    value: i64,
    min: i64,
    max: i64,
}

fn demo_struct() {
    let error = BoundsError { field: "workers", value: 0, min: 1, max: 256 };
    println!("{error}"); // value 0 is outside 1..=256 for "workers"
    // Formatting supports Debug with {:?} as well as Display.
    // Store useful structured values rather than only a preformatted String.
}

// ============================================================================
// 3. #[from] and ?
// ============================================================================

#[derive(Debug, Error)]
enum ReadNumberError {
    #[error("could not read the input")]
    Io(#[from] io::Error),
    #[error("input is not a valid unsigned integer")]
    Parse(#[from] ParseIntError),
}

fn read_number(mut reader: impl Read) -> Result<u16, ReadNumberError> {
    let mut text = String::new();
    reader.read_to_string(&mut text)?; // io::Error -> ReadNumberError::Io
    let number = text.trim().parse::<u16>()?; // ParseIntError -> Parse variant
    Ok(number)
}

fn demo_from() {
    println!("{:?}", read_number(&b"42\n"[..])); // Ok(42)
    println!("{:?}", read_number(&b"wrong"[..])); // Err(Parse(...))

    let original = io::Error::new(io::ErrorKind::PermissionDenied, "demo access denied");
    let wrapped: ReadNumberError = original.into();
    println!("{wrapped}"); // could not read the input

    // #[from] generates From<SourceError>; ? uses that conversion automatically.
    // #[from] also marks that field as the source; don't add #[source] too.
    // A #[from] variant can't also hold arbitrary context fields.
    // Two variants can't both derive From for the SAME source type: ambiguous.
}

// ============================================================================
// 4. Preserve context and source with map_err
// ============================================================================

#[derive(Debug, Error)]
#[error("invalid integer for {field}: {input:?}")]
struct FieldError {
    field: &'static str,
    input: String,
    #[source]
    cause: ParseIntError,
}

fn parse_field(field: &'static str, text: &str) -> Result<u16, FieldError> {
    text.parse::<u16>().map_err(|cause| FieldError {
        field, input: text.to_owned(), cause,
    })
}

fn demo_context() {
    match parse_field("workers", "many") {
        Ok(value) => println!("{value}"),
        Err(error) => {
            println!("{error}"); // invalid integer for workers: "many"
            println!("cause: {}", error.source().unwrap());
        }
    }
    // #[source] preserves the cause, but does NOT generate From.
    // map_err supplies the additional field / input context explicitly.
    // A field literally named source is recognized automatically; here we named
    // it cause to demonstrate the explicit attribute.
    // The cause isn't automatically appended to Display unless you format it.
}

// ============================================================================
// 5. Transparent forwarding
// ============================================================================

#[derive(Debug, Error)]
#[error(transparent)]
struct PublicFieldError(#[from] FieldError);

fn demo_transparent() {
    if let Err(error) = parse_field("port", "bad") {
        let public: PublicFieldError = error.into();
        println!("{public}"); // same Display as the wrapped FieldError
        println!("cause: {}", public.source().unwrap()); // underlying ParseIntError
    }
    // transparent forwards Display AND source() to the wrapped error.
    // source() returns the wrapped error's SOURCE, not an extra wrapper layer.
    // Useful for an opaque public wrapper, or an enum variant that adds no context.
}

// ============================================================================
// 6. Handle errors by variant
// ============================================================================

fn demo_matching() {
    let result: Result<(), ReadNumberError> = Err(io::Error::new(
        io::ErrorKind::NotFound, "demo input missing",
    ).into());

    match result {
        Ok(()) => println!("success"),
        Err(ReadNumberError::Io(error)) if error.kind() == io::ErrorKind::NotFound => {
            println!("choose a fallback for missing input");
        }
        Err(ReadNumberError::Io(error)) => println!("other I/O error: {error}"),
        Err(ReadNumberError::Parse(error)) => println!("ask for a valid integer: {error}"),
    }
    // Match variants / structured fields, not the human-readable message text.
    // A Result type alias can shorten repeated signatures; it's optional.
}

// ============================================================================
// 7. Inspect an error chain
// ============================================================================

fn print_chain(error: &dyn StdError) {
    println!("error: {error}");
    let mut next = error.source();
    while let Some(cause) = next {
        println!("caused by: {cause}");
        next = cause.source();
    }
}

fn demo_chain() {
    if let Err(error) = parse_field("workers", "oops") {
        print_chain(&error);
    }
    // error: invalid integer for workers: "oops"
    // caused by: invalid digit found in string (exact std wording may change)
    // &dyn Error borrows a concrete error; Box<dyn Error> can own heterogeneous
    // errors at an application boundary. Neither replaces typed variants when
    // callers need to distinguish recoverable failure categories.
}

// ============================================================================
// 8. Practical example: read, parse, validate
// ============================================================================

#[derive(Debug, Error)]
enum ConfigError {
    #[error("could not read worker settings")]
    Read(#[from] io::Error),
    #[error("worker count is empty")]
    Empty,
    #[error("invalid worker count {input:?}")]
    InvalidInteger { input: String, #[source] source: ParseIntError },
    #[error("worker count {value} must be in 1..=256")]
    OutOfRange { value: u16 },
}

fn load_workers(mut reader: impl Read) -> Result<u16, ConfigError> {
    let mut text = String::new();
    reader.read_to_string(&mut text)?;
    let text = text.trim();
    if text.is_empty() {
        return Err(ConfigError::Empty);
    }

    let value = text.parse::<u16>().map_err(|source| ConfigError::InvalidInteger {
        input: text.to_owned(), source,
    })?;
    if !(1..=256).contains(&value) {
        return Err(ConfigError::OutOfRange { value });
    }
    Ok(value)
}

fn demo_practical() {
    for input in ["4\n", "", "many", "0", "300", "70000"] {
        println!("input: {input:?}");
        match load_workers(input.as_bytes()) {
            Ok(workers) => println!("configured {workers} workers"),
            Err(error) => print_chain(&error),
        }
        println!();
    }
    // 4 succeeds; empty / invalid integer / out-of-range get distinct variants.
    // 70000 exceeds u16, so fails parsing before the 1..=256 validation.
    // The same function accepts a File, stdin lock, byte slice, or another Read.
    // I/O and parsing keep their original causes; domain validation has no cause.
}

// ============ //
//    main()    //
// ============ //

fn main() {
    println!();

    demo_enum();
    println!("\n============================\n");
    demo_struct();
    println!("\n============================\n");
    demo_from();
    println!("\n============================\n");
    demo_context();
    println!("\n============================\n");
    demo_transparent();
    println!("\n============================\n");
    demo_matching();
    println!("\n============================\n");
    demo_chain();
    println!("\n============================\n");
    demo_practical();
}

//! Bon: conversions, positional inputs, conditional building, and validation.
//!
//! Setup: cargo add bon@3.10
//! Save as src/bin/03_advanced_patterns.rs, then:
//! cargo run --bin 03_advanced_patterns
//!
//! Read 01_function_builders.rs and 02_structs_methods.rs first.
//! These examples use only bon and the standard library; no network or file IO.
//!
//! References:
//! https://bon-rs.com/guide/basics/into-conversions
//! https://bon-rs.com/guide/basics/custom-conversions
//! https://bon-rs.com/guide/basics/positional-members
//! https://bon-rs.com/guide/patterns/conditional-building
//! https://bon-rs.com/guide/patterns/fallible-builders

use bon::{bon, builder, Builder};
use std::num::ParseIntError;
use std::path::PathBuf;

fn main() {
    println!();

    demo_into_conversions();
    println!("\n============================\n");

    demo_shared_conversion_rules();
    println!("\n============================\n");

    demo_custom_setters();
    println!("\n============================\n");

    demo_positional_start();
    println!("\n============================\n");

    demo_positional_finish();
    println!("\n============================\n");

    demo_conditional_values();
    println!("\n============================\n");

    demo_conditional_builders();
    println!("\n============================\n");

    demo_fallible_function();
    println!("\n============================\n");

    demo_fallible_constructor();
    println!("\n============================\n");

    demo_fallible_setter();
    println!("\n============================\n");

    demo_practical_configuration();
}

// =====================================================
// 01. Accept convertible inputs with #[builder(into)]
// =====================================================

fn demo_into_conversions() {
    println!("01. Accept convertible inputs with #[builder(into)]");

    #[derive(Debug, Builder)]
    struct Output {
        #[builder(into)]
        name: String,
        #[builder(into)]
        path: PathBuf,
    }

    let output = Output::builder()
        .name("report")
        .path("results/report.csv")
        .build();
    println!("{} -> {}", output.name, output.path.display());
    // Normally String and PathBuf fields require those exact owned types.
    // #[builder(into)] lets the setter accept impl Into<FieldType> instead.
    // The destination fields still contain String and PathBuf, not &str.
    // No directories/files are created by constructing a PathBuf.

    let owned_name = String::from("backup");
    let owned_path = PathBuf::from("results/backup.csv");
    let backup = Output::builder().name(owned_name).path(owned_path).build();
    println!("{} -> {}", backup.name, backup.path.display());
    // Owned inputs also work and move into the builder.
    // Into is an infallible conversion, not parsing or validation:
    // a string does not become a u32 merely by enabling into.
}

// =============================================================
// 02. Share conversion rules and combine them with Option<T>
// =============================================================

fn demo_shared_conversion_rules() {
    println!("02. Share conversion rules and combine them with Option<T>");

    #[derive(Debug, Builder)]
    #[builder(on(String, into))]
    struct User {
        name: String,
        city: String,
        alias: Option<String>,
    }

    let user = User::builder()
        .name("Long")
        .city("Seoul")
        .alias("Rust learner")
        .build();
    println!("{} in {}, alias={:?}", user.name, user.city, user.alias);
    // on(String, into) applies the rule to String members in this builder.
    // It also covers the String inside an ordinarily optional Option<String>.
    // It does not automatically cover Vec<String> element-by-element.

    let configured_alias: Option<&str> = Some("Builder enthusiast");
    let other = User::builder()
        .name("Alice")
        .city("Paris")
        .maybe_alias(configured_alias)
        .build();
    println!("{} in {}, alias={:?}", other.name, other.city, other.alias);
    // maybe_alias accepts an Option of a convertible inner value.
    // A bare None can need a type annotation when generic conversion leaves
    // the input type unknown: .maybe_alias(None::<String>).
    // Omit the setter entirely when there is no value to forward.
}

// =====================================================
// 03. Custom setters can transform multiple arguments
// =====================================================

fn demo_custom_setters() {
    println!("03. Custom setters can transform multiple arguments");

    struct Point {
        x: i32,
        y: i32,
    }

    #[derive(Builder)]
    struct Marker {
        #[builder(with=|x: i32, y: i32| Point { x, y })]
        position: Point,
        #[builder(with=|text: &str| text.trim().to_owned())]
        label: String,
    }

    let marker = Marker::builder()
        .position(3, 4)
        .label("  meeting point  ")
        .build();
    println!("{} at ({}, {})", marker.label, marker.position.x, marker.position.y);
    // The closure's arguments become the setter's arguments.
    // position(3, 4) stores one Point; label(...) stores a trimmed String.
    // Conversion happens when the setter runs, before .build().
    // This custom position setter expects two i32s, not an existing Point.
    // Use into for standard conversions; with for a specific input API.
}

// =====================================================
// 04. Positional inputs at the beginning: start_fn
// =====================================================

fn demo_positional_start() {
    println!("04. Positional inputs at the beginning: start_fn");

    #[builder]
    fn greet(
        #[builder(start_fn)] name: &str,
        #[builder(default="Hello")] greeting: &str,
        #[builder(default="!")] punctuation: &str,
    ) -> String {
        format!("{greeting}, {name}{punctuation}")
    }

    println!("{}", greet("Alice").call());
    println!("{}", greet("Bob").greeting("Welcome").punctuation(".").call());
    // name is required at builder creation, so greet() without a name fails.
    // There is no .name(...) setter for this positional member.
    // This resembles Python def greet(name, *, greeting="Hello", ...).
    // The remaining members use named setters in any order.
}

// ==========================================================
// 05. Positional inputs at the end and custom finish names
// ==========================================================

fn demo_positional_finish() {
    println!("05. Positional inputs at the end and custom finish names");

    #[builder(finish_fn=render)]
    fn decorate(
        #[builder(finish_fn)] text: &str,
        #[builder(default="[")] prefix: &str,
        #[builder(default="]")] suffix: &str,
    ) -> String {
        format!("{prefix}{text}{suffix}")
    }

    let pending = decorate().prefix("<").suffix(">");
    println!("{}", pending.render("Rust"));
    println!("{}", decorate().render("Bon"));
    // Member-level finish_fn: text is passed to the finishing method.
    // Top-level finish_fn=render: rename .call() to .render().
    // Thus text is supplied at the last step, not through a .text(...) setter.
    // With several positional members, their declaration order matters.
    // Use this sparingly when configuration-first, input-last reads naturally.
}

// =====================================================
// 06. Conditional building: decide values first
// =====================================================

fn demo_conditional_values() {
    println!("06. Conditional building: decide values first");

    #[derive(Builder)]
    struct Settings {
        #[builder(default=3)]
        retries: u32,
        label: Option<&'static str>,
    }

    for debug in [true, false] {
        let retries = if debug { Some(0) } else { None };
        let label = debug.then_some("debug");

        let settings = Settings::builder()
            .maybe_retries(retries)
            .maybe_label(label)
            .build();
        println!("debug={debug}: retries={}, label={:?}", settings.retries, settings.label);
    }
    // Both runtime paths use the same setters and produce the same state type.
    // None for retries selects 3; None for label selects absence.
    // Optional values are often easier to branch on than typed builders.
}

// ==============================================================
// 07. Conditional building: finish inside different branches
// ==============================================================

fn demo_conditional_builders() {
    println!("07. Conditional building: finish inside different branches");

    #[derive(Builder)]
    struct Settings {
        name: &'static str,
        #[builder(default)]
        verbose: bool,
        label: Option<&'static str>,
    }

    for debug in [true, false] {
        let pending = Settings::builder().name("parser");
        let settings = if debug {
            pending.verbose(true).label("debug").build()
        } else {
            pending.build()
        };
        println!("{}: verbose={}, label={:?}", settings.name, settings.verbose, settings.label);
    }
    // Both branches return Settings even though they set different members.
    // The shared pending builder is consumed by whichever branch executes.
    // Do NOT try the following pattern (it does not compile):
    // let pending = if debug { pending.verbose(true) } else { pending };
    // The branches have different builder state types: verbose set vs unset.
    // Similarly, `pending = pending.verbose(true)` cannot change a variable's
    // inferred type. Use shadowing for an unconditional state transition,
    // or finish in the branches as above.
}

// =====================================================
// 08. A function builder can return Result
// =====================================================

fn demo_fallible_function() {
    println!("08. A function builder can return Result");

    #[builder]
    fn make_range(start: i32, end: i32) -> Result<std::ops::Range<i32>, &'static str> {
        if start >= end {
            return Err("start must be less than end");
        }
        Ok(start..end)
    }

    println!("valid range: {:?}", make_range().start(2).end(5).call());

    println!();

    match make_range().start(5).end(2).call() {
        Ok(range) => println!("range: {range:?}"),
        Err(error) => println!("expected error: {error}"),
    }
    // Required parameters are present, so the call compiles.
    // Their relationship is invalid; the body detects that at runtime.
    // .call() returns exactly Result<Range<i32>, &'static str>.
    // Bon does not automatically invent or execute validation rules.
}

// =====================================================
// 09. A constructor builder can return Result<Self, E>
// =====================================================

fn demo_fallible_constructor() {
    println!("09. A constructor builder can return Result<Self, E>");

    struct Account {
        name: String,
        quota: u32,
    }

    #[bon]
    impl Account {
        #[builder]
        fn new(
            #[builder(into)] name: String,
            #[builder(default=100)] quota: u32,
        ) -> Result<Self, &'static str> {
            let name = name.trim().to_owned();
            if name.is_empty() {
                return Err("name must not be blank");
            }
            if quota == 0 {
                return Err("quota must be greater than zero");
            }
            Ok(Self { name, quota })
        }
    }

    match Account::builder().name("  Alice  ").build() {
        Ok(account) => println!("{}: quota={}", account.name, account.quota),
        Err(error) => println!("error: {error}"),
    }

    println!();

    for (name, quota) in [("   ", 100), ("Bob", 0)] {
        match Account::builder().name(name).quota(quota).build() {
            Ok(account) => println!("{}: quota={}", account.name, account.quota),
            Err(error) => println!("expected error: {error}"),
        }
    }
    // .build() returns Result because new() returns Result.
    // Constructor validation runs at .build(), not at .name(...) or .quota(...).
    // &'static str keeps this demo dependency-free; a real library can use
    // a dedicated error enum, for example with thiserror.
}

// =====================================================
// 10. Fallible setters validate before the final call
// =====================================================

fn demo_fallible_setter() {
    println!("10. Fallible setters validate before the final call");

    #[builder]
    fn describe_port(
        #[builder(with=|text: &str| -> Result<_, ParseIntError> { text.parse() })]
        port: u16,
    ) -> String {
        format!("port={port}")
    }

    match describe_port().port("8080") {
        Ok(pending) => println!("{}", pending.call()),
        Err(error) => println!("parse error: {error}"),
    }
    // The setter returns Result<BuilderWithPortSet, ParseIntError>.
    // Match it (or use ? inside a Result-returning function) before .call().
    // describe_port().port("8080").call() would try .call() on a Result.

    println!();

    for input in ["oops", "70000"] {
        match describe_port().port(input) {
            Ok(pending) => println!("{}", pending.call()),
            Err(error) => println!("expected error for {input:?}: {error}"),
        }
    }
    // Parsing rejects non-numbers and values outside u16's range immediately.
    // It accepts 0; prohibiting 0 would be an additional domain rule.
    // A failed setter consumes its input builder; do not assume it is returned
    // with the error. Create a new builder to try another input.
    // Setter errors and final-call errors are separate stages; an API may use both.
}

// ================================================================
// 11. Put it together: construct a validated export configuration
// ================================================================

fn demo_practical_configuration() {
    println!("11. Put it together: construct a validated export configuration");

    struct ExportConfig {
        path: PathBuf,
        delimiter: char,
        batch_size: usize,
        note: Option<String>,
    }

    #[bon]
    impl ExportConfig {
        #[builder]
        fn new(
            #[builder(start_fn, into)] path: PathBuf,
            #[builder(default=',')] delimiter: char,
            #[builder(default=1_000)] batch_size: usize,
            #[builder(into)] note: Option<String>,
        ) -> Result<Self, &'static str> {
            if path.as_os_str().is_empty() {
                return Err("output path must not be empty");
            }
            if batch_size == 0 {
                return Err("batch size must be greater than zero");
            }
            if !matches!(delimiter, ',' | '\t' | ';' | '|') {
                return Err("unsupported delimiter");
            }
            Ok(Self { path, delimiter, batch_size, note })
        }
    }

    for debug in [true, false] {
        let note = debug.then_some("small batch for debugging");
        let batch_size = if debug { 10 } else { 1_000 };
        let result = ExportConfig::builder("results/data.csv")
            .delimiter(',')
            .batch_size(batch_size)
            .maybe_note(note)
            .build();

        match result {
            Ok(config) => println!("path={}, delimiter={:?}, batch={}, note={:?}",
                config.path.display(), config.delimiter, config.batch_size, config.note),
            Err(error) => println!("configuration error: {error}"),
        }
    }
    // Positional start input + Into conversions + optional configuration
    // + runtime validation, with required inputs still checked by the compiler.
    // This constructs configuration only; it does not create an export file.

    println!();

    match ExportConfig::builder("results/data.csv").batch_size(0).build() {
        Ok(config) => println!("path={}", config.path.display()),
        Err(error) => println!("expected error: {error}"),
    }
    // Practical rule: choose values, convert inputs, then validate invariants.
    // Typestate checks which inputs were supplied; validation checks their meaning.
}

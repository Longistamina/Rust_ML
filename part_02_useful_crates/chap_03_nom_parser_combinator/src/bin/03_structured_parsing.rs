/*
Structured parsing: a small settings language, from text to Rust data.

1. Define the grammar and reusable token parsers
2. Parse alternative value types into an enum
3. Parse a key/value setting into a struct
4. Parse and validate the entire document's syntax
5. Convert parsed settings into an owned application config
6. Separate syntax errors from semantic errors
7. Report error positions

Example input:
    name = "worker";
    workers = 4;
    enabled = true;
    ports = [8000, 8001];

Grammar: one or more key=value settings, separated by semicolons.
Values: quoted text, signed decimal i64, true/false, or a list of i64.
Whitespace is allowed around tokens; one final semicolon is optional.
Strings may be empty but cannot contain quotes, backslashes, CR, or LF.
No comments, escaping, nesting, or arbitrary value types inside lists.
This is a teaching language, not a TOML / JSON implementation.

Setup (same chapter project):
    cargo add nom@8

Keep this file beside Cargo.toml and add:
    [[bin]]
    name = "03_structured_parsing"
    path = "03_structured_parsing.rs"

Run:
    cargo run --bin 03_structured_parsing

Reference: https://docs.rs/nom/8.0.0/nom/
*/

use nom::branch::alt;
use nom::bytes::complete::{tag, take_till, take_while, take_while1};
use nom::character::complete::{char, i64 as integer, multispace0};
use nom::combinator::{all_consuming, cut, map, opt, recognize, value};
use nom::multi::separated_list1;
use nom::multi::separated_list0;
use nom::sequence::{delimited, terminated};
use nom::{IResult, Parser};

// ============================================================================
// 1. Grammar and reusable token parsers
// ============================================================================

fn parse_identifier(input: &str) -> IResult<&str, &str> {
    recognize((
        take_while1(|c: char| c.is_ascii_alphabetic() || c == '_'),
        take_while(|c: char| c.is_ascii_alphanumeric() || c == '_'),
    )).parse(input)
}

fn parse_text(input: &str) -> IResult<&str, &str> {
    let (input, _) = char('"').parse(input)?;
    // Once a quote opens, invalid contents / missing closing quote are fatal.
    cut(terminated(
        take_till(|c| matches!(c, '"' | '\\' | '\n' | '\r')),
        char('"'),
    )).parse(input)
}

fn demo_tokens() {
    println!("{:?}", parse_identifier("worker_count=4")); // Ok(("=4", "worker_count"))
    println!("{:?}", parse_identifier("123name")); // Err(...)
    println!("{:?}", parse_text(r#""worker";"#)); // Ok((";", "worker"))
    println!("{:?}", parse_text(r#""""#)); // Ok(("", "")): empty quoted string
    println!("{:?}", parse_text("\"unfinished")); // Err(Failure(...))
    // Token strings borrow their source input. No allocation is required here.
}

// ============================================================================
// 2. Alternative value types -> enum
// ============================================================================

#[derive(Debug, PartialEq)]
enum Value<'a> {
    Text(&'a str),
    Integer(i64),
    Boolean(bool),
    Integers(Vec<i64>),
}

fn parse_boolean(input: &str) -> IResult<&str, bool> {
    alt((value(true, tag("true")), value(false, tag("false")))).parse(input)
}

fn parse_integer_list(input: &str) -> IResult<&str, Vec<i64>> {
    let (input, _) = char('[').parse(input)?;
    cut(terminated(
        delimited(
            multispace0,
            separated_list0(
                delimited(multispace0, char(','), multispace0),
                integer,
            ),
            multispace0,
        ),
        char(']'),
    )).parse(input)
}

fn parse_value(input: &str) -> IResult<&str, Value<'_>> {
    alt((
        map(parse_text, Value::Text),
        map(parse_integer_list, Value::Integers),
        map(parse_boolean, Value::Boolean),
        map(integer, Value::Integer),
    )).parse(input)
}

fn demo_values() {
    for text in [r#""worker""#, "-42", "true", "[8000, 8001]", "[ ]", "[1,]"] {
        println!("{text:?} -> {:?}", parse_value(text));
    }
    // Outputs: Text("worker"), Integer(-42), Boolean(true),
    // Integers([8000, 8001]), Integers([]), then a parse failure.
    // parse_value("trueish") alone leaves "ish"; document validation rejects it.
    // cut inside quoted / list branches prevents retrying unrelated value types.
}

// ============================================================================
// 3. One setting -> struct
// ============================================================================

#[derive(Debug, PartialEq)]
struct Setting<'a> {
    key: &'a str,
    value: Value<'a>,
}

fn parse_setting(input: &str) -> IResult<&str, Setting<'_>> {
    let (input, key) = parse_identifier(input)?;
    let (input, _) = delimited(multispace0, char('='), multispace0).parse(input)?;
    let (input, parsed_value) = cut(parse_value).parse(input)?;
    Ok((input, Setting { key, value: parsed_value }))
}

fn demo_setting() {
    println!("{:?}", parse_setting("workers = 4;"));
    // Ok((";", Setting { key: "workers", value: Integer(4) }))
    println!("{:?}", parse_setting("workers = nope")); // Err(Failure(...))
    // After key=, a value is required: cut prevents an optional / repeated parser
    // from treating a malformed setting as simply absent.
}

// ============================================================================
// 4. Complete document -> Vec<Setting>
// ============================================================================

fn parse_separator(input: &str) -> IResult<&str, char> {
    delimited(multispace0, char(';'), multispace0).parse(input)
}

fn parse_document(input: &str) -> IResult<&str, Vec<Setting<'_>>> {
    all_consuming(delimited(
        multispace0,
        terminated(separated_list1(parse_separator, parse_setting), opt(parse_separator)),
        multispace0,
    )).parse(input)
}

fn demo_document() {
    let text = "name = \"worker\";\nworkers = 4; enabled = true; ports = [8000, 8001];\n";
    match parse_document(text) {
        Ok((rest, settings)) => {
            println!("remaining: {rest:?}"); // ""
            for setting in settings {
                println!("{setting:?}");
            }
        }
        Err(error) => println!("{error:?}"),
    }
    // all_consuming prevents accepting valid prefixes followed by garbage.
    // separated_list1 restores the separator when no next setting follows;
    // opt(parse_separator) then accepts one final semicolon.
    // An empty document, doubled separators, and trailing garbage are rejected.
}

// ============================================================================
// 5. Parsed settings -> owned application config
// ============================================================================

#[derive(Debug, PartialEq)]
struct Config {
    name: String,
    workers: u16,
    enabled: bool,
    ports: Vec<u16>,
}

fn build_config(settings: Vec<Setting<'_>>) -> Result<Config, String> {
    let (mut name, mut workers, mut enabled, mut ports) = (None, None, None, None);
    for Setting { key, value } in settings {
        match (key, value) {
            ("name", Value::Text(s)) if name.is_none() => {
                if s.is_empty() {
                    return Err("name must not be empty".into());
                }
                name = Some(s.to_owned());
            }
            ("workers", Value::Integer(n)) if workers.is_none() => {
                if !(1..=256).contains(&n) {
                    return Err("workers must be in 1..=256".into());
                }
                workers = Some(n as u16); // checked range above
            }
            ("enabled", Value::Boolean(b)) if enabled.is_none() => enabled = Some(b),
            ("ports", Value::Integers(values)) if ports.is_none() => {
                let mut checked = Vec::new();
                for n in values {
                    let port = u16::try_from(n).map_err(|_| format!("invalid port: {n}"))?;
                    if port == 0 {
                        return Err("port must be in 1..=65535".into());
                    }
                    checked.push(port);
                }
                ports = Some(checked);
            }
            ("name" | "workers" | "enabled" | "ports", _) => {
                return Err(format!("wrong type or duplicate setting: {key}"));
            }
            (other, _) => return Err(format!("unknown setting: {other}")),
        }
    }
    Ok(Config {
        name: name.ok_or("missing name")?,
        workers: workers.ok_or("missing workers")?,
        enabled: enabled.ok_or("missing enabled")?,
        ports: ports.ok_or("missing ports")?,
    })
    // All four keys are required, in any order. An empty ports list is permitted.
    // This function applies application rules; nom handles the text grammar.
    // String errors keep the example short; a reusable library should define
    // an error enum so callers can distinguish the different validation failures.
}

fn demo_owned_config() {
    let result = {
        let source = String::from("name=\"worker\"; workers=4; enabled=true; ports=[8000,8001]");
        match parse_document(&source) {
            Ok((_, settings)) => build_config(settings),
            Err(error) => Err(format!("syntax error: {error:?}")),
        }
    }; // source is dropped here; Config is still usable because it owns its data.
    match result {
        Ok(config) => println!("{config:#?}"),
        Err(error) => println!("{error}"),
    }
}

// ============================================================================
// 6. Syntax errors vs semantic errors
// ============================================================================

fn demo_validation() {
    for text in [
        "name=\"worker\"; workers=4; enabled=true; ports=[8000]", // valid
        "name=\"worker\"; workers=4; enabled=true; ports=[1,]", // syntax
        "name=\"worker\"; workers=0; enabled=true; ports=[8000]", // semantic
        "name=\"worker\"; workers=4; enabled=true; ports=[70000]", // semantic
        "name=\"worker\"; workers=4; enabled=true", // missing key
        "name=\"worker\"; workers=4; workers=5; enabled=true; ports=[]", // duplicate
    ] {
        match parse_document(text) {
            Ok((_, settings)) => println!("validation: {:?}", build_config(settings)),
            Err(error) => println!("syntax: {error:?}"),
        }
    }
    // Grammar: does the text have the required structure and representable numbers?
    // Semantics: do the parsed values satisfy this application's rules?
    // Keeping those steps separate makes each easier to change and reuse.
}

// ============================================================================
// 7. Report error positions
// ============================================================================

fn demo_error_position() {
    let source = "name=\"worker\";\nworkers = nope;";
    match parse_document(source) {
        Ok((_, settings)) => println!("{settings:?}"),
        Err(nom::Err::Error(error) | nom::Err::Failure(error)) => {
            // These parsers report suffixes of the original &str.
            let offset = source.len() - error.input.len();
            let prefix = &source[..offset];
            let line = prefix.bytes().filter(|&b| b == b'\n').count() + 1;
            let column = prefix.rsplit('\n').next().unwrap_or("").chars().count() + 1;
            println!("line {line}, column {column}, byte {offset}: {:?}", error.code);
            println!("remaining near error: {:?}", error.input);
            // line / column are 1-based; column counts Unicode scalars, not bytes
            // or screen width. This is the reported failure point, not a full
            // explanation of every expectation tried by alt().
        }
        Err(nom::Err::Incomplete(needed)) => println!("need more input: {needed:?}"),
    }
    // nom's default error stores an input position and ErrorKind only.
    // Rich context / expected-token messages need a richer error type.
}

// ============ //
//    main()    //
// ============ //

fn main() {
    println!();

    demo_tokens();
    println!("\n============================\n");
    demo_values();
    println!("\n============================\n");
    demo_setting();
    println!("\n============================\n");
    demo_document();
    println!("\n============================\n");
    demo_owned_config();
    println!("\n============================\n");
    demo_validation();
    println!("\n============================\n");
    demo_error_position();
}

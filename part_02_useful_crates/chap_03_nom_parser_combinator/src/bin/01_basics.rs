/*
nom basics: small parsers consume input and return a value plus the remainder.

1. Parser signature and IResult
2. Exact prefixes / characters: tag, tag_no_case, char
3. Character runs: alpha1, digit1, alphanumeric1, whitespace
4. Take characters / bytes; take_until and take_while1
5. Parse text into numeric values
6. Pass remaining input to the next parser
7. Complete vs streaming input; inspect errors

Setup (nom 8):
    cargo add nom@8

Keep this file beside Cargo.toml and add:
    [[bin]]
    name = "01_basic"
    path = "01_basic.rs"

Run:
    cargo run --bin 01_basic

nom 8 uses Parser::parse(input) for parser objects / composed parsers.
Some older tutorials use combinator(...)(input); prefer .parse(input) here.
Combinator composition comes next in 02_combinators.rs.
Reference: https://docs.rs/nom/8.0.0/nom/
*/

use nom::bytes::complete::{tag, tag_no_case, take, take_until, take_while1};
use nom::character::complete::{
    alpha1, alphanumeric1, char, digit1, multispace0, space0,
    i32 as parse_i32,
};
use nom::number::complete::double;
use nom::{IResult, Parser};

// ============================================================================
// 1. Parser signature and IResult
// ============================================================================

fn parse_hello(input: &str) -> IResult<&str, &str> {
    tag("HELLO").parse(input)
}

fn demo_result() {
    println!("{:?}", parse_hello("HELLO world")); // Ok((" world", "HELLO"))
    println!("{:?}", parse_hello("HELLO")); // Ok(("", "HELLO"))
    println!("{:?}", parse_hello("world HELLO")); // Err(...)

    // IResult<Input, Output> = Result<(remaining_input, output), nom::Err<...>>.
    // The default error type is nom::error::Error<Input>.
    // Parsing starts at the CURRENT input position; tag does not search ahead.
    // A successful parse can leave trailing input: success != full validation.
    if let Ok((remaining, value)) = parse_hello("HELLO!") {
        println!("value: {value}; remaining: {remaining:?}"); // HELLO; "!"
    }
    // &str output borrows the input: no allocation / copying for this token.
}

// ============================================================================
// 2. Exact prefixes and characters
// ============================================================================

fn parse_http(input: &str) -> IResult<&str, &str> {
    tag_no_case("HTTP").parse(input)
}

fn parse_open_paren(input: &str) -> IResult<&str, char> {
    char('(').parse(input)
}

fn demo_exact() {
    println!("{:?}", parse_http("http/1.1")); // Ok(("/1.1", "http"))
    println!("{:?}", parse_open_paren("(abc)")); // Ok(("abc)", '('))
    println!("{:?}", parse_open_paren("abc")); // Err(...)
    // tag returns the matched input slice; char returns a char.
    // tag_no_case preserves original spelling; it doesn't uppercase the input.
    // Parsers do not skip whitespace automatically.
}

// ============================================================================
// 3. Character runs and whitespace
// ============================================================================

fn parse_letters(input: &str) -> IResult<&str, &str> {
    alpha1.parse(input)
}

fn parse_digits(input: &str) -> IResult<&str, &str> {
    digit1.parse(input)
}

fn demo_runs_whitespace() {
    println!("{:?}", parse_letters("abc123")); // Ok(("123", "abc"))
    println!("{:?}", parse_digits("123abc")); // Ok(("abc", "123"))
    println!("{:?}", parse_digits("abc")); // Err(...)

    let result: IResult<&str, &str> = alphanumeric1.parse("abc123_rest");
    println!("{result:?}"); // Ok(("_rest", "abc123")); underscore isn't alphanumeric
    // alpha1 / digit1 / alphanumeric1 use ASCII letters / digits for &str.
    // Suffix 1 requires at least one character; variants ending in 0 allow empty.

    let result: IResult<&str, &str> = space0.parse(" \t\nnext");
    println!("{result:?}"); // Ok(("\nnext", " \t"))
    let result: IResult<&str, &str> = multispace0.parse(" \t\nnext");
    println!("{result:?}"); // Ok(("next", " \t\n"))
    let result: IResult<&str, &str> = space0.parse("next");
    println!("{result:?}"); // Ok(("next", ""))
    // space0: spaces / tabs. multispace0: spaces / tabs / CR / LF.
}

// ============================================================================
// 4. Take input by count, delimiter, or predicate
// ============================================================================

fn parse_three_chars(input: &str) -> IResult<&str, &str> {
    take(3usize).parse(input)
}

fn parse_three_bytes(input: &[u8]) -> IResult<&[u8], &[u8]> {
    take(3usize).parse(input)
}

fn parse_before_colon(input: &str) -> IResult<&str, &str> {
    take_until(":").parse(input)
}

fn parse_identifier(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_ascii_alphanumeric() || c == '_').parse(input)
}

fn demo_take() {
    println!("{:?}", parse_three_chars("abcdef")); // Ok(("def", "abc"))
    println!("{:?}", parse_three_chars("안녕!rest")); // Ok(("rest", "안녕!"))
    println!("{:?}", parse_three_bytes(b"abcdef")); // slices: b"def", b"abc"
    // Despite the bytes module name, take(n) counts Unicode scalars for &str,
    // and bytes for &[u8]. Unicode scalars aren't necessarily displayed characters.

    println!("{:?}", parse_before_colon("name:value")); // Ok((":value", "name"))
    println!("{:?}", parse_before_colon("no delimiter")); // Err(...)
    // take_until DOES NOT consume the delimiter; it can return an empty slice.

    println!("{:?}", parse_identifier("user_12!")); // Ok(("!", "user_12"))
    // This simple identifier allows leading digits; stricter rules come later.
}

// ============================================================================
// 5. Numeric output instead of digit text
// ============================================================================

fn parse_integer(input: &str) -> IResult<&str, i32> {
    parse_i32.parse(input)
}

fn parse_float(input: &str) -> IResult<&str, f64> {
    double.parse(input)
}

fn demo_numbers() {
    println!("{:?}", parse_digits("42!")); // Ok(("!", "42")): output &str
    println!("{:?}", parse_integer("-42!")); // Ok(("!", -42)): output i32
    println!("{:?}", parse_integer("+7 rest")); // Ok((" rest", 7))
    println!("{:?}", parse_float("1.25e2 rest")); // Ok((" rest", 125.0))
    println!("{:?}", parse_integer("999999999999999999")); // Err(...): i32 overflow
    // character::complete::i32 parses decimal TEXT, not four binary bytes.
    // For binary numbers, nom::number has parsers such as be_u32 / le_u32.
    // No whitespace is consumed unless you explicitly parse it.
}

// ============================================================================
// 6. Feed the remainder into the next parser
// ============================================================================

fn parse_assignment(input: &str) -> IResult<&str, (&str, i32)> {
    let (input, name) = alpha1.parse(input)?;
    let (input, _) = space0.parse(input)?;
    let (input, _) = char('=').parse(input)?;
    let (input, _) = space0.parse(input)?;
    let (input, value) = parse_integer(input)?;
    Ok((input, (name, value)))
}

fn demo_remainder() {
    println!("{:?}", parse_assignment("count = 42;")); // Ok((";", ("count", 42)))
    println!("{:?}", parse_assignment("count=-7")); // Ok(("", ("count", -7)))
    println!("{:?}", parse_assignment("count:42")); // Err(...): expected '='
    // Shadowing input tracks the unconsumed slice at every step.
    // ? returns immediately on parser errors; _ discards punctuation / spaces.
    // This manual sequence shows the data flow before learning combinators.
    // Full-input validation with all_consuming is covered in the next guide.
}

// ============================================================================
// 7. Complete vs streaming; inspect errors
// ============================================================================

fn demo_errors_streaming() {
    let complete: IResult<&str, &str> = tag("HELLO").parse("HEL");
    let streaming: IResult<&str, &str> = nom::bytes::streaming::tag("HELLO").parse("HEL");
    println!("complete: {complete:?}"); // Err(Error(...))
    println!("streaming: {streaming:?}"); // Err(Incomplete(...))
    // complete: the supplied input is all that's available; short input is an error.
    // streaming: this may be a chunk; more input could make the parser succeed.
    // 'complete' does NOT mean consume all input; it describes input availability.
    // Streaming parsers don't fetch or buffer more data for you.

    match parse_assignment("count:42") {
        Ok((rest, value)) => println!("value: {value:?}; remaining: {rest:?}"),
        Err(nom::Err::Error(error)) => {
            println!("error at {:?}: {:?}", error.input, error.code);
            // error at ":42": Char (input at the point that failed)
        }
        Err(nom::Err::Failure(error)) => println!("committed failure: {error:?}"),
        Err(nom::Err::Incomplete(needed)) => println!("need more input: {needed:?}"),
    }
    // Error: recoverable mismatch; a combinator may try another parser.
    // Failure: committed error; don't retry alternatives (cut comes later).
    // Incomplete: more input is needed under streaming semantics.
}

// ============ //
//    main()    //
// ============ //

fn main() {
    println!();

    demo_result();
    println!("\n============================\n");
    demo_exact();
    println!("\n============================\n");
    demo_runs_whitespace();
    println!("\n============================\n");
    demo_take();
    println!("\n============================\n");
    demo_numbers();
    println!("\n============================\n");
    demo_remainder();
    println!("\n============================\n");
    demo_errors_streaming();
}

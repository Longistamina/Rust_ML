/*
nom combinators: build larger parsers from small parsers.

1. Sequence: tuples, preceded, terminated
2. Delimiters / separators: delimited, separated_pair
3. Alternatives: alt and value
4. Optional input: opt
5. Repetition: many0, many1, count
6. Separated lists and whitespace
7. Transform outputs: map, map_res
8. Preserve text / validate values: recognize, verify
9. Check without consuming: peek, not
10. Commit to a branch: cut
11. Require the entire input: all_consuming

Setup (same chapter project as 01_basic.rs):
    cargo add nom@8

Keep this file beside Cargo.toml and add:
    [[bin]]
    name = "02_combinators"
    path = "02_combinators.rs"

Run:
    cargo run --bin 02_combinators

A combinator takes parsers / settings and produces another parser.
Call .parse(input) with nom::Parser in scope (nom 8).
All examples use complete input; parser success may still leave a remainder.
Reference: https://docs.rs/nom/8.0.0/nom/
*/

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, digit1, i32 as integer, space0};
use nom::combinator::{all_consuming, cut, map, map_res, not, opt, peek, recognize, value, verify};
use nom::multi::{count, many0, many1, separated_list0, separated_list1};
use nom::sequence::{delimited, preceded, separated_pair, terminated};
use nom::{IResult, Parser};

// ============================================================================
// 1. Sequence: tuples, preceded, terminated
// ============================================================================

fn parse_tuple(input: &str) -> IResult<&str, (&str, char, i32)> {
    (alpha1, char('='), integer).parse(input)
}

fn parse_prefixed_number(input: &str) -> IResult<&str, i32> {
    preceded(tag("id="), integer).parse(input)
}

fn parse_number_semicolon(input: &str) -> IResult<&str, i32> {
    terminated(integer, char(';')).parse(input)
}

fn demo_sequence() {
    println!("{:?}", parse_tuple("count=42!")); // Ok(("!", ("count", '=', 42)))
    println!("{:?}", parse_prefixed_number("id=7!")); // Ok(("!", 7))
    println!("{:?}", parse_number_semicolon("12;next")); // Ok(("next", 12))
    // Tuple: keep every output. preceded: discard first. terminated: discard last.
    // Every parser runs against the remainder left by the previous one.
    // nom 8 tuples implement Parser directly; sequence::tuple() is deprecated.
}

// ============================================================================
// 2. Delimiters and separators
// ============================================================================

fn parse_parenthesized(input: &str) -> IResult<&str, i32> {
    delimited(char('('), integer, char(')')).parse(input)
}

fn parse_assignment(input: &str) -> IResult<&str, (&str, i32)> {
    separated_pair(alpha1, char('='), integer).parse(input)
}

fn demo_delimiters() {
    println!("{:?}", parse_parenthesized("(42)!")); // Ok(("!", 42))
    println!("{:?}", parse_parenthesized("(42")); // Err(...): missing ')'
    println!("{:?}", parse_assignment("count=42;")); // Ok((";", ("count", 42)))
    // delimited(open, body, close): keep body; require both delimiters.
    // separated_pair(left, sep, right): keep (left, right); discard sep.
    // Whitespace is explicit: "( 42 )" doesn't match the parser above.
}

// ============================================================================
// 3. Alternatives: alt and value
// ============================================================================

fn parse_boolean(input: &str) -> IResult<&str, bool> {
    alt((value(true, tag("true")), value(false, tag("false")))).parse(input)
}

fn demo_alternatives() {
    println!("{:?}", parse_boolean("true!")); // Ok(("!", true))
    println!("{:?}", parse_boolean("false!")); // Ok(("!", false))
    println!("{:?}", parse_boolean("yes")); // Err(...)

    let result: IResult<&str, &str> = alt((tag("cat"), tag("caterpillar"))).parse("caterpillar");
    println!("{result:?}"); // Ok(("erpillar", "cat"))
    // alt tries branches in order; first SUCCESS wins, not longest match.
    // A recoverable Error retries from the same original input.
    // Failure / Incomplete stop alternatives. value replaces a successful output.
    // Prefix matching means "trueish" parses as true with remainder "ish".
}

// ============================================================================
// 4. Optional input: opt
// ============================================================================

fn parse_optional_unit(input: &str) -> IResult<&str, (i32, Option<&str>)> {
    (integer, opt(tag("kg"))).parse(input)
}

fn demo_optional() {
    println!("{:?}", parse_optional_unit("12kg!")); // Ok(("!", (12, Some("kg"))))
    println!("{:?}", parse_optional_unit("12!")); // Ok(("!", (12, None)))
    println!("{:?}", parse_optional_unit("12k!")); // Ok(("k!", (12, None)))
    // opt turns a recoverable mismatch into None and restores the child's input.
    // It doesn't swallow Failure or Incomplete. Invalid optional text may remain;
    // use full-input validation when leftovers should be rejected.
}

// ============================================================================
// 5. Repetition: many0, many1, count
// ============================================================================

fn demo_repetition() {
    let result: IResult<&str, Vec<&str>> = many0(tag("ha")).parse("hahaha!");
    println!("{result:?}"); // Ok(("!", ["ha", "ha", "ha"]))
    let result: IResult<&str, Vec<&str>> = many0(tag("ha")).parse("!");
    println!("{result:?}"); // Ok(("!", []))
    let result: IResult<&str, Vec<&str>> = many1(tag("ha")).parse("!");
    println!("{result:?}"); // Err(...): at least one required
    let result: IResult<&str, Vec<&str>> = count(tag("ha"), 2).parse("hahaha!");
    println!("{result:?}"); // Ok(("ha!", ["ha", "ha"]))
    // many0 / many1 stop on recoverable mismatches, leaving unmatched input.
    // An unbounded repetition's child MUST consume input on success.
    // many0(space0) is wrong: space0 can succeed without progress; nom errors
    // rather than looping forever. Use space1 if repetition is actually needed.
}

// ============================================================================
// 6. Separated lists and whitespace
// ============================================================================

fn parse_number_list(input: &str) -> IResult<&str, Vec<i32>> {
    delimited(
        char('['),
        separated_list0(char(','), delimited(space0, integer, space0)),
        char(']'),
    ).parse(input)
}

fn demo_lists() {
    println!("{:?}", parse_number_list("[1, 2, -3]!")); // Ok(("!", [1, 2, -3]))
    println!("{:?}", parse_number_list("[]")); // Ok(("", []))
    println!("{:?}", parse_number_list("[1,]")); // Err(...): trailing comma rejected
    // This parser intentionally doesn't allow spaces in an empty list: "[ ]".
    let result: IResult<&str, Vec<i32>> = separated_list1(char(','), integer).parse("1,2,");
    println!("{result:?}"); // Ok((",", [1, 2]))
    // A separator with no following element is left in the remainder.
    // list0 permits zero elements; list1 requires at least one.
    // A list by itself can stop early; enclosing delimiters catch some leftovers.
    // No quoted fields / escaped commas here: this is not a full CSV parser.
}

// ============================================================================
// 7. Transform outputs: map and map_res
// ============================================================================

fn parse_uppercase(input: &str) -> IResult<&str, String> {
    map(alpha1, |s: &str| s.to_uppercase()).parse(input)
}

fn parse_small_unsigned(input: &str) -> IResult<&str, u8> {
    map_res(digit1, str::parse::<u8>).parse(input)
}

fn demo_transform() {
    println!("{:?}", parse_uppercase("cat123")); // Ok(("123", "CAT"))
    println!("{:?}", parse_small_unsigned("255!")); // Ok(("!", 255))
    println!("{:?}", parse_small_unsigned("256!")); // Err(...): overflow
    // map: transform output with a function that returns a value.
    // map_res: transform with a function returning Result; Err becomes a parse error.
    // Under the default error type, conversion errors become ErrorKind::MapRes;
    // the original ParseIntError detail isn't retained.
    // Parser::map(...) is also available: alpha1.map(...).parse(input).
}

// ============================================================================
// 8. Preserve matched text / validate values
// ============================================================================

fn parse_signed_text(input: &str) -> IResult<&str, &str> {
    recognize((opt(char('-')), digit1)).parse(input)
}

fn parse_percentage(input: &str) -> IResult<&str, i32> {
    verify(integer, |n: &i32| (0..=100).contains(n)).parse(input)
}

fn demo_recognize_verify() {
    println!("{:?}", parse_signed_text("-042!")); // Ok(("!", "-042"))
    println!("{:?}", parse_percentage("75!")); // Ok(("!", 75))
    println!("{:?}", parse_percentage("101!")); // Err(...): Verify
    // recognize returns the input slice consumed by the child, discarding its output.
    // verify keeps the child's output only if a predicate accepts it.
    // Unlike numeric parsing, recognize preserves formatting such as leading zeros.
}

// ============================================================================
// 9. Check without consuming: peek and not
// ============================================================================

fn parse_keyword(input: &str) -> IResult<&str, &str> {
    terminated(tag("let"), not(alpha1)).parse(input)
}

fn demo_lookahead() {
    let result: IResult<&str, &str> = peek(tag("HTTP")).parse("HTTP/1.1");
    println!("{result:?}"); // Ok(("HTTP/1.1", "HTTP")): original input remains
    println!("{:?}", parse_keyword("let x")); // Ok((" x", "let"))
    println!("{:?}", parse_keyword("letter")); // Err(...)
    // peek checks a parser and returns its output without consuming input.
    // not succeeds with () if the child returns a recoverable Error.
    // These are parser-level lookahead; they don't require fancy_regex.
    // Our boundary check only excludes ASCII letters: "let1" still succeeds.
    // A real keyword parser should enforce its language's identifier rules.
}

// ============================================================================
// 10. Commit to a branch: cut
// ============================================================================

fn parse_loose_choice(input: &str) -> IResult<&str, &str> {
    alt((preceded(tag("id="), digit1), tag("id=unknown"))).parse(input)
}

fn parse_committed_choice(input: &str) -> IResult<&str, &str> {
    alt((preceded(tag("id="), cut(digit1)), tag("id=unknown"))).parse(input)
}

fn demo_cut() {
    println!("{:?}", parse_loose_choice("id=unknown")); // Ok(("", "id=unknown"))
    println!("{:?}", parse_committed_choice("id=unknown")); // Err(Failure(...))
    // After recognizing id=, cut requires digits; alt cannot retry the next branch.
    // Put cut AFTER a distinguishing prefix, when choosing that grammar branch
    // should commit. A cut placed too early can prevent valid alternatives.
    // This intentionally changes the accepted language; it's not just optimization.
}

// ============================================================================
// 11. Full-input validation: all_consuming
// ============================================================================

// fn parse_assignment(input: &str) -> IResult<&str, (&str, i32)> {
//     separated_pair(alpha1, char('='), integer).parse(input)
// }

fn parse_complete_assignment(input: &str) -> IResult<&str, (&str, i32)> {
    all_consuming(delimited(space0, parse_assignment, space0)).parse(input)
}

fn demo_all_consuming() {
    println!("{:?}", parse_assignment("count=42garbage")); // Ok(("garbage", ("count", 42)))
    println!("{:?}", parse_complete_assignment("count=42garbage")); // Err(...): Eof
    println!("{:?}", parse_complete_assignment(" count=42 \t")); // Ok(("", ("count", 42)))
    // all_consuming requires an empty remainder AFTER the child succeeds.
    // space0 permits surrounding spaces / tabs, not newlines.
    // This doesn't permit whitespace around '='; use explicit whitespace parsers
    // there if the grammar should accept "count = 42".
    // Next: combine these tools into structs / enums in 03_structured_parsing.rs.
}

// ============ //
//    main()    //
// ============ //

fn main() {
    println!();

    demo_sequence();
    println!("\n============================\n");
    demo_delimiters();
    println!("\n============================\n");
    demo_alternatives();
    println!("\n============================\n");
    demo_optional();
    println!("\n============================\n");
    demo_repetition();
    println!("\n============================\n");
    demo_lists();
    println!("\n============================\n");
    demo_transform();
    println!("\n============================\n");
    demo_recognize_verify();
    println!("\n============================\n");
    demo_lookahead();
    println!("\n============================\n");
    demo_cut();
    println!("\n============================\n");
    demo_all_consuming();
}

// 02_parsing_formatting.rs
// ========================
// Setup (inside chap_07_jiff_datetime):
//     cargo add jiff@0.2
// Run:
//     cargo run --bin 02_parsing_formatting
//
// Choose the target TYPE before choosing the parser:
//     Date/Time/DateTime: calendar and clock fields without a timezone.
//     Timestamp: an instant; the input must establish its UTC offset.
//     Zoned: an instant with a timezone representation.
//
// Standard text: text.parse::<T>() -> Result<T, jiff::Error>
// Custom text:   T::strptime(FORMAT, TEXT) -> Result<T, jiff::Error>
// Formatting:    value.strftime(FORMAT) -> a lazy Display adapter
//               strtime::format(FORMAT, value) -> Result<String, jiff::Error>
//
// Important Python difference:
//     Python: datetime.strptime(TEXT, FORMAT)
//     Jiff:   DateTime::strptime(FORMAT, TEXT)
// Jiff's standard parsing is not pandas-style natural-language inference.
//
// Official docs:
// https://docs.rs/jiff/latest/jiff/fmt/temporal/index.html
// https://docs.rs/jiff/latest/jiff/fmt/strtime/index.html
// https://docs.rs/jiff/latest/jiff/fmt/rfc2822/index.html

use jiff::{
    civil::{Date, DateTime, Time},
    fmt::{rfc2822, strtime, temporal::DateTimePrinter},
    Timestamp, Zoned,
};

fn main() -> Result<(), jiff::Error> {
    println!();

    demo_standard_civil()?;
    println!("\n============================\n");

    demo_standard_instants()?;
    println!("\n============================\n");

    demo_custom_parsing()?;
    println!("\n============================\n");

    demo_custom_formatting()?;
    println!("\n============================\n");

    demo_fractional_seconds()?;
    println!("\n============================\n");

    demo_offsets_and_zones()?;
    println!("\n============================\n");

    demo_configurable_printer()?;
    println!("\n============================\n");

    demo_round_trips()?;
    println!("\n============================\n");

    demo_rfc2822()?;
    println!("\n============================\n");

    demo_errors()?;
    println!("\n============================\n");

    demo_batch_parsing();

    Ok(())
}

// 01. Parse standard date/time strings; print with Display
// -------------------------------------------------------
fn demo_standard_civil() -> Result<(), jiff::Error> {
    println!("01. Standard civil parsing");

    let day: Date = "2024-06-18".parse()?;
    let clock = "14:30:45.123456789".parse::<Time>()?;
    let dt: DateTime = "2024-06-18T14:30:45".parse()?;
    println!("Date:     {day}");
    println!("Time:     {clock}");
    println!("DateTime: {dt}");
    // Type annotations or turbofish ::<Time> tell parse which type to produce.
    // ? propagates an unexpected parse failure; deliberate failures appear later.

    let owned: String = dt.to_string();
    println!("Owned String: {owned}");
    // println!("{dt}") uses Display directly; to_string() allocates a String.
    // None of these civil values contains a timezone or identifies an instant.

    Ok(())
}

// 02. Parse an instant with an offset; parse a zoned datetime
// ---------------------------------------------------------
fn demo_standard_instants() -> Result<(), jiff::Error> {
    println!("02. Standard Timestamp and Zoned parsing");

    let utc: Timestamp = "2024-06-18T03:00:00Z".parse()?;
    let offset: Timestamp = "2024-06-18T12:00:00+09:00".parse()?;
    let seoul: Zoned = "2024-06-18T12:00:00+09:00[Asia/Seoul]".parse()?;
    println!("UTC input:    {utc}");
    println!("Offset input: {offset}"); // normalized UTC display: 03:00:00Z
    println!("Zoned input:  {seoul}");
    println!("All identify the same instant: {}", utc == offset && utc == seoul.timestamp());
    // Z means UTC. +09:00 is an offset; [Asia/Seoul] preserves an IANA zone name.
    // Timestamp retains the instant, not the input's original offset spelling.
    // Zoned retains a timezone; its Display includes the zone annotation.
    // A numeric offset alone does not encode a region's future DST rules.

    Ok(())
}

// 03. Parse custom formats (format argument FIRST)
// ------------------------------------------------
fn demo_custom_parsing() -> Result<(), jiff::Error> {
    println!("03. Custom strptime formats");

    for (format, input) in [
        ("%d/%m/%Y", "18/02/2024"),
        ("%d-%m-%Y", "18-02-2024"),
        ("%m/%d/%Y", "02/18/2024"),
        ("%Y%m%d", "20240218"),
        ("%d %b %Y", "18 Feb 2024"),
    ] {
        let parsed = Date::strptime(format, input)?;
        println!("{input:?} using {format:?} -> {parsed}");
    }

    let dt = DateTime::strptime("%d/%m/%Y %H:%M:%S", "18/02/2024 14:30:45")?;
    let clock = Time::strptime("%I:%M %p", "02:30 PM")?;
    println!("Custom datetime: {dt}");
    println!("12-hour input:   {clock}");
    // Parse into Date for date-only data; explicitly attach a time afterward
    // if you need a DateTime. Do not assume Python's missing-field defaults.
    // Prefer separators; compact numeric formats can become ambiguous.

    Ok(())
}

// 04. Human-readable output and common format codes
// -------------------------------------------------
fn demo_custom_formatting() -> Result<(), jiff::Error> {
    println!("04. Custom strftime output");

    let dt: DateTime = "2024-02-18T14:30:45".parse()?;
    for format in [
        "%Y-%m-%d",                  // 2024-02-18
        "%A %d/%m/%Y",               // Sunday 18/02/2024
        "%a, %b %d, %Y",             // Sun, Feb 18, 2024
        "%I:%M %p",                  // 02:30 PM
        "%H:%M:%S",                  // 14:30:45
        "day=%j, weekday=%u, %%",     // ordinal day; Monday=1..Sunday=7; literal %
    ] {
        println!("{format:?} -> {}", dt.strftime(format));
    }

    let label: String = dt.strftime("%d-%m-%Y").to_string();
    println!("Owned label: {label}");
    // strftime returns a Display adapter; to_string materializes its output.
    // %m=month, %M=minute; %H=24-hour clock, %I=12-hour clock with %p.
    // Names are English: Jiff's strtime formatter does not provide localization.
    // Use matching fields: a Time cannot supply %Y, a DateTime cannot supply %z.

    Ok(())
}

// 05. Fractions: digits versus optional dot, and fixed output precision
// -------------------------------------------------------------------
fn demo_fractional_seconds() -> Result<(), jiff::Error> {
    println!("05. Fractional seconds");

    let precise = DateTime::strptime("%Y-%m-%d %H:%M:%S.%f", "2024-06-18 14:30:45.123456789")?;
    println!("Parsed with %f: {precise}");
    println!("Three digits: {}", precise.strftime("%H:%M:%S.%3f"));
    println!("Six digits:   {}", precise.strftime("%H:%M:%S.%6f"));
    println!("Nine digits:  {}", precise.strftime("%H:%M:%S.%9f"));
    // %f handles the fractional digits; the literal dot above is separate.
    // Unlike Python datetime's microsecond-limited %f, Jiff supports up to 9 digits.

    for input in ["14:30:45", "14:30:45.123"] {
        let clock = Time::strptime("%H:%M:%S%.f", input)?;
        println!("Optional fraction: {input:?} -> {clock}");
    }
    // %.f includes an optional dot and fraction. Do not prepend another dot.
    // Reducing printed precision truncates digits; it does not round the value.

    Ok(())
}

// 06. Custom offset parsing versus named-zone parsing
// ---------------------------------------------------
fn demo_offsets_and_zones() -> Result<(), jiff::Error> {
    println!("06. Offsets and named zones in custom formats");

    let instant = Timestamp::strptime("%Y-%m-%d %H:%M:%S %:z", "2024-06-18 12:00:00 +09:00")?;
    let zoned = Zoned::strptime("%Y-%m-%d %H:%M:%S %Q", "2024-06-18 12:00:00 Asia/Seoul")?;
    println!("Offset-based instant: {instant}");
    println!("Named-zone datetime:  {zoned}");
    println!("Compact offset (%z): {}", zoned.strftime("%z"));
    println!("Colon offset (%:z):  {}", zoned.strftime("%:z"));
    println!("Zone identifier (%Q): {}", zoned.strftime("%Q"));
    println!("Abbreviation (%Z):    {}", zoned.strftime("%Z"));
    // %z: +0900; %:z: +09:00; %Q: IANA identifier (or offset fallback).
    // %Z is for FORMATTING abbreviations, not parsing them with strptime.
    // Abbreviations such as CST are ambiguous; prefer offsets or IANA names.
    // Ambiguous/nonexistent local times and offset conflicts belong in file 04.

    Ok(())
}

// 07. Configure the standard printer
// -----------------------------------
fn demo_configurable_printer() -> Result<(), jiff::Error> {
    println!("07. Standard printer configuration");

    let dt: DateTime = "2024-06-18T14:30:45.123456789".parse()?;
    let printer = DateTimePrinter::new().separator(b' ').precision(Some(3));
    let label = printer.datetime_to_string(&dt);
    println!("Default:    {dt}");
    println!("Configured: {label}"); // 2024-06-18 14:30:45.123
    println!("Unchanged fraction: {}", dt.subsec_nanosecond()); // 123456789
    // precision(Some(3)) prints exactly 3 fractional digits, truncating extras.
    // precision(Some(0)) omits fractions; precision(None) uses automatic precision.
    // This changes the STRING, not the datetime. Actual rounding comes in file 05.

    Ok(())
}

// 08. Round trips: preserve enough information for your purpose
// -------------------------------------------------------------
fn demo_round_trips() -> Result<(), jiff::Error> {
    println!("08. Round trips");

    let original: Zoned = "2024-06-18T12:00:00.123456789+09:00[Asia/Seoul]".parse()?;
    let encoded = original.to_string();
    let decoded: Zoned = encoded.parse()?;
    println!("Full zoned string: {encoded}");
    println!("Same instant: {}", original.timestamp() == decoded.timestamp());
    println!("Same canonical output: {}", original.to_string() == decoded.to_string());
    // Zoned equality alone compares instants, so it cannot prove zone retention.

    let utc_text = original.timestamp().to_string();
    let restored: Timestamp = utc_text.parse()?;
    println!("UTC instant string: {utc_text}");
    println!("Instant preserved: {}", restored == original.timestamp());
    // The UTC string preserves the instant but does not record Asia/Seoul.

    let custom = "%Y/%m/%d %H:%M:%S.%9f";
    let local = original.datetime();
    let text = local.strftime(custom).to_string();
    let back = DateTime::strptime(custom, &text)?;
    println!("Custom civil round trip: {}", back == local);
    // Omitting the fraction, date, offset, or zone may lose useful information.
    // For machine storage, prefer full standard strings over display-only labels.

    Ok(())
}

// 09. Bonus: RFC 2822 email-style datetime input
// ---------------------------------------------
fn demo_rfc2822() -> Result<(), jiff::Error> {
    println!("09. RFC 2822 email-style datetime input");

    let mail = rfc2822::parse("Tue, 18 Jun 2024 12:00:00 +0900")?;
    println!("Parsed: {mail}");
    println!("Instant: {}", mail.timestamp());
    // Use the dedicated parser for email headers instead of building a regex.
    // The numeric offset here does not establish an IANA region such as Seoul.

    Ok(())
}

// 10. Expected parsing and formatting errors
// -------------------------------------------
fn demo_errors() -> Result<(), jiff::Error> {
    println!("10. Handle errors without stopping the guide");

    let clock: Time = "14:30:45".parse()?;
    println!("Valid time format: {}", strtime::format("%H:%M", clock)?);

    println!();

    for input in ["2023-02-29", "2024-13-01", "not a date"] {
        match input.parse::<Date>() {
            Ok(day) => println!("Parsed: {day}"),
            Err(error) => println!("Invalid date {input:?}: {error}"),
        }
        println!();
    }

    match "2024-06-18T12:00:00".parse::<Timestamp>() {
        Ok(ts) => println!("Parsed instant: {ts}"),
        Err(error) => println!("Missing UTC offset: {error}"),
    }

    println!();

    match Date::strptime("%Y-%m-%d %A", "2024-02-18 Monday") {
        Ok(day) => println!("Parsed: {day}"),
        Err(error) => println!("Weekday disagrees with date: {error}"), // was Sunday
    }

    println!();

    match strtime::format("%Y", clock) {
        Ok(text) => println!("Formatted: {text}"),
        Err(error) => println!("A Time has no year to format: {error}"),
    }
    // Prefer fallible strtime::format for user-supplied format strings.
    // clock.strftime("%Y").to_string() can panic when the adapter is formatted.
    // A displayed format error is handled here as Result instead.

    Ok(())
}

// 11. Parse a collection: preserve errors or represent invalid rows as None
// -------------------------------------------------------------------------
fn demo_batch_parsing() {
    println!("11. Collection parsing policies");

    let inputs = ["18/02/2024", "01/03/2024"];
    let strict: Result<Vec<Date>, jiff::Error> = inputs.iter()
        .map(|text| Date::strptime("%d/%m/%Y", text))
        .collect();
    println!("All valid: {strict:?}");

    println!();

    let mixed = ["18/02/2024", "wrong", "01/03/2024"];
    let strict: Result<Vec<Date>, jiff::Error> = mixed.iter()
        .map(|text| Date::strptime("%d/%m/%Y", text))
        .collect();
    println!("Stop at first invalid input: {strict:?}");

    println!();

    let tolerant: Vec<Option<Date>> = mixed.iter()
        .map(|text| Date::strptime("%d/%m/%Y", text).ok())
        .collect();
    println!("Keep positions; invalid becomes None: {tolerant:?}");
    // This explicit policy resembles pandas errors='coerce'/Polars strict=False.
    // .ok() discards the error details; keep Result values when you need them.
    // Iteration/collection are standard Rust operations, not a Jiff dataframe API.
}

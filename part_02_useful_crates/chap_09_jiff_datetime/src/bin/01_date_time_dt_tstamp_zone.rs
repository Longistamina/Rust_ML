// 01_date_time_dt_tstamp_zone.rs
// ===============================
// Setup (inside chap_07_jiff_datetime):
//     cargo add jiff@0.2
// Run:
//     cargo run --bin 01_basic
//
// Main types:
//     civil::Date     calendar date:                  2024-06-18
//     civil::Time     clock time:                     14:30:45
//     civil::DateTime date + clock time, no zone:     2024-06-18T14:30:45
//     Timestamp       precise instant, displayed UTC: 2024-06-18T05:30:45Z
//     Zoned           instant + zone + local fields:  ...+09:00[Asia/Seoul]
//
// A civil DateTime alone does not identify a unique instant on the timeline.
// Timestamp is NOT a direct equivalent of pandas.Timestamp: pandas' type can
// represent naive calendar fields too, whereas Jiff separates these concepts.
// Span and SignedDuration are introduced in 03_spans_arithmetic.rs.
//
// Docs (Jiff 0.2):
// https://docs.rs/jiff/latest/jiff/
// https://docs.rs/jiff/latest/jiff/civil/index.html
// https://docs.rs/jiff/latest/jiff/struct.Timestamp.html
// https://docs.rs/jiff/latest/jiff/struct.Zoned.html

use jiff::{civil::{date, Date, DateTime, Time}, tz::TimeZone, Timestamp, Zoned};

fn main() -> Result<(), jiff::Error> {
    println!();

    demo_construct_civil()?;
    println!("\n============================\n");

    demo_components()?;
    println!("\n============================\n");

    demo_split_and_combine()?;
    println!("\n============================\n");

    demo_current_time()?;
    println!("\n============================\n");

    demo_unix_timestamps()?;
    println!("\n============================\n");

    demo_civil_zoned_timestamp()?;
    println!("\n============================\n");

    demo_comparisons()?;
    println!("\n============================\n");

    demo_change_fields()?;
    println!("\n============================\n");

    demo_validation_and_limits();

    Ok(())
}

// 01. Construct dates, times, and civil datetimes
// -----------------------------------------------
fn demo_construct_civil() -> Result<(), jiff::Error> {
    println!("01. Construct civil values");

    let birthday = Date::new(1890, 5, 19)?;
    let clock = Time::new(14, 30, 45, 123_456_789)?;
    let appointment = DateTime::new(2024, 6, 18, 14, 30, 45, 123_456_789)?;
    println!("Date:     {birthday}");
    println!("Time:     {clock}");
    println!("DateTime: {appointment}");
    // Constructors return Result: invalid calendar/clock fields are errors.
    // The final argument is the FULL fraction of one second in nanoseconds.
    // Python microsecond=123456 corresponds to Jiff subsecond=123_456_000.

    let short = date(2024, 6, 18).at(14, 30, 45, 0);
    println!("Convenience construction: {short}");
    // date(...).at(...) is concise for known-valid literals, but invalid inputs
    // panic. Prefer the fallible new(...) constructors for runtime/user input.
    // ? propagates unexpected errors to main; expected errors are matched below.

    Ok(())
}

// 02. Extract calendar and clock fields
// -------------------------------------
fn demo_components() -> Result<(), jiff::Error> {
    println!("02. Extract fields and fractional seconds");

    let dt = DateTime::new(2024, 6, 18, 14, 30, 45, 123_456_789)?;
    println!("year={}, month={}, day={}", dt.year(), dt.month(), dt.day());
    println!("hour={}, minute={}, second={}", dt.hour(), dt.minute(), dt.second());
    println!("Weekday: {:?}", dt.weekday());

    println!();

    println!("millisecond component: {}", dt.millisecond()); // 123
    println!("microsecond component: {}", dt.microsecond()); // 456
    println!("nanosecond component:  {}", dt.nanosecond());  // 789
    println!("Full fraction in ns:   {}", dt.subsec_nanosecond()); // 123456789
    println!("Full fraction in us:   {}", dt.subsec_nanosecond() / 1_000); // 123456
    // The three component methods split the fraction into groups of 3 digits.
    // Python datetime.microsecond instead returns the full fraction in us.
    // Getters are methods in Rust: year(), not the Python attribute .year.
    // More calendar properties and ISO week numbering belong in file 05.

    Ok(())
}

// 03. Extract Date/Time, or combine them into DateTime
// ---------------------------------------------------
fn demo_split_and_combine() -> Result<(), jiff::Error> {
    println!("03. Split and combine");

    let dt = DateTime::new(2024, 6, 18, 14, 30, 45, 0)?;
    let day: Date = dt.date();
    let clock: Time = dt.time();
    let rebuilt = DateTime::from_parts(day, clock);
    println!("Date part: {day}");
    println!("Time part: {clock}");
    println!("Recombined: {rebuilt}");
    println!("Matches original: {}", rebuilt == dt);
    // Splitting needs no timezone conversion: these are just calendar fields.
    // Date, Time, DateTime, and Timestamp are Copy; Zoned is Clone, not Copy.

    Ok(())
}

// 04. Current instant, current local time, and today's date
// --------------------------------------------------------
fn demo_current_time() -> Result<(), jiff::Error> {
    println!("04. Current time");

    let now = Timestamp::now();
    let utc = now.to_zoned(TimeZone::UTC);
    let seoul = now.in_tz("Asia/Seoul")?;
    println!("Current instant (UTC): {now}");
    println!("Same instant in UTC:   {utc}");
    println!("Same instant in Seoul: {seoul}");
    println!("Today's Seoul date:    {}", seoul.date());

    println!();

    let local = Zoned::now();
    println!("System-local datetime: {local}");
    println!("System-local date:     {}", local.date());
    // Zoned::now() uses the system's detected timezone (with UTC fallback).
    // Do not assume the machine running this file is configured for Seoul.
    // "Today" depends on the timezone, especially around midnight.
    // This second clock read may differ slightly from the first Timestamp::now().
    // For elapsed-time measurement, use std::time::Instant instead of wall time.

    Ok(())
}

// 05. Unix epoch and explicit timestamp units
// -------------------------------------------
fn demo_unix_timestamps() -> Result<(), jiff::Error> {
    println!("05. Unix timestamps");

    println!("Epoch: {}", Timestamp::UNIX_EPOCH);
    let seconds = Timestamp::from_second(1_609_459_200)?;
    let millis = Timestamp::from_millisecond(1_609_459_200_123)?;
    let micros = Timestamp::from_microsecond(1_609_459_200_123_456)?;
    let nanos = Timestamp::from_nanosecond(1_609_459_200_123_456_789_i128)?;
    println!("From seconds:      {seconds}"); // 2021-01-01T00:00:00Z
    println!("From milliseconds: {millis}");
    println!("From microseconds: {micros}");
    println!("From nanoseconds:  {nanos}");

    println!();

    println!("Seconds since epoch:      {}", nanos.as_second());
    println!("Milliseconds since epoch: {}", nanos.as_millisecond());
    println!("Microseconds since epoch: {}", nanos.as_microsecond());
    println!("Nanoseconds since epoch:  {}", nanos.as_nanosecond());
    // These are TOTAL counts since the epoch, not clock fields or remainders.
    // Coarser units lose precision; nanosecond counts use i128.
    // Integers avoid float rounding and make the chosen unit explicit.

    let before_epoch = Timestamp::from_second(-1)?;
    println!("One second before epoch: {before_epoch}"); // 1969-12-31T23:59:59Z

    Ok(())
}

// 06. Civil DateTime -> Zoned -> Timestamp (and back)
// --------------------------------------------------
fn demo_civil_zoned_timestamp() -> Result<(), jiff::Error> {
    println!("06. Calendar fields versus an instant");

    let civil = DateTime::new(2024, 6, 18, 12, 0, 0, 0)?;
    let seoul: Zoned = civil.in_tz("Asia/Seoul")?;
    let instant: Timestamp = seoul.timestamp();
    let utc = instant.to_zoned(TimeZone::UTC);
    println!("Unzoned fields: {civil}");
    println!("Interpreted in Seoul: {seoul}");
    println!("Precise instant: {instant}"); // 2024-06-18T03:00:00Z
    println!("UTC representation: {utc}");
    println!("Local fields extracted again: {}", seoul.datetime());
    println!("UTC offset: {}", seoul.offset());
    // in_tz on civil time INTERPRETS the fields in the selected zone.
    // Extracting datetime() drops zone/instant information; it does not convert.
    // Using in_tz on Timestamp instead DISPLAYS an already-known instant there.
    // DST ambiguity and converting between zones are covered carefully in file 04.

    Ok(())
}

// 07. Comparisons: civil order versus timeline order
// --------------------------------------------------
fn demo_comparisons() -> Result<(), jiff::Error> {
    println!("07. Compare dates, datetimes, and instants");

    let earlier = Date::new(2024, 6, 18)?;
    let later = Date::new(2024, 6, 19)?;
    println!("Date ordering: {}", earlier < later);

    let morning = DateTime::new(2024, 6, 18, 9, 0, 0, 0)?;
    let afternoon = DateTime::new(2024, 6, 18, 15, 0, 0, 0)?;
    println!("Civil field ordering: {}", morning < afternoon);
    // A civil comparison alone cannot establish real-world order across zones.

    let seoul = DateTime::new(2024, 6, 18, 12, 0, 0, 0)?.in_tz("Asia/Seoul")?;
    let utc = seoul.timestamp().to_zoned(TimeZone::UTC);
    println!("Seoul: {seoul}");
    println!("UTC:   {utc}");
    println!("Same timestamp: {}", seoul.timestamp() == utc.timestamp());
    println!("Zoned equality: {}", seoul == utc); // true: compares instants
    println!("Same civil fields: {}", seoul.datetime() == utc.datetime()); // false
    // Zoned ordering/equality follow the instant, not the printed local fields
    // or the zone name. Subtraction/durations come in file 03.

    Ok(())
}

// 08. Change fields with with() ... build()
// -----------------------------------------
fn demo_change_fields() -> Result<(), jiff::Error> {
    println!("08. Build a modified value");

    let original = DateTime::new(2024, 6, 18, 14, 30, 45, 123_456_789)?;
    let christmas = original.with().month(12).day(25).build()?;
    let midnight = original.with()
        .hour(0).minute(0).second(0).subsec_nanosecond(0)
        .build()?;
    println!("Original:  {original}");
    println!("Christmas: {christmas}");
    println!("Midnight:  {midnight}");
    // with() starts a builder; build() validates and returns a new DateTime.
    // The original value is unchanged. Unspecified fields retain their values.
    // Changing a field is not arithmetic: month(12) sets December, not +12 months.

    println!();

    match original.with().month(2).day(30).build() {
        Ok(value) => println!("Unexpected valid datetime: {value}"),
        Err(error) => println!("Expected invalid-date error: {error}"),
    }

    Ok(())
}

// 09. Invalid inputs and type limits
// ----------------------------------
fn demo_validation_and_limits() {
    println!("09. Validation and limits");

    println!("Minimum Date: {}", Date::MIN);
    println!("Maximum Date: {}", Date::MAX);
    println!("Minimum Time: {}", Time::MIN);
    println!("Maximum Time: {}", Time::MAX);
    // Jiff's civil year range is -9999..=9999 and includes year 0.
    // This differs from Python datetime's 1..=9999 year range.
    // Use each type's own limits: Timestamp limits are not civil DateTime limits.

    println!();

    match Date::new(2023, 2, 29) {
        Ok(value) => println!("Unexpected valid date: {value}"),
        Err(error) => println!("Expected non-leap-year error: {error}"),
    }

    println!();

    match Time::new(25, 0, 0, 0) {
        Ok(value) => println!("Unexpected valid time: {value}"),
        Err(error) => println!("Expected invalid-hour error: {error}"),
    }
    // Handle errors explicitly for user input; avoid unwrap() on untrusted dates.
    // Strings/custom formats are the subject of 02_parsing_formatting.rs.
}

// 03_span_duration_arithmetic.rs
// ====================================
// Setup (inside chap_07_jiff_datetime):
//     cargo add jiff@0.2
// Run:
//     cargo run --bin 03_spans_arithmetic
//
// Three duration concepts:
//     Span: calendar/clock units (years, months, days, hours, ...).
//     SignedDuration: fixed elapsed seconds + nanoseconds; can be negative.
//     std::time::Duration: fixed elapsed seconds + nanoseconds; nonnegative.
//
// Python timedelta/pandas.Timedelta model fixed durations, not calendar months.
// Use Span for calendar intentions; SignedDuration for fixed elapsed quantities.
// A calendar day need not be 24 elapsed hours in a timezone with transitions.
// ToSpan adds methods such as 2.hours() to signed integer types.
//
// Official docs:
// https://docs.rs/jiff/latest/jiff/struct.Span.html
// https://docs.rs/jiff/latest/jiff/struct.SignedDuration.html
// https://docs.rs/jiff/latest/jiff/struct.SpanRound.html
// https://docs.rs/jiff/latest/jiff/trait.ToSpan.html

use jiff::{
    civil::{Date, DateTime},
    RoundMode, SignedDuration, Span, SpanRound, Timestamp, ToSpan, Unit, Zoned,
};
use std::time::Duration;

fn main() -> Result<(), jiff::Error> {
    println!();

    demo_build_spans()?;
    println!("\n============================\n");

    demo_negative_spans();
    println!("\n============================\n");

    demo_civil_arithmetic()?;
    println!("\n============================\n");

    demo_month_ends()?;
    println!("\n============================\n");

    demo_differences()?;
    println!("\n============================\n");

    demo_components_and_totals()?;
    println!("\n============================\n");

    demo_span_arithmetic()?;
    println!("\n============================\n");

    demo_comparisons()?;
    println!("\n============================\n");

    demo_rounding_and_balancing()?;
    println!("\n============================\n");

    demo_signed_duration()?;
    println!("\n============================\n");

    demo_calendar_day_vs_hours()?;
    println!("\n============================\n");

    demo_conversions()?;
    println!("\n============================\n");

    demo_deadline()?;

    Ok(())
}

// 01. Build, parse, and display spans
// -----------------------------------
fn demo_build_spans() -> Result<(), jiff::Error> {
    println!("01. Span construction and display");

    let explicit = Span::new().days(5).hours(1).minutes(10).seconds(30);
    let concise = 5.days().hours(1).minutes(10).seconds(30);
    println!("ISO-style: {explicit}"); // P5DT1H10M30S
    println!("Friendly:  {concise:#}");
    println!("Empty span is zero: {}", Span::new().is_zero());
    // Builder methods SET a unit; they do not add to its previous value.
    println!("2.hours().hours(3): {}", 2.hours().hours(3)); // PT3H
    // To add spans, use checked_add (section 07).

    for input in ["P5DT1H10M30S", "5d 1h 10m 30s", "2 hours, 30 minutes"] {
        let span: Span = input.parse()?;
        println!("{input:?} -> {span} / {span:#}");
    }
    // In ISO duration strings, P1M=one month, PT1M=one minute.
    // In friendly strings, use mo for months and m for minutes.
    // Fallible setters such as Span::new().try_hours(n)? are available for
    // runtime values that might exceed the unit's supported range.

    Ok(())
}

// 02. A span has ONE sign applying to all its fields
// -------------------------------------------------
fn demo_negative_spans() {
    println!("02. Negative spans");

    let forward = 2.days().hours(3);
    let backward = -forward;
    println!("Forward: {forward:#}");
    println!("Backward: {backward:#}");
    println!("days={}, hours={}", backward.get_days(), backward.get_hours()); // -2, -3
    println!("Absolute: {:#}", backward.abs());
    println!("Negative: {}, sign={}", backward.is_negative(), backward.signum());

    let surprising = 2.days().hours(-3);
    println!("2.days().hours(-3): {surprising:#}"); // negative 2 days AND 3 hours
    // A negative setter makes the WHOLE span negative. This does not mean
    // 'two days minus three hours'. Use checked_sub with a reference if needed.
    // Unlike Python timedelta, Jiff does not use a negative-day/positive-seconds
    // representation for this negative span.
}

// 03. Add/subtract spans from civil dates and datetimes
// ----------------------------------------------------
fn demo_civil_arithmetic() -> Result<(), jiff::Error> {
    println!("03. Civil arithmetic");

    let today = Date::new(2024, 6, 18)?;
    println!("Original:  {today}");
    println!("Tomorrow:  {}", today.checked_add(1.day())?);
    println!("Yesterday: {}", today.checked_sub(1.day())?);
    println!("Negative addition: {}", today.checked_add((-1).day())?);

    let meeting: DateTime = "2024-06-18T23:30:00".parse()?;
    println!("Meeting: {meeting}");
    println!("Plus 2h 45m: {}", meeting.checked_add(2.hours().minutes(45))?);
    // Clock arithmetic carries into the next date: 2024-06-19T02:15:00.
    // Civil arithmetic has no timezone transitions to consult.
    // checked_* returns Result; operator + may panic on an invalid/overflowing
    // operation. We use checked_* so failures can be handled explicitly.

    println!();

    match Date::MAX.checked_add(1.day()) {
        Ok(value) => println!("Unexpected valid date: {value}"),
        Err(error) => println!("Expected date overflow: {error}"),
    }

    Ok(())
}

// 04. Months are calendar units, not a fixed number of days
// --------------------------------------------------------
fn demo_month_ends() -> Result<(), jiff::Error> {
    println!("04. Calendar months and month-end constraints");

    let january = Date::new(2024, 1, 31)?;
    let february = january.checked_add(1.month())?;
    println!("Jan 31 + one month: {february}"); // 2024-02-29
    println!("Jan 31 + 30 days:   {}", january.checked_add(30.days())?); // 2024-03-01
    println!("Then subtract month: {}", february.checked_sub(1.month())?); // Jan 29
    // Jiff constrains an impossible day to the target month's final valid day.
    // Information can be lost: adding/subtracting a month need not restore Jan 31.

    let repeated = february.checked_add(1.month())?;
    let direct = january.checked_add(2.months())?;
    println!("Two successive +1 month operations: {repeated}"); // Mar 29
    println!("One +2 months operation:            {direct}"); // Mar 31
    // Decide how your application defines recurrence at month ends.
    // For 'always the last day', explicitly select the final day of each month.

    Ok(())
}

// 05. Differences: until points forward; since points backward
// -----------------------------------------------------------
fn demo_differences() -> Result<(), jiff::Error> {
    println!("05. Date and datetime differences");

    let start = Date::new(2024, 1, 1)?;
    let end = Date::new(2024, 3, 15)?;
    println!("Date difference in days: {:#}", start.until(end)?);
    println!("Date difference using months: {:#}", start.until((Unit::Month, end))?);
    println!("end.since(start): {:#}", end.since(start)?);
    println!("Reversed: {:#}", end.until(start)?);
    // until: other - self; since: self - other. Both return a Span in Result.
    // 'largest' controls which units the difference is expressed in.

    let a: DateTime = "2024-06-18T09:00:00".parse()?;
    let b: DateTime = "2024-06-20T12:30:00".parse()?;
    println!("Expressed through hours: {:#}", a.until((Unit::Hour, b))?); // 51h 30m
    println!("Expressed through days:  {:#}", a.until((Unit::Day, b))?);  // 2d 3h 30m
    // These are calendar-field differences. For elapsed real-world time, use
    // timestamps or zoned datetimes whose instants have been established.

    Ok(())
}

// 06. Stored components versus total duration in a selected unit
// --------------------------------------------------------------
fn demo_components_and_totals() -> Result<(), jiff::Error> {
    println!("06. Components are not totals");

    let clock_span = 1.hour().minutes(90);
    println!("Stored hours={}, minutes={}", clock_span.get_hours(), clock_span.get_minutes());
    println!("Total minutes: {}", clock_span.total(Unit::Minute)?); // 150
    println!("Total hours:   {}", clock_span.total(Unit::Hour)?);   // 2.5
    // Building a Span does not automatically normalize 90 minutes into hours.
    // get_seconds() is the stored seconds field, not Python timedelta.seconds.
    // total returns f64; very large/fine-grained totals may lose precision.

    let calendar_span = 5.days().hours(1).minutes(10).seconds(30);
    let reference = Date::new(2024, 6, 18)?;
    println!("Total seconds relative to a civil date: {}",
        calendar_span.total((Unit::Second, reference))?); // 436230
    // A civil reference gives ordinary 24-hour-day arithmetic. A zoned reference
    // instead accounts for timezone transitions over that interval.

    println!();

    match 1.month().total(Unit::Day) {
        Ok(total) => println!("Total: {total}"),
        Err(error) => println!("A month needs a reference date: {error}"),
    }
    println!("Month starting Feb 1: {} days",
        1.month().total((Unit::Day, Date::new(2024, 2, 1)?))?); // 29
    println!("Month starting Mar 1: {} days",
        1.month().total((Unit::Day, Date::new(2024, 3, 1)?))?); // 31

    Ok(())
}

// 07. Arithmetic between spans
// ------------------------------
fn demo_span_arithmetic() -> Result<(), jiff::Error> {
    println!("07. Add, subtract, and multiply spans");

    let first = 2.hours().minutes(20);
    let second = 40.minutes();
    println!("Sum:        {:#}", first.checked_add(second)?); // 3h
    println!("Difference: {:#}", first.checked_sub(second)?); // 1h 40m
    println!("Repeated:   {:#}", first.checked_mul(3)?);
    // checked_mul repeats the span's fields; it does not choose a reference date.

    let reference = Date::new(2024, 1, 1)?;
    let net = 2.days().checked_sub((3.hours(), reference))?;
    println!("Two days minus three hours: {net:#}");
    // The tuple supplies calendar context to checked_sub.
    // This expresses subtraction correctly, unlike 2.days().hours(-3).

    Ok(())
}

// 08. Semantic comparison versus fieldwise comparison
// ----------------------------------------------------
fn demo_comparisons() -> Result<(), jiff::Error> {
    println!("08. Compare spans");

    let hour = 1.hour();
    let minutes = 60.minutes();
    println!("Same stored fields: {}", hour.fieldwise() == minutes.fieldwise()); // false
    println!("Same duration: {:?}", hour.compare(minutes)?); // Equal
    // Plain Span does not implement ordinary Span-to-Span == or ordering.
    // fieldwise opts into exact field comparison; compare compares durations.

    let february = Date::new(2024, 2, 1)?;
    let march = Date::new(2024, 3, 1)?;
    println!("1 month vs 30 days from February: {:?}", 1.month().compare((30.days(), february))?);
    println!("1 month vs 30 days from March:    {:?}", 1.month().compare((30.days(), march))?);
    // Less in February; Greater in March. Calendar context changes the answer.

    Ok(())
}

// 09. Balancing changes the representation; rounding can change the duration
// --------------------------------------------------------------------------
fn demo_rounding_and_balancing() -> Result<(), jiff::Error> {
    println!("09. Balance and round spans");

    let unbalanced = 150.minutes();
    let balanced = unbalanced.round(SpanRound::new().largest(Unit::Hour))?;
    println!("150 minutes balanced through hours: {balanced:#}"); // 2h 30m
    println!("Back to minutes: {:#}", balanced.round(SpanRound::new().largest(Unit::Minute))?);
    // largest controls representation; smallest controls the rounding granularity.

    let span = 2.hours().minutes(37).seconds(40);
    for mode in [RoundMode::Floor, RoundMode::Ceil, RoundMode::HalfExpand] {
        let options = SpanRound::new().smallest(Unit::Minute).increment(15).mode(mode);
        println!("{mode:?}, 15-minute increments: {:#}", span.round(options)?);
    }
    // Floor -> 2h 30m; Ceil -> 2h 45m; nearest (HalfExpand) -> 2h 45m.
    // Floor means toward negative infinity, not toward zero, for negative spans.
    // Calendar-unit rounding may also require .relative(reference_date_or_zoned).

    Ok(())
}

// 10. SignedDuration: fixed elapsed time with ordinary ordering
// -------------------------------------------------------------
fn demo_signed_duration() -> Result<(), jiff::Error> {
    println!("10. Fixed signed durations");

    let duration = SignedDuration::new(90, 500_000_000);
    println!("Duration: {duration}");
    println!("Whole seconds: {}", duration.as_secs()); // 90
    println!("Fractional seconds: {}", duration.as_secs_f64()); // 90.5
    println!("Total milliseconds: {}", duration.as_millis()); // 90500
    println!("Subsecond nanoseconds: {}", duration.subsec_nanos()); // 500000000
    println!("Negated: {}", -duration);
    println!("Negative whole seconds: {}", (-duration).as_secs()); // -90, toward zero
    // SignedDuration's subsecond component is signed as well, unlike a
    // Python timedelta's normalized nonnegative sub-day remainder.

    let start: Timestamp = "2024-06-18T09:00:00Z".parse()?;
    let end = start.checked_add(duration)?;
    let elapsed = end.duration_since(start);
    println!("End: {end}");
    println!("Elapsed: {elapsed}");
    println!("Within two minutes: {}", elapsed <= SignedDuration::from_secs(120));
    // duration_since returns SignedDuration directly; since/until return a
    // configurable Span in Result. Reversing the instants produces a negative duration.
    // Use std::time::Instant for actual monotonic performance measurements.

    Ok(())
}

// 11. Preview: one calendar day versus 24 elapsed hours around DST
// ----------------------------------------------------------------
fn demo_calendar_day_vs_hours() -> Result<(), jiff::Error> {
    println!("11. One calendar day can be 23 elapsed hours");

    let start: Zoned = "2024-03-09T12:00:00-05:00[America/New_York]".parse()?;
    let next_day = start.checked_add(1.day())?;
    let after_24h = start.checked_add(SignedDuration::from_hours(24))?;
    println!("Start:            {start}");
    println!("+1 calendar day:  {next_day}");  // Mar 10, 12:00 -04:00
    println!("+24 elapsed hours: {after_24h}"); // Mar 10, 13:00 -04:00
    println!("Elapsed hours for +1 day: {}", next_day.duration_since(&start).as_hours()); // 23
    println!("One day as a duration here: {}", 1.day().to_duration(&start)?);
    // Jiff updates the offset during zoned arithmetic: no pytz.normalize step.
    // 24.hours() is also a fixed 24 hours; the distinction is the UNIT, not just
    // Span versus SignedDuration. Detailed timezone rules follow in file 04.

    Ok(())
}

// 12. Conversions between duration representations
// -------------------------------------------------
fn demo_conversions() -> Result<(), jiff::Error> {
    println!("12. Convert duration types explicitly");

    let standard = Duration::from_millis(1_500);
    let signed = SignedDuration::try_from(standard)?;
    let span = Span::try_from(signed)?;
    let back = Duration::try_from(signed)?;
    println!("std Duration: {standard:?}");
    println!("SignedDuration: {signed}");
    println!("Span: {span:#}");
    println!("Back to std: {back:?}");
    // Conversions are fallible because of differing ranges and sign constraints.
    // Calendar spans need a reference to become fixed durations.
    let month = 1.month().to_duration(Date::new(2024, 2, 1)?)?;
    println!("One month starting Feb 1, 2024: {} seconds", month.as_secs());

    println!();

    match Duration::try_from(SignedDuration::from_secs(-1)) {
        Ok(value) => println!("Unexpected unsigned duration: {value:?}"),
        Err(error) => println!("std Duration cannot represent negative time: {error}"),
    }
    // unsigned_abs() is available when you intentionally want the magnitude;
    // it would discard the direction, so do not silently use it to hide errors.

    Ok(())
}

// 13. Practical example: elapsed-time service deadline
// ----------------------------------------------------
fn demo_deadline() -> Result<(), jiff::Error> {
    println!("13. A fixed 24-hour response deadline");

    let created: Timestamp = "2024-06-18T09:00:00Z".parse()?;
    let resolved: Timestamp = "2024-06-18T18:30:00Z".parse()?;
    let allowance = SignedDuration::from_hours(24);
    let deadline = created.checked_add(allowance)?;
    let elapsed = resolved.duration_since(created);
    let remaining = deadline.duration_since(resolved);
    println!("Created: {created}");
    println!("Deadline: {deadline}");
    println!("Resolution time: {} hours", elapsed.as_secs_f64() / 3_600.0); // 9.5
    println!("Time remaining: {} hours", remaining.as_secs_f64() / 3_600.0); // 14.5
    println!("Resolved within allowance: {}", resolved >= created && resolved <= deadline);
    // Choose a fixed duration for 'within 24 elapsed hours'.
    // Choose zoned calendar arithmetic for a rule such as 'same local time tomorrow'.
    // Business hours/holidays require an additional application-specific calendar.

    Ok(())
}

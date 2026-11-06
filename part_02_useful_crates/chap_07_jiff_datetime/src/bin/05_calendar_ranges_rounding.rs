// 05_calendar_ranges_rounding.rs
// ==============================
// Setup (inside chap_07_jiff_datetime):
//     cargo add jiff@0.2
// Run:
//     cargo run --bin 05_calendar_ranges_rounding
//
// This guide combines Jiff calendar/rounding APIs with ordinary Rust iterators.
// Date sequences are not dataframe indexes. Grouping/resampling/rolling windows
// from the Python pandas/Polars references belong in a data-processing chapter.
//
// Range vocabulary:
//     [start, end] includes both boundaries.
//     [start, end) includes start and excludes end (useful for adjacent periods).
// series starts with the original value; always limit it for a demo.
// Round values with round(...); limiting printed precision only changes text.
//
// Docs:
// https://docs.rs/jiff/latest/jiff/civil/struct.Date.html
// https://docs.rs/jiff/latest/jiff/civil/struct.ISOWeekDate.html
// https://docs.rs/jiff/latest/jiff/civil/struct.DateTimeRound.html
// https://docs.rs/jiff/latest/jiff/struct.Zoned.html

use jiff::{
    civil::{Date, DateTime, DateTimeRound, ISOWeekDate, Weekday},
    RoundMode, ToSpan, Unit, Zoned,
};

fn main() -> Result<(), jiff::Error> {
    println!();

    demo_calendar_properties()?;
    println!("\n============================\n");

    demo_iso_weeks()?;
    println!("\n============================\n");

    demo_month_year_boundaries()?;
    println!("\n============================\n");

    demo_quarters()?;
    println!("\n============================\n");

    demo_nth_weekday()?;
    println!("\n============================\n");

    demo_rounding()?;
    println!("\n============================\n");

    demo_start_of_day()?;
    println!("\n============================\n");

    demo_date_ranges()?;
    println!("\n============================\n");

    demo_datetime_ranges()?;
    println!("\n============================\n");

    demo_month_end_schedule()?;
    println!("\n============================\n");

    demo_weekdays_only()?;
    println!("\n============================\n");

    demo_filter_events()?;

    Ok(())
}

// 01. Weekdays, ordinal days, leap years, and month lengths
// ---------------------------------------------------------
fn demo_calendar_properties() -> Result<(), jiff::Error> {
    println!("01. Calendar properties");

    let day = Date::new(2024, 2, 29)?;
    println!("Date: {day}");
    println!("Weekday: {:?}", day.weekday()); // Thursday
    println!("Python-style weekday (Monday=0): {}", day.weekday().to_monday_zero_offset()); // 3
    println!("ISO weekday (Monday=1): {}", day.weekday().to_monday_one_offset()); // 4
    println!("Day of year: {}", day.day_of_year()); // 60
    println!("Leap year: {}", day.in_leap_year());
    println!("Days in month: {}", day.days_in_month()); // 29
    println!("Days in year: {}", day.days_in_year());   // 366
    // Choose weekday numbering explicitly rather than casting the enum.

    for year in [1900, 2000, 2023, 2024] {
        let date = Date::new(year, 1, 1)?;
        println!("{year}: leap={}", date.in_leap_year());
    }
    // Gregorian rule: divisible by 4, except centuries not divisible by 400.
    // These cover Python calendar.isleap and the month-length part of monthrange.

    Ok(())
}

// 02. An ISO week-year can differ from the ordinary calendar year
// ---------------------------------------------------------------
fn demo_iso_weeks() -> Result<(), jiff::Error> {
    println!("02. ISO week dates");

    let day = Date::new(2021, 1, 1)?;
    let iso = day.iso_week_date();
    println!("Calendar date: {day}; calendar year={}", day.year());
    println!("ISO week-year={}, week={}, weekday={:?}", iso.year(), iso.week(), iso.weekday());
    // January 1, 2021 was Friday in ISO week 53 of 2020.
    println!("ISO week runs from {} through {}", iso.first_of_week()?.date(), iso.last_of_week()?.date());
    println!("Weeks in this ISO year: {}", iso.weeks_in_year()); // 53

    let constructed = ISOWeekDate::new(2020, 53, Weekday::Friday)?;
    println!("Constructed ISO week date -> {}", constructed.date()); // 2021-01-01
    // ISO weeks start Monday. Week 1 is the week containing January 4.
    // Grouping by week number alone loses the ISO week-year near year boundaries.

    Ok(())
}

// 03. Month/year boundaries and derived boolean checks
// -----------------------------------------------------
fn demo_month_year_boundaries() -> Result<(), jiff::Error> {
    println!("03. Month and year boundaries");

    let day = Date::new(2024, 2, 18)?;
    println!("Month: {} through {}", day.first_of_month(), day.last_of_month());
    println!("Year:  {} through {}", day.first_of_year(), day.last_of_year());
    println!("Is month start: {}", day == day.first_of_month());
    println!("Is month end:   {}", day == day.last_of_month());
    println!("Is year start:  {}", day == day.first_of_year());
    println!("Is year end:    {}", day == day.last_of_year());
    // These comparisons replace pandas' is_month_start/is_month_end, etc.
    // A Date has no time component, so last_of_month is a whole calendar date.

    let dt: DateTime = "2024-02-18T14:30:45".parse()?;
    println!("DateTime.first_of_month(): {}", dt.first_of_month());
    // DateTime boundary methods retain its clock time: February 1 at 14:30:45.
    // Use .date().first_of_month().at(0, 0, 0, 0) for a civil midnight boundary.

    Ok(())
}

// 04. Derive ordinary calendar quarters (not fiscal quarters)
// ------------------------------------------------------------
fn demo_quarters() -> Result<(), jiff::Error> {
    println!("04. Calendar quarters");

    let day = Date::new(2024, 5, 18)?;
    let quarter = (day.month() - 1) / 3 + 1;
    let first_month = (quarter - 1) * 3 + 1;
    let first = Date::new(day.year(), first_month, 1)?;
    let last = Date::new(day.year(), first_month + 2, 1)?.last_of_month();
    println!("{day} belongs to Q{quarter}: {first} through {last}");
    println!("Quarter start: {}", day == first);
    println!("Quarter end: {}", day == last);
    // Quarter numbering and boundaries here are simple application logic.
    // For fiscal calendars, explicitly define which month begins the year.

    Ok(())
}

// 05. Find the nth weekday in a month
// ------------------------------------
fn demo_nth_weekday() -> Result<(), jiff::Error> {
    println!("05. Dates such as the second Tuesday or last Friday");

    let month = Date::new(2024, 3, 1)?;
    println!("Second Tuesday: {}", month.nth_weekday_of_month(2, Weekday::Tuesday)?); // Mar 12
    println!("Last Friday:    {}", month.nth_weekday_of_month(-1, Weekday::Friday)?); // Mar 29
    // Positive nth counts from the start of the month; negative from its end.
    // The receiver can be any date in that month, not only the first day.

    println!();

    match month.nth_weekday_of_month(5, Weekday::Monday) {
        Ok(value) => println!("Fifth Monday: {value}"),
        Err(error) => println!("No fifth Monday in March 2024: {error}"),
    }

    Ok(())
}

// 06. Floor, ceil, nearest, and increments on actual datetime values
// -----------------------------------------------------------------
fn demo_rounding() -> Result<(), jiff::Error> {
    println!("06. Round datetimes");

    let original: DateTime = "2024-06-18T08:37:23.123456789".parse()?;
    for mode in [RoundMode::Floor, RoundMode::Ceil, RoundMode::HalfExpand] {
        let options = DateTimeRound::new().smallest(Unit::Minute).increment(15).mode(mode);
        println!("{mode:?}, 15-minute grid: {}", original.round(options)?);
    }
    // Floor -> 08:30; Ceil -> 08:45; nearest -> 08:30.
    // HalfExpand resolves exact ties away from zero; choose the mode explicitly.

    let second = original.round(DateTimeRound::new()
        .smallest(Unit::Second).mode(RoundMode::Floor))?;
    println!("Fraction removed from VALUE: {second}");
    println!("Original unchanged: {original}");

    let late: DateTime = "2024-06-18T23:59:40".parse()?;
    println!("Ceil to minute carries into next day: {}", late.round(
        DateTimeRound::new().smallest(Unit::Minute).mode(RoundMode::Ceil)
    )?);
    // Datetime rounding differs from Span rounding in file 03 and from reducing
    // printed precision in file 02. Use calendar boundary APIs for months/years.

    println!();

    match original.round(DateTimeRound::new().smallest(Unit::Minute).increment(7)) {
        Ok(value) => println!("Rounded: {value}"),
        Err(error) => println!("Invalid minute increment: {error}"),
    }
    // Minute increments must divide 60 evenly (and be smaller than 60).

    Ok(())
}

// 07. Start of day: civil midnight versus the first instant of a zoned day
// ------------------------------------------------------------------------
fn demo_start_of_day() -> Result<(), jiff::Error> {
    println!("07. Start of day");

    let civil: DateTime = "2024-06-18T14:30:45".parse()?;
    println!("Civil start: {}", civil.start_of_day()); // midnight

    let zoned: Zoned = "2024-03-10T12:00:00-04:00[America/New_York]".parse()?;
    let start = zoned.start_of_day()?;
    let next_date = zoned.date().tomorrow()?;
    let next_start = next_date.at(0, 0, 0, 0)
        .to_zoned(zoned.time_zone().clone())?
        .start_of_day()?;
    println!("Zoned start: {start}"); // midnight -05:00, before DST transition
    println!("Next start:  {next_start}"); // midnight -04:00
    println!("Hours in this local day: {}", next_start.duration_since(&start).as_hours()); // 23
    // A zoned day's first valid time can differ from 00:00 in some regions/dates.
    // start_of_day uses timezone rules; do not replace it with an offset shortcut.
    // This example's next date has a valid midnight. A wholly skipped calendar
    // date needs an explicit scheduling policy rather than an assumed boundary.

    Ok(())
}

// 08. Date ranges with explicit direction and endpoint inclusion
// ---------------------------------------------------------------
fn demo_date_ranges() -> Result<(), jiff::Error> {
    println!("08. Bounded date sequences");

    let start = Date::new(2024, 2, 27)?;
    let end = Date::new(2024, 3, 2)?;
    let inclusive: Vec<Date> = start.series(1.day()).take_while(|day| *day <= end).collect();
    let exclusive: Vec<Date> = start.series(1.day()).take_while(|day| *day < end).collect();
    println!("[start, end]: {inclusive:?}");
    println!("[start, end): {exclusive:?}");

    let every_other: Vec<Date> = start.series(2.days()).take(4).collect();
    let reverse: Vec<Date> = end.series((-1).day()).take(4).collect();
    println!("Four dates, every 2 days: {every_other:?}");
    println!("Four dates backward:     {reverse:?}");
    // series includes the starting value, then advances according to its span.
    // It stops at representable limits; take/take_while gives the desired bound.
    // Use a positive day step with an increasing end-bound predicate. A zero or
    // wrongly directed step can make a take_while-only loop fail to terminate.

    Ok(())
}

// 09. Datetime ranges provide finer clock precision than Date ranges
// -----------------------------------------------------------------
fn demo_datetime_ranges() -> Result<(), jiff::Error> {
    println!("09. Appointment slots");

    let start: DateTime = "2024-06-18T09:00:00".parse()?;
    let end: DateTime = "2024-06-18T11:00:00".parse()?;
    for slot in start.series(30.minutes()).take_while(|slot| *slot < end) {
        println!("Slot: {slot}"); // 09:00, 09:30, 10:00, 10:30
    }
    // These are unzoned local slots. Attach a zone with a deliberate ambiguity
    // policy before treating a slot as a real meeting (see file 04).
    // Date.series with a sub-day step can repeat dates because Date has no clock.

    Ok(())
}

// 10. Month-end recurrence: generate month starts, then select each month's end
// -----------------------------------------------------------------------------
fn demo_month_end_schedule() -> Result<(), jiff::Error> {
    println!("10. Month-end schedule without drift");

    let first = Date::new(2024, 1, 1)?;
    let month_ends: Vec<Date> = first.series(1.month()).take(6)
        .map(|month_start| month_start.last_of_month())
        .collect();
    println!("Month ends: {month_ends:?}"); // Jan 31, Feb 29, Mar 31, Apr 30, May 31, Jun 30
    // Generating valid first-of-month dates keeps recurrence intention explicit.
    // Repeatedly adding a month to Jan 31 can drift to the 29th after February.
    // Date.series calculates its sequence from its original anchor, but explicit
    // first-of-month -> last_of_month also documents the 'always month end' rule.

    Ok(())
}

// 11. Weekday-only dates: application logic, without a holiday calendar
// ---------------------------------------------------------------------
fn demo_weekdays_only() -> Result<(), jiff::Error> {
    println!("11. Weekday-only schedule");

    let start = Date::new(2024, 6, 14)?; // Friday
    let weekdays: Vec<Date> = start.series(1.day())
        .filter(|day| !matches!(day.weekday(), Weekday::Saturday | Weekday::Sunday))
        .take(5)
        .collect();
    println!("Five weekdays including start when eligible: {weekdays:?}");
    // Friday, Monday, Tuesday, Wednesday, Thursday.
    // Filter BEFORE take: take(5) then filter would inspect just five calendar dates.
    // Weekday-only does not mean business-day/holiday aware. Public holidays,
    // regional workweeks, and observed holidays require application-specific rules.

    Ok(())
}

// 12. Practical filtering with a half-open calendar-month interval
// ----------------------------------------------------------------
fn demo_filter_events() -> Result<(), jiff::Error> {
    println!("12. Filter events into a calendar month");

    let month = Date::new(2024, 6, 18)?.first_of_month();
    let start = month.at(0, 0, 0, 0);
    let end = month.checked_add(1.month())?.at(0, 0, 0, 0);
    let events: Vec<DateTime> = [
        "2024-05-31T23:59:59",
        "2024-06-01T00:00:00",
        "2024-06-18T14:30:00",
        "2024-06-30T23:59:59.999999999",
        "2024-07-01T00:00:00",
    ].into_iter().map(str::parse).collect::<Result<_, _>>()?;

    let selected: Vec<&DateTime> = events.iter()
        .filter(|event| **event >= start && **event < end)
        .collect();
    println!("Interval: [{start}, {end})");
    for event in selected {
        println!("Included: {event}");
    }
    // June 1 midnight is included; July 1 midnight is excluded. No need to guess
    // the final nanosecond of June or risk excluding fractional-second events.
    // Adjacent half-open months meet without overlap or gaps.
    // These events share one civil-time interpretation. For actual instants,
    // resolve the local boundaries in the reporting timezone, then compare their
    // timestamps. Do not compare naive fields from unrelated timezones.

    Ok(())
}

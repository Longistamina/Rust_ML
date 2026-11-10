// 04_timezone_dst.rs
// ==================
// Setup (inside chap_07_jiff_datetime):
//     cargo add jiff@0.2
// Run:
//     cargo run --bin 04_timezone_dst
//
// A timezone is a set of rules, not merely an offset.
//     Asia/Seoul: IANA identifier for regional rules/history.
//     +09:00:     one numeric offset, without regional transition rules.
//
// Two very different operations:
//     civil.in_tz(name): interpret these local fields in that zone.
//     zoned.in_tz(name): preserve the instant; display it in another zone.
//
// Gap:  clocks jump forward; some local times do not exist.
// Fold: clocks jump backward; some local times occur twice.
// Default 'Compatible': later for gaps, earlier for folds.
// Use explicit disambiguation when your application's policy matters.
//
// Named-zone examples need an available IANA timezone database.
// Default Jiff features use platform timezone data. On systems without it,
// embedding is an option: cargo add jiff@0.2 --features tzdb-bundle-always
// Available names and future rules depend on the database version.
//
// Docs:
// https://docs.rs/jiff/latest/jiff/tz/index.html
// https://docs.rs/jiff/latest/jiff/tz/struct.TimeZone.html
// https://docs.rs/jiff/latest/jiff/tz/struct.AmbiguousZoned.html
// https://docs.rs/jiff/latest/jiff/tz/enum.OffsetConflict.html

use jiff::{
    civil::DateTime,
    fmt::temporal::DateTimeParser,
    tz::{self, Disambiguation, OffsetConflict, TimeZone},
    SignedDuration, Timestamp, ToSpan, Zoned,
};

fn main() -> Result<(), jiff::Error> {
    println!();

    demo_lookup_and_names()?;
    println!("\n============================\n");

    demo_system_timezone();
    println!("\n============================\n");

    demo_localize_and_convert()?;
    println!("\n============================\n");

    demo_fixed_offsets()?;
    println!("\n============================\n");

    demo_seasonal_offsets()?;
    println!("\n============================\n");

    demo_spring_gap()?;
    println!("\n============================\n");

    demo_autumn_fold()?;
    println!("\n============================\n");

    demo_parser_policies()?;
    println!("\n============================\n");

    demo_offset_conflicts()?;
    println!("\n============================\n");

    demo_arithmetic_across_dst()?;
    println!("\n============================\n");

    demo_transitions()?;
    println!("\n============================\n");

    demo_meeting_schedule()?;

    Ok(())
}

// 01. Look up a zone and list available IANA identifiers
// -----------------------------------------------------
fn demo_lookup_and_names() -> Result<(), jiff::Error> {
    println!("01. Timezone lookup and available names");

    let seoul = TimeZone::get("Asia/Seoul")?;
    let vietnam = tz::db().get("Asia/Ho_Chi_Minh")?;
    println!("Seoul identifier: {:?}", seoul.iana_name()); // Some("Asia/Seoul")
    println!("Vietnam identifier: {:?}", vietnam.iana_name());
    // TimeZone::get is a convenience lookup through the global tz::db().
    // TimeZone values are immutable and cheap to clone for reuse.

    let mut names: Vec<String> = tz::db().available()
        .map(|name| name.to_string())
        .filter(|name| name.starts_with("Asia/"))
        .collect();
    names.sort();
    println!("Available Asia/* names: {}", names.len());
    for name in names.iter().take(8) {
        println!("  {name}");
    }
    // Filtering/sorting are standard Rust; no regex dependency is needed here.
    // Do not depend on fixed numeric positions in a timezone-name list.

    println!();

    match TimeZone::get("Not/A_Real_Zone") {
        Ok(zone) => println!("Unexpected timezone: {zone:?}"),
        Err(error) => println!("Expected unknown-zone error: {error}"),
    }

    Ok(())
}

// 02. Discover the system zone, or choose your application zone explicitly
// -----------------------------------------------------------------------
fn demo_system_timezone() {
    println!("02. System timezone");

    match TimeZone::try_system() {
        Ok(zone) => {
            println!("System IANA name, if known: {:?}", zone.iana_name());
            println!("Current local time: {}", Timestamp::now().to_zoned(zone));
        }
        Err(error) => println!("System timezone discovery failed: {error}"),
    }
    // try_system reports discovery errors. system() falls back to UTC on error.
    // A valid system zone need not have an IANA name: iana_name() returns Option.
    // Prefer explicit zones for reproducible examples and business rules.
    // There is no need to modify process-global TZ or call Python-style tzset().
}

// 03. Localizing civil fields versus converting an existing instant
// ----------------------------------------------------------------
fn demo_localize_and_convert() -> Result<(), jiff::Error> {
    println!("03. Localization versus conversion");

    let fields: DateTime = "2024-06-18T12:00:00".parse()?;
    let seoul = fields.in_tz("Asia/Seoul")?;
    let new_york = seoul.in_tz("America/New_York")?;
    let utc = seoul.with_time_zone(TimeZone::UTC);
    println!("Civil fields: {fields}");
    println!("Interpreted in Seoul: {seoul}");
    println!("Same instant in New York: {new_york}"); // June 17, 23:00 -04:00
    println!("Same instant in UTC: {utc}");           // June 18, 03:00
    println!("Instant preserved: {}", seoul.timestamp() == new_york.timestamp());
    // civil.in_tz resembles pytz.localize / pandas.tz_localize.
    // zoned.in_tz resembles astimezone / pandas.tz_convert.
    // with_time_zone accepts an already-looked-up TimeZone and preserves the instant.

    let reinterpreted = seoul.datetime().in_tz("America/New_York")?;
    println!("Same LOCAL FIELDS reinterpreted in New York: {reinterpreted}");
    println!("Different instant: {}", reinterpreted.timestamp() != seoul.timestamp());
    // datetime() extracts fields and drops their zone/instant association.
    // Reattaching a zone to those fields changes their meaning, unlike conversion.

    Ok(())
}

// 04. Fixed offsets have no DST transitions
// ------------------------------------------
fn demo_fixed_offsets() -> Result<(), jiff::Error> {
    println!("04. Fixed offsets versus regional zones");

    let fixed_eastern = TimeZone::fixed(tz::offset(-5));
    let fixed_seoul = TimeZone::fixed(tz::offset(9));
    let instant: Timestamp = "2024-07-01T12:00:00Z".parse()?;
    println!("Fixed -05:00: {}", instant.to_zoned(fixed_eastern)); // 07:00
    println!("New York:     {}", instant.in_tz("America/New_York")?); // 08:00 -04:00
    println!("Fixed +09:00: {}", instant.to_zoned(fixed_seoul.clone()));
    println!("Fixed zone IANA name: {:?}", fixed_seoul.iana_name()); // None
    // tz::offset takes whole signed hours; it is not an IANA database lookup.
    // A fixed +09:00 zone and Asia/Seoul agree here, but only Asia/Seoul carries
    // the regional history. Never substitute a winter offset for a whole region.

    Ok(())
}

// 05. Offset and abbreviation are properties at a PARTICULAR instant
// -----------------------------------------------------------------
fn demo_seasonal_offsets() -> Result<(), jiff::Error> {
    println!("05. New York in winter and summer");

    let zone = TimeZone::get("America/New_York")?;
    for input in ["2024-01-15T12:00:00Z", "2024-07-15T12:00:00Z"] {
        let instant: Timestamp = input.parse()?;
        let local = instant.to_zoned(zone.clone());
        println!("{local}; offset={}, abbreviation={}", local.offset(), local.strftime("%Z"));
    }
    // Winter: -05:00 / EST. Summer: -04:00 / EDT for these dates in New York.
    // Abbreviations are display labels; they are not unique global identifiers.
    // Not every zone observes DST, and transition dates are region-specific.
    // Keep a TimeZone, then derive the offset for the relevant instant.

    Ok(())
}

// 06. Spring gap: 02:30 never occurred on this date in New York
// ------------------------------------------------------------
fn demo_spring_gap() -> Result<(), jiff::Error> {
    println!("06. Spring gap: skipped local time");

    let zone = TimeZone::get("America/New_York")?;
    let fields: DateTime = "2024-03-10T02:30:00".parse()?;
    let ambiguous = zone.to_ambiguous_zoned(fields);
    // This produces AmbiguousZoned directly, not Result<Zoned>.
    // It records the local fields, zone, and possible offset ambiguity.
    println!("Requested fields: {fields}");
    println!("Ambiguous: {}", ambiguous.is_ambiguous());
    println!("Offset situation: {:?}", ambiguous.offset()); // Gap
    println!("Earlier:    {}", ambiguous.clone().earlier()?);    // 01:30 -05:00
    println!("Later:      {}", ambiguous.clone().later()?);      // 03:30 -04:00
    println!("Compatible: {}", ambiguous.clone().compatible()?); // 03:30 -04:00
    println!("Default in_tz: {}", fields.in_tz("America/New_York")?);
    // Neither earlier nor later can preserve nonexistent 02:30. They shift by
    // the gap size while preserving the position within that skipped interval.
    // Compatible chooses Later for a gap; in_tz uses Compatible by default.

    println!();

    match ambiguous.unambiguous() {
        Ok(value) => println!("Unambiguous: {value}"),
        Err(error) => println!("Rejected nonexistent local time: {error}"),
    }

    Ok(())
}

// 07. Autumn fold: 01:30 occurred twice, one elapsed hour apart
// ------------------------------------------------------------
fn demo_autumn_fold() -> Result<(), jiff::Error> {
    println!("07. Autumn fold: repeated local time");

    let zone = TimeZone::get("America/New_York")?;
    let fields: DateTime = "2024-11-03T01:30:00".parse()?;
    let ambiguous = zone.to_ambiguous_zoned(fields);
    let first = ambiguous.clone().earlier()?;
    let second = ambiguous.clone().later()?;
    println!("Offset situation: {:?}", ambiguous.offset()); // Fold
    println!("Earlier: {first}");  // 01:30 -04:00, 05:30Z
    println!("Later:   {second}"); // 01:30 -05:00, 06:30Z
    println!("Same local fields: {}", first.datetime() == second.datetime());
    println!("Same instant: {}", first.timestamp() == second.timestamp());
    println!("Elapsed hours apart: {}", second.duration_since(&first).as_hours()); // 1
    println!("Compatible: {}", ambiguous.clone().compatible()?); // Earlier
    // Earlier/Later refer to the resulting INSTANT, not numeric offset order.
    // Log an offset or full zoned string to distinguish these two occurrences.

    println!();

    match ambiguous.unambiguous() {
        Ok(value) => println!("Unambiguous: {value}"),
        Err(error) => println!("Rejected repeated local time: {error}"),
    }

    Ok(())
}

// 08. Make parser ambiguity policy explicit
// ------------------------------------------
fn demo_parser_policies() -> Result<(), jiff::Error> {
    println!("08. Configure disambiguation during parsing");

    // No numeric offset: the zone must resolve these local fields.
    let input = "2024-11-03T01:30:00[America/New_York]";
    for policy in [Disambiguation::Earlier, Disambiguation::Later, Disambiguation::Compatible] {
        let parser = DateTimeParser::new().disambiguation(policy);
        println!("{policy:?}: {}", parser.parse_zoned(input)?);
    }
    // The same enum can be used on an AmbiguousZoned value with
    // ambiguous.disambiguate(policy), useful for runtime application settings.

    println!();

    let strict = DateTimeParser::new().disambiguation(Disambiguation::Reject);
    match strict.parse_zoned(input) {
        Ok(value) => println!("Parsed: {value}"),
        Err(error) => println!("Strict parser rejected ambiguity: {error}"),
    }
    // Reject accepts ordinary unique local times; it rejects BOTH gaps and folds.
    // If a valid explicit offset selects one fold occurrence, there need not be
    // unresolved ambiguity. Offset consistency is a separate policy, shown next.

    Ok(())
}

// 09. Offset conflict: the supplied offset disagrees with the zone rules
// ----------------------------------------------------------------------
fn demo_offset_conflicts() -> Result<(), jiff::Error> {
    println!("09. Numeric offset versus named timezone");

    let valid: Zoned = "2024-07-01T12:00:00-04:00[America/New_York]".parse()?;
    println!("Consistent input: {valid}");

    println!();

    let input = "2024-07-01T12:00:00-05:00[America/New_York]";
    match input.parse::<Zoned>() {
        Ok(value) => println!("Parsed: {value}"),
        Err(error) => println!("Default rejects inconsistent offset: {error}"),
    }
    // New York used -04:00 on this date; the input incorrectly supplies -05:00.

    println!();

    let trust_offset = DateTimeParser::new().offset_conflict(OffsetConflict::AlwaysOffset);
    let trust_zone = DateTimeParser::new().offset_conflict(OffsetConflict::AlwaysTimeZone);
    println!("Trust supplied offset: {}", trust_offset.parse_zoned(input)?); // 13:00 -04:00
    println!("Trust local fields + zone: {}", trust_zone.parse_zoned(input)?); // 12:00 -04:00
    // AlwaysOffset preserves the instant implied by 12:00-05:00, displaying it
    // correctly in New York. AlwaysTimeZone discards the supplied offset.
    // These choices differ by one hour. Keep Reject unless your data policy
    // explicitly chooses one meaning; do not just suppress the error.
    // Real conflicts can also arise when timezone rules change after data is saved.

    Ok(())
}

// 10. Zoned arithmetic automatically follows timezone rules
// ----------------------------------------------------------
fn demo_arithmetic_across_dst() -> Result<(), jiff::Error> {
    println!("10. Arithmetic across transitions");

    let before: Zoned = "2024-03-10T01:30:00-05:00[America/New_York]".parse()?;
    let after = before.checked_add(3.hours())?;
    println!("Before spring transition: {before}");
    println!("Three elapsed hours later: {after}"); // 05:30 -04:00
    // No pytz.normalize() call is required: the new local fields and offset agree.

    println!();

    let autumn: Zoned = "2024-11-02T12:00:00-04:00[America/New_York]".parse()?;
    let tomorrow = autumn.checked_add(1.day())?;
    let after_24h = autumn.checked_add(SignedDuration::from_hours(24))?;
    println!("Start: {autumn}");
    println!("+1 calendar day: {tomorrow}"); // Nov 3, 12:00 -05:00
    println!("+24 elapsed hours: {after_24h}"); // Nov 3, 11:00 -05:00
    println!("Elapsed hours for +1 day: {}", tomorrow.duration_since(&autumn).as_hours()); // 25
    // This complements the 23-hour spring example in file 03.
    // Calendar recurrence and fixed elapsed duration express different intentions.

    Ok(())
}

// 11. Bonus: inspect a bounded number of timezone transitions
// -----------------------------------------------------------
fn demo_transitions() -> Result<(), jiff::Error> {
    println!("11. Inspect upcoming transitions from a fixed reference");

    let zone = TimeZone::get("America/New_York")?;
    let reference: Timestamp = "2024-01-01T00:00:00Z".parse()?;
    for transition in zone.following(reference).take(2) {
        let local = transition.timestamp().to_zoned(zone.clone());
        println!("At {local}; new offset={}, abbreviation={}",
            transition.offset(), transition.abbreviation());
    }
    // Following yields transitions strictly after the reference, in time order.
    // preceding(reference) goes backward. Always bound iteration in a demo.
    // Fixed-offset zones have no transitions; these iterators can be empty.
    // Most applications should use Zoned operations rather than inspect tables.

    Ok(())
}

// 12. Practical scheduling: reject ambiguous input, then share one instant
// ------------------------------------------------------------------------
fn demo_meeting_schedule() -> Result<(), jiff::Error> {
    println!("12. Schedule a meeting across regions");

    let requested: DateTime = "2024-06-18T09:00:00".parse()?;
    let organizer_zone = TimeZone::get("America/New_York")?;
    let meeting = organizer_zone.to_ambiguous_zoned(requested).unambiguous()?;
    println!("Organizer: {meeting}");
    println!("Instant for storage: {}", meeting.timestamp()); // 13:00Z

    for name in ["UTC", "Asia/Seoul", "Asia/Ho_Chi_Minh"] {
        println!("{name}: {}", meeting.in_tz(name)?);
    }
    // Seoul: 22:00; Ho Chi Minh City: 20:00 on this date.
    // The strict localization step lets a UI ask for clarification for a fold,
    // or request another time for a gap, instead of silently making a choice.
    // Store the instant for a resolved one-off event. Also retain the intended
    // IANA zone/local schedule when future calendar recurrence matters.

    Ok(())
}

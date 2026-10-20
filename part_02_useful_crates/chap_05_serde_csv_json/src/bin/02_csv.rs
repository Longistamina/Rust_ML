/*
csv (BurntSushi/rust-csv): read / write CSV, TSV, and typed records.

1. Read records and inspect headers
2. Headerless data and alternate field names
3. Collect records / build dictionary-like rows
4. TSV and other delimiters
5. Write records to files
6. Read / write typed rows with Serde
7. Configure trimming, comments, and variable-width rows
8. Quoting styles and escaped quotes
9. Process large inputs in bounded row batches
10. Reuse record buffers; read non-UTF-8 bytes
11. Report record / conversion errors
12. Append rows without repeating headers

Setup (run in this chapter's Cargo package directory):
    cargo add csv@1.4
    cargo add serde --features derive

Save as src/bin/02_csv.rs and run:
    cargo run --bin 02_csv

No [[bin]] entry is needed for files under src/bin/.
main() reserves a temporary directory and removes it after all demos.
All input is embedded / generated; no downloaded datasets are required.
csv::Reader / Writer already buffer I/O; no extra BufReader / BufWriter needed.
The library handles CSV quoting and record boundaries, including quoted newlines.
Don't parse CSV with split(',') or by treating every physical line as one record.
Reference: https://docs.rs/csv/1.4.0/csv/
*/

use csv::{ByteRecord, QuoteStyle, Reader, ReaderBuilder, StringRecord, Terminator, Trim, Writer, WriterBuilder};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::{self, OpenOptions};
use std::io;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

type DemoResult = Result<(), Box<dyn std::error::Error>>;

const PEOPLE: &str = "Name,Age,City,Score\nAlice,28,\"New York, NY\",85\nBob,35,Chicago,\nClaire,31,Seoul,90\n";

// ============================================================================
// 1. Read records and headers
// ============================================================================

fn demo_read() -> DemoResult {
    let mut reader = Reader::from_reader(PEOPLE.as_bytes());
    println!("headers: {:?}", reader.headers()?); // Name, Age, City, Score

    for result in reader.records() {
        let record = result?;
        println!("{record:?}");
        println!("name: {:?}; age text: {:?}", record.get(0), record.get(1));
    }
    // StringRecord stores UTF-8 fields as text; it doesn't infer numeric types.
    // get(index) returns Option<&str>; record[index] can panic if out of bounds.
    // By default the first record is a HEADER and records() omits it.
    // Unlike Python csv.reader, you normally DON'T call next() to skip headers.
    // from_path(path)? opens files; from_reader accepts any std::io::Read input.
    Ok(())
}

// ============================================================================
// 2. Headerless input / alternate field names
// ============================================================================

fn demo_headerless() -> DemoResult {
    let data = "Alice,28\nBob,35\n";
    let mut reader = ReaderBuilder::new().has_headers(false).from_reader(data.as_bytes());
    let names = ["person", "years"];

    for result in reader.records() {
        let record = result?;
        println!("{}={}, {}={}", names[0], &record[0], names[1], &record[1]);
    }
    // Alice=... is the first DATA record, so don't let it become a header.
    // If headerless mode is used on a file WITH a header, that header is yielded
    // as a data record too. Supplying your own names doesn't itself skip a row.
    Ok(())
}

// ============================================================================
// 3. Collect records / dictionary-like rows
// ============================================================================

fn demo_collect_maps() -> DemoResult {
    let mut reader = Reader::from_reader(PEOPLE.as_bytes());
    let headers = reader.headers()?.clone(); // owned copy avoids borrowing reader
    let rows: Vec<StringRecord> = reader.records().collect::<Result<_, _>>()?;
    println!("stored rows: {}", rows.len()); // 3

    for row in &rows {
        let mapped: BTreeMap<&str, &str> = headers.iter().zip(row.iter()).collect();
        println!("{mapped:?}"); // header -> text field, like Python DictReader
    }
    // Both headers and rows stay alive while these borrowed maps are used.
    // Duplicate header names would overwrite keys in a map; validate names if needed.
    // zip stops at the shorter side. Default fixed-width validation protects these
    // examples; flexible(true) requires your own policy for missing / extra fields.

    let alternate_names = ["N", "A", "C", "S"];
    let mapped: BTreeMap<_, _> = alternate_names.into_iter().zip(rows[0].iter()).collect();
    println!("alternate names: {mapped:?}");
    // Collecting loads every row into memory. Stream or batch large files instead.
    Ok(())
}

// ============================================================================
// 4. TSV / semicolon / pipe-separated data
// ============================================================================

fn demo_delimiters(root: &Path) -> DemoResult {
    let mut writer = WriterBuilder::new().delimiter(b'\t').from_path(root.join("scores.tsv"))?;
    writer.write_record(["Name", "Score"])?;
    writer.write_record(["Alice", "85"])?;
    writer.write_record(["Bob", "90"])?;
    writer.flush()?;
    drop(writer); // close before opening the file again

    let mut reader = ReaderBuilder::new().delimiter(b'\t').from_path(root.join("scores.tsv"))?;
    for row in reader.records() {
        println!("TSV: {:?}", row?);
    }

    let mut reader = ReaderBuilder::new().delimiter(b';').from_reader(&b"name;score\nAlice;85\n"[..]);
    for row in reader.records() {
        println!("semicolon: {:?}", row?);
    }
    // Delimiters are SINGLE bytes: b'\t', b';', b'|', not multi-character strings.
    Ok(())
}

// ============================================================================
// 5. Write records and round-trip special fields
// ============================================================================

fn demo_write(root: &Path) -> DemoResult {
    let path = root.join("notes.csv");
    let mut writer = Writer::from_path(&path)?;
    writer.write_record(["Name", "Note"])?;
    writer.write_record(["Alice", "commas, stay inside this field"])?;
    writer.write_record(["Bob", "said \"hello\"\non two lines"])?;
    writer.flush()?; // report buffered write errors explicitly
    drop(writer);

    println!("{}", fs::read_to_string(&path)?);
    let mut reader = Reader::from_path(&path)?;
    for row in reader.records() {
        println!("read back: {:?}", row?);
    }

    let fields = ["Name", "Age", "City"];
    let row = BTreeMap::from([("Name", "Alice"), ("Age", "28"), ("City", "Seoul")]);
    let mut writer = Writer::from_path(root.join("dictionary.csv"))?;
    writer.write_record(fields)?;
    writer.write_record(fields.iter().map(|field| row.get(field).copied().unwrap_or("")))?;
    writer.flush()?;
    // DictWriter-like output: choose the column order explicitly and look up values.
    // Here absent keys become empty fields; reject them instead if required.
    // For a known schema, use a Serde struct (next demo) rather than a map.

    // write_record doesn't synthesize headers; write them yourself for raw records.
    // Writer::from_path creates / TRUNCATES the file. Appending needs OpenOptions.
    // csv uses byte I/O, without Python text mode's newline translation issue.
    // Default writer records end with LF; use Terminator::CRLF for explicit CRLF.
    Ok(())
}

// ============================================================================
// 6. Typed rows with Serde
// ============================================================================

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Age")]
    age: u8,
    #[serde(rename = "City")]
    city: String,
    #[serde(rename = "Score")]
    score: Option<u32>,
}

fn demo_serde(root: &Path) -> DemoResult {
    let mut reader = Reader::from_reader(PEOPLE.as_bytes());
    let mut writer = Writer::from_path(root.join("typed_people.csv"))?;

    for result in reader.deserialize::<Person>() {
        let person = result?;
        println!("{person:?}"); // Bob's empty Score becomes None
        writer.serialize(&person)?;
    }
    writer.flush()?;
    // serialize() writes struct field names as a header on the first row by default.
    // Serde rename connects Rust fields to the input / output column names.
    // Headers map by NAME; without headers, tuple / struct fields map by position.
    // String -> number conversion is supported by the CSV deserializer; invalid
    // numeric text still errors. This differs from strict JSON numeric typing.
    // Option<T> accepts empty fields as None, but invalid nonempty T still errors.
    // Nested structs, maps, lists, and rich enums are constrained by CSV's flat model.
    Ok(())
}

// ============================================================================
// 7. Trimming, comments, and variable field counts
// ============================================================================

fn demo_options() -> DemoResult {
    let data = "# generated data\n Name ; Age \n Alice ; 28 \n";
    let mut reader = ReaderBuilder::new().delimiter(b';').trim(Trim::All)
        .comment(Some(b'#')).from_reader(data.as_bytes());
    println!("trimmed headers: {:?}", reader.headers()?);
    for row in reader.records() {
        println!("trimmed: {:?}", row?); // ["Alice", "28"]
    }
    // Trim::Headers / Fields / All select where trimming occurs; default is None.

    let mut reader = ReaderBuilder::new().flexible(true)
        .from_reader(&b"a,b\n1\n2,3,4\n"[..]);
    for row in reader.records() {
        println!("variable width: {:?}", row?); // ["1"], ["2", "3", "4"]
    }
    // flexible(true) disables width checks; it doesn't fill missing fields or
    // make typed deserialization accept incompatible records automatically.
    Ok(())
}

// ============================================================================
// 8. Quoting styles and escaped quotes
// ============================================================================

fn demo_quoting() -> DemoResult {
    for (label, style) in [
        ("necessary", QuoteStyle::Necessary),
        ("always", QuoteStyle::Always),
        ("non-numeric", QuoteStyle::NonNumeric),
        ("never", QuoteStyle::Never),
    ] {
        let mut writer = WriterBuilder::new().quote_style(style)
            .terminator(Terminator::CRLF).from_writer(Vec::new());
        writer.write_record(["Alice", "28", "New York, NY", "said \"hi\""])?;
        let bytes = writer.into_inner()?; // flush and recover underlying Vec<u8>
        println!("{label}: {:?}", String::from_utf8(bytes)?);
    }
    // Necessary (default): quote fields requiring it; like Python QUOTE_MINIMAL.
    // Always: quote every field. Never: fields may become ambiguous / invalid CSV!
    // NonNumeric decides by whether the field TEXT parses as a number, not the
    // original Rust / Python value type. A numeric-looking string "28" can stay unquoted.
    // This differs from Python QUOTE_NONNUMERIC, which distinguishes input types.
    // Reader records() still returns text; quoting doesn't cause automatic float conversion.
    // Default quote escaping doubles a quote: said "hi" -> "said ""hi""".
    // Custom quote / escape / double_quote options must agree between reader and writer.
    Ok(())
}

// ============================================================================
// 9. Bounded row batches
// ============================================================================

fn demo_batches() -> DemoResult {
    let mut reader = Reader::from_reader(PEOPLE.as_bytes());
    let batch_size = 2;
    let mut batch = Vec::with_capacity(batch_size);

    for result in reader.deserialize::<Person>() {
        batch.push(result?);
        if batch.len() == batch_size {
            println!("process {} rows: {batch:?}", batch.len());
            batch.clear(); // reuse capacity; don't accumulate every batch
        }
    }
    if !batch.is_empty() {
        println!("process final {} rows: {batch:?}", batch.len());
    }
    // Batches: 2, then 1. Replace println with your processing logic.
    // Readers stream already; batching is for downstream work, not a requirement.
    // A row limit bounds the number of stored rows, not their total byte size.
    Ok(())
}

// ============================================================================
// 10. Reusable records / raw bytes
// ============================================================================

fn demo_buffers_bytes() -> DemoResult {
    let mut reader = Reader::from_reader(PEOPLE.as_bytes());
    let mut record = StringRecord::new();
    while reader.read_record(&mut record)? {
        println!("reused buffer: {:?}", record.get(0));
    }
    // read_record reuses allocated field storage. Process or clone a record before
    // the next read overwrites it; borrowed field slices can't outlive that buffer.

    let mut reader = Reader::from_reader(&b"name\n\xFF\n"[..]);
    let mut bytes = ByteRecord::new();
    while reader.read_byte_record(&mut bytes)? {
        println!("raw field: {:?}", bytes.get(0)); // Some([255])
    }
    // StringRecord validates UTF-8. ByteRecord can preserve arbitrary field bytes.
    // ByteRecord does not transcode legacy encodings into Unicode for you.
    Ok(())
}

// ============================================================================
// 11. Handle structural and conversion errors
// ============================================================================

fn demo_errors() -> DemoResult {
    let data = "Name,Age,City,Score\nAlice,28,Seoul,85\nBob,wrong,Chicago,90\n";
    let mut reader = Reader::from_reader(data.as_bytes());
    for result in reader.deserialize::<Person>() {
        match result {
            Ok(person) => println!("valid: {person:?}"),
            Err(error) => {
                println!("conversion error: {error}");
                if let Some(pos) = error.position() {
                    println!("record={}, line={}, byte={}", pos.record(), pos.line(), pos.byte());
                }
            }
        }
    }
    // record / byte offsets are zero-based; line numbers are one-based.
    // A quoted multiline field means line number and record number can differ.

    let mut reader = Reader::from_reader(&b"a,b\n1\n"[..]);
    for row in reader.records() {
        if let Err(error) = row {
            println!("width error: {:?}", error.kind()); // UnequalLengths
        }
    }
    // Use result? for fail-fast processing, or match to log selected row failures.
    // Not all I/O errors permit useful recovery. Don't silently drop every Err.
    // This parser is permissive about some CSV syntax; it isn't a strict validator
    // that guarantees every malformed quoting convention produces an error.
    Ok(())
}

// ============================================================================
// 12. Append without another header
// ============================================================================

fn demo_append(root: &Path) -> DemoResult {
    let path = root.join("append.csv");
    {
        let mut writer = Writer::from_path(&path)?;
        writer.write_record(["Name", "Age"])?;
        writer.write_record(["Alice", "28"])?;
        writer.flush()?;
    }

    let file = OpenOptions::new().append(true).open(&path)?;
    let mut writer = WriterBuilder::new().has_headers(false).from_writer(file);
    writer.serialize(("Bob", 35u8))?;
    writer.flush()?;
    drop(writer);
    println!("{}", fs::read_to_string(&path)?);
    // Name,Age\nAlice,28\nBob,35\n
    // has_headers(false) disables automatic Serde headers; it does not inspect the
    // existing file. This demo created a nonempty file ending with a record terminator.
    // For arbitrary existing files, verify schema / final terminator before appending.
    Ok(())
}

// ============ //
//    main()    //
// ============ //

fn main() -> DemoResult {
    println!();

    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = std::env::temp_dir().join(format!("rust_csv_{}_{stamp}", std::process::id()));
    fs::create_dir(&root)?;
    println!("Demo directory: {}\n", root.display());

    let result: DemoResult = (|| {
        demo_read()?;
        println!("\n============================\n");
        demo_headerless()?;
        println!("\n============================\n");
        demo_collect_maps()?;
        println!("\n============================\n");
        demo_delimiters(&root)?;
        println!("\n============================\n");
        demo_write(&root)?;
        println!("\n============================\n");
        demo_serde(&root)?;
        println!("\n============================\n");
        demo_options()?;
        println!("\n============================\n");
        demo_quoting()?;
        println!("\n============================\n");
        demo_batches()?;
        println!("\n============================\n");
        demo_buffers_bytes()?;
        println!("\n============================\n");
        demo_errors()?;
        println!("\n============================\n");
        demo_append(&root)?;
        Ok(())
    })();

    let cleanup: io::Result<()> = fs::remove_dir_all(&root); // attempt even after demo errors
    result?;
    cleanup?;
    println!("\nDemo directory removed.");
    Ok(())
}

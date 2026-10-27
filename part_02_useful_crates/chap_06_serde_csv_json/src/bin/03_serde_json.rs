/*
serde_json: read, construct, modify, and write JSON data.

1. Parse a string into Value (Python json.loads)
2. Checked access; missing vs null; JSON Pointer
3. Construct / modify values with json!
4. Parse nested data into typed Rust structs
5. Convert between typed data and Value
6. Compact / pretty output; UTF-8 bytes (Python json.dumps)
7. Custom indentation
8. File read / write (Python json.load / json.dump)
9. JSON types and Rust conversions
10. Diagnose syntax / schema errors
11. Read a stream of separate JSON values
12. Read / write JSON Lines (NDJSON)

Setup (run in this chapter's Cargo package directory):
    cargo add serde --features derive
    cargo add serde_json

Save as src/bin/03_serde_json.rs and run:
    cargo run --bin 03_serde_json

No [[bin]] entry is needed for src/bin/.
01_serde.rs explains derives / attributes; this guide focuses on JSON operations.
main() creates its own temporary directory and removes it after the demos.
No external data files are required.
Reference: https://docs.rs/serde_json/latest/serde_json/
*/

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

type DemoResult = Result<(), Box<dyn std::error::Error>>;

const PEOPLE: &str = r#"{
    "People": [
        {"Name":"John", "Age":30, "City":"New York"},
        {"Name":"Anna", "Age":22, "City":"London"},
        {"Name":"Mike", "Age":32, "City":"Chicago"}
    ]
}"#;

// ============================================================================
// 1. Parse a string into Value
// ============================================================================

fn demo_loads() -> DemoResult {
    let data: Value = serde_json::from_str(PEOPLE)?;
    println!("{}", data["People"][0]["Age"]); // 30
    println!("{}", data["People"][0]["Name"]); // "John" (JSON string includes quotes)

    if let Some(name) = data["People"][0]["Name"].as_str() {
        println!("{name}"); // John, as an ordinary Rust &str
    }
    // Value is an enum, not a dict: Object, Array, String, Number, Bool, Null.
    // from_str can parse any JSON root value, including arrays and scalars.
    // JSON 30 is a number; JSON "30" remains a string. No automatic coercion.
    Ok(())
}

// ============================================================================
// 2. Checked access; missing vs null
// ============================================================================

fn demo_access() -> DemoResult {
    let data = json!({"name":"Alice", "email":null, "scores":[85,90]});
    println!("{:?}", data.get("missing")); // None
    println!("{:?}", data.get("email")); // Some(Null)
    println!("{}", data["missing"]); // null
    println!("{}", data["email"]); // null
    // Immutable indexing returns Null for missing keys / wrong types / bad indices.
    // Use get() to distinguish missing from an explicit null.

    let first_score = data.get("scores").and_then(|v| v.get(0)).and_then(Value::as_u64);
    println!("{first_score:?}"); // Some(85)
    println!("{:?}", data.pointer("/scores/1").and_then(Value::as_u64)); // Some(90)
    // JSON Pointer navigates nested data; escape key '~' as ~0 and '/' as ~1.
    // as_str / as_bool / as_i64 / as_array etc. return Option, not coerced values.
    // unwrap() below is used only when a demo constructed a known, fixed shape.
    Ok(())
}

// ============================================================================
// 3. Construct and modify values
// ============================================================================

fn demo_json_macro() -> DemoResult {
    let name = "Alice";
    let age = 28;
    let mut data = json!({"name":name, "age":age, "scores":[85], "temporary":true});

    data["age"] = json!(age + 1);
    data["city"] = json!("Seoul"); // insert a missing OBJECT key
    data["scores"].as_array_mut().unwrap().push(json!(90));
    let removed = data.as_object_mut().unwrap().remove("temporary");
    println!("removed: {removed:?}"); // Some(Bool(true))
    println!("{}", serde_json::to_string_pretty(&data)?);
    // Mutable indexing can panic for incompatible shapes / out-of-bounds arrays.
    // For uncertain input, use get_mut / as_object_mut / as_array_mut and handle None.
    // Object key order isn't semantically meaningful. Don't depend on insertion
    // order unless you deliberately configure serde_json's preserve_order feature.
    Ok(())
}

// ============================================================================
// 4. Nested typed data
// ============================================================================

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "PascalCase")]
struct Person {
    name: String,
    age: u8,
    city: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct People {
    #[serde(rename = "People")]
    people: Vec<Person>,
}

fn demo_typed() -> DemoResult {
    let data: People = serde_json::from_str(PEOPLE)?;
    for person in &data.people {
        println!("{}: age {}, city {}", person.name, person.age, person.city);
    }
    // Structs give checked field names / types. Vec indices still need bounds checks.
    // Use Value for unknown / varying schemas; typed structs for known schemas.
    // Derive attributes map external Name / Age / City to Rust name / age / city.
    // Required fields and numeric ranges are checked, not arbitrary business rules.
    Ok(())
}

// ============================================================================
// 5. Value <-> typed data
// ============================================================================

fn demo_value_conversion() -> DemoResult {
    let person = Person { name: "Alice".into(), age: 28, city: "Seoul".into() };
    let mut value = serde_json::to_value(&person)?;
    value["Age"] = json!(29);

    let updated: Person = serde_json::from_value(value)?;
    println!("{updated:?}"); // Person { name: "Alice", age: 29, city: "Seoul" }
    // to_value avoids a text intermediate. from_value consumes the Value.
    // Clone a Value first only when you need to retain it; large clones cost memory.
    Ok(())
}

// ============================================================================
// 6. Compact / pretty text and bytes
// ============================================================================

fn demo_dumps() -> DemoResult {
    let person = Person { name: "Alice".into(), age: 28, city: "서울".into() };
    println!("{}", serde_json::to_string(&person)?); // compact JSON
    println!("{}", serde_json::to_string_pretty(&person)?); // default two-space indentation

    let bytes: Vec<u8> = serde_json::to_vec(&person)?;
    let restored: Person = serde_json::from_slice(&bytes)?;
    assert_eq!(person, restored);
    println!("UTF-8 bytes: {}", bytes.len());
    // to_vec_pretty also exists. Non-ASCII text is normally emitted as UTF-8,
    // not forced into \u escapes like Python json.dumps' default ensure_ascii=True.
    // Pretty vs compact affects layout, not the JSON value.
    Ok(())
}

// ============================================================================
// 7. Custom indentation
// ============================================================================

fn demo_formatting() -> DemoResult {
    let data = json!({"name":"Alice", "scores":[85,90]});
    let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
    let mut bytes = Vec::new();
    let mut serializer = serde_json::Serializer::with_formatter(&mut bytes, formatter);
    data.serialize(&mut serializer)?;
    drop(serializer); // release the mutable borrow before using bytes
    println!("{}", String::from_utf8(bytes)?); // four-space indentation

    // JSON requires ',' between elements and ':' between keys / values.
    // The Python script's separators=(';', '__') produces text that is NOT JSON;
    // don't reproduce that as an interoperable JSON format.
    Ok(())
}

// ============================================================================
// 8. File I/O: load / dump equivalents
// ============================================================================

fn demo_files(root: &Path) -> DemoResult {
    let person = Person { name: "Bob".into(), age: 35, city: "San Francisco".into() };
    let path = root.join("person.json");

    let mut writer = BufWriter::new(File::create(&path)?);
    serde_json::to_writer_pretty(&mut writer, &person)?;
    writer.write_all(b"\n")?;
    writer.flush()?; // to_writer doesn't flush your BufWriter for you
    drop(writer);

    let reader = BufReader::new(File::open(&path)?);
    let loaded: Person = serde_json::from_reader(reader)?;
    println!("{loaded:?}");
    // File::create truncates existing data. These demos write only to their own
    // temporary directory. Unlike csv, buffering a File is useful for serde_json.
    // from_reader constructs an owned result; it doesn't lend input &str slices.

    let text = fs::read_to_string(&path)?;
    let loaded: Person = serde_json::from_str(&text)?;
    println!("loaded from text: {}", loaded.name);
    // Reading the whole file first can be faster, but also keeps all source text
    // in memory. from_reader avoids that text buffer; the result may still be large.
    Ok(())
}

// ============================================================================
// 9. JSON types and Rust conversions
// ============================================================================

fn demo_types() -> DemoResult {
    let data: Value = serde_json::from_str(r#"[null,true,false,"hello",42,1.5,[1,2],{"x":1}]"#)?;
    for value in data.as_array().unwrap() {
        let kind = match value {
            Value::Null => "null", Value::Bool(_) => "boolean", Value::Number(_) => "number",
            Value::String(_) => "string", Value::Array(_) => "array", Value::Object(_) => "object",
        };
        println!("{kind}: {value}");
    }
    // JSON null -> None for an Option<T>; booleans -> bool; strings -> String.
    // Arrays -> Vec<T> / fixed arrays / tuples, depending on the target schema.
    // Objects -> structs / string-keyed maps. Numbers -> chosen numeric Rust type.
    // JSON object keys are strings; not every serializable Rust map key fits JSON.

    assert!(serde_json::from_str::<u8>("256").is_err()); // overflow, not truncation
    assert!(serde_json::from_str::<u8>(r#""28""#).is_err()); // string != numeric value
    assert!(serde_json::from_str::<Value>("NaN").is_err()); // NaN isn't JSON syntax
    assert!(serde_json::Number::from_f64(f64::NAN).is_none());
    // Default Value numbers use finite floating point / signed or unsigned 64-bit
    // integer storage. JSON numeric precision is not Python's arbitrary-size int.
    // An optional arbitrary_precision feature changes Value's number representation.
    Ok(())
}

// ============================================================================
// 10. Syntax / EOF / schema errors
// ============================================================================

fn demo_errors() -> DemoResult {
    for text in [
        r#"{"Name":"Alice",}"#, // trailing comma -> syntax error
        r#"{"Name":"Alice""#, // missing closing brace -> EOF
        r#"{"Name":"Alice","Age":"28","City":"Seoul"}"#, // wrong field type -> data error
    ] {
        match serde_json::from_str::<Person>(text) {
            Ok(person) => println!("{person:?}"),
            Err(error) => println!("{:?} at {}:{}: {}", error.classify(), error.line(), error.column(), error),
        }
    }
    // Error categories: Io, Syntax, Data, Eof. Positions are diagnostics, not
    // character indices to slice by. Expected bad inputs are handled, so demos continue.
    // from_str rejects trailing non-whitespace after a JSON document.
    // Standard JSON has no comments / single-quoted strings / trailing commas.
    Ok(())
}

// ============================================================================
// 11. Stream separate JSON values
// ============================================================================

fn demo_stream() -> DemoResult {
    let text = r#"{"Name":"Alice","Age":28,"City":"Seoul"}
                  {"Name":"Bob","Age":35,"City":"Chicago"}"#;
    let stream = serde_json::Deserializer::from_str(text).into_iter::<Person>();
    for result in stream {
        let person = result?;
        println!("streamed: {}", person.name); // Alice, Bob
    }
    // Reads successive top-level JSON values, not the elements of a single array.
    // Separating values with whitespace avoids numeric / literal boundary ambiguity.
    // Use Vec<Person> for a normal JSON array; that materializes all its elements.
    // StreamDeserializer isn't a general recover-after-invalid-JSON mechanism.
    Ok(())
}

// ============================================================================
// 12. JSON Lines (NDJSON)
// ============================================================================

fn demo_json_lines(root: &Path) -> DemoResult {
    let path = root.join("people.jsonl");
    let people: People = serde_json::from_str(PEOPLE)?;

    let mut writer = BufWriter::new(File::create(&path)?);
    for person in &people.people {
        serde_json::to_writer(&mut writer, person)?; // COMPACT output: one value per line
        writer.write_all(b"\n")?;
    }
    writer.flush()?;
    drop(writer);

    for (index, line) in BufReader::new(File::open(&path)?).lines().enumerate() {
        let line = line?;
        match serde_json::from_str::<Person>(&line) {
            Ok(person) => println!("line {}: {}", index + 1, person.name),
            Err(error) => println!("invalid JSON on line {}: {error}", index + 1),
        }
    }
    // One complete JSON value per physical line; not a single ordinary JSON document.
    // Don't pretty-print JSONL records: embedded formatting newlines break framing.
    // Newlines INSIDE string values are escaped by the serializer, so remain safe.
    // This loop can handle bad lines independently; policy for blank lines is up to you.
    // It holds one source line at a time, though an individual line can still be huge.
    Ok(())
}

// ============ //
//    main()    //
// ============ //

fn main() -> DemoResult {
    println!();

    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = std::env::temp_dir().join(format!("rust_json_{}_{stamp}", std::process::id()));
    fs::create_dir(&root)?;
    println!("Demo directory: {}\n", root.display());

    let result: DemoResult = (|| {
        demo_loads()?;
        println!("\n============================\n");
        demo_access()?;
        println!("\n============================\n");
        demo_json_macro()?;
        println!("\n============================\n");
        demo_typed()?;
        println!("\n============================\n");
        demo_value_conversion()?;
        println!("\n============================\n");
        demo_dumps()?;
        println!("\n============================\n");
        demo_formatting()?;
        println!("\n============================\n");
        demo_files(&root)?;
        println!("\n============================\n");
        demo_types()?;
        println!("\n============================\n");
        demo_errors()?;
        println!("\n============================\n");
        demo_stream()?;
        println!("\n============================\n");
        demo_json_lines(&root)?;
        Ok(())
    })();

    let cleanup = fs::remove_dir_all(&root); // attempt cleanup even if a demo failed
    result?;
    cleanup?;
    println!("\nDemo directory removed.");
    Ok(())
}

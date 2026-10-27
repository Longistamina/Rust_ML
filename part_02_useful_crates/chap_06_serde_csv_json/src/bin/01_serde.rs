/*
Serde: describe how Rust types serialize and deserialize.

1. Serialize / Deserialize derives and a round trip
2. Nested structs, collections, and maps
3. Optional fields: missing, null, Some, None
4. Field names: rename, rename_all, alias
5. Defaults for missing fields
6. Skip fields and omit optional values
7. Unknown fields and strict schemas
8. Flatten shared fields / capture extra keys
9. Enums: default external tagging
10. Internally / adjacently tagged enums
11. Untagged alternatives
12. Newtype wrappers and transparent
13. Generic data structures
14. Borrowed vs owned data
15. Custom field deserialization

Setup:
    cargo add serde --features derive
    cargo add serde_json

Keep this file beside Cargo.toml and add:
    [[bin]]
    name = "01_serde"
    path = "01_serde.rs"

Run:
    cargo run --bin 01_serde

Serialize: Rust value -> a format's representation.
Deserialize: a format's representation -> Rust value.
serde defines traits / derives; serde_json implements the JSON format.
JSON below makes Serde behavior visible; JSON file I/O belongs to 03_serde_json.rs.
Serde attributes can apply across formats, but each format has limitations.
For example, nested structures supported by JSON may not fit a flat CSV record.
Derive only the traits a type needs; Debug is for printing, not serialization.
References:
https://serde.rs/attributes.html
https://serde.rs/field-attrs.html
https://serde.rs/enum-representations.html
*/

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!();

    // ============================================================================
    // 1. Derives and a round trip
    // ============================================================================
    {
        #[derive(Debug, Serialize, Deserialize, PartialEq)]
        struct Person {
            name: String,
            age: u8,
        }

        let original = Person { name: "Alice".into(), age: 28 };
        let encoded = serde_json::to_string(&original)?;
        println!("{encoded}"); // {"name":"Alice","age":28}

        let decoded: Person = serde_json::from_str(&encoded)?;
        println!("{decoded:?}"); // Person { name: "Alice", age: 28 }
        assert_eq!(original, decoded);

        // Serialization borrows original; it doesn't move it.
        // The target type annotation tells Deserialize what to construct.
        // Deriving traits doesn't supply field defaults or application validation.
        // Deserialize rejects negative / overflowing u8 ages, but age=250 is
        // representable; enforce a realistic age separately if required.

        println!("\n============================\n");
    }

    // ============================================================================
    // 2. Nested types, collections, and maps
    // ============================================================================
    {
        #[derive(Debug, Serialize, Deserialize)]
        struct Address { city: String }

        #[derive(Debug, Serialize, Deserialize)]
        struct Team {
            name: String,
            address: Address,
            members: Vec<String>,
            scores: BTreeMap<String, u32>,
        }

        let team: Team = serde_json::from_str(r#"{
            "name":"Lab", "address":{"city":"Seoul"},
            "members":["Alice","Bob"], "scores":{"Alice":10,"Bob":8}
        }"#)?;
        println!("{}", serde_json::to_string(&team)?);

        // All fields must themselves support the corresponding Serde trait.
        // Common primitives, Vec, Option, tuples, arrays, and maps already do.
        // BTreeMap gives ordered keys. HashMap iteration doesn't guarantee order.
        // JSON object keys must be representable as strings; arbitrary Rust map
        // keys that implement Serialize aren't necessarily valid JSON object keys.

        println!("\n============================\n");
    }

    // ============================================================================
    // 3. Option: absent, null, and present
    // ============================================================================
    {
        #[derive(Debug, Serialize, Deserialize)]
        struct Contact { email: Option<String> }

        for text in [r#"{}"#, r#"{"email":null}"#, r#"{"email":"alice@example.com"}"#] {
            let contact: Contact = serde_json::from_str(text)?;
            println!("{contact:?}"); // None, None, Some("alice@example.com")
        }

        println!("{}", serde_json::to_string(&Contact { email: None })?); // {"email":null}
        // For this derived struct, missing and null both become None.
        // They are different JSON inputs, but this type deliberately loses that distinction.
        // Use skip_serializing_if to omit None instead of writing null (section 6).

        println!("\n============================\n");
    }

    // ============================================================================
    // 4. Rename fields and accept legacy names
    // ============================================================================
    {
        #[derive(Debug, Serialize, Deserialize)]
        #[serde(rename_all = "camelCase")]
        struct User {
            user_id: u32,
            #[serde(rename = "displayName", alias = "name")]
            display_name: String,
        }

        let user: User = serde_json::from_str(r#"{"userId":7,"name":"Alice"}"#)?;
        println!("{}", serde_json::to_string(&user)?); // {"userId":7,"displayName":"Alice"}
        // rename affects both directions; alias accepts additional INPUT names only.
        // Supplying both the canonical and alias key is a duplicate-field error.

        println!("\n============================\n");
    }

    // ============================================================================
    // 5. Defaults for missing fields
    // ============================================================================
    {
        fn default_workers() -> u16 { 4 }

        #[derive(Debug, Serialize, Deserialize)]
        struct Settings {
            #[serde(default = "default_workers")]
            workers: u16,
            #[serde(default)]
            tags: Vec<String>,
        }

        let settings: Settings = serde_json::from_str("{}")?;
        println!("{settings:?}"); // Settings { workers: 4, tags: [] }
        assert!(serde_json::from_str::<Settings>(r#"{"workers":null}"#).is_err());
        // Defaults apply when absent, not when null / invalid text is supplied.
        // A struct-level #[serde(default)] instead uses the struct's Default impl
        // to fill absent fields. Derive Default or implement it yourself.

        println!("\n============================\n");
    }

    // ============================================================================
    // 6. Skip fields / omit None
    // ============================================================================
    {
        #[derive(Debug, Serialize, Deserialize)]
        struct Record {
            id: u32,
            #[serde(skip)]
            cache: Vec<u8>,
            #[serde(skip_serializing_if = "Option::is_none")]
            note: Option<String>,
        }

        let record: Record = serde_json::from_str(r#"{"id":1,"cache":[1,2]}"#)?;
        println!("{}", record.cache.len()); // 0: skipped on input, filled by Default
        println!("{}", serde_json::to_string(&record)?); // {"id":1}

        // skip: omit on output AND ignore on input; default the field on input.
        // skip_serializing: omit on output, still deserialize normally.
        // skip_deserializing: default on input, still serialize normally.
        // skip_serializing_if only affects output; ensure missing input can be
        // handled too (Option does that here; other types may need default).

        println!("\n============================\n");
    }

    // ============================================================================
    // 7. Unknown fields and type errors
    // ============================================================================
    {
        #[derive(Debug, Serialize, Deserialize)]
        #[serde(deny_unknown_fields)]
        struct Strict { count: u16 }

        for text in [r#"{"count":3}"#, r#"{"count":3,"extra":true}"#, r#"{"count":"3"}"#, "{}"] {
            match serde_json::from_str::<Strict>(text) {
                Ok(value) => println!("{value:?}"),
                Err(error) => println!("error: {error}"),
            }
        }
        // Only the first succeeds. Quoted "3" is a string, not a number.
        // With ordinary derive, unknown JSON fields are ignored by default;
        // deny_unknown_fields rejects them. Required missing fields still error.

        println!("\n============================\n");
    }

    // ============================================================================
    // 8. Flatten shared fields and capture extras
    // ============================================================================
    {
        #[derive(Debug, Serialize, Deserialize)]
        struct Identity { id: u32 }

        #[derive(Debug, Serialize, Deserialize)]
        struct Item {
            #[serde(flatten)]
            identity: Identity,
            #[serde(flatten)]
            extra: BTreeMap<String, serde_json::Value>,
        }

        let item: Item = serde_json::from_str(r#"{"id":1,"color":"blue","active":true}"#)?;
        println!("id: {}; extra keys: {:?}", item.identity.id, item.extra.keys().collect::<Vec<_>>());
        // id: 1; extra keys: ["active", "color"]
        println!("{}", serde_json::to_string(&item)?);
        // {"id":1,"active":true,"color":"blue"}: no nested identity / extra objects
        // flatten cannot be combined with deny_unknown_fields. Avoid key collisions.

        println!("\n============================\n");
    }

    // ============================================================================
    // 9. Enums: default external tagging
    // ============================================================================
    {
        #[derive(Debug, Serialize, Deserialize)]
        enum Event {
            Ready,
            Message(String),
            Move { x: i32, y: i32 },
        }

        for event in [Event::Ready, Event::Message("hello".into()), Event::Move { x: 1, y: 2 }] {
            println!("{}", serde_json::to_string(&event)?);
        }
        // "Ready"
        // {"Message":"hello"}
        // {"Move":{"x":1,"y":2}}
        // The variant name identifies which Rust enum variant to construct.

        println!("\n============================\n");
    }

    // ============================================================================
    // 10. Internal and adjacent enum tags
    // ============================================================================
    {
        #[derive(Debug, Serialize, Deserialize)]
        #[serde(tag = "type", rename_all = "snake_case")]
        enum Action { Move { x: i32, y: i32 }, Stop }

        let action: Action = serde_json::from_str(r#"{"type":"move","x":1,"y":2}"#)?;
        println!("{action:?}"); // Move { x: 1, y: 2 }
        // Internal tags put the discriminator beside struct fields.
        // Internally tagged enums cannot contain tuple variants.

        #[derive(Debug, Serialize, Deserialize)]
        #[serde(tag = "kind", content = "data", rename_all = "snake_case")]
        enum Payload { Number(i32), Point(i32, i32) }

        println!("{}", serde_json::to_string(&Payload::Point(1, 2))?);
        // {"kind":"point","data":[1,2]}; adjacent tags also support tuple variants.

        println!("\n============================\n");
    }

    // ============================================================================
    // 11. Untagged alternatives
    // ============================================================================
    {
        #[derive(Debug, Serialize, Deserialize)]
        #[serde(untagged)]
        enum NumberOrText { Number(i64), Text(String) }

        for text in ["42", r#""42""#] {
            let value: NumberOrText = serde_json::from_str(text)?;
            println!("{value:?}"); // Number(42), Text("42")
        }
        // Tries variants in declaration order; the first successful variant wins.
        // Overlapping schemas can be ambiguous; an explicit tag is often clearer.
        // This represents two valid input types; it doesn't coerce them to one type.

        println!("\n============================\n");
    }

    // ============================================================================
    // 12. Transparent wrappers
    // ============================================================================
    {
        #[derive(Debug, Serialize, Deserialize)]
        #[serde(transparent)]
        struct UserId { value: u64 }

        let id: UserId = serde_json::from_str("123")?;
        println!("{id:?}"); // UserId { value: 123 }
        println!("{}", serde_json::to_string(&id)?); // 123, not {"value":123}
        // A transparent wrapper serializes as its single data field.
        // A one-field tuple newtype (struct UserId(u64)) already behaves similarly.
        // Wrapping the type doesn't automatically enforce application invariants.

        println!("\n============================\n");
    }

    // ============================================================================
    // 13. Generic structures
    // ============================================================================
    {
        #[derive(Debug, Serialize, Deserialize)]
        struct Page<T> { items: Vec<T>, total: usize }

        let page: Page<u32> = serde_json::from_str(r#"{"items":[10,20],"total":2}"#)?;
        println!("{page:?}"); // Page { items: [10, 20], total: 2 }
        // Derive adds suitable trait bounds for T; every concrete item type must
        // support the relevant operation. Usually no manual #[serde(bound)] is needed.

        println!("\n============================\n");
    }

    // ============================================================================
    // 14. Borrowed vs owned data
    // ============================================================================
    {
        #[derive(Debug, Serialize, Deserialize)]
        struct Borrowed<'a> { name: &'a str }

        let source = String::from(r#"{"name":"Alice"}"#);
        let person: Borrowed<'_> = serde_json::from_str(&source)?;
        println!("{}", person.name); // Alice; borrows from source
        // person cannot outlive source. &str borrowing works when the format can
        // lend the original text; escaped JSON strings can require allocation and
        // therefore cannot always deserialize directly into a borrowed &str.
        // Prefer String for ordinary owned records and reader/file-based loading.
        // Generic reader APIs commonly require DeserializeOwned; it means the
        // result cannot borrow from the input, not merely that it has Deserialize.

        println!("\n============================\n");
    }

    // ============================================================================
    // 15. Explicit conversion with deserialize_with
    // ============================================================================
    {
        fn from_decimal_string<'de, D>(deserializer: D) -> Result<u16, D::Error>
        where D: serde::Deserializer<'de> {
            let text = String::deserialize(deserializer)?;
            text.parse::<u16>().map_err(serde::de::Error::custom)
        }

        #[derive(Debug, Serialize, Deserialize)]
        struct Endpoint {
            #[serde(deserialize_with = "from_decimal_string")]
            port: u16,
        }

        let endpoint: Endpoint = serde_json::from_str(r#"{"port":"8080"}"#)?;
        println!("{endpoint:?}"); // Endpoint { port: 8080 }
        println!("{}", serde_json::to_string(&endpoint)?); // {"port":8080}
        // This conversion only changes INPUT: it requires a string, then parses it.
        // Ordinary serialization still emits a number, so this schema is asymmetric.
        // serialize_with customizes output; with="module" groups both directions.
        // For most types, derive + attributes are enough; manual trait implementations
        // and custom visitors are advanced topics rather than prerequisites.

        println!("\n============================\n");
    }
    Ok(())
}

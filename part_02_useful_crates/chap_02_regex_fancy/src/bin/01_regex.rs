/*
regex: search, extract, split, and replace text.

1. Compile a regex; raw strings; is_match / find
2. Character classes, escapes, dot, boundaries, anchors
3. Quantifiers: greedy / lazy / bounded repetition
4. Alternation and non-capturing groups
5. Captures: numbered / named / optional groups
6. Find all matches: find_iter vs captures_iter
7. Match positions and Unicode byte offsets
8. Split and splitn
9. Replace, replace_all, replacen; group expansion
10. Replacement closures
11. Flags and RegexBuilder
12. Escape literal text; errors; compile once and reuse

Setup:
    cargo add regex

Keep this file beside Cargo.toml and add:
    [[bin]]
    name = "04_regex"
    path = "04_regex.rs"

Run:
    cargo run --bin 04_regex
*/

use regex::{Captures, NoExpand, Regex, RegexBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!();

    // ============================================================================
    // 1. Compile and search
    // ============================================================================
    {
        let text = "The rain in Spain";
        let re = Regex::new(r"The.*Spain")?;
        println!("{}", re.is_match(text)); // true
        if let Some(m) = re.find(text) {
            println!("{}", m.as_str()); // The rain in Spain
        }

        // r"\d+" avoids double escaping; ordinary "\\d+" is equivalent.
        // r#"..."..."# is useful when the pattern contains double quotes.
        // Searches are unanchored by default, like Python re.search().
        // Python re.match(): add \A to anchor at the start of the entire input.
        // Python re.fullmatch(): wrap the pattern as \A(?:pattern)\z.
        assert!(Regex::new(r"\AThe")?.is_match(text));
        assert!(Regex::new(r"\A(?:The.*Spain)\z")?.is_match(text));
        assert!(Regex::new("Portugal")?.find(text).is_none());

        println!("\n====================================\n")
    }

    // ============================================================================
    // 2. Pattern syntax
    // ============================================================================
    {
        let re = Regex::new(r"[a-m]")?;
        let matches: Vec<_> = re.find_iter("The rain in Spain").map(|m| m.as_str()).collect();
        println!("{matches:?}"); // ["h", "e", "a", "i", "i", "a", "i"]

        // [arn] one of a/r/n; [a-z] range; [^arn] anything except a/r/n.
        // [0-9] ASCII digit; [a-zA-Z] ASCII letter; [+] literal plus.
        // \d / \D: Unicode decimal digit / non-digit.
        // \s / \S: Unicode whitespace / non-whitespace.
        // \w / \W: Unicode word character / non-word character; \w includes '_'.
        // \b / \B: word boundary / non-boundary; \A / \z: input start / end.
        // ^ / $: input start / end, or line boundaries with (?m).
        // Rust uses \z, not Python's \Z. $ does not have Python's special
        // before-final-newline behavior without multiline mode.
        // . matches one Unicode scalar except '\n'; \. matches a literal dot.
        let digits = Regex::new(r"\d+")?;
        println!("{}", digits.find("That will be 59 dollars").unwrap().as_str()); // 59
        println!("{}", Regex::new(r"he..o")?.find("hello planet").unwrap().as_str()); // hello

        assert!(Regex::new(r"\bSpain\b")?.is_match("The rain in Spain"));
        assert!(Regex::new(r"\.")?.is_match("file.txt"));
        // unwrap() here is safe for these fixed examples; handle Option for user input.

        println!("\n====================================\n")
    }

    // ============================================================================
    // 3. Quantifiers
    // ============================================================================
    {
        // * zero or more;
        // + one or more;
        // ? zero or one.
        // {n} exactly n; {n,} at least n; {n,m} between n and m.
        // Append ? for lazy matching: *?, +?, ??, {n,m}?.
        let text = "a aa aaa aaaa b";
        let re = Regex::new(r"a+")?;
        println!("{:?}", re.find_iter(text).map(|m| m.as_str()).collect::<Vec<_>>());
        // ["a", "aa", "aaa", "aaaa"]
        // a* / a? can match EMPTY strings too. Empty-match iteration details
        // can differ between engines; don't copy Python's full result list blindly.

        let text = "<a>first</a><b>second</b>";
        println!("{}", Regex::new(r"<b.*>")?.find(text).unwrap().as_str()); // <b>second</b>
        println!("{}", Regex::new(r"<b.*?>")?.find(text).unwrap().as_str()); // <b>
        println!("{}", Regex::new(r"<b.+?>")?.find(text).unwrap().as_str()); // <b>second</b>
        // Lazy + still needs a character: it consumes the first '>' and continues.
        // To stop at the next '>', use an explicit class, e.g. <[^>]+>.

        let re = Regex::new(r"\d{1,3}")?;
        println!("{:?}", re.find_iter("1234 56 7 89012").map(|m| m.as_str()).collect::<Vec<_>>());
        // ["123", "4", "56", "7", "890", "12"] (not whole-number validation)

        assert!(Regex::new(r"he.{2}o")?.is_match("hello"));
        assert!(Regex::new(r"colou?r")?.is_match("color"));

        println!("\n====================================\n")
    }

    // ============================================================================
    // 4. Alternation / non-capturing groups
    // ============================================================================
    {
        let re = Regex::new(r"falls|stays|mainly")?;
        println!("{:?}", re.find_iter("rain falls mainly in the plain").map(|m| m.as_str()).collect::<Vec<_>>());
        // ["falls", "mainly"]
        // Alternatives prefer the first matching branch, not necessarily the longest.

        // (?:...) groups without creating a capture; use it to scope alternation.
        let re = Regex::new(r"\+(?:82|84|87)-([0-9]+)")?;
        for caps in re.captures_iter("+82-1012345678, +84-526913, +87-2352345267") {
            println!("{}", &caps[1]); // only the number after the country prefix
        }

        println!("\n====================================\n")
    }

    // ============================================================================
    // 5. Numbered / named / optional captures
    // ============================================================================
    {
        let re = Regex::new(r"\A(?P<position>[0-9]+\.*[0-9]*)(?P<label>[a-zA-Z]+)\z")?;
        for text in ["312.1C", "42.6gap", "100A"] {
            if let Some(caps) = re.captures(text) {
                println!("whole: {}; position: {}; label: {}", &caps[0], &caps[1], &caps["label"]);
            }
        }
        // Group 0 = whole match;
        // groups 1.. = capture parentheses.
        // (?P<name>...) and (?<name>...) both name a group.
        // Indexing caps[n] / caps["name"] panics if the group didn't participate.

        let re = Regex::new(r"\A([a-z]+)(?:-([0-9]+))?\z")?;
        let caps = re.captures("item").unwrap();
        println!("optional digits: {:?}", caps.get(2).map(|m| m.as_str())); // None
        // caps.name("group") also returns Option<Match>.

        println!("\n====================================\n")
    }

    // ============================================================================
    // 6. All matches vs capture groups
    // ============================================================================
    {
        let text = "cat123 dog456 bird789";
        let re = Regex::new(r"([a-z]+)([0-9]+)")?;
        let whole: Vec<_> = re.find_iter(text).map(|m| m.as_str()).collect();
        println!("{whole:?}"); // ["cat123", "dog456", "bird789"]

        let groups: Vec<_> = re.captures_iter(text)
            .map(|caps| (caps.get(1).unwrap().as_str(), caps.get(2).unwrap().as_str()))
            .collect();
        println!("{groups:?}"); // [("cat", "123"), ("dog", "456"), ("bird", "789")]

        // find_iter always yields WHOLE matches, even if the pattern has captures.
        // Python findall changes its result shape based on capture groups;
        // Rust keeps find_iter and captures_iter separate. Matches don't overlap.
        println!("count: {}", re.find_iter(text).count()); // 3

        println!("\n====================================\n")
    }

    // ============================================================================
    // 7. Match positions / Unicode
    // ============================================================================
    {
        let text = "안녕 Spain";
        if let Some(m) = Regex::new(r"\bSpain\b")?.find(text) {
            println!("{}: {}..{}", m.as_str(), m.start(), m.end()); // Spain: 7..12
            println!("{:?}", m.range()); // 7..12 (end-exclusive)
            println!("{}", &text[m.range()]); // Spain
        }
        // Offsets are UTF-8 BYTES, unlike Python str match character offsets.
        // Match boundaries from regex::Regex are valid UTF-8 slicing boundaries.
        // Rust DOES support &text[start..end] slicing at valid boundaries.
        // Keep the original text yourself (Python's match.string equivalent).

        assert!(Regex::new(r"\A\d+\z")?.is_match("１２３")); // Unicode decimal digits
        assert!(!Regex::new(r"\A[0-9]+\z")?.is_match("１２３")); // ASCII only

        println!("\n====================================\n")
    }

    // ============================================================================
    // 8. Split / splitn
    // ============================================================================
    {
        let text = "The rain in Spain";
        let re = Regex::new(r"\s+")?;
        println!("{:?}", re.split(text).collect::<Vec<_>>()); // ["The", "rain", "in", "Spain"]
        println!("{:?}", re.splitn(text, 2).collect::<Vec<_>>()); // ["The", "rain in Spain"]
        println!("{:?}", re.splitn(text, 3).collect::<Vec<_>>()); // ["The", "rain", "in Spain"]
        // Rust splitn(text, n): maximum n OUTPUT pieces.
        // Python re.split(..., maxsplit=k): maximum k SPLITS => splitn(..., k+1).
        // Capturing groups in the separator are NOT included in Rust's result.
        // For literal whitespace splitting, str::split_whitespace() is often enough.

        println!("\n====================================\n")
    }

    // ============================================================================
    // 9. Replace / group expansion
    // ============================================================================
    {
        let text = "The rain in Spain";
        let re = Regex::new(r"\s")?;
        println!("{}", re.replace(text, "_")); // The_rain in Spain (FIRST match)
        println!("{}", re.replace_all(text, "_")); // The_rain_in_Spain
        println!("{}", re.replacen(text, 2, "_")); // The_rain_in Spain
        // replacen(..., 0, ...) replaces ALL matches, not zero matches.
        // Returns Cow<str>; call .into_owned() if you need an owned String.

        let re = Regex::new(r"(?P<year>[0-9]{4})-(?P<month>[0-9]{2})-(?P<day>[0-9]{2})")?;
        println!("{}", re.replace("2026-10-14", "${day}/${month}/${year}")); // 14/10/2026
        // Replacement syntax: $1, $name, ${name}; $$ inserts a literal dollar.
        // Braces disambiguate names: ${1}_suffix rather than $1_suffix.

        // This is capture expansion, not a backreference inside the regex pattern.
        let re = Regex::new(r"[0-9]+")?;
        println!("{}", re.replace("price=10", NoExpand("$1"))); // price=$1 (literal)

        println!("\n====================================\n")
    }

    // ============================================================================
    // 10. Replacement closures
    // ============================================================================
    {
        let re = Regex::new(r"[a-z]+")?;
        let result = re.replace_all("cat123 dog456", |caps: &Captures<'_>| caps[0].to_uppercase());
        println!("{result}"); // CAT123 DOG456
        // A closure computes each replacement from its captures, like re.sub(fn,...).
        // Its returned text is inserted literally, without $group expansion.

        println!("\n====================================\n")
    }

    // ============================================================================
    // 11. Flags / RegexBuilder
    // ============================================================================
    {
        // (?i) case-insensitive; (?m) multiline anchors; (?s) dot includes newline.
        // (?x) verbose: ignore pattern whitespace / # comments.
        // (?i:...) scopes a flag to one group; (?-i:...) disables it locally.
        assert!(Regex::new(r"(?i)spain")?.is_match("SPAIN"));

        let re = Regex::new(r"(?m)^cat$")?;
        assert!(re.is_match("dog\ncat\nbird"));

        assert!(Regex::new(r"(?s)\Aa.*b\z")?.is_match("a\nb"));

        let re = RegexBuilder::new(r"spain").case_insensitive(true).build()?;
        assert!(re.is_match("SPAIN"));
        // Builder equivalents also include multi_line(), dot_matches_new_line(),
        // and ignore_whitespace(). Unicode matching is enabled by default.
    }

    // ============================================================================
    // 12. Literal escaping / errors / reuse
    // ============================================================================
    {
        let literal = "file[1].txt";
        let pattern = format!(r"\A{}\z", regex::escape(literal));
        assert!(Regex::new(&pattern)?.is_match(literal));

        // Escape user-provided literal text before inserting it into a pattern.
        match Regex::new("[") {
            Ok(_) => println!("valid pattern"),
            Err(error) => eprintln!("invalid regex: {error}"),
        }

        // Compile OUTSIDE the loop. Searching doesn't return Result for this crate.
        let re = Regex::new(r"\A[0-9]+\z")?;
        for text in ["123", "abc", "456"] {
            println!("{text}: {}", re.is_match(text)); // true, false, true
        }
        // Request only what you need: is_match for a bool, find for a whole match,
        // captures when extracting groups. All capture strings borrow the input.
    }
    Ok(())
}

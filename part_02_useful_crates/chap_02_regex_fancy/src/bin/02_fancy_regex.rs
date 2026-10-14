/*
fancy_regex: look-around, backreferences, and other backtracking features.

1. API differences: searching can fail at runtime
2. Positive / negative lookahead
3. Positive / negative lookbehind
4. Combine lookbehind and lookahead
5. Numbered / named backreferences
6. Variable-length lookbehind (version / feature dependent)
7. Conditional replacements inside parentheses
8. Captures / splitting / replacement expansion
9. Atomic groups and conditionals
10. Compile errors, runtime errors, and backtracking limits

Setup (examples target fancy-regex 0.19.2 with default features):
    cargo add fancy-regex@0.19.2

Keep this file beside this chapter's Cargo.toml and add:
    [[bin]]
    name = "02_fancy_regex"
    path = "02_fancy_regex.rs"

Run:
    cargo run --bin 02_fancy_regex
*/

use fancy_regex::{Regex, RegexBuilder};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!();

    // ============================================================================
    // 1. API differences: searching can fail at runtime
    // ============================================================================
    {
        let re = Regex::new(r"\b([a-z]+)\s+\1\b")?;
        let text = "cat cat, dog bird";
        println!("{}", re.is_match(text)?); // true
        if let Some(m) = re.find(text)? {
            println!("{}", m.as_str()); // cat cat
        }

        // Regex::new(...) -> Result<Regex>
        // is_match(...)  -> Result<bool> (ordinary regex returns bool)
        // find(...)      -> Result<Option<Match>>
        // captures(...)  -> Result<Option<Captures>>
        // find_iter / captures_iter yield Result items: apply ? to EACH item.
        // Ok(None) means no match; Err(...) means execution failed.
        let matches: Vec<_> = re.find_iter(text).collect::<Result<_, _>>()?;
        println!("count: {}", matches.len()); // 1

        println!("\n============================\n");
    }

    // ============================================================================
    // 2. Positive / negative lookahead
    // ============================================================================
    {
        // X(?=Y): match X only if followed by Y; Y is not consumed.
        let re = Regex::new(r"[0-9]+(?= dollars)")?;
        let text = "59 dollars, 20 euros";
        for result in re.find_iter(text) {
            println!("{}", result?.as_str()); // 59
        }

        // X(?!Y): match X only if NOT followed by Y.
        let re = Regex::new(r"\b[A-Za-z]+\b(?!_&)")?;
        let text = "Apple &_apple_& Banana &_banana_& Cherry &_cherry_&";
        for result in re.find_iter(text) {
            println!("{}", result?.as_str()); // Apple, Banana, Cherry
        }
        // Look-around checks a position; it does not add text to the whole match.
        // Backtracking can shorten X to satisfy an assertion. Use boundaries or
        // a precise X pattern when a partial word / number would be incorrect.

        println!("\n============================\n");
    }

    // ============================================================================
    // 3. Positive / negative lookbehind
    // ============================================================================
    {
        // (?<=Y)X: match X only if preceded by Y; Y is not consumed.
        let re = Regex::new(r"(?<=&_)\w+\W")?;
        let text = "Apple &_apple_& Banana &_banana_& Cherry &_cherry_&";
        let matches = re.find_iter(text).collect::<Result<Vec<_>, _>>()?;
        println!("{:?}", matches.iter().map(|m| m.as_str()).collect::<Vec<_>>());
        // ["apple_&", "banana_&", "cherry_&"]

        // (?<!Y)X: match X only if NOT preceded by Y.
        let re = Regex::new(r"(?<!AU)\$([0-9]+)")?;
        for result in re.captures_iter("AU$10, $20, AU$30, $40") {
            let caps = result?;
            println!("{}", &caps[1]); // 20, 40
        }
        // \w includes underscores; boundaries near '_' may be surprising.

        println!("\n============================\n");
    }

    // ============================================================================
    // 4. Combine lookbehind and lookahead
    // ============================================================================
    {
        let re = Regex::new(r"(?<=&_)\w+?(?=_&)")?;
        let text = "Apple &_apple_& Banana &_banana_& Cherry &_cherry_&";
        for result in re.find_iter(text) {
            println!("{}", result?.as_str()); // apple, banana, cherry
        }
        // Both delimiters stay outside the match. +? stops before the first _&.
        if let Some(m) = re.find(text)? {
            println!("{}..{}", m.start(), m.end()); // 8..13, UTF-8 BYTE offsets
            println!("{}", &text[m.start()..m.end()]); // apple
        }

        println!("\n============================\n");
    }

    // ============================================================================
    // 5. Numbered / named backreferences
    // ============================================================================
    {
        // \1 matches the EXACT text captured by group 1, not the group's pattern.
        let re = Regex::new(r"\A([a-z]+)\s+\1\z")?;
        println!("{}", re.is_match("cat cat")?); // true
        println!("{}", re.is_match("cat dog")?); // false

        let re = Regex::new(r"\A(?P<word>[a-z]+)\s+(?P=word)\z")?;
        assert!(re.is_match("bird bird")?);
        // (?P=word) is Python-style; \k<word> is another named-backreference form.
        // Pattern backreference: \1. Replacement capture expansion: $1 / ${word}.

        let re = Regex::new(r#"(["'])(.*?)\1"#)?;
        for result in re.captures_iter(r#"'cat' and "dog""#) {
            let caps = result?;
            println!("{}", &caps[2]); // cat, dog
        }
        // The closing quote must equal the captured opening quote.
        // Toy example only: does not handle escaped quotes.

        println!("\n============================\n");
    }

    // ============================================================================
    // 6. Variable-length lookbehind
    // ============================================================================
    {
        // Current 0.19.2 supports this through variable-lookbehinds (default on).
        // Older versions / builds without that feature may reject such patterns.
        // Variable lookbehind has restrictions: do not assume every Python regex
        // module pattern is portable, especially with backreferences inside it.
        let re = Regex::new(r"(?<=prefix[0-9]+:)\w+")?;
        for result in re.find_iter("prefix1:cat prefix123:dog") {
            println!("{}", result?.as_str()); // cat, dog
        }

        let text = "XXXAAATGXXXCCCTCGXXXTCGTGCXXXGATTGAAGAXXXACC";
        let re = Regex::new(r"(?<=ATG.*?)(XXX)(?=.*?(?:TAA|TAG|TGA))")?;
        let matches = re.find_iter(text).collect::<Result<Vec<_>, _>>()?;
        println!("count: {}", matches.len()); // 3
        // Python-file example: XXX with ATG somewhere before and a stop motif after.
        // This is a text-matching demo; it does not check codon reading frames.

        println!("\n============================\n");
    }

    // ============================================================================
    // 7. Replace only inside parentheses
    // ============================================================================
    {
        let text = "x = func(a = 5, b = 10)\ny = other(c = 15, d = 20)";
        let re = Regex::new(r"(?<=\([^()]*)\s+=\s+(?=[^()]*\))")?;
        println!("{}", re.try_replacen(text, 0, "=")?);
        // x = func(a=5, b=10)
        // y = other(c=15, d=20)
        // [^()]* confines the assertions to simple, non-nested parentheses.
        // This is not a parser for nested parentheses or quoted strings.
        // try_replacen(..., 0, ...) replaces all matches and preserves runtime errors.
        // limit=1 replaces only the first; limit=2 replaces at most two.
        // replace / replace_all / replacen return Cow<str> and can PANIC on a
        // matching error. Prefer try_replacen when handling runtime failures.

        println!("\n============================\n");
    }

    // ============================================================================
    // 8. Captures / splitting / replacement expansion
    // ============================================================================
    {
        let re = Regex::new(r"(?<=\$)(?P<amount>[0-9]+)")?;
        if let Some(caps) = re.captures("price=$20")? {
            println!("{}", caps.name("amount").unwrap().as_str()); // 20
            // get(n) / name(...) return Option; indexing a missing group panics.
        }
        println!("{}", re.try_replacen("price=$20", 0, "${amount} USD")?);
        // price=$20 USD (the $ was outside the match, so remains)

        let re = Regex::new(r",\s*(?=[A-Z])")?;
        let parts: Vec<_> = re.split("Apple, Banana, lowercase, Cherry")
            .collect::<Result<_, _>>()?;
        println!("{parts:?}"); // ["Apple", "Banana, lowercase", "Cherry"]
        // split / splitn yield Result<&str>; propagate errors while collecting.
        // splitn(text, n) limits the number of output pieces, as in regex.
        // find_iter().count() would count error items too; collect Results first
        // or count successful matches explicitly in a loop using entry?.

        println!("\n============================\n");
    }

    // ============================================================================
    // 9. Atomic groups and conditionals
    // ============================================================================
    {
        // (?>...) commits to its choice; later failure can't backtrack inside it.
        let ordinary = Regex::new(r"\Aa(?:bc|b)c\z")?;
        let atomic = Regex::new(r"\Aa(?>bc|b)c\z")?;
        println!("{}", ordinary.is_match("abc")?); // true: retries branch b
        println!("{}", atomic.is_match("abc")?); // false: commits to bc
        println!("{}", atomic.is_match("abcc")?); // true

        // (?(1)yes|no): choose a branch based on whether group 1 participated.
        let re = Regex::new(r"\A(\()?cat(?(1)\)|)\z")?;
        assert!(re.is_match("cat")?);
        assert!(re.is_match("(cat)")?);
        assert!(!re.is_match("(cat")?);
        assert!(!re.is_match("cat)")?);

        println!("\n============================\n");
    }

    // ============================================================================
    // 10. Compile errors / runtime errors / limits
    // ============================================================================
    {
        match Regex::new("[") {
            Ok(_) => println!("valid pattern"),
            Err(error) => eprintln!("compile error: {error}"),
        }
        let re = RegexBuilder::new(r"\b([a-z]+)\s+\1\b")
            .backtrack_limit(100_000)
            .build()?;
        for text in ["cat cat", "cat dog"] {
            match re.is_match(text) {
                Ok(true) => println!("{text}: match"),
                Ok(false) => println!("{text}: no match"),
                Err(error) => eprintln!("{text}: runtime error: {error}"),
            }
        }
        // A backtrack limit is an operation budget, not a timeout in milliseconds.
        // Exceeding it yields an error, not a valid 'no match' result.
        // Compile once outside loops; constrain patterns to avoid needless backtracking.
        // .filter_map(Result::ok) silently discards runtime errors from iterators.

        println!("\n============================\n");
    }
    Ok(())
}

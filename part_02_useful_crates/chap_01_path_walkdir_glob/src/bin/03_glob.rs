/*
glob: search filesystem paths using wildcard patterns.

1. Basic search: Python Path.glob()
2. Wildcards: *, ?, character sets, ranges, negated sets
3. Recursive search: ** (Python Path.rglob())
4. Filter / exclude / collect / count
5. MatchOptions: case sensitivity and dotfiles
6. Pattern: match names without searching the filesystem; escape literals
7. Errors: PatternError vs GlobError

Setup in your existing Cargo project:
    cargo add glob

Keep this file beside Cargo.toml and add:
    [[bin]]
    name = "03_glob"
    path = "03_glob.rs"

Run:
    cargo run --bin 03_glob

glob(pattern)? returns a Paths iterator; each item is Result<PathBuf, GlobError>.
glob() searches the filesystem. Pattern::matches() only checks a name.
Wildcards are not regex: no regex anchors, groups, or alternation.
main() creates temporary example files and removes them after the demos.
Reference: https://docs.rs/glob/0.3.4/glob/
*/

use glob::{glob, glob_with, MatchOptions, Pattern};
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

type DemoResult = Result<(), Box<dyn std::error::Error>>;

// ====================================================
// 1. Basic search: Python Path.glob()
// ====================================================

fn demo_basic(root: &Path) -> DemoResult {
    // With a fixed relative directory, the basic form is:
    // for entry in glob("data/*.txt")? {
    //     println!("{}", entry?.display());
    // }
    // Relative patterns start from the process's current working directory.

    for result in glob(&search_pattern(root, "*.txt")?)? {
        let p = result?;
        println!("txt: {}", p.strip_prefix(root)?.display());
    }
    // 01_notes.txt, 12_report.txt, a.txt, b.txt
    // Only immediate matches; nested/child.txt is not returned.
    // Matching directories are returned too, if their names match the pattern.
    Ok(())
}

// ====================================================
// 2. Wildcards and character sets
// ====================================================

fn demo_wildcards(root: &Path) -> DemoResult {
    for pattern in ["*.txt", "?.txt", "[ab].txt", "[1-2][0-9]_*.txt", "[!0-9]*.txt"] {
        println!("pattern: {pattern}");
        for result in glob(&search_pattern(root, pattern)?)? {
            println!("  {}", result?.strip_prefix(root)?.display());
        }
    }
    // *.txt          => all four top-level .txt files
    // ?.txt          => a.txt, b.txt (exactly one character before .txt)
    // [ab].txt       => a.txt, b.txt
    // [1-2][0-9]_*.txt => 12_report.txt
    // [!0-9]*.txt    => a.txt, b.txt (first character isn't a digit)
    // [!...] negates ONE character position, not an entire filename / extension.
    // "*[!.txt]" does NOT mean "exclude .txt files".
    // Brace alternatives like *.{txt,md} aren't supported; filter or run two searches.
    Ok(())
}

// ====================================================
// 3. Recursive search: **
// ====================================================

fn demo_recursive(root: &Path) -> DemoResult {
    for result in glob(&search_pattern(root, "**/*.txt")?)? {
        println!("recursive txt: {}", result?.strip_prefix(root)?.display());
    }
    // Top-level .txt files AND nested/child.txt AND nested/deep/item.txt.
    // ** matches zero or more directory levels: no separate rglob() function.
    // ** must occupy an entire path component: **a, b**, and *** are invalid.
    // Use "nested/**/*.txt" to restrict recursion to a particular subtree.
    // Use "**/*" for entries of any name, including directories.
    // For explicit depth limits, subtree pruning, and link policy, use walkdir.
    Ok(())
}

// ====================================================
// 4. Filter / exclude / collect / count
// ====================================================

fn demo_filter_collect(root: &Path) -> DemoResult {
    let paths: Vec<PathBuf> = glob(&search_pattern(root, "**/*")?)?
        .collect::<Result<_, _>>()?; // preserves traversal errors

    let mut selected: Vec<PathBuf> = paths.into_iter()
        .filter(|p| p.is_file()) // includes symlinks whose targets are files
        .filter(|p| p.extension() != Some(OsStr::new("txt")))
        .collect();
    selected.sort();
    for p in &selected {
        println!("non-txt file: {}", p.strip_prefix(root)?.display());
    }
    // .hidden.md, README.md, UPPER.TXT, literal[1].md
    println!("non-txt count: {}", selected.len()); // 4
    // Exact extension comparison is case-sensitive: UPPER.TXT is included.
    // is_file() maps metadata errors to false; use fs::metadata(p)? if you
    // need to distinguish a missing / inaccessible path from a non-file.
    // glob(...).count() counts Result items, INCLUDING errors and directories!
    // For large searches, process matches directly instead of collecting them all.
    Ok(())
}

// ====================================================
// 5. MatchOptions: case sensitivity and dotfiles
// ====================================================

fn demo_options(root: &Path) -> DemoResult {
    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: true,
        require_literal_leading_dot: true,
    };
    for result in glob_with(&search_pattern(root, "*.txt")?, options)? {
        println!("case-insensitive txt: {}", result?.strip_prefix(root)?.display());
    }
    // Includes UPPER.TXT as well as the four lowercase .txt files.
    // glob() / MatchOptions::new(): case_sensitive=true, other fields=false.
    // Surprise: MatchOptions::default() uses case_sensitive=false instead!
    // require_literal_leading_dot=true: wildcard matches must explicitly start
    // with '.' to include a dotfile (e.g. ".*.md" instead of "*.md").
    // Default glob() DOES match dotfiles. Windows hidden attributes are separate.
    // glob_with always enforces literal separators during filesystem searches;
    // require_literal_separator mainly matters for Pattern::matches_with().
    Ok(())
}

// ====================================================
// 6. Match existing names; escape literals
// ====================================================

fn demo_pattern() -> DemoResult {
    let pattern = Pattern::new("[0-9][0-9]_*.txt")?;
    println!("{}", pattern.matches("12_report.txt")); // true
    println!("{}", pattern.matches("a.txt")); // false
    println!("{}", pattern.matches_path(Path::new("12_report.txt"))); // true
    // These do not access the filesystem; the file need not exist.
    // Reuse the compiled Pattern for many names (e.g. walkdir entry filenames).

    let literal_name = "literal[1].md";
    let escaped = Pattern::escape(literal_name);
    println!("{escaped}"); // literal[[]1[]].md
    assert!(Pattern::new(&escaped)?.matches(literal_name));
    // Escape literal user-provided names / base directories before composing a glob.
    // Pattern matching uses UTF-8 strings; avoid lossy conversion for exact names.
    Ok(())
}

// ====================================================
// 7. PatternError vs GlobError
// ====================================================

fn demo_errors(root: &Path) -> DemoResult {
    match glob("[") {
        Ok(_) => println!("valid pattern"),
        Err(error) => eprintln!("invalid pattern: {error}"), // PatternError
    }
    for result in glob(&search_pattern(root, "**/*.txt")?)? {
        match result {
            Ok(p) => println!("match: {}", p.strip_prefix(root)?.display()),
            Err(error) => { // GlobError: an I/O error while traversing
                eprintln!("{}: {}", error.path().display(), error.error());
            }
        }
    }
    // glob(pattern)? handles pattern syntax; result? handles traversal I/O.
    // A valid search with no matches normally produces an empty iterator.
    // .filter_map(Result::ok) silently drops traversal errors.
    Ok(())
}

// ====================================================
// Demo setup: literal base directory + wildcard suffix
// ====================================================

fn search_pattern(root: &Path, suffix: &str) -> io::Result<String> {
    let base = root.to_str().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "glob requires a UTF-8 base path")
    })?;
    // Escape ONLY the literal base; keep the suffix wildcards active.
    // '/' works as a glob path separator on Unix and Windows.
    Ok(format!("{}/{suffix}", Pattern::escape(base)))
}

fn create_fixture(root: &Path) -> io::Result<()> {
    fs::create_dir_all(root.join("nested/deep"))?;
    for name in [
        "01_notes.txt", "12_report.txt", "a.txt", "b.txt", "UPPER.TXT",
        "README.md", ".hidden.md", "literal[1].md",
        "nested/child.txt", "nested/deep/item.txt",
    ] {
        fs::write(root.join(name), "demo\n")?;
    }
    Ok(())
}

// ============ //
//    main()    //
// ============ //

fn main() -> DemoResult {
    println!();
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = env::temp_dir().join(format!("rust_glob_{}_{stamp}", std::process::id()));
    fs::create_dir(&root)?;

    println!("Demo directory: {}\n", root.display());
    let result: DemoResult = (|| {
        create_fixture(&root)?;

        demo_basic(&root)?;

        println!("\n====================================================\n");

        demo_wildcards(&root)?;

        println!("\n====================================================\n");

        demo_recursive(&root)?;

        println!("\n====================================================\n");

        demo_filter_collect(&root)?;

        println!("\n====================================================\n");

        demo_options(&root)?;

        println!("\n====================================================\n");

        demo_pattern()?;

        println!("\n====================================================\n");

        demo_errors(&root)?;

        Ok(())
    })();

    let cleanup = fs::remove_dir_all(&root);
    result?;
    cleanup?;

    println!("\nDemo directory removed.");
    Ok(())
}

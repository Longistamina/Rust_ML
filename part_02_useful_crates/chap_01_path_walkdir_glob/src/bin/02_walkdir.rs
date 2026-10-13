/*
walkdir: recursively iterate through a directory tree.

1. Basic traversal and DirEntry methods
2. Depth limits: min_depth / max_depth
3. Filter files by extension; collect / count / total size
4. Skip whole subtrees with filter_entry
5. Sorting and traversal order
6. Symlinks and filesystem boundaries
7. Handle errors and stop traversal early

Python Path.walk() yields (dirpath, dirnames, filenames).
WalkDir yields ONE Result<DirEntry, walkdir::Error> per entry instead.
The starting path is included at depth 0; its children have depth 1.
Traversal is lazy: entries are read as you consume the iterator.

Setup in your existing Cargo project:
    cargo add walkdir

Keep this file beside Cargo.toml and add:
    [[bin]]
    name = "02_walkdir"
    path = "02_walkdir.rs"

Run:
    cargo run --bin 02_walkdir
*/

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};
use walkdir::{DirEntry, WalkDir};

type DemoResult = Result<(), Box<dyn std::error::Error>>;

// ====================================================
// 1. Basic traversal and DirEntry methods
// ====================================================

fn demo_basic(root: &Path) -> DemoResult {
    for entry in WalkDir::new(root).sort_by_file_name() {
        let entry = entry?;
        println!("depth {}: {}", entry.depth(), entry.path().strip_prefix(root)?.display());
        // entry.path() -> &Path; entry.into_path() -> owned PathBuf
        // entry.file_name() -> &OsStr (not necessarily UTF-8)
        // entry.file_type() -> FileType, without an extra metadata query
        // entry.metadata() -> Result<Metadata, walkdir::Error>
        // entry.depth() -> usize
        if entry.file_type().is_dir() {
            println!("  directory: {:?}", entry.file_name());
        }
    }
    // Without sorting, sibling order is unspecified.
    // WalkDir::new(file) also works: it yields only that file at depth 0.
    Ok(())
}

// ====================================================
// 2. Depth limits
// ====================================================

fn demo_depth(root: &Path) -> DemoResult {
    for entry in WalkDir::new(root).min_depth(1).max_depth(1).sort_by_file_name() {
        println!("child: {:?}", entry?.file_name());
    }
    // ".hidden", "README.md", "data", "target"
    // min_depth(1): omit root itself (but still descend into it).
    // max_depth(1): immediate children only, like Python iterdir().
    // max_depth(2): children AND grandchildren; deeper directories aren't read.
    Ok(())
}

// ====================================================
// 3. Filter files; collect / count / total size
// ====================================================

fn demo_filter_collect(root: &Path) -> DemoResult {
    // Convert Result<DirEntry> items into Result<Vec<DirEntry>> first.
    // collect() propagates the first error instead of silently dropping it.
    let entries: Vec<DirEntry> = WalkDir::new(root)
        .min_depth(1)
        .into_iter()
        .collect::<Result<_, _>>()?;

    let mut txt_paths: Vec<PathBuf> = entries.iter()
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension() == Some(OsStr::new("txt")))
        .map(|e| e.path().to_path_buf())
        .collect();
    txt_paths.sort();
    for p in &txt_paths {
        println!("txt: {}", p.strip_prefix(root)?.display());
    }
    println!("txt count: {}", txt_paths.len()); // 4 (including hidden / target)
    // Extension matching above is case-sensitive and excludes directories.

    let mut bytes = 0;
    for entry in &entries {
        if entry.file_type().is_file() {
            bytes += entry.metadata()?.len();
        }
    }
    println!("total file bytes: {bytes}"); // 24
    // For large trees, filter / process in the for loop to avoid collecting all entries.
    Ok(())
}

// ====================================================
// 4. Skip whole subtrees: filter_entry vs filter
// ====================================================

fn keep_entry(entry: &DirEntry) -> bool {
    // Always allow root; its name could itself start with a dot.
    entry.depth() == 0 || (
        entry.file_name() != OsStr::new("target")
        && !entry.file_name().to_str().is_some_and(|s| s.starts_with('.'))
    )
}

fn demo_prune(root: &Path) -> DemoResult {
    for entry in WalkDir::new(root).sort_by_file_name().into_iter().filter_entry(keep_entry) {
        let entry = entry?;
        println!("kept: {}", entry.path().strip_prefix(root)?.display());
    }
    // Skips .hidden AND everything inside it; likewise target.
    // filter_entry receives &DirEntry, not &Result<DirEntry>.
    // It hides rejected entries and prevents descent into rejected directories.
    // Ordinary Iterator::filter only hides entries; traversal still enters directories.
    // This is the equivalent of pruning Python walk()'s dirnames list.
    // Do not reject every directory to select files: that would prevent recursion!
    Ok(())
}

// ====================================================
// 5. Sorting and traversal order
// ====================================================

fn demo_order(root: &Path) -> DemoResult {
    for entry in WalkDir::new(root).sort_by_file_name().contents_first(true) {
        println!("bottom-up: {}", entry?.path().strip_prefix(root)?.display());
    }
    // Default: depth-first, directory before its contents (like topdown=True).
    // contents_first(true): contents before directory (like topdown=False).
    // sort_by_file_name sorts siblings; it isn't a global sort of every full path.
    // Subtree pruning with filter_entry requires the default contents_first(false):
    // bottom-up traversal has already visited children before testing their directory.
    Ok(())
}

// ====================================================
// 6. Symlinks and filesystem boundaries
// ====================================================

fn demo_links(root: &Path) -> DemoResult {
    for entry in WalkDir::new(root).follow_links(false).follow_root_links(false) {
        let entry = entry?;
        if entry.path_is_symlink() {
            println!("symlink: {}", entry.path().display());
        }
    }
    // No links are created in this fixture, so the loop prints nothing.
    // follow_links(false) is default: nested directory symlinks aren't traversed.
    // Root symlinks are followed by default; follow_root_links(false) disables that.
    // follow_links(true) follows nested links; broken links / cycles yield errors.
    // When following links, file_type() describes the target; path() keeps link path.
    // path_is_symlink() identifies a link even when its target is followed.

    // Additional builder options (set BEFORE into_iter()):
    // WalkDir::new(root).same_file_system(true) // don't descend across filesystems
    // WalkDir::new(root).max_open(10)          // cap open directory handles
    Ok(())
}

// ====================================================
// 7. Errors and early stopping
// ====================================================

fn demo_errors_early_stop(root: &Path) -> DemoResult {
    for result in WalkDir::new(root.join("missing")) {
        match result {
            Ok(entry) => println!("{}", entry.path().display()),
            Err(error) => {
                eprintln!("walk error: {error}");
                eprintln!("path: {:?}; depth: {}", error.path(), error.depth());
                // error.io_error() -> Option<&io::Error>
                // error.loop_ancestor() -> Option<&Path> for a symlink cycle
            }
        }
    }
    // Logging and continuing lets traversal visit other accessible entries.
    // Using entry? (as above) stops the demo at the first traversal error.
    // .filter_map(Result::ok) is shorter, but silently discards EVERY walk error.

    for entry in WalkDir::new(root).sort_by_file_name() {
        let entry = entry?;
        if entry.file_type().is_file() && entry.path().extension() == Some(OsStr::new("txt")) {
            println!("first txt: {}", entry.path().strip_prefix(root)?.display());
            break; // stop reading the tree as soon as a match is found
        }
    }
    Ok(())
}

// ============ //
//    main()    //
// ============ //

fn create_fixture(root: &Path) -> io::Result<()> {
    for dir in ["data/nested", ".hidden", "target"] {
        fs::create_dir_all(root.join(dir))?;
    }
    for (name, content) in [
        ("README.md", "guide\n"),
        ("data/a.txt", "alpha\n"),
        ("data/nested/b.txt", "beta\n"),
        (".hidden/secret.txt", "shh\n"),
        ("target/build.txt", "ok\n"),
    ] {
        fs::write(root.join(name), content)?;
    }
    Ok(())
}

fn main() -> DemoResult {
    println!();
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = env::temp_dir().join(format!("rust_walkdir_{}_{stamp}", std::process::id()));
    fs::create_dir(&root)?;
    println!("Demo directory: {}\n", root.display());
    let result: DemoResult = (|| {
        create_fixture(&root)?;

        demo_basic(&root)?;

        println!("\n====================================================\n");

        demo_depth(&root)?;

        println!("\n====================================================\n");

        demo_filter_collect(&root)?;

        println!("\n====================================================\n");

        demo_prune(&root)?;

        println!("\n====================================================\n");

        demo_order(&root)?;

        demo_links(&root)?;

        println!("\n====================================================\n");

        demo_errors_early_stop(&root)?;

        Ok(())
    })();

    let cleanup = fs::remove_dir_all(&root);
    result?;
    cleanup?;
    println!("\nDemo directory removed.");
    Ok(())
}

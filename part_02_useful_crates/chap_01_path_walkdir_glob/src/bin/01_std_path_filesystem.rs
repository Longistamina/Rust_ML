/*
`std::path` + `std::fs`: concise equivalents of Python pathlib.

1. Path / PathBuf: create, borrow, join, push, pop
2. Current directory, home directory, temporary directory
3. Components: file_name, file_stem, extension, parent, ancestors, components
4. Change names / extensions; starts_with, ends_with, strip_prefix
5. Display paths and handle non-UTF-8 names; AsRef<Path>
6. Filesystem checks; absolute vs canonicalize
7. Create directories / files; read, write, copy, rename, remove
8. Metadata and permissions
9. Symlinks (Unix example; Windows equivalents in comments)
10. Directory entries and simple filename filtering (no recursion)

&Path : borrowed path, like &str.
PathBuf : owned path, like String.

Both use native OS path rules; neither requires the path to exist.
Path operations manipulate names; std::fs performs filesystem operations.
Unlike Python PureWindowsPath, std::path does not select foreign OS rules.
*/

use std::env;
use std::ffi::OsStr;
use std::fs::{self, FileTimes, OpenOptions};
use std::io::{self, Write};
use std::path::{self, Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

// ====================================================
// 1. Path / PathBuf: create, borrow, join, push, pop
// ====================================================

fn demo_create_join() {
    let borrowed: &Path = Path::new("parent/child/example.txt");
    let owned: PathBuf = borrowed.to_path_buf();
    let borrowed_again: &Path = owned.as_path();
    println!("{}", borrowed_again.display()); // parent/child/example.txt

    // join() creates a new PathBuf; the original path stays unchanged.
    let joined = Path::new("parent").join("child").join("example.txt");
    println!("{}", joined.display());

    // push() / pop() modify an owned PathBuf.
    let mut buffer = PathBuf::from("parent");
    buffer.push("child");
    buffer.push("example.txt");
    assert!(buffer.pop()); // remove example.txt
    println!("{}", buffer.display()); // parent/child

    // An absolute argument to join() / push() replaces the previous path.
    // Windows also has special rules for rooted paths and drive prefixes.
    // Path::new("") is an EMPTY path, not "." as in Python pathlib.
    assert!(Path::new("").as_os_str().is_empty());
}

// ====================================================
// 2. Current / home / temporary directory
// ====================================================

fn demo_directories() -> io::Result<()> {
    println!("cwd: {}", env::current_dir()?.display()); // Python Path.cwd()

    if let Some(home) = env::home_dir() {
        println!("home: {}", home.display()); // Python Path.home()
    }

    println!("temp: {}", env::temp_dir().display());

    // Paths do not expand ~ or $HOME automatically.
    // env::set_current_dir(path)? changes the process-wide working directory.
    Ok(())
}

// ====================================================
// 3. Extract components
// ====================================================

fn demo_components() {
    let p = Path::new("parent/child/archive.tar.gz");

    println!("name: {:?}", p.file_name()); // Some("archive.tar.gz")
    println!("stem: {:?}", p.file_stem()); // Some("archive.tar")
    println!("extension: {:?}", p.extension()); // Some("gz"), WITHOUT the dot
    println!("parent: {:?}", p.parent()); // Some("parent/child")

    // These return Option: roots / empty paths may have no name or parent.
    // Names are OsStr; parent() returns &Path.
    // There is no direct Python .suffixes equivalent here.
    // A dotted directory name also has an extension: these methods are lexical.
    assert_eq!(Path::new(".gitignore").extension(), None);
    assert_eq!(Path::new("file.txt").parent(), Some(Path::new("")));

    println!();
    for ancestor in p.ancestors() {
        println!("ancestor: {:?}", ancestor);
    }
    // "parent/child/archive.tar.gz", "parent/child", "parent", ""
    // ancestors() includes SELF; use .skip(1) for Python .parents-like output.

    println!();
    for component in p.components() {
        println!("component: {component:?}");
    }
    // Normal("parent"), Normal("child"), Normal("archive.tar.gz")
    // Other variants: Prefix (Windows), RootDir, CurDir, ParentDir.
    // components() removes redundant separators and most "." components,
    // but preserves ".." because resolving it can depend on symlinks.

    println!();
    println!("absolute: {}", p.is_absolute()); // false
    println!("relative: {}", p.is_relative()); // true
    println!("has root: {}", p.has_root()); // false
    // On Windows, has_root() alone does not imply is_absolute().
}

// ====================================================
// 4. Change names / extensions and compare paths
// ====================================================

fn demo_modify_compare() -> Result<(), std::path::StripPrefixError> {
    let p = Path::new("parent/child/example.txt");
    println!("{:?}", p.with_file_name("notes.md")); // parent/child/notes.md
    println!("{:?}", p.with_extension("csv")); // parent/child/example.csv
    println!("{:?}", p.with_extension("")); // parent/child/example
    // with_* returns a new PathBuf; it does not rename any real file.

    println!();
    let mut owned = p.to_path_buf();
    owned.set_file_name("archive.tar.gz");
    owned.set_extension("xz");
    println!("{:?}", owned); // parent/child/archive.tar.xz

    println!();
    println!("{}", p.starts_with("parent")); // true; Python is_relative_to()
    println!("{}", p.starts_with("par")); // false: whole components, not text
    println!("{}", p.ends_with("child/example.txt")); // true
    println!("{}", p.strip_prefix("parent")?.display()); // child/example.txt
    // strip_prefix() returns Result<&Path, StripPrefixError>.
    // These are lexical operations; they do not resolve symlinks or "..".
    Ok(())
}

// ====================================================
// 5. Display / strings / accepting paths in functions
// ====================================================

fn demo_strings() {
    let p = Path::new("data/example.txt");
    println!("{}", p.display()); // human-readable display
    println!("{p:?}"); // debugging, quotes / escaped characters
    println!("{:?}", p.to_str()); // Some("data/example.txt")
    println!("{}", p.to_string_lossy()); // replaces invalid Unicode if needed

    // OS filenames need not be UTF-8. Keep Path / OsStr for filesystem work;
    // use to_str() when an API requires UTF-8, and handle None.
    // Compare extensions without allocating or assuming UTF-8:
    println!();
    assert_eq!(p.extension(), Some(OsStr::new("txt")));
    show_path("data/example.txt");
    show_path(p);
    show_path(PathBuf::from("data/example.txt"));
}

fn show_path(p: impl AsRef<Path>) {
    // Accepts &str, String, &Path, PathBuf, and references to them.
    println!("path: {}", p.as_ref().display());
}

// ====================================================
// 6. Filesystem checks; absolute vs canonicalize
// ====================================================

fn demo_checks_resolution(root: &Path) -> io::Result<()> {
    println!("exists: {}", root.exists()); // true
    println!("is_dir: {}", root.is_dir()); // true
    println!("is_file: {}", root.is_file()); // false
    println!("is_symlink: {}", root.is_symlink()); // false
    println!("try_exists: {}", root.try_exists()?); // true; preserves I/O errors
    // exists / is_file / is_dir hide I/O errors behind false.
    // They follow symlinks; is_symlink checks the link itself.
    // A broken symlink can have exists() == false and is_symlink() == true.
    // A check does not guarantee that a later filesystem operation will succeed.

    println!("absolute: {}", path::absolute("not_created/file.txt")?.display());
    // No existence requirement; does not resolve symlinks.
    // On Unix, ".." is preserved. An empty path produces an error.

    println!("canonical: {}", root.canonicalize()?.display());
    // Like Python resolve(strict=True): requires an existing path,
    // resolves symlinks / "." / "..", and returns an absolute path.
    // Windows canonical paths may use the extended-length \\?\ prefix.
    Ok(())
}

// ====================================================
// 7. Directory and file operations
// ====================================================

fn demo_file_operations(root: &Path) -> io::Result<()> {
    let empty_dir = root.join("empty");
    fs::create_dir(&empty_dir)?; // mkdir(); fails if it already exists
    fs::remove_dir(&empty_dir)?; // rmdir(); requires an empty directory
    fs::create_dir_all(root.join("data/raw"))?; // mkdir(parents=True, exist_ok=True)

    let p = root.join("data/example.txt");
    // Create if missing, without truncating existing contents.
    let file = OpenOptions::new().write(true).create(true).truncate(false).open(&p)?;
    file.set_times(FileTimes::new().set_modified(SystemTime::now()))?;
    drop(file);
    // A touch-like example: create if missing, update modification time.
    // File::create(path)? creates OR TRUNCATES; it is not a safe touch substitute.
    // File::create_new(path)? creates only if missing (fails if already present).

    fs::write(&p, "hello\n")?; // write_text(); overwrites existing contents
    let mut file = OpenOptions::new().append(true).open(&p)?;
    writeln!(file, "world")?;
    drop(file);
    println!("{}", fs::read_to_string(&p)?); // hello\nworld\n
    println!("bytes: {}", fs::read(&p)?.len()); // 12
    // read_to_string requires UTF-8; read returns Vec<u8>.

    let copied = root.join("data/copy.txt");
    println!("copied bytes: {}", fs::copy(&p, &copied)?); // 12
    // copy() copies one file and its permission bits, not a directory tree.
    // It overwrites an existing destination; it is not Python shutil.copy2().

    let moved = root.join("data/renamed.txt");
    fs::rename(&copied, &moved)?; // move / rename file or directory
    // rename may replace a destination; exact rules depend on the OS.
    // Moving across filesystem boundaries may fail.

    fs::remove_file(&moved)?; // unlink(); errors if missing
    // Python unlink(missing_ok=True): ignore ONLY ErrorKind::NotFound.
    match fs::remove_file(&moved) {
        Ok(()) => {}
        Err(e) if e.kind() == io::ErrorKind::NotFound => {}
        Err(e) => return Err(e),
    }
    // fs::remove_dir_all(path)? removes a directory tree (used in main cleanup).
    Ok(())
}

// ====================================================
// 8. Metadata and permissions
// ====================================================

fn demo_metadata_permissions(root: &Path) -> io::Result<()> {
    let p = root.join("data/example.txt");
    let metadata = p.metadata()?; // Python stat(); follows symlinks
    println!("size: {} bytes", metadata.len()); // 12
    println!("modified: {:?}", metadata.modified()?); // SystemTime
    println!("accessed: {:?}", metadata.accessed()); // Result<SystemTime>
    println!("created: {:?}", metadata.created()); // can be unsupported
    println!("readonly: {}", metadata.permissions().readonly());
    if let Ok(elapsed) = metadata.modified()?.duration_since(UNIX_EPOCH) {
        println!("mtime: {} seconds since epoch", elapsed.as_secs());
    }
    // created() is birth time; Unix ctime means metadata CHANGE time.
    // std does not format timestamps as calendar dates.

    #[cfg(unix)]
    {
        use std::os::unix::fs::{MetadataExt, PermissionsExt};
        println!("mode: {:o}", metadata.mode() & 0o777);
        println!("ctime: {} seconds since epoch", metadata.ctime());
        fs::set_permissions(&p, fs::Permissions::from_mode(0o644))?;
        // chmod(0o644): owner read/write, group and others read.
        // readonly() only describes write permission; it does not cover all modes.
    }
    // Portable readonly change:
    // let mut permissions = p.metadata()?.permissions();
    // permissions.set_readonly(true);
    // fs::set_permissions(&p, permissions)?;
    Ok(())
}

// ====================================================
// 9. Symlinks
// ====================================================

fn demo_symlinks(root: &Path) -> io::Result<()> {
    #[cfg(unix)]
    {
        use std::os::unix::fs::symlink;
        let link = root.join("example_link.txt");
        // A relative target is interpreted relative to the link's parent.
        symlink("data/example.txt", &link)?;
        println!("link: {}", link.is_symlink()); // true
        println!("target: {}", fs::read_link(&link)?.display()); // data/example.txt
        println!("resolved: {}", link.canonicalize()?.display());
        println!("link metadata: {:?}", link.symlink_metadata()?.file_type());
        // metadata() follows the link; symlink_metadata() inspects the link.
        fs::remove_file(&link)?; // removes link, preserves target
    }
    #[cfg(not(unix))]
    let _ = root;
    // Windows: std::os::windows::fs::{symlink_file, symlink_dir}.
    // Creating them may require Developer Mode or elevated privileges.
    // Remove Windows directory symlinks with remove_dir(), file links with remove_file().
    // A directory symlink refers to its target; it does not copy its contents.
    Ok(())
}

// ====================================================
// 10. Directory entries and simple filename filtering
// ====================================================

fn demo_read_dir_filter(root: &Path) -> io::Result<()> {
    let data = root.join("data");
    fs::write(data.join("notes.md"), "notes\n")?;
    let mut txt_files = Vec::new();

    for entry in fs::read_dir(&data)? { // Python iterdir(); one directory level
        let entry = entry?; // read_dir AND each iterator item can fail
        let p = entry.path(); // PathBuf
        let kind = entry.file_type()?; // does NOT follow symlinks
        println!("{:?}: {:?}", entry.file_name(), kind);
        if kind.is_file() && p.extension() == Some(OsStr::new("txt")) {
            txt_files.push(p);
        }
        // kind.is_dir() / kind.is_symlink() classify other entries.
        // Use p.is_file() instead if the filter should follow file symlinks.
    }
    // Filesystem iteration order is unspecified. Sort collected paths if needed.
    txt_files.sort();
    for p in &txt_files {
        println!("txt: {}", p.display());
    }
    println!("txt count: {}", txt_files.len()); // 1

    // Name filtering without regex / glob:
    // entry.file_name().to_str().is_some_and(|name| name.starts_with("notes"))
    // std::path / std::fs have no glob() or rglob().
    // Use the glob crate for wildcard patterns; regex for regular expressions.
    // Recursive directory walking is saved for the later walkdir guide.
    Ok(())
}

// ============ //
//    main()    //
// ============ //

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!();

    demo_create_join();

    println!("\n====================================================\n");

    demo_directories()?;

    println!("\n====================================================\n");

    demo_components();

    println!("\n====================================================\n");

    demo_modify_compare()?;

    println!("\n====================================================\n");

    demo_strings();

    println!("\n====================================================\n");

    // Reserve a new directory before any filesystem demos run.
    let stamp = SystemTime::now().duration_since(UNIX_EPOCH)?.as_nanos();
    let root = env::temp_dir().join(format!("rust_path_guide_{}_{stamp}", std::process::id()));
    fs::create_dir(&root)?;
    println!("Demo directory: {}\n", root.display());

    let result: io::Result<()> = (|| {
        demo_checks_resolution(&root)?;
        demo_file_operations(&root)?;
        demo_metadata_permissions(&root)?;
        demo_symlinks(&root)?;
        demo_read_dir_filter(&root)?;
        Ok(())
    })();

    let cleanup = fs::remove_dir_all(&root); // attempt cleanup even if a demo failed
    result?;
    cleanup?;

    println!("\nDemo directory removed.");
    Ok(())
}

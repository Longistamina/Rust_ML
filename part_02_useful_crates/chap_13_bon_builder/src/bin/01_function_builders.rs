//! Bon: function builders, named arguments, defaults, and optional values.
//!
//! Setup: cargo add bon@3.10
//! Save as src/bin/01_function_builders.rs, then:
//! cargo run --bin 01_function_builders
//!
//! Rust has no native Python-style keyword/default arguments. Bon generates
//! a typed builder: function().parameter(value).other(value).call().
//! Creating/configuring the builder does NOT execute the function body;
//! .call() executes it and returns the function's declared return type.
//!
//! References:
//! https://bon-rs.com/guide/overview
//! https://bon-rs.com/guide/basics/optional-members
//!
//! Struct/method builders and advanced conversions belong to the next files.

use bon::builder;
use std::collections::BTreeMap;

fn main() {
    println!();

    demo_named_arguments();
    println!("\n============================\n");

    demo_customer_defaults();
    println!("\n============================\n");

    demo_default_trait();
    println!("\n============================\n");

    demo_optional_values();
    println!("\n============================\n");

    demo_forwarding_options();
    println!("\n============================\n");

    demo_optional_vs_default();
    println!("\n============================\n");

    demo_deferred_call();
    println!("\n============================\n");

    demo_ownership();
    println!("\n============================\n");

    demo_collections();
    println!("\n============================\n");

    demo_salary();
    println!("\n============================\n");

    demo_compile_time_checks();
}

// =====================================================
// 01. Named arguments and setter order
// =====================================================

fn demo_named_arguments() {
    println!("01. Named arguments and setter order");

    // An ordinary Rust function uses positional arguments.
    fn add_plain(x: i32, y: i32) -> i32 {
        x + y
    }

    #[builder]
    fn add_numbers(x: i32, y: i32) -> i32 {
        x + y
    }

    println!("ordinary function: {}", add_plain(5, 10));
    println!("named arguments:   {}", add_numbers().x(5).y(10).call());
    println!("reversed setters:  {}", add_numbers().y(10).x(5).call());
    // All three return 15. Setter order does not change parameter matching.
    // The macro changes this function's calling API: add_numbers(5, 10)
    // is not its generated builder interface.
    // This is .x(5), NOT Python syntax x=5 inside the function call.
}

// ===========================================================
// 02. Explicit defaults: customer information and greetings
// ===========================================================

fn demo_customer_defaults() {
    println!("02. Explicit defaults: customer information and greetings");

    #[builder]
    fn customer_info(
        name: &str,
        #[builder(default=30)] age: u32,
        #[builder(default="Unknown")] job: &str,
    ) -> String {
        format!("Name: {name}, age: {age}, job: {job}")
    }

    #[builder]
    fn greet_user(
        name: &str,
        #[builder(default="Hello")] greeting: &str,
    ) -> String {
        format!("{greeting}, {name}!")
    }

    println!("{}", customer_info().name("Alice").call());
    println!("{}", customer_info().name("Bob").age(25).job("Engineer").call());
    println!("{}", customer_info().job("Researcher").name("Long").call());

    println!();

    println!("{}", greet_user().name("Alice").call());
    println!("{}", greet_user().greeting("Bonjour").name("Bob").call());
    // Defaults apply when their setters are omitted; supplied values override them.
    // Bon's named members do not need Python's required-before-default ordering.
}

// ==============================================================
// 03. Default::default() versus a custom default expression
// ==============================================================

fn demo_default_trait() {
    println!("03. Default::default() versus a custom default expression");

    #[builder]
    fn describe_options(
        #[builder(default)] verbose: bool,
        #[builder(default)] retries: u32,
        #[builder(default)] labels: Vec<String>,
        #[builder(default=String::from("report.txt"))] output: String,
    ) -> String {
        format!("verbose={verbose}, retries={retries}, labels={labels:?}, output={output}")
    }

    println!("{}", describe_options().call());
    // bool -> false, u32 -> 0, Vec -> empty; output uses its custom expression.

    println!("{}", describe_options().verbose(true).retries(3).call());
    // Omission is allowed, but the function body receives bool/u32/Vec/String,
    // not Option<T>. No unwrap() is necessary for these parameters.
    // Default expressions are evaluated when the call needs the default;
    // owned defaults are not a single shared mutable object as in Python.
}

// =====================================================
// 04. Option<T>: omitted, present, and explicitly None
// =====================================================

fn demo_optional_values() {
    println!("04. Option<T>: omitted, present, and explicitly None");

    #[builder]
    fn greet_with_title(name: &str, title: Option<&str>) -> String {
        match title {
            Some(title) => format!("Hello, {title} {name}!"),
            None => format!("Hello, {name}!"),
        }
    }

    println!("{}", greet_with_title().name("Alice").call());
    println!("{}", greet_with_title().name("Alice").title("Dr.").call());
    println!("{}", greet_with_title().name("Alice").maybe_title(None).call());
    // An Option<T> member is optional by default and becomes None if omitted.
    // .title("Dr.") takes T and wraps it in Some(...).
    // .maybe_title(Some("Dr.")) takes Option<T> directly.
    // .title(Some("Dr.")) is therefore the wrong argument type here.
}

// =====================================================
// 05. Forward Option values with maybe_* setters
// =====================================================

fn demo_forwarding_options() {
    println!("05. Forward Option values with maybe_* setters");

    #[builder]
    fn request_summary(
        url: &str,
        #[builder(default=3)] retries: u32,
        token: Option<&str>,
    ) -> String {
        // Print only whether a token exists, not its contents.
        format!("url={url}, retries={retries}, authenticated={}", token.is_some())
    }

    for (retries, token) in [(Some(5), Some("demo-token")), (None, None)] {
        println!("{}", request_summary()
            .url("https://example.com")
            .maybe_retries(retries)
            .maybe_token(token)
            .call());
    }
    // maybe_* also exists for #[builder(default)] members.
    // maybe_retries(None) selects the default 3; maybe_token(None) selects None.
    // This forwards optional configuration without unwrap_or(...) at every call.
    // Passing None still counts as calling the setter; it does not reset it
    // so that it can be called again.
}

// ==================================================================
// 06. Optional to the caller versus optional inside the function
// ==================================================================

fn demo_optional_vs_default() {
    println!("06. Optional to the caller versus optional inside the function");

    #[builder]
    fn compare(
        #[builder(default=30)] age: u32,
        nickname: Option<&str>,
    ) -> String {
        format!("age={age}, nickname={nickname:?}")
    }

    println!("{}", compare().call()); // age=30, nickname=None
    println!("{}", compare().age(0).nickname("").call());
    // Explicit 0 and an empty string are values; they do not select defaults.
    // Both setters can be omitted, but their parameter types remain different:
    // age always has a u32 value; nickname can represent absence.

    #[builder]
    fn choose_alias(#[builder(required)] alias: Option<&str>) -> String {
        format!("explicit alias choice: {alias:?}")
    }

    println!();

    println!("{}", choose_alias().alias(Some("Long")).call());
    println!("{}", choose_alias().alias(None).call());
    // #[builder(required)] opts out of Option's usual optional-member behavior.
    // The required setter takes Option<&str> directly (no implicit Some).
    // The caller must make an explicit choice, including explicitly choosing None.
}

// =====================================================
// 07. Configure now, execute later
// =====================================================
fn demo_deferred_call() {
    println!("07. Configure now, execute later");

    #[builder]
    fn announce(name: &str, #[builder(default="Hello")] greeting: &str) -> String {
        println!("function body is running now");
        format!("{greeting}, {name}!")
    }

    let pending = announce().name("Alice");
    println!("builder prepared; announce's body has not run yet");

    let result = pending.greeting("Welcome").call();
    println!("returned: {result}");
    // Builders normally consume self when setting members or calling.
    // A saved builder represents a partially configured call, not its result.
    // Its type tracks which members are set; use shadowing to advance it.
    // Do not assume a builder can be reused after .call().
}

// ===========================================================
// 08. Owned and borrowed arguments still follow Rust's rules
// ===========================================================

fn demo_ownership() {
    println!("08. Owned and borrowed arguments still follow Rust's rules");

    #[builder]
    fn owned_message(text: String, #[builder(default="!")] suffix: &str) -> String {
        format!("{text}{suffix}")
    }

    #[builder]
    fn borrowed_length(text: &str) -> usize {
        text.len()
    }

    let text = String::from("Hello from Rust");
    println!("borrowed byte length: {}", borrowed_length().text(&text).call());
    println!("still usable after borrowing: {text}");

    let pending = owned_message().text(text);
    // text has moved into the builder, even before the function executes.
    println!("{}", pending.suffix(" :33").call());
    // A String parameter needs a String unless conversion is configured.
    // A borrowed argument must remain valid until the builder has used it.
    // Bon does not bypass ownership, lifetimes, or argument type checking.
}

// ============================================================
// 09. Python *args and **kwargs: pass collections explicitly
// ============================================================

fn demo_collections() {
    println!("09. Python *args and **kwargs: pass collections explicitly");

    #[builder]
    fn sum_numbers(numbers: &[i32]) -> i32 {
        numbers.iter().sum()
    }

    #[builder]
    fn describe_metadata(metadata: BTreeMap<String, String>) -> String {
        metadata.into_iter()
            .map(|(key, value)| format!("{key}={value}"))
            .collect::<Vec<_>>()
            .join(", ")
    }

    println!("sum: {}", sum_numbers().numbers(&[1, 2, 3, 4, 5]).call());
    println!("empty sum: {}", sum_numbers().numbers(&[]).call());

    let metadata = BTreeMap::from([
        (String::from("name"), String::from("Alice")),
        (String::from("city"), String::from("Seoul")),
    ]);
    println!("{}", describe_metadata().metadata(metadata).call());
    // Bon generates only the setters declared by the function signature.
    // It does not accept arbitrary .city(...) or .anything(...) setters.
    // Map keys are runtime data, not compile-time-checked parameter names.
    // This map uses String values; mixed types require an enum or another
    // explicitly chosen representation. Bon does not make Rust dynamically typed.
}

// ===================================================================
// 10. Practical example: required salary inputs and a default bonus
// ===================================================================

fn demo_salary() {
    println!("10. Practical example: required salary inputs and a default bonus");

    #[builder]
    fn calculate_salary(
        name: &str,
        salary_daily: u64,
        working_days: u32,
        #[builder(default)] bonus: u64,
    ) -> (String, u64) {
        let base = salary_daily * u64::from(working_days);
        (name.to_owned(), base + bonus)
    }

    // Amounts are integer cents for this small example.
    let (name, total) = calculate_salary()
        .name("Alice")
        .salary_daily(10_000)
        .working_days(20)
        .bonus(50_000)
        .call();
    println!("{name}: {total} cents"); // 250000

    let (name, total) = calculate_salary()
        .working_days(20)
        .name("Bob")
        .salary_daily(10_000)
        .call();
    println!("{name}: {total} cents"); // 200000, default bonus=0

    // Unlike Python **kwargs, required fields need no manual presence checks:
    // missing name/salary_daily/working_days prevents .call() from compiling.
    // Presence and types are checked; business rules are still your job.
    // Production arithmetic should handle overflow and validate relevant limits.
    // The return type is fixed: (String, u64), not different tuple shapes
    // depending on a flag. Bon does not change Rust's return type rules.
}

// =====================================================
// 11. Compile-time mistakes versus runtime validation
// =====================================================

fn demo_compile_time_checks() {
    println!("11. Compile-time mistakes versus runtime validation");

    #[builder]
    fn repeat_message(text: &str, #[builder(default=1)] times: usize) -> String {
        text.repeat(times)
    }

    println!("valid: {}", repeat_message().text("Hi! ").times(2).call());

    println!();

    println!("The examples below intentionally stay commented out:");
    // repeat_message().call();
    // ERROR: the required text member is missing.

    // repeat_message().text("Hi").text("Again").call();
    // ERROR: the same member is set twice, not silently overwritten.

    // repeat_message().text("Hi").times("two").call();
    // ERROR: times expects usize, not &str.

    // repeat_message().text("Hi").unknown(true).call();
    // ERROR: no generated setter named unknown.

    println!("Missing members, repeated setters, wrong types, unknown names -> compile errors.");
    // These cannot be demonstrated with match/try at runtime: the program
    // must compile first. Uncomment ONE example at a time to inspect the error.
    // An empty text or times=0 is valid here; a domain rule prohibiting those
    // would need explicit validation in the body, usually returning Result.
}

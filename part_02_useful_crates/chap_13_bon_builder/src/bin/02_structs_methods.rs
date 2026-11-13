//! Bon: struct builders, constructors, and method builders.
//!
//! Setup: cargo add bon@3.10
//! Save as src/bin/02_structs_methods.rs, then:
//! cargo run --bin 02_structs_methods
//!
//! #[derive(Builder)] on a struct -> Type::builder().field(...).build()
//! #[bon] on impl + #[builder] on new -> Type::builder().argument(...).build()
//! #[bon] on impl + #[builder] on another method -> method().argument(...).call()
//!
//! A builder stores inputs. The finish method (.build() or .call()) constructs
//! the value or executes the method body. Rust ownership rules still apply.
//!
//! References:
//! https://bon-rs.com/guide/overview
//! https://bon-rs.com/guide/basics/optional-members
//!
//! Conversions, custom setters, and fallible builders belong to the next file.

use bon::{bon, Builder};

fn main() {
    println!();

    demo_struct_builder();
    println!("\n============================\n");

    demo_struct_defaults();
    println!("\n============================\n");

    demo_borrowed_fields();
    println!("\n============================\n");

    demo_nested_structs();
    println!("\n============================\n");

    demo_constructor_builder();
    println!("\n============================\n");

    demo_associated_function();
    println!("\n============================\n");

    demo_shared_receiver();
    println!("\n============================\n");

    demo_mutable_receiver();
    println!("\n============================\n");

    demo_owned_receiver();
    println!("\n============================\n");

    demo_choosing_an_api();
}

// =====================================================
// 01. Generate a builder for a struct
// =====================================================

fn demo_struct_builder() {
    println!("01. Generate a builder for a struct");

    #[derive(Debug, Builder)]
    struct Customer {
        name: String,
        age: u32,
        city: String,
    }

    let customer = Customer::builder()
        .city(String::from("Seoul"))
        .name(String::from("Alice"))
        .age(30)
        .build();

    println!("{customer:?}");
    println!("{} is {} and lives in {}", customer.name, customer.age, customer.city);
    // #[derive(Builder)] generates Customer::builder() and its setter API.
    // .build() returns Customer, not Result<Customer, ...> or Option<Customer>.
    // Every field here is required, and setters can be called in any order.
    // Debug is a separate standard derive used only to make printing convenient.

    let literal = Customer {
        name: String::from("Bob"),
        age: 25,
        city: String::from("Paris"),
    };
    println!("struct literal still works: {literal:?}");
    // The derive does not remove ordinary struct-literal construction.
    // Field privacy still controls where a struct literal is allowed.
}

// =============================================================
// 02. Defaults and optional fields work just like parameters
// =============================================================

fn demo_struct_defaults() {
    println!("02. Defaults and optional fields work just like parameters");

    #[derive(Debug, Builder)]
    struct ReportOptions {
        title: String,
        #[builder(default="text")]
        format: &'static str,
        #[builder(default)]
        verbose: bool,
        #[builder(default)]
        columns: Vec<String>,
        limit: Option<usize>,
    }

    let basic = ReportOptions::builder()
        .title(String::from("Summary"))
        .build();
    println!("defaults: {basic:?}");
    println!("{}: {}, verbose={}, columns={:?}, limit={:?}",
        basic.title, basic.format, basic.verbose, basic.columns, basic.limit);
    // format="text", verbose=false, columns=[], limit=None.
    // The derive does NOT make every field optional automatically.

    let limit_from_config = Some(100);
    let custom = ReportOptions::builder()
        .title(String::from("Measurements"))
        .format("csv")
        .verbose(true)
        .columns(vec![String::from("time"), String::from("value")])
        .maybe_limit(limit_from_config)
        .build();
    println!("custom: {custom:?}");
    // .limit(100) accepts usize; .maybe_limit(Some(100)) accepts Option<usize>.
    // #[builder(default)] uses Default::default() for that field.
    // Builder defaults do not add a Default implementation to ReportOptions.
}

// =====================================================
// 03. Borrowed fields and lifetimes
// =====================================================

fn demo_borrowed_fields() {
    println!("03. Borrowed fields and lifetimes");

    #[derive(Debug, Builder)]
    struct TextView<'a> {
        text: &'a str,
        #[builder(default="untitled")]
        label: &'a str,
    }

    let source = String::from("A borrowed piece of text");
    let view = TextView::builder().text(&source).label("preview").build();
    println!("{}: {}", view.label, view.text);
    println!("original is still owned here: {source}");
    // No String clone is needed: view holds references into existing text.
    // source must live long enough for both the builder and the resulting view.
    // Use owned String fields when the constructed value must own its text.
}

// =====================================================
// 04. Compose builders for nested structs
// =====================================================

fn demo_nested_structs() {
    println!("04. Compose builders for nested structs");

    #[derive(Debug, Builder)]
    struct Address {
        city: String,
        #[builder(default=String::from("Unknown"))]
        country: String,
    }

    #[derive(Debug, Builder)]
    struct Profile {
        name: String,
        address: Address,
    }

    let address = Address::builder()
        .city(String::from("Seoul"))
        .country(String::from("South Korea"))
        .build();

    let profile = Profile::builder()
        .name(String::from("Long"))
        .address(address)
        .build();

    println!("{} lives in {}, {}", profile.name, profile.address.city, profile.address.country);
    // The inner .build() returns an Address; the outer setter consumes it.
    // Bon does not automatically turn nested fields into a flattened setter API.
    // Separate builders keep each type's required inputs explicit.
}

// ==========================================================
// 05. Constructor builders: put logic in an impl's new()
// ==========================================================

fn demo_constructor_builder() {
    println!("05. Constructor builders: put logic in an impl's new()");

    // No derive(Builder) here: the constructor generates the builder instead.
    #[derive(Debug)]
    struct Customer {
        name: String,
        display_label: String,
    }

    #[bon]
    impl Customer {
        #[builder]
        fn new(name: &str, #[builder(default="Guest")] role: &str) -> Self {
            println!("constructor body is running");
            let name = name.trim().to_owned();
            let display_label = format!("{role}: {name}");
            Self { name, display_label }
        }
    }

    let pending = Customer::builder().name("  Alice  ");
    println!("inputs prepared; constructor has not run yet");

    let customer = pending.role("Member").build();
    println!("name={:?}, label={:?}", customer.name, customer.display_label);
    // new() is special: its generated entry/finish methods are builder()/build().
    // Its parameters become setters; they need not match the struct's fields.
    // Here role is an input, display_label is computed, and name is normalized.
    // #[bon] processes the impl; #[builder] selects methods within that impl.
    // The generated API replaces the ordinary Customer::new(name, role) call.
    // Do not also derive Builder on this Customer with the default names:
    // both approaches would try to generate Customer::builder().
}

// =============================================================
// 06. Associated functions use their own name and .call()
// =============================================================

fn demo_associated_function() {
    println!("06. Associated functions use their own name and .call()");

    struct Labels;

    #[bon]
    impl Labels {
        #[builder]
        fn make(name: &str, #[builder(default="INFO")] prefix: &str) -> String {
            format!("[{prefix}] {name}")
        }
    }

    println!("{}", Labels::make().name("Starting").call());
    println!("{}", Labels::make().prefix("DONE").name("Finished").call());
    // No receiver: start with Type::method().
    // The method is named make, not new, so its finish method is .call().
    // The return type is String; an associated builder need not construct Self.
}

// =====================================================
// 07. Methods with &self: borrow an existing value
// =====================================================

fn demo_shared_receiver() {
    println!("07. Methods with &self: borrow an existing value");

    struct Greeter {
        name: String,
    }

    #[bon]
    impl Greeter {
        #[builder]
        fn greet(
            &self,
            target: &str,
            #[builder(default="Hello")] greeting: &str,
            punctuation: Option<&str>,
        ) -> String {
            let punctuation = punctuation.unwrap_or("!");
            format!("{greeting}, {target}{punctuation} From {}.", self.name)
        }

        // An unannotated method retains its ordinary Rust API.
        fn name(&self) -> &str {
            &self.name
        }
    }

    let greeter = Greeter { name: String::from("Long") };
    println!("{}", greeter.greet().target("Alice").call());
    println!("{}", greeter.greet().target("Bob").greeting("Welcome").punctuation(".").call());
    println!("receiver is still usable: {}", greeter.name());
    // .greet() captures &greeter; you do not pass self through a setter.
    // .call() executes the original method body using that borrowed receiver.
    // The builder consumes itself, but this method only borrows the Greeter.
}

// =====================================================
// 08. Methods with &mut self: defer a mutation
// =====================================================

fn demo_mutable_receiver() {
    println!("08. Methods with &mut self: defer a mutation");

    #[derive(Debug)]
    struct Notebook {
        lines: Vec<String>,
    }

    #[bon]
    impl Notebook {
        #[builder]
        fn append(&mut self, text: &str, #[builder(default="- ")] prefix: &str) -> usize {
            self.lines.push(format!("{prefix}{text}"));
            self.lines.len()
        }
    }

    let mut notebook = Notebook { lines: Vec::new() };
    let pending = notebook.append().text("Learn Bon");
    // pending holds a mutable borrow: do not inspect/mutate notebook here.
    // The line is not appended until .call() executes the body.

    let count = pending.call();
    println!("line count after call: {count}");
    println!("notebook: {notebook:?}");

    let count = notebook.append().text("Build something").prefix("* ").call();
    println!("line count after another call: {count}");
    println!("notebook: {notebook:?}");
    // After each call, that builder is consumed and its borrow ends here,
    // so the notebook can be inspected or borrowed again.
}

// =====================================================
// 09. Methods with self: consume an existing value
// =====================================================

fn demo_owned_receiver() {
    println!("09. Methods with self: consume an existing value");

    struct Draft {
        text: String,
    }

    #[bon]
    impl Draft {
        #[builder]
        fn finish(self, #[builder(default="")] suffix: &str) -> String {
            format!("{}{suffix}", self.text)
        }
    }

    let draft = Draft { text: String::from("Finished learning builders") };
    let pending = draft.finish();
    // draft moves into the builder immediately, not only at .call().
    let published = pending.suffix(" :33").call();
    println!("{published}");
    // draft is no longer available. The method consumed it and returned String.
    // &self borrows, &mut self mutably borrows, self moves: the receiver
    // determines ownership behavior, just as it does for ordinary Rust methods.
}

// ===========================================================
// 10. Choose the simplest API and recognize common mistakes
// ===========================================================

fn demo_choosing_an_api() {
    println!("10. Choose the simplest API and recognize common mistakes");

    #[derive(Debug, Builder)]
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point::builder().y(4).x(3).build();
    println!("point=({}, {})", point.x, point.y);
    // For a tiny type, Point { x: 3, y: 4 } is already clear.
    // Builders become more useful with many parameters or optional settings.

    println!();

    println!("The examples below intentionally stay commented out:");
    // Point::builder().x(3).build();
    // ERROR: required y is missing.

    // Point::builder().x(3).x(5).y(4).build();
    // ERROR: x was already set; setters are not ordinary overwrite operations.

    // Point::builder().x(3).y(4).call();
    // ERROR: this struct builder finishes with .build(), not .call().

    // Point::builder().x("three").y(4).build();
    // ERROR: wrong type; x expects i32.

    // These are compilation failures, so no runtime try/match can catch them.
    // Uncomment one at a time to inspect the diagnostic.

    println!();

    println!("Struct derive: construct fields directly with .build().");
    println!("Constructor new: execute construction logic with .build().");
    println!("Other functions/methods: execute an operation with .call().");
    // All required members being set does not imply semantic validity.
    // For example, a constructor still needs to validate user input itself.
    // The next file covers fallible builders returning Result and more controls.
}

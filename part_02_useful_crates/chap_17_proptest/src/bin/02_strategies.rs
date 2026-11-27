//! Proptest strategies: describe the inputs your properties need.
//!
//! Setup: cargo add proptest@1 --dev
//! Save as src/bin/02_strategies.rs.
//! Run tests: cargo test --bin 02_strategies
//! Print outline: cargo run --bin 02_strategies
//!
//! A strategy is a recipe for generating AND shrinking values, not one sample.
//! Compose recipes rather than manually sampling random values inside tests.
//! Generated domains should reflect the contract being tested, including invalid
//! inputs when rejection/error handling is part of that contract.
//! All tests here are intended to pass; no deliberately failing tests.
//!
//! References:
//! https://docs.rs/proptest/latest/proptest/strategy/trait.Strategy.html
//! https://docs.rs/proptest/latest/proptest/macro.prop_compose.html
//! https://docs.rs/proptest/latest/proptest/macro.prop_oneof.html

fn main() {
    println!();

    println!("01. Primitive strategies and tuples");
    println!("Ranges constrain values; tuples combine strategies.");
    println!("\n============================\n");

    println!("02. Strings generated from regex patterns");
    println!("Describe a small text grammar without depending on the regex crate.");
    println!("\n============================\n");

    println!("03. Collections and their size bounds");
    println!("Generate vectors and sets; choose useful, affordable sizes.");
    println!("\n============================\n");

    println!("04. Options, fixed values, and sampled choices");
    println!("Combine optional data, Just, and sample::select.");
    println!("\n============================\n");

    println!("05. prop_map: construct rectangles from dimensions");
    println!("Connect generated structures to the Chapter 11 can_hold example.");
    println!("\n============================\n");

    println!("06. prop_compose!: reusable parameterized strategies");
    println!("Name a strategy when several properties need the same domain.");
    println!("\n============================\n");

    println!("07. prop_oneof!: generate enum variants");
    println!("Map branch strategies into one shared output type.");
    println!("\n============================\n");

    println!("08. prop_flat_map: generate dependent inputs");
    println!("Generate a nonempty vector, then an index valid for that vector.");
    println!("\n============================\n");

    println!("09. Filtering versus constructing valid inputs");
    println!("Reject sparingly; generate constraints directly when possible.");
    println!();
    println!("Run everything: cargo test --bin 02_strategies");
    println!("Run one topic: cargo test --bin 02_strategies rectangles");
    // cargo test runs the test harness, not main. Numbered sections organize the
    // source; they do not prescribe execution order for independent tests.
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    // =====================================================
    // 01. Primitive strategies and tuples
    // =====================================================

    proptest! {
        #[test]
        fn tuples_combine_inputs(pair in (0u16..=100, any::<bool>())) {
            let (number, enabled) = pair;
            let actual = if enabled { Some(number) } else { None };

            prop_assert_eq!(actual.is_some(), enabled);
            if let Some(value) = actual {
                prop_assert!(value <= 100);
            }
        }
    }
    // (strategy_a, strategy_b) generates (value_a, value_b).
    // any::<bool>() describes bool inputs; 0u16..=100 includes both endpoints.
    // A tuple does not automatically create relationships between its members.
    // These early tests demonstrate strategy mechanics. Later examples exercise
    // a small application API, rather than just checking the generated shape.

    // =====================================================
    // 02. Strings generated from regex patterns
    // =====================================================

    proptest! {
        #[test]
        fn generated_identifiers(name in "[a-z][a-z0-9_]{0,11}") {
            prop_assert!((1..=12).contains(&name.len()));
            prop_assert!(name.as_bytes()[0].is_ascii_lowercase());
            prop_assert!(name.bytes().all(|b| b.is_ascii_lowercase()
                || b.is_ascii_digit() || b == b'_'));
        }
    }
    // The pattern produces a String: one lowercase letter plus 0..=11 allowed
    // characters. It generates matching text, rather than searching existing text.
    // Proptest supplies this facility; no direct regex dependency is needed here.
    // This domain is ASCII, so byte length and character count coincide.
    // Don't generalize that equality to Unicode strings. A restricted identifier
    // strategy also isn't a substitute for testing a parser with malformed text.

    // =====================================================
    // 03. Collections and their size bounds
    // =====================================================

    proptest! {
        #[test]
        fn vectors_can_be_empty(values in proptest::collection::vec(-20i32..=20, 0..16)) {
            let mut doubled = values.clone();
            doubled.extend_from_slice(&values);
            prop_assert_eq!(doubled.len(), values.len() * 2);
        }

        #[test]
        fn ordered_sets_are_unique(values in proptest::collection::btree_set(0u8..20, 0..10)) {
            let ordered: Vec<_> = values.into_iter().collect();
            prop_assert!(ordered.windows(2).all(|pair| pair[0] < pair[1]));
        }
    }
    // vec(element_strategy, 0..16) has length 0..=15. Passing 8 instead of a
    // range would request exactly 8 elements. Bounds influence shrinking too.
    // Sets may need retries for duplicate candidates: don't request more unique
    // elements than the element domain can provide. Here at most 9 of 20 values.
    // Arrays, maps, and other collections have strategies too; keep sizes bounded.

    // =====================================================
    // 04. Options, fixed values, and sampled choices
    // =====================================================

    proptest! {
        #[test]
        fn optional_settings(
            limit in proptest::option::of(1u16..=100),
            mode in proptest::sample::select(vec!["fast", "safe", "debug"]),
            marker in Just("tutorial")
        ) {
            let effective_limit = limit.unwrap_or(10);
            prop_assert!((1..=100).contains(&effective_limit));
            prop_assert!(["fast", "safe", "debug"].contains(&mode));
            prop_assert_eq!(marker, "tutorial");
        }
    }
    // option::of generates None or Some(generated_value).
    // sample::select chooses from a NONEMPTY collection of supplied values.
    // Just(value) always supplies that value; useful when composing alternatives.
    // Generated variation isn't a coverage guarantee. Keep explicit ordinary tests
    // for particular cases that must be exercised on every run.

    // =====================================================
    // 05. prop_map: construct rectangles from dimensions
    // =====================================================

    #[derive(Clone, Debug)]
    struct Rectangle {
        width: u16,
        height: u16,
    }

    impl Rectangle {
        fn can_hold(&self, other: &Self) -> bool {
            self.width > other.width && self.height > other.height
        }
    }

    proptest! {
        #[test]
        fn rectangles_hold_strictly_smaller_ones(
            small in (1u16..=100, 1u16..=100)
                .prop_map(|(width, height)| Rectangle { width, height }),
            extra_width in 1u16..=20,
            extra_height in 1u16..=20
        ) {
            let large = Rectangle {
                width: small.width + extra_width,
                height: small.height + extra_height,
            };
            prop_assert!(large.can_hold(&small));
            prop_assert!(!small.can_hold(&large));
            prop_assert!(!small.can_hold(&small)); // Strict inequalities.
        }
    }
    // prop_map transforms each generated tuple into a Rectangle. It returns a
    // NEW strategy, not a Rectangle immediately. Values shrink through the input
    // strategy and are mapped again; no custom shrinking implementation is needed.
    // Integer dimensions keep this review focused; floating-point NaN/rounding
    // would introduce separate contract questions. All sums above fit u16.
    // Constructing larger dimensions guarantees the relationship without rejection.

    // =====================================================
    // 06. prop_compose!: reusable parameterized strategies
    // =====================================================

    prop_compose! {
        fn rectangles(max_side: u16)
            (width in 1u16..=max_side, height in 1u16..=max_side)
            -> Rectangle
        {
            Rectangle { width, height }
        }
    }
    // First parentheses: ordinary arguments passed by the caller.
    // Second parentheses: generated inputs, which can use those arguments.
    // -> Rectangle names the GENERATED type. The macro actually creates a
    // function returning impl Strategy<Value=Rectangle>, not a Rectangle.
    // This form packages tuple + prop_map. Caller contract: max_side >= 1.

    proptest! {
        #[test]
        fn rectangles_holding_is_asymmetric(a in rectangles(100), b in rectangles(100)) {
            if a.can_hold(&b) {
                prop_assert!(!b.can_hold(&a));
            }
            prop_assert!(!a.can_hold(&a));
        }
    }
    // Review: tests inside this module can call private application methods.
    // A property with an if condition only checks its branch when the premise
    // occurs. The constructed pair in section 05 always exercises containment.

    // =====================================================
    // 07. prop_oneof!: generate enum variants
    // =====================================================

    #[derive(Clone, Debug)]
    enum Action {
        Reset,
        Set(u16),
    }

    impl Action {
        fn apply(&self, state: &mut u16) {
            match self {
                Self::Reset => *state = 0,
                Self::Set(value) => *state = *value,
            }
        }
    }

    fn actions() -> impl Strategy<Value=Action> {
        prop_oneof![
            1 => Just(Action::Reset),
            3 => (0u16..=100).prop_map(Action::Set),
        ]
    }
    // Both branches generate Action, even though their strategy types differ.
    // Weights guide random selection (1:3); they aren't guaranteed run counts.
    // Put a simple alternative first: branch order also participates in shrinking.
    // Returning impl Strategy<Value=Action> hides the composed strategy's type.
    // This function builds a reusable recipe, not an already sampled action.

    proptest! {
        #[test]
        fn action_updates_state(initial in 0u16..=100, action in actions()) {
            let mut state = initial;
            action.apply(&mut state);
            match action {
                Action::Reset => prop_assert_eq!(state, 0),
                Action::Set(value) => prop_assert_eq!(state, value),
            }
            prop_assert!(state <= 100);
        }
    }
    // The generated action is passed to a tiny state-changing API. A larger
    // stateful API benefits from operation sequences and a reference model.

    // =====================================================
    // 08. prop_flat_map: generate dependent inputs
    // =====================================================

    fn indexed_vectors() -> impl Strategy<Value=(Vec<i32>, usize)> {
        proptest::collection::vec(-100i32..=100, 1..20)
            .prop_flat_map(|values| {
                let length = values.len();
                (Just(values), 0usize..length)
            })
    }
    // First generate a NONEMPTY vector, then build an index strategy from its len.
    // flat_map's closure returns another STRATEGY; map's returns a VALUE.
    // The runner manages dependent generation/shrinking so the pair stays valid.
    // An independent any::<usize>() would almost always be out of bounds.
    // Just carries the chosen vector into the second-stage tuple.

    proptest! {
        #[test]
        fn removing_a_valid_index(pair in indexed_vectors()) {
            let (mut values, index) = pair;
            let original_length = values.len();
            let expected = values[index];
            let removed = values.remove(index);

            prop_assert_eq!(removed, expected);
            prop_assert_eq!(values.len(), original_length - 1);
        }
    }

    // =====================================================
    // 09. Filtering versus constructing valid inputs
    // =====================================================

    proptest! {
        #[test]
        fn filter_excludes_zero(divisor in (-20i32..=20)
            .prop_filter("divisor must be nonzero", |value| *value != 0))
        {
            prop_assert_eq!(0 / divisor, 0);
        }

        #[test]
        fn assumption_rejects_a_whole_case(a in -20i32..=20, b in -20i32..=20) {
            prop_assume!(a != b);
            prop_assert_ne!(a, b);
        }

        #[test]
        fn construct_even_numbers(value in (0u16..=100).prop_map(|n| n * 2)) {
            prop_assert_eq!(value % 2, 0);
        }
    }
    // prop_filter retries values that fail its predicate. prop_assume rejects
    // the current generated test case, rather than declaring success or failure.
    // Both are useful for occasional exclusions. Excessive rejection can exhaust
    // configured limits and abort a test; don't filter for extremely rare cases.
    // For evens, construct n * 2 instead of discarding half the integers.
    // For index/vector relationships, use flat_map rather than mostly rejecting.
    // Review: assertions describe expected behavior; assumptions define the domain.
    // Never assume away a failing input that your function is required to handle.
    // The filtering examples are mechanics demos, not strong application tests.
}

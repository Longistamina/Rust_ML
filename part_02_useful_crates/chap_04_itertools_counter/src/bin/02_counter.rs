/*
counter: a Python-inspired frequency table / multiset for Rust.

1. Construct from characters, words, and collections
2. Missing keys, get(), and mutable indexing
3. Most common items and top k
4. Explicit tie-breaking
5. Arithmetic: +, -, &, | and assignment operators
6. update() and subtract(): differences from Python
7. Preserve zero / negative results with signed counts
8. Python elements() equivalent
9. Weighted counts: collect / extend from (item, count) pairs
10. Distinct keys, total counts, zeros, and equality
11. Multiset subset / superset checks
12. HashMap access and into_map()
13. Rust-specific count types, capacity, and key types
14. Word / DNA / duplicate / vote counting
15. Count derived categories

Setup:
    cargo add counter@0.7.1

Keep this file beside Cargo.toml and add:
    [[bin]]
    name = "02_counter"
    path = "02_counter.rs"

Run:
    cargo run --bin 02_counter

Counter<T, N = usize, S = RandomState>: key type, count type, hash builder.
No itertools dependency needed for this file.
Hash-based storage does not preserve Python Counter's first-seen order.
Examples use most_common_ordered() or sorted vectors for deterministic output.
This is not a drop-in Python Counter: subtraction, equality, and APIs differ.
Reference: https://docs.rs/counter/0.7.1/counter/struct.Counter.html
*/

use counter::Counter;
use std::collections::HashMap;
use std::iter;

fn main() {
    println!();

    // ============================================================================
    // 1. Construct a Counter
    // ============================================================================
    {
        let chars: Counter<char> = "mississippi".chars().collect();
        println!("{:?}", chars.most_common_ordered());
        // [('i', 4), ('s', 4), ('p', 2), ('m', 1)]

        let fruits = ["apple", "banana", "apple", "orange", "banana", "apple"];
        let counts: Counter<&str> = fruits.into_iter().collect();
        println!("{:?}", counts.most_common_ordered());
        // [("apple", 3), ("banana", 2), ("orange", 1)]

        let numbers = vec![1, 1, 2, 3, 3];
        let counts: Counter<i32> = numbers.iter().copied().collect();
        println!("{:?}", counts.most_common_ordered()); // [(1, 2), (3, 2), (2, 1)]
        // .iter().copied() counts copied values; .into_iter() moves the Vec's items.
        // Counter<_> lets Rust infer keys and uses usize counts by default.
        // Use collect(); the older Counter::init() constructor is deprecated.

        println!("\n============================\n");
    }

    // ============================================================================
    // 2. Missing keys and mutable indexing
    // ============================================================================
    {
        let mut counts: Counter<char> = "aaa".chars().collect();
        println!("{}", counts[&'a']); // 3
        println!("{}", counts[&'z']); // 0, NOT a panic
        println!("{:?}", counts.get(&'z')); // None
        println!("{}", counts.contains_key(&'z')); // false
        // Reading counts[&missing] gives zero without inserting the key.

        counts[&'b'] += 2; // mutable indexing inserts a missing key at zero first
        println!("{:?}", counts.most_common_ordered()); // [('a', 3), ('b', 2)]
        // get() distinguishes 'missing' from an explicitly stored zero.
        // HashMap indexing normally panics for missing keys; Counter overrides it.

        println!("\n============================\n");
    }

    // ============================================================================
    // 3. Most common and top k
    // ============================================================================
    {
        let counts: Counter<_> = [1, 1, 1, 2, 2, 3, 3, 3, 3, 4, 5, 5].into_iter().collect();
        println!("{:?}", counts.most_common()); // descending frequency; tied order unspecified
        println!("{:?}", counts.most_common_ordered());
        // [(3, 4), (1, 3), (2, 2), (5, 2), (4, 1)]

        println!("{:?}", counts.k_most_common_ordered(3)); // [(3, 4), (1, 3), (2, 2)]
        // Python most_common(k) -> Rust k_most_common_ordered(k).
        // most_common_ordered() breaks ties by the key's natural ascending order.
        // k larger than len() returns every stored entry; k=0 returns an empty Vec.
        // Ranking clones keys/counts into a Vec; it doesn't consume the Counter.

        println!("\n============================\n");
    }

    // ============================================================================
    // 4. Custom tie-breaking
    // ============================================================================
    {
        let counts: Counter<_> = "eaddbbccc".chars().collect();
        println!("{:?}", counts.most_common_ordered());
        // [('c', 3), ('b', 2), ('d', 2), ('a', 1), ('e', 1)]

        let ranked = counts.most_common_tiebreaker(|a, b| b.cmp(a));
        println!("{ranked:?}"); // [('c', 3), ('d', 2), ('b', 2), ('e', 1), ('a', 1)]
        // Frequency still sorts descending; this closure only orders tied keys.
        // Python Counter uses first-seen order for ties; this crate doesn't store it.
        // Python can implement custom ranking with sorted(); Rust exposes this helper.

        println!("\n============================\n");
    }

    // ============================================================================
    // 5. Arithmetic and multiset operations
    // ============================================================================
    {
        let a: Counter<char> = ['a', 'b', 'c', 'a', 'b', 'b'].into_iter().collect();
        let b: Counter<char> = ['a', 'b', 'd', 'd'].into_iter().collect();

        println!("sum: {:?}", (a.clone() + b.clone()).most_common_ordered());
        // [('b', 4), ('a', 3), ('d', 2), ('c', 1)]

        println!("a-b: {:?}", (a.clone() - b.clone()).most_common_ordered());
        // [('b', 2), ('a', 1), ('c', 1)]

        println!("b-a: {:?}", (b.clone() - a.clone()).most_common_ordered()); // [('d', 2)]

        println!("intersection: {:?}", (a.clone() & b.clone()).most_common_ordered());
        // [('a', 1), ('b', 1)]: minimum shared counts

        println!("union: {:?}", (a.clone() | b.clone()).most_common_ordered());
        // [('b', 3), ('a', 2), ('d', 2), ('c', 1)]: maximum counts

        let mut changed = a.clone();
        changed += b.clone();
        changed -= b.clone();
        changed &= a.clone();
        changed |= b;
        println!("assigned union: {:?}", changed.most_common_ordered());
        // Same counts as a | b for this positive-count example.
        // Binary operators consume their operands; clone to preserve originals.
        // These examples use ordinary positive multisets. Don't assume arithmetic
        // on explicitly stored zero / negative entries matches Python in all cases.

        println!("\n============================\n");
    }

    // ============================================================================
    // 6. update() and subtract()
    // ============================================================================
    {
        let mut counts: Counter<char> = "abc".chars().collect();
        counts.update("abd".chars());
        println!("{:?}", counts.most_common_ordered()); // [('a', 2), ('b', 2), ('c', 1), ('d', 1)]

        counts.subtract("abb".chars());
        println!("{:?}", counts.most_common_ordered()); // [('a', 1), ('c', 1), ('d', 1)]
        println!("{}", counts.contains_key(&'b')); // false: b was removed at zero
        counts.subtract("zzzz".chars()); // absent keys aren't inserted as negatives

        counts += "aa".chars(); // operator also accepts an iterable of keys
        counts -= "a".chars();
        println!("{}", counts[&'a']); // 2
        // Rust subtract() performs multiset-style removal for positive counts.
        // Python subtract() keeps zeros / negatives. Merely choosing a signed count
        // type does NOT make Rust subtract() preserve those results.
        // update() accepts individual items; weighted updates use extend(), below.

        println!("\n============================\n");
    }

    // ============================================================================
    // 7. Signed counts: preserve raw subtraction results
    // ============================================================================
    {
        let mut balance: Counter<char, i64> = "abb".chars().collect();
        for ch in "abbbz".chars() {
            balance[&ch] -= 1;
        }
        println!("{:?}", balance.most_common_ordered()); // [('a', 0), ('b', -1), ('z', -1)]
        // Direct arithmetic on stored signed counts preserves zero / negative values.
        // Default usize cannot represent negatives; raw subtraction could underflow.
        // This loop is the equivalent of Python Counter.subtract() on individual items.

        balance.retain(|_, n| *n > 0); // explicit positive-only cleanup
        println!("{}", balance.is_empty()); // true

        println!("\n============================\n");
    }

    // ============================================================================
    // 8. Expand counts: Python elements() equivalent
    // ============================================================================
    {
        let counts: Counter<char> = [('a', 3usize), ('b', 2), ('c', 1)].into_iter().collect();
        let mut elements: Vec<char> = counts.iter()
            .flat_map(|(&ch, &n)| iter::repeat(ch).take(n)).collect();
        elements.sort_unstable(); // HashMap traversal doesn't define element order
        println!("{elements:?}"); // ['a', 'a', 'a', 'b', 'b', 'c']
        // The crate has no direct elements() method; expand (key,count) pairs.

        let signed: Counter<char, i64> = [('a', 2), ('b', 0), ('c', -1)].into_iter().collect();
        let elements: Vec<_> = signed.iter().filter(|(_, n)| **n > 0)
            .flat_map(|(&ch, &n)| iter::repeat(ch).take(usize::try_from(n).unwrap()))
            .collect();
        println!("{elements:?}"); // ['a', 'a']; ignore non-positive counts
        // Positive i64 values can still exceed usize on a narrower platform; use
        // checked conversion with error handling for untrusted / enormous counts.
        // Expanding huge counts can allocate huge output; iterate lazily when possible.

        println!("\n============================\n");
    }

    // ============================================================================
    // 9. Weighted counts and extending another Counter
    // ============================================================================
    {
        let mut counts: Counter<char, i64> = [('a', 3), ('b', 2), ('a', 4)].into_iter().collect();
        println!("{:?}", counts.most_common_ordered()); // [('a', 7), ('b', 2)]
        // With a Counter<char,...> target, collect reads these as (item,count) pairs.
        // Repeated keys ADD their supplied weights, rather than overwrite them.

        counts.extend([('a', -2), ('c', 5)]);
        println!("{:?}", counts.most_common_ordered()); // [('a', 5), ('c', 5), ('b', 2)]

        let extra: Counter<char, i64> = "cc".chars().collect();
        counts.extend(&extra); // accepts borrowed (&key, &count) pairs, cloning as needed
        println!("{:?}", counts.most_common_ordered()); // [('c', 7), ('a', 5), ('b', 2)]
        // extend(weighted_pairs) ADDS counts. HashMap::extend would overwrite keys.
        // insert(key,count), inherited from HashMap, DOES overwrite the old count.

        println!("\n============================\n");
    }

    // ============================================================================
    // 10. Stored keys, total counts, and explicit zeros
    // ============================================================================
    {
        let mut counts: Counter<char> = "abb".chars().collect();
        println!("keys: {}; total: {}", counts.len(), counts.total::<usize>()); // 2; 3
        counts.insert('z', 0);
        println!("keys: {}; total: {}", counts.len(), counts.total::<usize>()); // 3; 3

        let original: Counter<char> = "abb".chars().collect();
        println!("{}", counts == original); // false: explicit z=0 is stored
        // This crate compares backing maps. Modern Python Counter equality treats
        // a missing key and an explicit zero count as equivalent.
        counts.retain(|_, n| *n != 0);
        println!("{}", counts == original); // true after removing the zero entry
        // total::<N>() sums counts; unlike len(), it counts multiplicities.
        // With signed counts, total includes negative values too.

        println!("\n============================\n");
    }

    // ============================================================================
    // 11. Subset / superset checks use multiplicities
    // ============================================================================
    {
        let needed: Counter<char> = "aab".chars().collect();
        let available: Counter<char> = "aaabbc".chars().collect();
        println!("{}", needed.is_subset(&available)); // true
        println!("{}", available.is_superset(&needed)); // true
        let too_many: Counter<char> = "aaaa".chars().collect();
        println!("{}", too_many.is_subset(&available)); // false: needs 4 a, only 3 available
        // This checks counts, not just key membership. Python offers comparable
        // checks with <= / >=; this crate exposes named methods instead.

        println!("\n============================\n");
    }

    // ============================================================================
    // 12. HashMap interoperability
    // ============================================================================
    {
        let mut counts: Counter<char> = "aa-bb-c".chars().collect();
        counts.remove(&'-');
        *counts.entry('d').or_insert(0) += 4;
        println!("{:?}", counts.most_common_ordered()); // [('d', 4), ('a', 2), ('b', 2), ('c', 1)]

        let map: HashMap<char, usize> = counts.into_map(); // consumes Counter
        println!("missing via map: {:?}", map.get(&'z')); // None
        // map[&'z'] would panic. The zero-returning indexing belongs to Counter.
        let restored: Counter<char> = map.into_iter().collect();
        println!("{}", restored[&'z']); // 0
        // Counter implements Deref / DerefMut to its map: get, insert, remove,
        // retain, entry, keys, values, reserve, etc. are available directly.

        println!("\n============================\n");
    }

    // ============================================================================
    // 13. Rust-specific types and capacity
    // ============================================================================
    {
        let wide: Counter<char, u128> = "aaab".chars().collect();
        println!("{}", wide.total::<u128>()); // 4
        // Choose usize, u128, i64, etc. to fit the application. Counts are still
        // bounded machine integers; use checked operations if overflow is possible.
        // Python int counts already grow arbitrarily large; u128 is not that feature.

        let mut reserved: Counter<char> = Counter::with_capacity(4);
        reserved.update("ACGTACGT".chars());
        println!("distinct: {}", reserved.len()); // 4
        // Capacity reserves DISTINCT keys, not the sum of the counts.

        #[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
        enum Base { A, C, G, T }
        let bases: Counter<Base> = [Base::A, Base::C, Base::G, Base::T, Base::A].into_iter().collect();
        println!("{:?}", bases.most_common_ordered()); // [(A, 2), (C, 1), (G, 1), (T, 1)]
        // Key types need Eq + Hash; ranking adds Clone, and ordered ranking adds Ord.
        // Python can also count custom hashable objects. Rust enforces these traits
        // at compile time. The third Counter generic parameter selects a hash builder.

        println!("\n============================\n");
    }

    // ============================================================================
    // 14. Practical examples from the Python guide
    // ============================================================================
    {
        let words: Counter<&str> = "the quick brown fox jumps over the lazy dog the fox"
            .split_whitespace().collect();
        println!("words: {:?}", words.k_most_common_ordered(2)); // [("the", 3), ("fox", 2)]
        // split_whitespace doesn't strip punctuation or lowercase words.

        let dna: Counter<char> = "ATCGATCGATCGTAGCTAGCTA".chars().collect();
        println!("DNA: {:?}", dna.most_common_ordered()); // [('A', 6), ('T', 6), ('C', 5), ('G', 5)]

        let counts: Counter<i32> = [1, 2, 3, 2, 4, 5, 3, 6, 3].into_iter().collect();
        let mut duplicates: Vec<_> = counts.iter().filter(|(_, n)| **n > 1)
            .map(|(&item, _)| item).collect();
        duplicates.sort_unstable();
        println!("duplicates: {duplicates:?}"); // [2, 3]

        let votes: Counter<&str> = ["Alice", "Bob", "Alice", "Charlie", "Alice", "Bob", "Alice"]
            .into_iter().collect();
        if let Some((name, n)) = votes.k_most_common_ordered(1).first() {
            println!("Winner: {name} with {n} votes"); // Alice with 4 votes
        }
        // Equal vote counts are resolved alphabetically here, not by first appearance.

        println!("\n============================\n");
    }

    // ============================================================================
    // 15. Count categories rather than exact values
    // ============================================================================
    {
        let mut categories: Counter<&str> = Counter::new();
        for ch in "P@ssw0rd123!".chars() {
            let category = if ch.is_ascii_alphabetic() {
                "letters"
            } else if ch.is_ascii_digit() {
                "digits"
            } else {
                "special"
            };
            categories[&category] += 1;
        }
        println!("{:?}", categories.most_common_ordered());
        // [("letters", 6), ("digits", 4), ("special", 2)]
        // ASCII classification is explicit; Unicode letter/digit rules may differ
        // between languages. This counts character categories, not password strength.

        println!("\n============================\n");
    }
}

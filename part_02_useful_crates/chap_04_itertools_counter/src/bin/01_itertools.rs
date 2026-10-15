/*
Iterator utilities: Python equivalents and additional itertools features.

[std] means standard library; [itertools] means the external crate.

1. count(start, step) -> ranges / step_by [std]
2. cycle [std]
3. repeat [std]
4. accumulate -> scan [std]
5. chain [std]
6. compress -> zip + filter_map [std]
7. dropwhile -> skip_while [std]
8. filterfalse -> negated predicate [std]
9. groupby -> chunk_by; global grouping -> into_group_map [itertools]
10. islice -> skip / take / step_by [std]
11. pairwise -> array_windows / circular_array_windows [itertools]
12. starmap -> map with tuple destructuring [std]
13. takewhile -> take_while [std]
14. tee [itertools]; cloning an iterator [std]
15. zip_longest [itertools]
16. product -> cartesian_product / iproduct! / multi_cartesian_product [itertools]
17. permutations [itertools]
18. combinations [itertools]
19. combinations_with_replacement [itertools]
20. Frequency counting: counts / counts_by [itertools]
21. Collection and display [itertools]
22. Sorting and selecting the smallest k [itertools]
23. Global uniqueness vs consecutive deduplication [itertools]
24. Global grouping by a derived key [itertools]
25. Aggregate values directly per group [itertools]
26. Fixed-size batches: chunks [itertools]
27. Fixed-size collection [itertools]
28. All subsets: powerset [itertools]
29. Interleave and intersperse [itertools]
30. Merge sorted streams and compare keys [itertools]
31. Partition and transform in one pass [itertools]
32. Multiple iterators and columns [itertools]
33. Min/max and cardinality validation [itertools]
34. Result-aware operations [itertools]
35. Merge adjacent items: coalesce [itertools]
36. Positions, padding, and inclusive stopping [itertools]

Setup (itertools 0.15):
    cargo add itertools@0.15

Keep this file beside Cargo.toml and add:
    [[bin]]
    name = "01_itertools"
    path = "01_itertools.rs"

Run:
    cargo run --bin 01_itertools

Bring Itertools into scope to add its methods to iterators.
Adapters are usually lazy; collect / for / count / sum drive iteration.
Terminal operations such as counts() consume the iterator immediately.
collect_vec() is shorthand for collect::<Vec<_>>(), not a lazy operation.
.iter() borrows items; .copied() copies Copy items; .into_iter() takes ownership.
An owned collection moved into its iterator can't then be reused as before.
Counter-specific arithmetic is reserved for 02_counter.rs.
Reference: https://docs.rs/itertools/latest/itertools/trait.Itertools.html
*/

use itertools::{iproduct, izip, Either, EitherOrBoth, Itertools};
use std::cmp::Reverse;
use std::iter;

fn main() {
    println!();

    // ============================================================================
    // 1. Consecutive numbers: Python count(start=10, step=2) [std]
    // ============================================================================
    {
        let numbers: Vec<_> = (10u64..).step_by(2).take(5).collect();
        println!("{numbers:?}"); // [10, 12, 14, 16, 18]
        // Bound examples with take() or break; don't collect an unbounded source.
        // Rust integers have finite ranges, unlike Python's arbitrary-size ints.

        // For a descending or custom progression, map a finite index range:
        println!("{:?}", (0..4).map(|i| 10 - 2 * i).collect::<Vec<_>>());
        // [10, 8, 6, 4]; step_by() itself takes a positive usize step.
        // Iterator::count() is unrelated: it CONSUMES an iterator and counts items.

        println!("\n============================\n");
    }

    // ============================================================================
    // 2. Cycle [std]
    // ============================================================================
    {
        let colors = ["red", "green", "blue"];
        for (i, color) in colors.iter().cycle().take(6).enumerate() {
            println!("{i} - {color}"); // red, green, blue, red, green, blue
        }
        // cycle() requires a Clone iterator so it can restart it.
        // Cycling an empty iterator stays empty.

        println!("\n============================\n");
    }

    // ============================================================================
    // 3. Repeat [std]
    // ============================================================================
    {
        println!("{:?}", iter::repeat("Hello").take(3).collect::<Vec<_>>());
        // ["Hello", "Hello", "Hello"]
        // repeat(value) clones the value; repeat_with(closure) constructs each item.
        // For references, cloning repeats references to the same target.
        // Python itertools.repeat(...) IS an iterator too.

        println!("\n============================\n");
    }

    // ============================================================================
    // 4. Accumulation: scan [std]
    // ============================================================================
    {
        let sums: Vec<_> = (1..=5).scan(0, |state, x| { // `0` -> `state` (initial value)
            *state += x;
            Some(*state)
        }).collect();
        println!("{sums:?}"); // [1, 3, 6, 10, 15]

        let products: Vec<_> = (1..=5).scan(1, |state, x| {
            *state *= x;
            Some(*state)
        }).collect();
        println!("{products:?}"); // [1, 2, 6, 24, 120]

        // scan keeps mutable state; the initial state is not itself emitted.
        // Python accumulate(..., initial=seed) also emits seed; prepend it if needed.

        println!("\n============================\n");
    }

    // ============================================================================
    // 5. Chain [std]
    // ============================================================================
    {
        println!("{:?}", [1, 2].into_iter().chain([3, 4]).chain([5]).collect::<Vec<_>>());
        // [1, 2, 3, 4, 5]

        println!("{}", "Hello".chars().chain("World".chars()).collect::<String>());
        // HelloWorld
        // Chained iterators must yield the SAME item type. Python's mixed int/str
        // example needs a shared Rust representation, such as an enum, first.

        println!("\n============================\n");
    }

    // ============================================================================
    // 6. Compress with a selector mask [std]
    // ============================================================================
    {
        let data = ['A', 'B', 'C', 'D', 'E'];
        let mask = [true, false, true, false, true];

        let selected: Vec<_> = data.into_iter().zip(mask) // zip() -> [('A', true), ('B', false), ('C', true), ...]
            .filter_map(|(item, keep)| keep.then_some(item)).collect();
            // ('A', true) -> true.then_some('A') -> Some('A') -> filter_map() returns 'A'
            // ('B', false) -> false.then_some('B') -> None -> filter_map() ignores

        println!("{selected:?}"); // ['A', 'C', 'E']
        // zip stops at the shorter iterator, like Python compress.
        // Rust uses explicit booleans; integers don't implicitly act as truth values.

        println!("\n============================\n");
    }

    // ============================================================================
    // 7. Dropwhile: skip_while [std]
    // ============================================================================
    {
        let data = [1, 2, 3, 4, 5, 1, 2];
        println!("{:?}", data.into_iter().skip_while(|&x| x < 4).collect::<Vec<_>>());
        // [4, 5, 1, 2]: once the predicate is false, keep ALL remaining items.

        println!("\n============================\n");
    }

    // ============================================================================
    // 8. Filterfalse: negate the predicate [std]
    // ============================================================================
    {
        let values: Vec<_> = (0..30).filter(|x| !(x % 10 != 0)).collect();
        println!("{values:?}"); // [0, 10, 20]
        // Negation shown to match filterfalse(lambda x: x % 10, ...).
        // In ordinary Rust code, just write |x| x % 10 == 0.

        println!("\n============================\n");
    }

    // ============================================================================
    // 9. Consecutive grouping vs global grouping [itertools]
    // ============================================================================
    {
        let data = [('A', 1), ('A', 2), ('B', 3), ('A', 4)];
        let groups = data.into_iter().chunk_by(|&(key, _)| key);
        for (key, group) in &groups {
            println!("{key}: {:?}", group.collect_vec());
        }
        // A: [('A', 1), ('A', 2)]; B: [('B', 3)]; A: [('A', 4)]
        // `chunk_by` only groups CONSECUTIVE equal keys, like Python groupby.
        // The older itertools .group_by() name is deprecated; use .chunk_by().
        // Keep the grouping object alive and iterate by reference.

        let global = data.into_iter().into_group_map();
        for (key, values) in global.into_iter().sorted_by_key(|(key, _)| *key) {
            println!("global {key}: {values:?}");
        }
        // A: [1, 2, 4]; B: [3]. Collects a HashMap, combining non-adjacent keys.
        // Global grouping allocates; sorting keys here makes display deterministic.

        println!("\n============================\n");
    }

    // ============================================================================
    // 10. Islice: skip / take / step_by [std]
    // ============================================================================
    {
        println!("{:?}", (0..10).skip(2).take(6).step_by(2).collect::<Vec<_>>());
        // [2, 4, 6]: Python islice(data, 2, 8, 2)
        // take(stop - start) BEFORE step_by(step); order changes the result.

        println!("{:?}", ['a', 'b', 'c', 'd'].into_iter().take(2).collect::<Vec<_>>());
        // ['a', 'b']; use skip(2) alone for islice(data, 2, None).

        println!("\n============================\n");
    }

    // ============================================================================
    // 11. Pairwise: array_windows [itertools]
    // ============================================================================
    {
        println!("{:?}", "abcd".chars().array_windows::<2>().collect_vec());
        // [['a', 'b'], ['b', 'c'], ['c', 'd']]

        println!("{:?}", (1..=4).array_windows::<3>().collect_vec());
        // [[1, 2, 3], [2, 3, 4]]
        println!("{:?}", (1..=3).circular_array_windows::<2>().collect_vec());
        // [[1, 2], [2, 3], [3, 1]]: wraps around to the beginning.

        // Array-based methods replace the deprecated tuple-based window methods.
        // Overlapping windows clone items. For an existing slice, standard
        // slice.windows(2) gives borrowed windows without an itertools dependency.

        println!("\n============================\n");
    }

    // ============================================================================
    // 12. Starmap: destructure tuple arguments [std]
    // ============================================================================
    {
        let dimensions = [(1.5, 2.0), (2.0, 3.0), (3.0, 4.0)];
        let areas: Vec<_> = dimensions.into_iter().map(|(h, w)| h * w).collect();
        println!("{areas:?}"); // [3.0, 6.0, 12.0]
        // Rust map passes ONE item; destructure that tuple inside the closure.

        println!("\n============================\n");
    }

    // ============================================================================
    // 13. Takewhile: take_while [std]
    // ============================================================================
    {
        println!("{:?}", [1, 2, 3, 4, 1].into_iter().take_while(|&x| x < 4).collect::<Vec<_>>());
        // [1, 2, 3]: stops at the first false predicate, even if later items qualify.
        // The first failing item is consumed from the underlying iterator.

        println!("\n============================\n");
    }

    // ============================================================================
    // 14. Tee and cloning an iterator [itertools / std]
    // ============================================================================
    {
        let (mut first, second) = (1..=4).tee();
        println!("{:?}", first.next()); // Some(1)
        println!("{:?}", second.collect_vec()); // [1, 2, 3, 4]
        println!("{:?}", first.collect_vec()); // [2, 3, 4]
        // tee creates TWO branches and buffers items when one gets ahead.
        // Large differences in progress can require substantial memory.

        // If the iterator implements Clone, cloning is usually simpler / cheaper:
        let source = 1..=3;
        println!("{:?}", source.clone().collect::<Vec<_>>());
        println!("{:?}", source.collect::<Vec<_>>()); // both [1, 2, 3]

        println!("\n============================\n");
    }

    // ============================================================================
    // 15. Zip longest [itertools]
    // ============================================================================
    {
        use EitherOrBoth::{Both, Left, Right};
        let pairs = [1, 2, 3].into_iter().zip_longest(['a', 'b']);
        for pair in pairs {
            match pair {
                Both(x, y) => println!("{x}, {y}"),
                Left(x) => println!("{x}, missing"),
                Right(y) => println!("missing, {y}"),
            }
        }
        // 1, a;
        // 2, b;
        // 3, missing. The enum represents which side is absent.
        // Two input iterators; unlike Python, there is no automatic None fill.

        let filled = [1, 2, 3].into_iter().zip_longest([10]).map(|pair| match pair {
            Both(x, y) => (x, y), Left(x) => (x, 0), Right(y) => (0, y),
        }).collect_vec();
        println!("{filled:?}"); // [(1, 10), (2, 0), (3, 0)]

        println!("\n============================\n");
    }

    // ============================================================================
    // 16. Cartesian products [itertools]
    // ============================================================================
    {
        println!("{:?}", [1, 2].into_iter().cartesian_product(['a', 'b']).collect_vec());
        // [(1, 'a'), (1, 'b'), (2, 'a'), (2, 'b')]

        let triples = iproduct!([1, 2], ['a', 'b'], [true, false]).collect_vec();
        println!("{triples:?}"); // 8 tuples; rightmost input advances fastest
        // iproduct! supports different item types in the different tuple positions.

        let repeated = iproduct!([1, 2], ['a', 'b'], [1, 2], ['a', 'b']);
        println!("repeat=2 count: {}", repeated.count()); // 16
        // Python product(A, B, repeat=2) repeats the WHOLE input sequence A,B,A,B.

        let dynamic = iter::repeat([0, 1]).take(3).multi_cartesian_product().collect_vec();
        println!("{dynamic:?}"); // all 8 length-three binary vectors
        // multi_cartesian_product handles a runtime number of same-item-type inputs.
        // Product sizes multiply: avoid collecting huge result spaces.

        println!("\n============================\n");
    }

    // ============================================================================
    // 17. Permutations [itertools]
    // ============================================================================
    {
        let letters = ['A', 'B', 'C'];
        println!("{:?}", letters.into_iter().permutations(2).collect_vec());
        // [['A','B'], ['A','C'], ['B','A'], ['B','C'], ['C','A'], ['C','B']]

        println!("all-length count: {}", letters.into_iter().permutations(letters.len()).count()); // 6
        // Rust requires k explicitly. Output items are Vecs, not Python tuples.
        // Positions are distinct: repeated input values may produce duplicate results.

        println!("\n============================\n");
    }

    // ============================================================================
    // 18. Combinations [itertools]
    // ============================================================================
    {
        println!("{:?}", ['A', 'B', 'C'].into_iter().combinations(2).collect_vec());
        // [['A', 'B'], ['A', 'C'], ['B', 'C']]

        let pairs: Vec<[char; 2]> = ['A', 'B', 'C'].into_iter().array_combinations().collect();
        println!("{pairs:?}"); // [['A', 'B'], ['A', 'C'], ['B', 'C']]
        // combinations(k) -> Vec per result; array_combinations::<N>() -> [Item; N].
        // The Vec<[char; 2]> annotation above lets Rust infer N = 2.
        // Order of selection doesn't create another result: AB and BA are one choice.
        println!("\n============================\n");
    }

    // ============================================================================
    // 19. Combinations with replacement [itertools]
    // ============================================================================
    {
        println!("{:?}", [0, 1, 2].into_iter().combinations_with_replacement(2).collect_vec());
        // [[0, 0], [0, 1], [0, 2], [1, 1], [1, 2], [2, 2]]

        println!("{:?}", [0, 1].into_iter().array_combinations_with_replacement::<2>().collect_vec());
        // [[0, 0], [0, 1], [1, 1]]: fixed-size array output.

        // The same input position may be selected repeatedly; order stays irrelevant.
        // Both permutation and combination result counts can grow very quickly.

        println!("\n============================\n");
    }

    // ============================================================================
    // 20. Frequency counting: counts / counts_by [itertools]
    // ============================================================================
    {
        let counts = "mississippi".chars().counts();
        println!("{:?}", counts.into_iter().sorted_by_key(|&(c, _)| c).collect_vec());
        // [('i', 4), ('m', 1), ('p', 2), ('s', 4)]

        let fruits = ["apple", "banana", "apple", "orange", "banana", "apple"];
        let counts = fruits.into_iter().counts();
        println!("apple: {}", counts["apple"]); // 3
        println!("grape: {}", counts.get("grape").copied().unwrap_or(0)); // 0
        // counts["grape"] would PANIC; this is a HashMap, not a Counter object.
        // Counts use usize; missing-key zeros and negative counts aren't built in.

        let ranked = counts.into_iter().sorted_by_key(|&(word, n)| (Reverse(n), word)).collect_vec();
        println!("{ranked:?}"); // [("apple", 3), ("banana", 2), ("orange", 1)]
        // Descending frequency, then alphabetical tie-break. HashMap order itself
        // is unspecified; Python Counter uses first-seen order for tied counts.

        println!("top two: {:?}", ranked.iter().take(2).collect_vec());
        let lengths = ["cat", "dog", "bird"].into_iter().counts_by(|s| s.chars().count());
        println!("{:?}", lengths.into_iter().sorted_by_key(|&(n, _)| n).collect_vec());
        // [(3, 2), (4, 1)]: count items sharing a derived key, not their exact value.
        // Word counts: text.split_whitespace().counts(); punctuation stays attached.
        // Don't confuse counts() (frequency map) with count() (total item count).
        println!("\n============================\n");
    }

    // ============================================================================
    // 21. Collection and display
    // ============================================================================
    {
        println!("{:?}", (1..=3).collect_vec()); // [1, 2, 3]
        println!("{}", (1..=3).join(" - ")); // 1 - 2 - 3
        println!("[{}]", (1..=3).format(", ")); // [1, 2, 3]
        // collect_vec = collect::<Vec<_>>(). join formats Display items into String.
        // format returns a display adapter, avoiding an intermediate joined String.

        println!("\n============================\n");
    }

    // ============================================================================
    // 22. Sorting and selecting the smallest k
    // ============================================================================
    {
        println!("{:?}", [4, 1, 3, 2].into_iter().sorted().collect_vec()); // [1, 2, 3, 4]

        let words = ["pear", "fig", "banana"];
        println!("{:?}", words.into_iter().sorted_by_key(|s| s.len()).collect_vec());
        // ["fig", "pear", "banana"]

        println!("{:?}", [7, 1, 5, 2, 9].into_iter().k_smallest(3).collect_vec()); // [1, 2, 5]
        // sorted buffers ALL items before yielding; use finite inputs.
        // k_smallest still reads the entire input, but needn't retain all its items.
        // k_largest is the corresponding largest-k operation.

        println!("\n============================\n");
    }

    // ============================================================================
    // 23. Global uniqueness vs consecutive deduplication
    // ============================================================================
    {
        let data = [1, 1, 2, 1, 3, 3];
        println!("{:?}", data.into_iter().unique().collect_vec()); // [1, 2, 3]
        println!("{:?}", data.into_iter().dedup().collect_vec()); // [1, 2, 1, 3]
        println!("{:?}", data.into_iter().duplicates().collect_vec()); // [1, 3]

        println!("{:?}", data.into_iter().dedup_with_count().collect_vec());
        // [(2, 1), (1, 2), (1, 1), (2, 3)]: (run length, value)

        println!("{:?}", ["cat", "dog", "bird"].into_iter().unique_by(|s| s.len()).collect_vec());
        // ["cat", "bird"]: first item per derived key
        // unique stores seen keys; dedup only compares adjacent items.
        // duplicates yields each duplicated value once, when its repeat is detected.

        println!("\n============================\n");
    }

    // ============================================================================
    // 24. Global grouping by a derived key
    // ============================================================================
    {
        let by_length = ["cat", "bird", "dog"].into_iter().into_group_map_by(|s| s.len());
        println!("{:?}", by_length.into_iter().sorted_by_key(|&(n, _)| n).collect_vec());
        // [(3, ["cat", "dog"]), (4, ["bird"])]
        // Non-adjacent keys combine. Output is a HashMap<Key, Vec<Item>>.
        // HashMap iteration order is unspecified; examples sort entries for display.

        println!("\n============================\n");
    }

    // ============================================================================
    // 25. Aggregate values directly per group
    // ============================================================================
    {
        let totals = [("A", 10), ("B", 5), ("A", 7)].into_iter()
            .into_grouping_map().sum();
        println!("{:?}", totals.into_iter().sorted_by_key(|&(key, _)| key).collect_vec());
        // [("A", 17), ("B", 5)]
        // Unlike into_group_map, this doesn't build a Vec of values for every key.
        // GroupingMap also offers min / max / fold / reduce for other aggregations.

        println!("\n============================\n");
    }

    // ============================================================================
    // 26. Fixed-size batches: chunks
    // ============================================================================
    {
        let batches = (1..=7).chunks(3);
        for batch in &batches {
            println!("{:?}", batch.collect_vec());
        }
        // [1, 2, 3], [4, 5, 6], [7]; final batch can be shorter.
        // chunks is iterable by reference, like chunk_by. Size must be > 0.
        // This works on iterators; slice.chunks() in std works on existing slices.

        println!("\n============================\n");
    }

    // ============================================================================
    // 27. Fixed-size collection
    // ============================================================================
    {
        let mut source = 1..=5;
        println!("{:?}", source.next_array::<3>()); // Some([1, 2, 3])
        println!("{:?}", source.collect_vec()); // [4, 5]

        println!("{:?}", (1..=3).collect_array::<3>()); // Some([1, 2, 3])
        println!("{:?}", (1..=4).collect_array::<3>()); // None: too many items
        // next_array takes the next N items; collect_array requires EXACTLY N total.
        // If next_array has too few items, it consumes those items and returns None.

        println!("\n============================\n");
    }

    // ============================================================================
    // 28. All subsets: powerset
    // ============================================================================
    {
        println!("{:?}", ['A', 'B'].into_iter().powerset().collect_vec());
        // [[], ['A'], ['B'], ['A', 'B']]
        // Includes the empty and full subsets. n items produce 2^n subsets.

        println!("\n============================\n");
    }

    // ============================================================================
    // 29. Interleave and intersperse
    // ============================================================================
    {
        println!("{:?}", [1, 2, 3].into_iter().interleave([10, 20]).collect_vec());
        // [1, 10, 2, 20, 3]: keep leftovers after one side ends
        // Explicit trait call avoids conflicts if std offers the same method name.

        println!("{:?}", Itertools::intersperse([1, 2, 3].into_iter(), 0).collect_vec());
        // [1, 0, 2, 0, 3]: insert only BETWEEN items

        println!("\n============================\n");
    }

    // ============================================================================
    // 30. Merge sorted streams and compare keys
    // ============================================================================
    {
        println!("{:?}", [1, 3, 5].into_iter().merge([2, 4, 6]).collect_vec());
        // [1, 2, 3, 4, 5, 6]; each input must already be sorted.

        println!("{:?}", [[1, 4], [2, 5], [3, 6]].into_iter().kmerge().collect_vec());
        // [1, 2, 3, 4, 5, 6]: merge several sorted inputs

        let compared = [1, 3, 5].into_iter().merge_join_by([2, 3, 4], |a, b| a.cmp(b));
        println!("{:?}", compared.collect_vec());
        // [Left(1), Right(2), Both(3, 3), Right(4), Left(5)]
        // merge_join_by returns EitherOrBoth; Both identifies equal keys.
        // Duplicate keys are paired as encountered, not a Cartesian SQL-style join.

        println!("\n============================\n");
    }

    // ============================================================================
    // 31. Partition and transform in one pass
    // ============================================================================
    {
        let (even, odd): (Vec<i32>, Vec<String>) = (1..=5).partition_map(|n| {
            if n % 2 == 0 { Either::Left(n) } else { Either::Right(format!("odd:{n}")) }
        });
        println!("{even:?}; {odd:?}"); // [2, 4]; ["odd:1", "odd:3", "odd:5"]
        // Unlike std partition, the two output collections can contain different types.

        println!("\n============================\n");
    }

    // ============================================================================
    // 32. Multiple iterators and columns
    // ============================================================================
    {
        let rows = izip!([1, 2], ['a', 'b'], [true, false]).collect_vec();
        println!("{rows:?}"); // [(1, 'a', true), (2, 'b', false)]
        // izip! returns flat tuples and stops at the shortest input.

        let (ids, letters, flags): (Vec<_>, Vec<_>, Vec<_>) = rows.into_iter().multiunzip();
        println!("{ids:?}; {letters:?}; {flags:?}"); // [1, 2]; ['a', 'b']; [true, false]

        println!("\n============================\n");
    }

    // ============================================================================
    // 33. Min/max and cardinality validation
    // ============================================================================
    {
        println!("{:?}", [4, 1, 9, 2].into_iter().minmax()); // MinMax(1, 9)
        // MinMaxResult also has NoElements and OneElement(value) variants.

        println!("{:?}", [42].into_iter().exactly_one()); // Ok(42)
        println!("{}", [1, 2].into_iter().exactly_one().is_err()); // true

        let empty: [i32; 0] = [];
        println!("{:?}", empty.into_iter().at_most_one()); // Ok(None)
        // exactly_one rejects both zero and multiple items; at_most_one permits zero.

        println!("\n============================\n");
    }

    // ============================================================================
    // 34. Result-aware operations
    // ============================================================================
    {
        let values: [Result<i32, &str>; 3] = [Ok(1), Err("bad"), Ok(3)];
        println!("{:?}", values.into_iter().map_ok(|n| n * 10).collect_vec());
        // [Ok(10), Err("bad"), Ok(30)]: errors are preserved

        let (good, bad): (Vec<_>, Vec<_>) = values.into_iter().partition_result();
        println!("{good:?}; {bad:?}"); // [1, 3]; ["bad"] (visits ALL items)

        let parsed: Result<Vec<i32>, _> = ["1", "2", "oops"].into_iter()
            .map(str::parse::<i32>).try_collect();
        println!("{}", parsed.is_err()); // true: stops at first error
        // try_collect is convenient syntax for collect::<Result<Vec<_>, _>>().

        println!("\n============================\n");
    }

    // ============================================================================
    // 35. Merge adjacent items: coalesce
    // ============================================================================
    {
        let runs = [('a', 1), ('a', 2), ('b', 1), ('b', 3)].into_iter()
            .coalesce(|(a, n), (b, m)| {
                if a == b { Ok((a, n + m)) } else { Err(((a, n), (b, m))) }
            }).collect_vec();
        println!("{runs:?}"); // [('a', 3), ('b', 4)]
        // Ok(merged) combines two adjacent items; Err((left,right)) keeps them separate.
        // An Err here is a control signal, not an application failure.

        println!("\n============================\n");
    }

    // ============================================================================
    // 36. Positions, padding, and inclusive stopping
    // ============================================================================
    {
        println!("{:?}", [2, 5, 4, 7].into_iter().positions(|n| n % 2 == 0).collect_vec());
        // [0, 2]: ALL matching indices; std position() finds only the first.
        println!("{:?}", [10, 20].into_iter().pad_using(5, |index| index as i32).collect_vec());
        // [10, 20, 2, 3, 4]: extend to at least 5 items; doesn't truncate longer input.
        println!("{:?}", (1..=6).take_while_inclusive(|&n| n < 4).collect_vec());
        // [1, 2, 3, 4]: includes the first failing item, unlike std take_while().

        println!("\n============================\n");
    }
}

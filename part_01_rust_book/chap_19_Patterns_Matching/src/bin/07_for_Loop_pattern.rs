// In a for loop, the value that directly follows the keyword for is a pattern

fn main() {
    println!();

    let v = vec!['a', 'b', 'c'];

    for (index, value) in v.iter().enumerate() {
        println!("{value} is at index {index}");
    }
}

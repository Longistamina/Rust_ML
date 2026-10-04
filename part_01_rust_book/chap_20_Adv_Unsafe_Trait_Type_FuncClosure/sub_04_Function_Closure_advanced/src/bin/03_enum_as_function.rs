/*
Enum variants can also act as a function,
can be passed as argument of other functions.
*/

#[derive(Debug)]
#[allow(dead_code)]
enum Status {
    Value(u32),
    Stop
}

fn main() {
    println!();

    let list_of_statuses: Vec<Status> = (0u32..20)
        .map(Status::Value) // pass variant `Status::Value` as function of `map`
        .collect();

    println!("statuses = {:?}", list_of_statuses)
}
/*
statuses = [Value(0), Value(1), Value(2), Value(3), Value(4), Value(5), Value(6), Value(7), Value(8), Value(9),
Value(10), Value(11), Value(12), Value(13), Value(14), Value(15), Value(16), Value(17), Value(18), Value(19)]
*/

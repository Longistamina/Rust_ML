// ---------------------------------------------------- //
// ------------------ loop infinite ------------------- //
// ---------------------------------------------------- //

fn _demo_loop_infinite() { // Use "_" before function name to avoid compiler's warning
    loop { // "loop" key word is used to declare an infinite loop
        println!("Again!")
    }
    // To stop this loop -> use Cltr+C (keyboard interupt)
}

// -------------------------------------------- //
// ------------------ break ------------------- //
// -------------------------------------------- //

fn _demo_break() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter*2 // Breaks the loop if the condition is met, 
        }                   // then return the value = counter*2

    };

    println!("The result is: {result}")
}


// -------------------------------------------------------------- //
// ------------------ nested loops and Labels ------------------- //
// -------------------------------------------------------------- //

// Loop can be nested to handle complicated logic. However, this can affect readability
// => Use labeled loop to enhance readibility

fn _demo_nested_loop_with_labels() {
    let mut count = 0;
    
    'counting_up: loop { // use single quote ' to name the outer loop with a label
        println!("Count = {count}");
        let mut remaining = 10;

        loop { // inner loop
            println!("Remaining = {remaining}");

            if remaining == 9 {
                break; // Break the inner loop
            }

            if count == 2 {
                break 'counting_up; // Break the outer loop using its label
            }

            remaining -= 1;
        }

        count += 1;
        
        // Prints "=" 20 times
        let line = "=".repeat(20);
        println!("{line}")
    }

// Count = 0
// Remaining = 10
// Remaining = 9
// ====================
// Count = 1
// Remaining = 10
// Remaining = 9
// ====================
// Count = 2
// Remaining = 10
}


// --------------------------------------------------------------------- //
// ------------------ while loop with exit condition ------------------- //
// --------------------------------------------------------------------- //

fn _demo_while() {
    let mut number = 5;

    while number != 0 { // While the condition is true, the loop runs. 
        println!("number = {number}");
        number -= 1;
    }

    println!("While loop done!")
}


// ---------------------------------------------------------------- //
// ------------------ for loop with collections ------------------- //
// ---------------------------------------------------------------- //

fn _demo_for() {
    let arr = [10, 20, 30, 40, 50];

    for element in arr {
        println!("current element = {element}")
    }

    println!("For loop done!")
}

fn _demo_for_reverse() {
    for number in (1..6).rev() {
        println!("current number = {number}")
    }

    println!("For loop reversed done!")
}


// ----------------------------------------------- //
// ------------------ continue ------------------- //
// ----------------------------------------------- //

#[allow(unreachable_code)] // This silences the warning for the unreachable codes
fn _demo_continue() {
    for number in 1..=10 {
        if (number % 2).eq(&0) {
            println!("Even number = {number}");
            continue
        }
        else {continue}

        println!("The code reached here")
    }
}


// ------------------------------------------- //
// ------------------ pass ------------------- //
// ------------------------------------------- //

fn _demo_pass() {
    for number in 1..=10 {
        if (number % 2).eq(&0) {
            println!("Even number = {number}");
            continue
        }
        else {} // Just use empty scope to "pass" (like in python)

        println!("The code reached here")
    }
}

/////////////////////////////////
///          main()           ///
/////////////////////////////////

fn main() {
    // _demo_loop_infinite();
    // _demo_break();
    // _demo_nested_loop_with_labels();
    // _demo_while();
    // _demo_for_reverse();
    // _demo_continue();
    _demo_pass();
}
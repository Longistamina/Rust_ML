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

fn demo_break() {
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

fn demo_nested_loop_with_labels() {
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

fn main() {
    // _demo_loop_infinite();
    demo_break();
    demo_nested_loop_with_labels();
}
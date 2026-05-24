/*
 * ``match`` is a control flow construct that allows comparing a value against a series of patterns,
 * then execute codes based on the comparison result.
 *
 * match something {
 *      pattern1 arm => code1 arm,
 *      pattern2 arm => code2 arm
 * }
 *
 * ``match`` can also be used with ``Option<T>`` to get value ``T`` out of ``Some(T)``
 *
 * NOTE: when use ``match``, must cover all the cases.
 * Can use ``_`` place holder to represent remaining cases.
 */

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska
}

#[derive(Debug)]
 enum Coin {
     Penny,
     Nickel,
     Dime,
     Quarter(UsState) // ``Quarter`` is a pattern that binds to values (quarter coins have states' flag on their surface)
 }

 fn value_in_cents(coin: Coin) -> u8 {
     match coin {
        Coin::Penny => 1, // ``Coin::Penny`` is the pattern arm, ``1`` is the code arm
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("Quarter from state {state:?}");
            25
            }
    }
 }

// ################ //
//      main()      //
// ################ //

fn main() {
    println!();

    // ------------------------------------------------------------------ //
    // -------------------------- match and Enum ------------------------ //
    // ------------------------------------------------------------------ //

    let penny = Coin::Penny;
    let penny_cent = value_in_cents(penny);
    println!("Value of a penny = {penny_cent} cent(s)");

    let nickel_cent = value_in_cents(Coin::Nickel);
    println!("Value of a nickel = {nickel_cent} cent(s)");

    let dime_cent = value_in_cents(Coin::Dime);
    println!("Value of a dime = {dime_cent} cent(s)");

    println!();

    let quarter_alabama = Coin::Quarter(UsState::Alabama);
    let quarter_cent = value_in_cents(quarter_alabama);
    println!("Value of a quarter = {quarter_cent} cent(s)");

    println!();

    let quarter_cent = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("Value of a quarter = {quarter_cent} cent(s)");

    println!("\n=================================================================\n");

    // ----------------------------------------------------------------------------------------------- //
    // -------------------------- match and Option<T> to get T out of Some(T) ------------------------ //
    // ----------------------------------------------------------------------------------------------- //

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(number) => Some(number + 1)
        }
    }

        let five = Some(5);
        let six = plus_one(five);
        println!("six = {six:?}");

        let something = plus_one(None);
        println!("something = {something:?}");

        println!("\n=================================================================\n");

    // ----------------------------------------------------------------------------------------------- //
    // -------------------------- match and Option<T> to get T out of Some(T) ------------------------ //
    // ----------------------------------------------------------------------------------------------- //

    // let x = Some(3.2);

    // let result = match x {
    //     Some(3.2) => 3.2
    // };

    // println!("{}", result)

    /*
     * This part will cause compiler panic because the ``match`` does not cover all possibilities (not include None)
     */

     // ---------------------------------------------------------------------------------------------------- //
     // -------------------------- Catch the remaining cases and ``_`` place holder ------------------------ //
     // ---------------------------------------------------------------------------------------------------- //

     let dice_roll = 9;

     match dice_roll {
         3 => println!("Roll = 3"),
         7 => println!("Roll = 7"),
         other => println!("Other = {other}") // ``other`` variable is used to capture remaining possibilites
     }

     /////////////////////////

     let score = 25;

     match score {
         9 => println!{"Good score!"},
         8 => println!("Nice score!"),
         remain => println!("Unknow score: {remain}") // ``remain`` variable is used to capture remaining possibilites
     }

     /////////////////////////

     let choice = 0;

     match choice {
         0 => println!("No"),
         1 => println!("Yes"),
         _ => {
             println!("Invalid option.");
             repeat_something()
         }
         // ``_`` is a placeholder capturing remaining possiblities, but don't need use as input for other functions
     }

     fn repeat_something() {}

     /////////////////////////

     let switch = "On";

     match switch {
         "On" => turn_on(),
         "Off" => turn_off(),
         _ => {} // This means do nothing, return unit type ``()``
     }

     fn turn_on() {println!("Turned on!")}
     fn turn_off() {println!("Turned off")}

     /////////////////////////

     let customer = "He Who Remains";

     match customer {
         "VIP" => {println!("You are granted special offers!!!")},
         "Normal" => {println!("You have no any offer!!!")},
         _ => () // return unit type ``()``
     }

}

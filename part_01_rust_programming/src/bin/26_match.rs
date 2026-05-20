/*
 * ``match`` is a control flow construct that allows comparing a value against a series of patterns,
 * then execute codes based on the comparison result.
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
     Quarter(UsState) // quarter coins have states' flag on their surface
 }

 fn value_in_cents(coin: Coin) -> u8 {
     match coin {
     Coin::Penny => 1,
     Coin::Nickel => 5,
     Coin::Dime => 10,
     Coin::Quarter(state) => {
         println!("Quarter from state {state:?}");
         25
        }
    }
 }

fn main() {
    println!();

    let penny = Coin::Penny;
    let penny_cent = value_in_cents(penny);
    println!("Value of a penny = {penny_cent} cent(s)")

}

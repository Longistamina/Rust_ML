use rand::Rng; // must use rand = "0.8.5" in Cargo.toml for this to work

fn main() {
    println!("Let's generate a random number!");
    
    let random_number = rand::thread_rng().gen_range(1..=100); // must use rand = "0.8.5" in Cargo.toml for this to work

    println!("The generated random number is: {random_number}")
}
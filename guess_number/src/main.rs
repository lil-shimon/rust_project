use std::io;
use rand::Rng;

fn main() {
    let mut input_number = String::new();

    // create random number
    let random_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess number game");
    println!("input your guess");

    // get player number
    io::stdin().read_line(&mut input_number).expect("error");

    // check player number is collect or not

    println!("{}", random_number);
    println!("Your guess is: {}", input_number);
}

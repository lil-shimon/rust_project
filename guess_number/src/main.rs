use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // create random number
    let random_number = rand::thread_rng().gen_range(1..=100);
    loop {
        let mut input_number = String::new();

        println!("Guess number game");
        println!("input your guess");

        // get player number
        io::stdin().read_line(&mut input_number).expect("error");

        // input_number convert string to number (u32)
        // same variable name = shadowing
        let input_number: u32 = input_number.trim().parse().expect("plz input number");

        println!("Your guess is: {}", input_number);

        // check player number is collect or not
        match input_number.cmp(&random_number) {
            Ordering::Greater => println!("TOO BIG!!!"),
            Ordering::Less => println!("too small..."),
            Ordering::Equal => {
                println!("collect!!!");
                break;
            }
        }
    }
}

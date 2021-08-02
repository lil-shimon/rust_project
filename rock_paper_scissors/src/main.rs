use std::io;
use rand::{Rng, thread_rng};

fn main() {

    let win = "win";
    let lose = "lose";
    let draw = "draw";

    let options = ["rock", "paper", "scissors"];

    loop {

        // player_hand
        let mut player_input = String::new();

        // computer hand is generated random
        let mut computer_hand = pick_random(&options);

        println!("enter your hand plz: Type rock, paper, scissors");

        if player_input == "rock" || player_input == "paper" || player_input == "scissors" {
            println!("invalid input");
            break;
        }

        // handle standard input
        // read_line is one of function of str::io
        // return [&mut String]
        // expect is panic func
        io::stdin().read_line(&mut player_input).expect("failed");

        println!("your hand: {}", player_input);
        println!("computer hand: {}", computer_hand);
    }
}

fn pick_random(options: &[&str]) -> String {
    // Returns a random member from an array of strs
    let mut rng = thread_rng();
    let index = rng.gen_range(0..options.len());
    return String::from(options[index]);
}

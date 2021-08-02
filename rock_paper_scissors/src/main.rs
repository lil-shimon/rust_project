use rand::{thread_rng, Rng};
use std::io;

fn main() {
    let win = "win";
    let lose = "lose";
    let draw = "draw";

    let options = ["rock", "paper", "scissors"];

    loop {
        // player_hand
        let mut player_input = String::new();

        // computer hand is random
        let mut computer_hand = pick_random(&options);

        println!("enter your hand plz: Type rock, paper, scissors");
        // handle standard input
        // read_line is one of function of str::io
        // return [&mut String]
        // expect is panic func
        io::stdin().read_line(&mut player_input).expect("failed");

        let player_hand = trim_space(&player_input);
        // check player_hand is valid or not
        if player_hand == "rock" || player_hand == "paper" || player_hand == "scissors" {
            println!("your hand: {}", player_input);
            println!("computer hand: {}", computer_hand);
            continue;
        } else {
            println!("invalid input");
            break;
        }
    }
}

// remove white space
fn trim_space (input: &String)-> String {
    return input.trim().to_string();
}

fn pick_random(options: &[&str]) -> String {
    // Returns a random member from an array of strs
    let mut rng = thread_rng();
    let index = rng.gen_range(0..options.len());
    return String::from(options[index]);
}

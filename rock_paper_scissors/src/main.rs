use rand::{thread_rng, Rng};
use std::io;

// 'static --> data will not disappear until the end of our program
static WIN: &'static str = "win";
static LOSE: &'static str = "lose";
static DRAW: &'static str = "draw";

fn main() {
    // enum is better
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

        // remove all white space
        let player_hand = trim_space(&player_input);

        // check player_hand is valid or not
        if player_hand == "rock" || player_hand == "paper" || player_hand == "scissors" {
            println!("{}", compare_hand(&player_hand, &computer_hand));
        } else {
            println!("invalid input");
            break;
        }
        // player_hand is dropped here
        // computer_hand is dropped here
    }
}

// remove white space
fn trim_space(input: &String) -> String {
    return input.trim().to_string();
}

fn pick_random(options: &[&str]) -> String {
    // Returns a random member from an array of strs
    let mut rng = thread_rng();
    let index = rng.gen_range(0..options.len());
    return String::from(options[index]);
}

// compare hands
// return String result
fn compare_hand(p: &String, c: &String) -> String {
    println!("your hand: {}", p);
    println!("computer hand: {}", c);
    return if p == c {
        DRAW.to_string()
    } else if p == "rock" || c == "paper" {
        LOSE.to_string()
    } else if p == "rock" || c == "scissors" {
        WIN.to_string()
    } else if p == "paper" || c == "scissors" {
        LOSE.to_string()
    } else if p == "paper" || c == "rock" {
        WIN.to_string()
    } else if p == "scissors" || c == "rock" {
        LOSE.to_string()
    } else {
        WIN.to_string()
    };
    // p and c's ownership is drop here
}

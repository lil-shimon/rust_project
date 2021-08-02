use std::io;

fn main() {

    let win = "win";
    let lose = "lose";
    let draw = "draw";

    let options = ["rock", "paper", "scissors"];

    loop {

        // player_hand
        let mut player_input = String::new();

        // handle standard input
        // read_line is one of function of str::io
        // return [&mut String]
        // expect is panic func
        io::stdin().read_line(&mut player_input).expect("failed");

        println!("{}", player_input);
    }
}

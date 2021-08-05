mod game;

const PLAYER: game::Player = game::Player::X;
const COM: game::Player = game::Player::O;
const START_USER: game::Player = game::Player::X;

fn main() {
    println!("{} {} {}", PLAYER, COM, START_USER);
}


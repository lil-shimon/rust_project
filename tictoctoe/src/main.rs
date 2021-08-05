mod game;

const PLAYER: game::Player = game::Player::X;
const COM: game::Player = game::Player::O;
const START_USER: game::Player = game::Player::X;

fn main() {
    /* constructor */
    let mut board = game::Board::origin();

    println!("{} {} {}", PLAYER, COM, START_USER);
    println!("{}", board);
}

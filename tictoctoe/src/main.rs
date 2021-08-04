fn main() {
    // state definition ( Board )

    // Player definition
    // x => you
    // o => computer
    pub enum Player {
        x,
        o,
    }

    // board structure
    // include board          [Vec<Vec>] : 2d vec
    //         current_player [String]   : current_player state
    pub struct Board {
        board: Vec<Vec<char>>,
        current_player: Player,
    }

    // score definition

    // move func

    // check func  (is win, lose)
}

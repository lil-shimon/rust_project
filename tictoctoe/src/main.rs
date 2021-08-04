fn main() {
    // state definition ( Board )

    // Player definition
    // X => you
    // O => computer
    #[derive(Clone, Copy, Debug, PartialEq)]
    pub enum Player {
        X,
        O,
    }

    // single part of board
    // Player X , O and None is available
    #[derive(Clone, Copy)]
    pub struct Square {
        player: Option<Player>,
    }

    // board structure
    // include board          [Vec<Vec>] : 2d vec
    //         current_player [String]   : current_player state
    pub struct Board {
        board: Vec<Vec<Square>>,
        current_player: Player,
    }

    impl Board {
        pub fn origin() -> Board {
            let default = Square { player: None };
            return Board {
                board: vec![vec![default; 3]; 3],
                current_player: Player::X,
            };
        }
    }

    // score definition

    // move func

    // check func  (is win, lose)
}

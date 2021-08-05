use std::fmt;
use std::vec::Vec;

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

    // Movement structure
    pub struct Move {
        pub player: Player,
        pub loc: (usize, usize),
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

    impl fmt::Display for Square {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self.player {
                Some(x) if x == Player::X => write!(f, "{}", x)?,
                Some(o) => write!(f, "{}", o)?,
                None => write!(f, " ")?,
            };
            Ok(())
        }
    }

    impl fmt::Display for Board {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            for row in &self.board {
                writeln!(f, "----------")?;
                write!(f, "|")?;
                for val in row {
                    write!(f, " {} |", val)?;
                }
                writeln!(f, "")?;
            }
            writeln!(f, "----------")?;
            Ok(())
        }
    }

    impl fmt::Display for Player {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Player::X => write!(f, "X")?,
                Player::O => write!(f, "O")?,
            }
            Ok(())
        }
    }
    // score definition

    // move func

    // check func  (is win, lose)
}

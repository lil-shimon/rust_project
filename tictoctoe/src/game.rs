use std::fmt;
use std::vec::Vec;

// state definition ( Board ) // Player definition X => you
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
    pub board: Vec<Vec<Square>>,
    pub current_player: Player,
}

impl Board {

    /** constructor */
    pub fn origin() -> Board {
        let default = Square { player: None };
        return Board {
            board: vec![vec![default; 3]; 3],
            current_player: Player::X,
        };
    }

    /** switch player */
    pub fn next_player (&self) -> Player {
        let next = if self.current_player == Player::X { Player::O } else { Player::X };
        return next;
    }

    /** update board state and switch current_player
     *  return -> updated Board struct
     */
    pub fn update_board(&mut self) -> Board {
        println!("prev player is: {}", self.current_player);
        let next = self.next_player();
        println!("next player is: {}", next);
        let moved = &self.board;
        return Board {
            board: moved.to_vec(),
            current_player: next,
        };
    }
}

/** each board */
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

/** Board */
impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in &self.board {
            writeln!(f, "-------------")?;
            write!(f, "|")?;
            for val in row {
                write!(f, " {} |", val)?;
            }
            writeln!(f, "")?;
        }
        writeln!(f, "-------------")?;
        Ok(())
    }
}

/** player */
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

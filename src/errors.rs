use game::Player;
use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum TicTacError {
    CellAlreadyOccupied(usize, Player),
}

impl fmt::Display for TicTacError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TicTacError::CellAlreadyOccupied(ref index, ref player) => {
                write!(f, "Cell {} already occupied by player {}", index, player)
            }
        }
    }
}

impl Error for TicTacError {
    fn description(&self) -> &str {
        match *self {
            TicTacError::CellAlreadyOccupied(_, _) => "cell already occupied",
        }
    }
}

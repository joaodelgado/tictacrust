use std::io;
use std::io::Write;
use std::fmt;
use errors::TicTacError;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Player {
    X,
    O,
}

impl Player {
    pub fn swap(&self) -> Player {
        match *self {
            Player::X => Player::O,
            Player::O => Player::X,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub owner: Option<Player>,
}

pub struct Board {
    pub cells: [Cell; 3 * 3],
    pub current_player: Player,
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match *self {
            Player::X => write!(f, "X"),
            Player::O => write!(f, "O"),
        }
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self.owner {
            Some(player) => write!(f, "{:?}", player),
            None => write!(f, " "),
        }
    }
}

impl Board {
    pub fn new(player: Player) -> Board {
        Board {
            cells: [Cell { owner: None }; 3 * 3],
            current_player: player,
        }
    }

    pub fn prompt(&mut self) {
        print!("Player {} -- enter your move: ", self.current_player);
        let _ = io::stdout().flush();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => {
                println!("Error reading your move. Please try again");
                return;
            }
        }

        let position = match self.read_move(input) {
            Some(play) => play,
            None => return,
        };

        match self.play(position) {
            Err(_) => {
                println!("Cell already occupied!");
                return;
            }
            Ok(_) => {}
        }
    }

    fn read_move(&self, input: String) -> Option<usize> {
        let index = match input.trim().parse::<usize>() {
            Ok(n) => n - 1,
            Err(_) => {
                println!(
                    "Your move must be a valid number between 1 and 9. Received: '{}'",
                    input.trim()
                );
                return None;
            }
        };

        if index > 8 {
            println!("Your move must be a valid number between 1 and 9.");
            return None;
        }

        Some(index)
    }

    pub fn play(&mut self, play: usize) -> Result<(), TicTacError> {
        match self.cells[play].owner {
            None => {
                self.cells[play].owner = Some(self.current_player);
                self.current_player = self.current_player.swap();
                Ok(())
            }
            Some(owner) => Err(TicTacError::CellAlreadyOccupied(play, owner)),
        }
    }

    pub fn is_over(&self) -> bool {
        match self.winner() {
            Some(_) => true,
            None => !self.cells.iter().any(|c| c.owner == None),
        }
    }

    pub fn winner(&self) -> Option<Player> {
        // Check rows
        for row in self.cells.chunks(3) {
            match check_line(row) {
                None => continue,
                result => return result,
            }
        }

        // Check columns
        for c in 0..3 {
            let column = [self.cells[c], self.cells[c + 3], self.cells[c + 6]];
            match check_line(&column) {
                None => continue,
                result => return result,
            }
        }

        // Check positive diagonal
        let positive_diagonal = [self.cells[0], self.cells[4], self.cells[8]];
        match check_line(&positive_diagonal) {
            None => {}
            result => return result,
        }


        // Check negative diagonal
        let negative_diagonal = [self.cells[2], self.cells[4], self.cells[6]];
        match check_line(&negative_diagonal) {
            None => None,
            result => return result,
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(
            f,
            " {} | {} | {}
---+---+---
 {} | {} | {}
---+---+---
 {} | {} | {} ",
            self.cells[6],
            self.cells[7],
            self.cells[8],
            self.cells[3],
            self.cells[4],
            self.cells[5],
            self.cells[0],
            self.cells[1],
            self.cells[2]
        )
    }
}

/// Check if 3 cells are non empty and have the same player.
/// Returns the player that owns all cells, if any.
fn check_line(cells: &[Cell]) -> Option<Player> {
    assert!(cells.len() >= 3);

    match cells[0].owner {
        Some(_) => {
            if cells.iter().all(|&p| p.owner == cells[0].owner) {
                cells[0].owner
            } else {
                None
            }
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_line() {
        let x = Cell { owner: Some(Player::X) };
        let o = Cell { owner: Some(Player::O) };
        let e = Cell { owner: None };

        assert_eq!(check_line(&[x, x, x]), Some(Player::X));
        assert_eq!(check_line(&[o, o, o]), Some(Player::O));
        assert_eq!(check_line(&[o, x, o]), None);
        assert_eq!(check_line(&[o, e, o]), None);
        assert_eq!(check_line(&[e, e, e]), None);
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    #[test]
    fn test_board_winner() {
        let x = Cell { owner: Some(Player::X) };
        let o = Cell { owner: Some(Player::O) };
        let e = Cell { owner: None };

        let empty = Board { cells: [
                e, e, e,
                e, e, e,
                e, e, e
        ], current_player: Player::X};
        let no_winner = Board { cells: [
                x, x, e,
                e, o, e,
                e, e, o
        ], current_player: Player::X};
        let row_x = Board { cells: [
                x, x, x,
                e, o, e,
                e, e, o
        ], current_player: Player::X};
        let column_o = Board { cells: [
                x, o, e,
                x, o, o,
                o, o, x
        ], current_player: Player::X};
        let positive_diagonal_x = Board { cells: [
                e, e, x,
                o, x, o,
                x, o, x
        ], current_player: Player::X};
        let negative_diagonal_o = Board { cells: [
                o, o, e,
                x, o, o,
                x, e, o
        ], current_player: Player::X};

        assert_eq!(empty.winner(), None);
        assert_eq!(no_winner.winner(), None);
        assert_eq!(row_x.winner(), Some(Player::X));
        assert_eq!(column_o.winner(), Some(Player::O));
        assert_eq!(positive_diagonal_x.winner(), Some(Player::X));
        assert_eq!(negative_diagonal_o.winner(), Some(Player::O));
    }

    #[cfg_attr(rustfmt, rustfmt_skip)]
    #[test]
    fn test_board_is_over() {
        let x = Cell { owner: Some(Player::X) };
        let o = Cell { owner: Some(Player::O) };
        let e = Cell { owner: None };

        let empty = Board { cells: [
                e, e, e,
                e, e, e,
                e, e, e
        ], current_player: Player::X};
        let no_winner = Board { cells: [
                x, x, e,
                e, o, e,
                e, e, o
        ], current_player: Player::X};
        let winner = Board { cells: [
                x, x, x,
                e, o, e,
                e, e, o
        ], current_player: Player::X};
        let no_winner_full = Board { cells: [
                x, o, x,
                o, o, x,
                x, x, o
        ], current_player: Player::X};

        assert_eq!(empty.is_over(), false);
        assert_eq!(no_winner.is_over(), false);
        assert_eq!(winner.is_over(), true);
        assert_eq!(no_winner_full.is_over(), true);
    }
}

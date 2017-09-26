mod ai;
mod game;
mod errors;

use ai::Ai;
use game::{Board, Player};

fn main() {
    let mut board = Board::new(Player::X);

    let ai = Ai { player: Player::O };

    while !board.is_over() {
        println!();
        println!("{}", board);
        println!();
        if board.current_player == Player::O {
            println!("AI playing...");
            let m = ai.next_move(&mut board);
            match board.play(m) {
                Ok(()) => {}
                Err(_) => unreachable!(),
            }
        } else {
            board.prompt();
        }
    }

    println!("{}", board);
    match board.winner() {
        Some(player) => println!("Player {} won!", player),
        None => println!("It's a draw!"),
    }
}

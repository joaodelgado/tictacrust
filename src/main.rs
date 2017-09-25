mod game;

use game::Board;

fn main() {
    let mut board = Board::new();

    while !board.is_over() {
        println!("{}", board);
        board.prompt();
    }

    println!("{}", board);
    match board.winner() {
        Some(player) => println!("Player {} won!", player),
        None => println!("It's a draw!"),
    }
}

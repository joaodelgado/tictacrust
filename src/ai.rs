use game::{Board, Player};

pub struct Ai {
    pub player: Player,
}

impl Ai {
    pub fn next_move(&self, board: &mut Board) -> usize {
        if board.is_over() {
            panic!("Evaluating position, but game is over");
        }

        if self.player != board.current_player {
            panic!("Evaluating position, but AI isn't the current player");
        }

        let mut moves: Vec<(usize, isize)> = self.available_moves(board)
            .iter()
            .map(|&m| (m, self.eval_max(board, self.player, m, 0)))
            .collect();
        moves.sort_by(|&(_, a), &(_, b)| a.cmp(&b));

        return moves
            .pop()
            .expect("No available moves, but no game over was detected")
            .0;
    }

    fn eval_max(&self, board: &mut Board, player: Player, play: usize, indent: usize) -> isize {
        assert!(board.cells[play].owner.is_none());

        // Simulate play
        board.cells[play].owner = Some(player);

        let best_score: isize;
        if board.is_over() {
            best_score = match board.winner() {
                Some(player) if self.player == player => 1,
                None => 0,
                _ => -1,
            };
        } else {
            best_score = self.available_moves(board)
                .iter()
                .map(|&m| self.eval_min(board, player.swap(), m, indent + 4))
                .min()
                .expect("No available moves, but no game over was detected");
        }

        // Revert board to the previous state
        board.cells[play].owner = None;

        best_score
    }

    fn eval_min(&self, board: &mut Board, player: Player, play: usize, indent: usize) -> isize {
        assert!(board.cells[play].owner.is_none());

        // Simulate play
        board.cells[play].owner = Some(player);

        let best_score: isize;

        if board.is_over() {
            best_score = match board.winner() {
                Some(player) if self.player == player => 1,
                None => 0,
                _ => -1,
            };

        } else {
            best_score = self.available_moves(board)
                .iter()
                .map(|&m| self.eval_max(board, player.swap(), m, indent + 4))
                .max()
                .expect("No available moves, but no game over was detected");
        }

        // Revert board to the previous state
        board.cells[play].owner = None;
        best_score
    }

    fn available_moves(&self, board: &Board) -> Vec<usize> {
        board
            .cells
            .iter()
            .enumerate()
            .filter(|&(_, &c)| c.owner.is_none())
            .map(|(i, _)| i)
            .collect()
    }
}

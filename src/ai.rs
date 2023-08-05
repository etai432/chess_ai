use crate::chess::Chess;
use std::f32::INFINITY;

#[derive(Debug, Clone)]
pub struct AI {
    depth: i32, //ms
}

impl AI {
    pub fn new(depth: i32) -> Self {
        AI { depth }
    }
    pub fn best_move(&mut self, mut chess: Chess) -> (u8, u8) {
        let mut max = -INFINITY;
        let mut best_move = (64, 64);
        for (from, to) in chess.get_all_moves() {
            let chess_move = chess.move_piece(from, to);
            let eval = -self.search(self.depth - 1, -INFINITY, INFINITY, &mut chess);
            chess.undo_move(chess_move);
            if eval > max {
                max = eval;
                best_move = (from, to);
            }
        }
        best_move
    }
    pub fn search(&self, depth: i32, mut alpha: f32, beta: f32, chess: &mut Chess) -> f32 {
        if depth == 0 {
            return self.eval(chess);
        }
        let moves = chess.get_all_moves();
        if moves.is_empty() {
            if chess.is_check(chess.king_loc()) {
                return -INFINITY;
            }
            return 0.0;
        }
        for (from, to) in moves {
            let chess_move = chess.move_piece(from, to);
            let eval = -self.search(depth - 1, -beta, -alpha, chess);
            chess.undo_move(chess_move);
            if eval >= beta {
                return beta;
            }
            alpha = alpha.max(eval);
        }
        alpha
    }
    pub fn eval(&self, chess: &Chess) -> f32 {
        let mut eval = 0.0;
        for piece in &chess.board {
            eval += piece.evaluate();
        }
        if chess.is_white_turn {
            eval
        } else {
            -eval
        }
    }
    pub fn count_moves(depth: i32, chess: &mut Chess) -> i32 {
        if depth == 0 {
            return 1;
        }
        let mut num = 0;
        let moves = chess.get_all_moves();
        for move1 in moves.into_iter() {
            let chess_move = chess.move_piece(move1.0, move1.1);
            num += AI::count_moves(depth - 1, chess);
            chess.undo_move(chess_move);
        }
        num
    }
}

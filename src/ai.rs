use crate::chess::Chess;
use macroquad::prelude::*;
use std::thread;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct AI {}

impl AI {
    pub fn new() -> Self {
        AI {}
    }
    // pub fn best_move(&mut self) {}
}

pub struct GameManager {
    pub ai: AI,
    pub chess: Chess,
    mouse_pos: usize,
}

impl GameManager {
    pub fn new() -> Self {
        GameManager {
            ai: AI::new(),
            chess: Chess::new(),
            mouse_pos: 0,
        }
    }
    pub fn get_mouse_pos(&mut self) {
        let mouse_pos = mouse_position();
        if mouse_pos.0 as usize / 100 + mouse_pos.1 as usize / 100 * 8 > 63 {
            self.mouse_pos = 63;
        }
        self.mouse_pos = mouse_pos.0 as usize / 100 + mouse_pos.1 as usize / 100 * 8;
    }
    pub async fn player_turn(&mut self) {
        loop {
            self.chess.draw();
            self.chess.draw_moves();
            next_frame().await;
            thread::sleep(Duration::from_millis(100));
            if self.chess.board[self.mouse_pos].is_white() == self.chess.is_white_turn {
                let piece_index = self.mouse_pos;
                self.chess.get_legals(piece_index);
                while !is_mouse_button_pressed(MouseButton::Left) {
                    self.chess.draw();
                    self.chess.draw_moves();
                    next_frame().await;
                }
                self.get_mouse_pos();
                if self.chess.moves.contains(&self.mouse_pos) {
                    self.chess.move_piece(piece_index, self.mouse_pos, true);
                    self.chess.moves = vec![];
                    break;
                }
            } else {
                self.chess.moves = vec![];
                while !is_mouse_button_pressed(MouseButton::Left) {
                    self.chess.draw();
                    self.chess.draw_moves();
                    next_frame().await;
                }
                self.get_mouse_pos();
            }
        }
        self.chess.is_white_turn = !self.chess.is_white_turn;
    }
    // pub fn ai_turn(&mut self) {
    //     //find best move, make the move, possible time limit?
    //     //this function is not related to the graphics but makes the move and returns nothing
    //     self.chess.is_white_turn = !self.chess.is_white_turn;
    // }
}

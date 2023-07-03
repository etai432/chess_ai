use crate::ai::AI;
use crate::chess::{Chess, Piece};
use macroquad::prelude::*;
use std::thread;
use std::time::Duration;

pub struct GameManager {
    ai: AI,
    chess: Chess,
    mouse_pos: usize,
    textures: [Texture2D; 13],
}

impl GameManager {
    pub fn new() -> Self {
        GameManager {
            ai: AI::new(),
            chess: Chess::new(),
            mouse_pos: 0,
            textures: [
                Texture2D::from_file_with_format(include_bytes!(r".\images\board.png"), None),
                Texture2D::from_file_with_format(include_bytes!(r".\images\white_king.png"), None),
                Texture2D::from_file_with_format(include_bytes!(r".\images\white_queen.png"), None),
                Texture2D::from_file_with_format(include_bytes!(r".\images\white_rook.png"), None),
                Texture2D::from_file_with_format(
                    include_bytes!(r".\images\white_bishop.png"),
                    None,
                ),
                Texture2D::from_file_with_format(
                    include_bytes!(r".\images\white_knight.png"),
                    None,
                ),
                Texture2D::from_file_with_format(include_bytes!(r".\images\white_pawn.png"), None),
                Texture2D::from_file_with_format(include_bytes!(r".\images\black_king.png"), None),
                Texture2D::from_file_with_format(include_bytes!(r".\images\black_queen.png"), None),
                Texture2D::from_file_with_format(include_bytes!(r".\images\black_rook.png"), None),
                Texture2D::from_file_with_format(
                    include_bytes!(r".\images\black_bishop.png"),
                    None,
                ),
                Texture2D::from_file_with_format(
                    include_bytes!(r".\images\black_knight.png"),
                    None,
                ),
                Texture2D::from_file_with_format(include_bytes!(r".\images\black_pawn.png"), None),
            ],
        }
    }
    pub fn draw(&self) {
        draw_texture(self.textures[0], 0.0, 0.0, WHITE);
        for (i, piece) in self.chess.board.iter().enumerate() {
            let row = i / 8;
            let col = i % 8;
            let x = col as f32 * 100.0;
            let y = row as f32 * 100.0;
            match piece {
                Piece::Wking => draw_texture(self.textures[1], x, y, WHITE),
                Piece::Wqueen => draw_texture(self.textures[2], x, y, WHITE),
                Piece::Wrook => draw_texture(self.textures[3], x, y, WHITE),
                Piece::Wbishop => draw_texture(self.textures[4], x, y, WHITE),
                Piece::Wknight => draw_texture(self.textures[5], x, y, WHITE),
                Piece::Wpawn => draw_texture(self.textures[6], x, y, WHITE),
                Piece::Bking => draw_texture(self.textures[7], x, y, WHITE),
                Piece::Bqueen => draw_texture(self.textures[8], x, y, WHITE),
                Piece::Brook => draw_texture(self.textures[9], x, y, WHITE),
                Piece::Bbishop => draw_texture(self.textures[10], x, y, WHITE),
                Piece::Bknight => draw_texture(self.textures[11], x, y, WHITE),
                Piece::Bpawn => draw_texture(self.textures[12], x, y, WHITE),
                Piece::Empty => (),
            }
        }
    }
    pub fn draw_moves(&self) {
        for i in self.chess.moves.clone() {
            if self.chess.board[i] == Piece::Empty {
                draw_circle(
                    50.0 + (i % 8 * 100) as f32,
                    50.0 + (i / 8 * 100) as f32,
                    20.0,
                    Color {
                        r: 0.4,
                        g: 0.4,
                        b: 0.4,
                        a: 0.5,
                    },
                );
            } else {
                draw_circle_lines(
                    50.0 + (i % 8 * 100) as f32,
                    50.0 + (i / 8 * 100) as f32,
                    49.0,
                    7.0,
                    Color {
                        r: 0.4,
                        g: 0.4,
                        b: 0.4,
                        a: 0.5,
                    },
                );
            }
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
            self.draw();
            self.draw_moves();
            next_frame().await;
            thread::sleep(Duration::from_millis(100));
            if self.chess.board[self.mouse_pos].is_white() == self.chess.is_white_turn {
                let piece_index = self.mouse_pos;
                self.chess.get_legals(piece_index);
                while !is_mouse_button_pressed(MouseButton::Left) {
                    self.draw();
                    self.draw_moves();
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
                    self.draw();
                    self.draw_moves();
                    next_frame().await;
                }
                self.get_mouse_pos();
            }
        }
        self.chess.is_white_turn = !self.chess.is_white_turn;
    }
    pub fn ai_turn(&mut self) {
        self.ai.best_move();
        //find best move, make the move, possible time limit?
        //this function is not related to the graphics but makes the move and returns nothing
        // self.chess.is_white_turn = !self.chess.is_white_turn;
    }
}

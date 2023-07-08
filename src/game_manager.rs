use crate::ai::AI;
use crate::chess::{Chess, Piece};
use macroquad::prelude::*;
use std::borrow::Borrow;
use std::thread;
use std::time::Duration;

pub struct GameManager {
    ai: AI,
    chess: Chess,
    mouse_pos: usize,
    textures: [Texture2D; 13],
    pos: (f32, f32),
    timer: Timer,
}

impl GameManager {
    pub fn new(start: u64, add: Option<u64>) -> Self {
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
            pos: (100.0, 200.0),
            timer: Timer::new(start, add),
        }
    }
    pub fn draw(&self) {
        draw_texture(self.textures[0], self.pos.0, self.pos.1, WHITE);
        for (i, piece) in self.chess.board.iter().enumerate() {
            let row = i / 8;
            let col = i % 8;
            let x = col as f32 * 100.0 + self.pos.0;
            let y = row as f32 * 100.0 + self.pos.1;
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
        let window_width = screen_width();
        let window_height = screen_height();

        let outer_width = 600.0;
        let outer_height = 400.0;
        let outer_margin = 300.0;
        let inner_margin = 20.0;
        let timer_font_size = 200.0;

        let outer_x = window_width - outer_width - outer_margin;
        let outer_y = (window_height - outer_height) / 2.0 + 60.0;

        let inner_width = outer_width - inner_margin * 2.0;
        let inner_height = (outer_height - inner_margin * 3.0) / 2.0;

        let white_timer_x = outer_x + inner_margin + 50.0;
        let white_timer_y =
            outer_y + inner_margin + inner_height / 2.0 - timer_font_size / 3.0 + 80.0;

        let black_timer_x = outer_x + inner_margin + 50.0;
        let black_timer_y =
            outer_y + inner_margin + inner_height + inner_margin + inner_height / 2.0
                - timer_font_size / 3.0
                + 80.0;

        // Draw outer rectangles
        draw_rectangle(
            window_width - outer_width - outer_margin,
            (window_height - outer_height) / 2.0 + 60.0,
            outer_width,
            outer_height,
            GREEN,
        );
        draw_rectangle(
            window_width - outer_width - outer_margin + inner_margin,
            (window_height - outer_height) / 2.0 + 60.0 + inner_margin,
            inner_width,
            inner_height,
            BLACK,
        );
        draw_rectangle(
            window_width - outer_width - outer_margin + inner_margin,
            (window_height - outer_height) / 2.0
                + 60.0
                + inner_margin
                + inner_height
                + inner_margin,
            inner_width,
            inner_height,
            WHITE,
        );

        // Draw timer texts
        let white_minutes = self.timer.time_white as u32 / 60;
        let white_seconds = self.timer.time_white as u32 % 60;
        let white_time = format!("{:02}:{:02}", white_minutes, white_seconds);

        let black_minutes = self.timer.time_black as u32 / 60;
        let black_seconds = self.timer.time_black as u32 % 60;
        let black_time = format!("{:02}:{:02}", black_minutes, black_seconds);

        let white_text_width = white_time.len() as f32 * timer_font_size * 0.6;
        let white_text_x = white_timer_x + (inner_width - white_text_width) / 2.0;

        let black_text_width = black_time.len() as f32 * timer_font_size * 0.6;
        let black_text_x = black_timer_x + (inner_width - black_text_width) / 2.0;

        draw_text(
            &white_time,
            white_text_x + 30.0,
            white_timer_y + 30.0,
            timer_font_size,
            WHITE,
        );
        draw_text(
            &black_time,
            black_text_x + 30.0,
            black_timer_y + 30.0,
            timer_font_size,
            BLACK,
        );

        // egui_macroquad::ui(|egui_ctx| {
        //     egui::Window::new("menu")
        //         .fixed_pos(egui::Pos2::new(0.0, 0.0))
        //         .show(egui_ctx, |ui| {
        //             //ui code goes here
        //         });
        // });
        // egui_macroquad::draw();
    }
    pub fn draw_moves(&self) {
        for i in self.chess.moves.clone() {
            if self.chess.board[i] == Piece::Empty {
                draw_circle(
                    50.0 + (i % 8 * 100) as f32 + self.pos.0,
                    50.0 + (i / 8 * 100) as f32 + self.pos.1,
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
                    50.0 + (i % 8 * 100) as f32 + self.pos.0,
                    50.0 + (i / 8 * 100) as f32 + self.pos.1,
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
        if mouse_pos.0 > self.pos.0
            && mouse_pos.0 < self.pos.0 + 800.0
            && mouse_pos.1 > self.pos.1
            && mouse_pos.1 < self.pos.1 + 800.0
        {
            self.mouse_pos = (mouse_pos.0 - self.pos.0) as usize / 100
                + (mouse_pos.1 - self.pos.1) as usize / 100 * 8;
        }
    }
    pub async fn player_turn(&mut self) {
        self.chess.moves = vec![];
        loop {
            self.draw();
            self.draw_moves();
            next_frame().await;
            thread::sleep(Duration::from_millis(100));
            if (0..64).contains(&self.mouse_pos)
                && self.chess.board[self.mouse_pos].is_white() == self.chess.is_white_turn
            {
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
        //find best move, make the move, possible time limit (would be epic (maybe just quit after x time))?
        //this function is not related to the graphics but makes the move and returns nothing
        self.chess.is_white_turn = !self.chess.is_white_turn;
    }
    pub fn is_ending(&mut self) -> i32 {
        if !self.has_moves() {
            if self.chess.is_check() {
                if self.chess.is_white_turn {
                    println!("black checkmate");
                    return -1;
                } else {
                    println!("white checkmate");
                    return 1;
                }
            } else {
                println!("stalemate");
                return 0;
            }
        }
        2
    }
    pub fn has_moves(&mut self) -> bool {
        for (i, p) in self.chess.board.into_iter().enumerate() {
            if p.is_white() == self.chess.is_white_turn {
                self.chess.get_legals(i);
                if !self.chess.moves.is_empty() {
                    return true;
                }
            }
        }
        false
    }
}

struct Timer {
    start_time: std::time::Instant,
    time_white: u64,
    time_black: u64,
    add: Option<u64>,
}
impl Timer {
    fn new(start_time: u64, add: Option<u64>) -> Self {
        Self {
            start_time: std::time::Instant::now(),
            time_white: start_time,
            time_black: start_time,
            add,
        }
    }
    fn update_white(&mut self) {
        self.time_white -= self.start_time.elapsed().as_secs();
        self.time_white += self.add.unwrap_or(0);
        self.reset();
    }
    fn update_black(&mut self) {
        self.time_black -= self.start_time.elapsed().as_secs();
        self.time_black += self.add.unwrap_or(0);
        self.reset();
    }
    fn reset(&mut self) {
        self.start_time = std::time::Instant::now();
    }
}

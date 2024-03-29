use crate::ai::AI;
use crate::bitboard::Bitboard;
use crate::chess::{Chess, Piece};
use crate::BlackWhite;
use macroquad::prelude::*;
use std::thread;
use std::time::{Duration, Instant};

pub struct GameManager {
    pub ai: AI,
    pub chess: Chess,
    mouse_pos: Option<usize>,
    textures: [Texture2D; 13],
    pos: (f32, f32),
    timer: Timer,
    game_state: i32,
    player_vs_ai: BlackWhite,
}

impl GameManager {
    pub fn new(start: f32, add: f32, ai_depth: Option<i32>, player_vs_ai: BlackWhite) -> Self {
        let mut g = GameManager {
            ai: AI::new(ai_depth.unwrap_or(1), player_vs_ai == BlackWhite::White),
            chess: Chess::new(),
            mouse_pos: None,
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
            game_state: -2,
            player_vs_ai,
        };
        if player_vs_ai == BlackWhite::White {
            g.timer.time_black = 0.0;
        } else if player_vs_ai == BlackWhite::Black {
            g.timer.time_white = 0.0;
        }
        g
    }
    fn draw_bitboard(&self, bitboard: Bitboard) {
        for row in 0..8 {
            for col in 0..8 {
                let index = row * 8 + col;
                let bit = (bitboard.0 >> index) & 1;
                if bit == 1 {
                    let x = col as f32 * 100.0 + self.pos.0;
                    let y = row as f32 * 100.0 + self.pos.1;
                    draw_rectangle(x, y, 100.0, 100.0, RED);
                }
            }
        }
    }
    fn draw_move(&self, from: u8, to: u8) {
        if (0..64).contains(&from) && (0..64).contains(&to) {
            let from_x = (from % 8) as f32 * 100.0 + self.pos.0;
            let from_y = (from / 8) as f32 * 100.0 + self.pos.1;
            draw_rectangle(from_x, from_y, 100.0, 100.0, YELLOW);
            let to_x = (to % 8) as f32 * 100.0 + self.pos.0;
            let to_y = (to / 8) as f32 * 100.0 + self.pos.1;
            draw_rectangle(to_x, to_y, 100.0, 100.0, YELLOW);
        }
    }
    fn draw_check(&self) {
        if self.chess.is_check(self.chess.king_loc()) {
            let x = (self.chess.king_loc() % 8) as f32 * 100.0 + self.pos.0;
            let y = (self.chess.king_loc() / 8) as f32 * 100.0 + self.pos.1;
            draw_rectangle(x, y, 100.0, 100.0, RED);
        }
    }
    pub fn draw(&self) {
        //switch to move later
        draw_texture(self.textures[0], self.pos.0, self.pos.1, WHITE);
        //self.draw_bitboard(self.chess.white_pins);
        self.draw_move(self.chess.last_move.0, self.chess.last_move.1);
        //self.draw_check();
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
        let outer_x = window_width - outer_width - outer_margin;
        let outer_y = (window_height - outer_height) / 2.0 + 60.0;
        let inner_width = outer_width - inner_margin * 2.0;
        let inner_height = (outer_height - inner_margin * 3.0) / 2.0;
        let timer_x = outer_x + inner_margin + 50.0;
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
        draw_text(
            &format!(
                "{:02}:{:02}",
                self.timer.time_black as u32 / 60,
                self.timer.time_black as u32 % 60
            ),
            timer_x + (inner_width - 5.0 * 120.0) / 2.0 + 30.0,
            outer_y + inner_margin + inner_height / 2.0 - 200.0 / 3.0 + 110.0,
            200.0,
            WHITE,
        );
        draw_text(
            &format!(
                "{:02}:{:02}",
                self.timer.time_white as u32 / 60,
                self.timer.time_white as u32 % 60
            ),
            timer_x + (inner_width - 5.0 * 120.0) / 2.0 + 30.0,
            outer_y + inner_margin + inner_height + inner_margin + inner_height / 2.0 - 200.0 / 3.0
                + 110.0,
            200.0,
            BLACK,
        );
        self.draw_title();
    }
    pub fn draw_moves(&self) {
        for i in self.chess.moves.clone() {
            if self.chess.board[i as usize] == Piece::Empty {
                draw_circle(
                    50.0 + (i as i32 % 8 * 100) as f32 + self.pos.0,
                    50.0 + (i as i32 / 8 * 100) as f32 + self.pos.1,
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
                    50.0 + (i as i32 % 8 * 100) as f32 + self.pos.0,
                    50.0 + (i as i32 / 8 * 100) as f32 + self.pos.1,
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
            self.mouse_pos = Some(
                (mouse_pos.0 - self.pos.0) as usize / 100
                    + (mouse_pos.1 - self.pos.1) as usize / 100 * 8,
            );
        } else {
            self.mouse_pos = None;
        }
    }
    pub async fn player_turn(&mut self) {
        self.chess.moves = vec![];
        loop {
            self.draw();
            self.draw_moves();
            next_frame().await;
            thread::sleep(Duration::from_millis(100));
            if let Some(x) = self.mouse_pos {
                if (0..64).contains(&x)
                    && self.chess.board[x].is_white() == self.chess.is_white_turn
                {
                    let piece_index = x;
                    self.chess.get_legals(piece_index);
                    while !is_mouse_button_pressed(MouseButton::Left) {
                        self.draw();
                        self.draw_moves();
                        next_frame().await;
                    }
                    self.get_mouse_pos();
                    if self.mouse_pos.is_some()
                        && self
                            .chess
                            .moves
                            .contains(&(self.mouse_pos.expect("liam is obese") as u8))
                    {
                        self.chess.move_piece(
                            piece_index as u8,
                            self.mouse_pos.expect("liam is obese") as u8,
                        );
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
            } else {
                while !is_mouse_button_pressed(MouseButton::Left) {
                    self.draw();
                    self.draw_moves();
                    next_frame().await;
                }
                self.get_mouse_pos();
            }
        }
        if self.chess.is_white_turn {
            self.timer.update_black();
        } else {
            self.timer.update_white();
        }
    }
    pub fn ai_turn(&mut self) {
        let (from, to) = self.ai.best_move(&mut self.chess);
        self.chess.move_piece(from, to);
    }
    pub fn game_state(&mut self) -> i32 {
        //firstly handle timers
        match self.chess.is_ending() {
            2 => {
                if self.chess.is_white_turn {
                    self.game_state = -2;
                } else {
                    self.game_state = -3;
                }
                return 2;
            }
            x => self.game_state = x,
        };
        self.game_state
    }
    pub async fn pvp(&mut self) {
        clear_background(BLACK);
        while self.game_state() == 2 {
            self.draw();
            if is_mouse_button_pressed(MouseButton::Left) {
                self.get_mouse_pos();
                self.player_turn().await;
            }
            next_frame().await;
        }
        loop {
            self.draw();
            next_frame().await;
        }
    }
    pub async fn pvai(&mut self) {
        clear_background(BLACK);
        if self.player_vs_ai == BlackWhite::Black {
            let ai_start_time = Instant::now();
            self.ai_turn();
            let ai_duration = ai_start_time.elapsed();
            //println!("AI's turn duration: {:?}", ai_duration);
        }
        while self.game_state() == 2 {
            self.draw();
            if is_mouse_button_pressed(MouseButton::Left) {
                self.get_mouse_pos();
                self.player_turn().await;
                self.draw();
                next_frame().await;

                let ai_start_time = Instant::now();
                self.ai_turn();
                let ai_duration = ai_start_time.elapsed();
                //println!("AI's turn duration: {:?}", ai_duration);
            }
            next_frame().await;
        }
        // loop {
        //     self.draw();
        //     next_frame().await;
        // }
    }
    fn winning_title(&self) {
        let text = match self.game_state {
            0 => "tie by stalemate",
            -1 => "black won by checkmate",
            1 => "white won by checkmate",
            3 => "tie by insufficient material",
            4 => "tie by threefold repetition",
            5 => "tie by the 50 move rule",
            6 => "time forfeit",
            _ => "undefined",
        };
        let text_width = measure_text(text, None, 100, 1.0).width;
        let screen_width = screen_width();
        let x_centered = (screen_width - text_width) / 2.0;
        draw_text(text, x_centered, 150.0, 100.0, GREEN);
    }
    fn draw_title(&self) {
        if self.game_state == -3 {
            draw_text("black turn", 1070.0, 350.0, 100.0, GREEN);
        } else if self.game_state == -2 {
            draw_text("white turn", 1070.0, 350.0, 100.0, GREEN);
        } else {
            self.winning_title();
        }
        if self.game_state == -1 {
            let loc = self.chess.black_king;
            let x = (loc % 8) as f32 * 100.0 + self.pos.0;
            let y = (loc / 8) as f32 * 100.0 + self.pos.1 - 35.0;
            draw_texture(
                Texture2D::from_file_with_format(include_bytes!(r".\images\crown.png"), None),
                x,
                y,
                WHITE,
            );
        }
        if self.game_state == 1 {
            let loc = self.chess.white_king;
            let x = (loc % 8) as f32 * 100.0 + self.pos.0;
            let y = (loc / 8) as f32 * 100.0 + self.pos.1 - 35.0;
            draw_texture(
                Texture2D::from_file_with_format(include_bytes!(r".\images\crown.png"), None),
                x,
                y,
                WHITE,
            );
        }
        if self.game_state == 6 {} // winner titles and crowns for time forfiet
        if self.game_state == 0 || (3..6).contains(&self.game_state) {
            //draw title
            let loc = self.chess.white_king;
            let x = (loc % 8) as f32 * 100.0 + self.pos.0;
            let y = (loc / 8) as f32 * 100.0 + self.pos.1 - 35.0;
            draw_texture(
                Texture2D::from_file_with_format(include_bytes!(r".\images\crown.png"), None),
                x,
                y,
                WHITE,
            );
            let loc = self.chess.black_king;
            let x = (loc % 8) as f32 * 100.0 + self.pos.0;
            let y = (loc / 8) as f32 * 100.0 + self.pos.1 - 35.0;
            draw_texture(
                Texture2D::from_file_with_format(include_bytes!(r".\images\crown.png"), None),
                x,
                y,
                WHITE,
            );
        }
    }
}

struct Timer {
    start_time: std::time::Instant,
    time_white: f32,
    time_black: f32,
    add: f32,
}
impl Timer {
    fn new(start_time: f32, add: f32) -> Self {
        Self {
            start_time: std::time::Instant::now(),
            time_white: start_time,
            time_black: start_time,
            add,
        }
    }
    fn update_white(&mut self) {
        self.time_white -= self.start_time.elapsed().as_secs() as f32;
        self.time_white += self.add;
        self.reset();
    }
    fn update_black(&mut self) {
        self.time_black -= self.start_time.elapsed().as_secs() as f32;
        self.time_black += self.add;
        self.reset();
    }
    fn reset(&mut self) {
        self.start_time = std::time::Instant::now();
    }
}

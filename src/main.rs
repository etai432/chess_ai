mod ai;
mod bitboard;
mod chess;
mod game_manager;
use crate::{ai::AI, chess::Chess};
use game_manager::GameManager;
use macroquad::{
    prelude::{
        draw_rectangle, draw_text, draw_texture, is_mouse_button_down, is_mouse_button_pressed,
        mouse_position, next_frame, screen_height, screen_width, Conf, MouseButton, Texture2D,
        BLACK, DARKGRAY, GREEN, WHITE,
    },
    time::get_fps,
    window::clear_background,
};
use rand::Rng;
use std::time::Instant;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "chess".to_owned(),
        // window_height: 800,
        // window_width: 800,
        window_resizable: false,
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");
    menu().await;
}

// fn main() {
//     test_move_generation_speed(5);
//     // benchmark_chess();
//     // println!("hello");
// }

// fn benchmark_chess() {
//     // Initialize chessboard
//     let mut chess = Chess::new();
//     // Measure time for move_gen
//     let start_time_move_gen = Instant::now();
//     chess.get_all_moves();
//     let elapsed_time_move_gen = start_time_move_gen.elapsed();

//     let start_time_rook_bishop = Instant::now();
//     chess.gen_moves_rook(28);
//     chess.gen_moves_bishop(28);
//     let elapsed_time_rook_bishop = start_time_rook_bishop.elapsed();

//     // Measure time for update_attacked_squares
//     let start_time_update_attacked_squares = Instant::now();
//     chess.update_attacked_squares();
//     let elapsed_time_update_attacked_squares = start_time_update_attacked_squares.elapsed();

//     // Measure time for move_piece
//     let start_time_move_piece = Instant::now();
//     let chess_move = chess.move_piece(48, 40);
//     let elapsed_time_move_piece = start_time_move_piece.elapsed();

//     // Measure time for undo_move
//     let start_time_undo_move = Instant::now();
//     chess.undo_move(chess_move);
//     let elapsed_time_undo_move = start_time_undo_move.elapsed();

//     // Print the elapsed times
//     println!("move_gen elapsed time: {:?}", elapsed_time_move_gen);
//     println!("rook_bishop elapsed time: {:?}", elapsed_time_rook_bishop);
//     println!(
//         "update_attacked_squares elapsed time: {:?}",
//         elapsed_time_update_attacked_squares
//     );
//     println!("move_piece elapsed time: {:?}", elapsed_time_move_piece);
//     println!("undo_move elapsed time: {:?}", elapsed_time_undo_move);
// }

// fn test_move_generation_speed(depth_ai: i32) {
//     let start_time = Instant::now();
//     let mut chess = Chess::new();
//     let moves = AI::count_moves(depth_ai, &mut chess);
//     let duration = start_time.elapsed();
//     println!("Generated {} moves in {:?}", moves, duration);
// }

async fn menu() {
    let mut pv = Pv::Pvai;
    let mut blackwhite = BlackWhite::Random;
    let mut time: f32 = 300.0;
    let mut additional_time_per_move: f32 = 2.0;
    let mut depth_ai = 4;
    let button_width = 400.0;
    let button_height = 200.0;
    let button_pos = egui::Pos2::new(
        (screen_width() - button_width) / 2.0 - 30.0,
        (screen_height() - button_height) / 2.0 + 130.0,
    );
    loop {
        clear_background(DARKGRAY);
        draw_pieces();
        egui_macroquad::ui(|egui_ctx| {
            let panel_width = 300.0;
            let panel_height = 700.0;
            let panel_pos = egui::Pos2::new(
                (screen_width() - panel_width) / 2.0 - 40.0,
                (screen_height() - panel_height) / 2.0,
            );
            egui::containers::Window::new(
                egui::RichText::new("Game Settings")
                    .heading()
                    .color(egui::Color32::GREEN),
            )
            .id(egui::Id::new("game_settings_panel"))
            .fixed_pos(panel_pos)
            .fixed_size((panel_width, panel_height))
            .resizable(false)
            .show(egui_ctx, |ui| {
                ui.visuals_mut().override_text_color = Some(egui::Color32::WHITE);
                ui.separator();
                ui.heading(
                    egui::RichText::new("Choose the Game Mode")
                        .heading()
                        .color(egui::Color32::GREEN),
                );
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 10.0;
                    ui.radio_value(&mut pv, Pv::Pvai, "Player vs AI");
                    ui.radio_value(&mut pv, Pv::Pvp, "Player vs Player");
                });
                if pv == Pv::Pvai {
                    ui.separator();
                    ui.heading(
                        egui::RichText::new("Choose Your Player")
                            .heading()
                            .color(egui::Color32::GREEN),
                    );
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 10.0;
                        ui.radio_value(&mut blackwhite, BlackWhite::Random, "Random");
                        ui.radio_value(&mut blackwhite, BlackWhite::White, "White");
                        ui.radio_value(&mut blackwhite, BlackWhite::Black, "Black");
                    });
                }
                ui.separator();
                ui.heading(
                    egui::RichText::new("Time")
                        .heading()
                        .color(egui::Color32::GREEN),
                );
                ui.label(
                    egui::RichText::new("Total Game Time (seconds):")
                        .heading()
                        .color(egui::Color32::LIGHT_BLUE),
                );
                ui.add(
                    egui::Slider::new(&mut time, 30.0..=600.0)
                        .text("")
                        .suffix("s")
                        .clamp_to_range(true),
                );
                ui.heading(
                    egui::RichText::new("Additional Time")
                        .heading()
                        .color(egui::Color32::GREEN),
                );
                ui.label(
                    egui::RichText::new("Additional Time per Move (seconds):")
                        .heading()
                        .color(egui::Color32::LIGHT_BLUE),
                );
                ui.add(
                    egui::Slider::new(&mut additional_time_per_move, 0.0..=10.0)
                        .text("")
                        .suffix("s")
                        .clamp_to_range(true),
                );
                if pv == Pv::Pvai {
                    ui.heading(
                        egui::RichText::new("AI Time")
                            .heading()
                            .color(egui::Color32::GREEN),
                    );
                    ui.label(
                        egui::RichText::new("Depth (AI):")
                            .heading()
                            .color(egui::Color32::LIGHT_BLUE),
                    );
                    ui.add(
                        egui::Slider::new(&mut depth_ai, 2..=4)
                            .text("")
                            .clamp_to_range(true),
                    );
                }
            });
        });
        draw_text("CHESS", 700.0, 150.0, 200.0, GREEN);
        draw_rectangle(
            button_pos.x,
            button_pos.y,
            button_width,
            button_height,
            GREEN,
        );
        draw_text(
            "START",
            button_pos.x + 30.0,
            button_pos.y + 130.0,
            150.0,
            BLACK,
        );
        // Check for mouse click on the button
        if is_mouse_button_down(MouseButton::Left)
            && is_mouse_button_pressed(MouseButton::Left)
            && mouse_position().0 >= button_pos.x
            && mouse_position().0 <= button_pos.x + button_width
            && mouse_position().1 >= button_pos.y
            && mouse_position().1 <= button_pos.y + button_height
        {
            if pv == Pv::Pvp {
                let mut game =
                    GameManager::new(time, additional_time_per_move, None, BlackWhite::Random);
                game.pvp().await;
                return;
            } else {
                let mut rng = rand::thread_rng();
                if blackwhite == BlackWhite::Random {
                    blackwhite = if rng.gen_range(0..2) == 0 {
                        BlackWhite::White
                    } else {
                        BlackWhite::Black
                    };
                }
                let mut game =
                    GameManager::new(time, additional_time_per_move, Some(depth_ai), blackwhite);
                game.pvai().await;
                return;
            }
        }
        egui_macroquad::draw();
        next_frame().await;
    }
}

fn draw_pieces() {
    let mut piece: Texture2D;
    piece = Texture2D::from_file_with_format(include_bytes!(r".\images\white_king.png"), None);
    draw_texture(piece, 100.0, 100.0, WHITE);
    piece = Texture2D::from_file_with_format(include_bytes!(r".\images\white_queen.png"), None);
    draw_texture(piece, 300.0, 225.0, WHITE);
    piece = Texture2D::from_file_with_format(include_bytes!(r".\images\white_rook.png"), None);
    draw_texture(piece, 500.0, 350.0, WHITE);
    piece = Texture2D::from_file_with_format(include_bytes!(r".\images\white_bishop.png"), None);
    draw_texture(piece, 100.0, 475.0, WHITE);
    piece = Texture2D::from_file_with_format(include_bytes!(r".\images\white_knight.png"), None);
    draw_texture(piece, 300.0, 600.0, WHITE);
    piece = Texture2D::from_file_with_format(include_bytes!(r".\images\white_pawn.png"), None);
    draw_texture(piece, 500.0, 725.0, WHITE);
    piece = Texture2D::from_file_with_format(include_bytes!(r".\images\black_king.png"), None);
    draw_texture(piece, 1700.0, 100.0, WHITE);
    piece = Texture2D::from_file_with_format(include_bytes!(r".\images\black_queen.png"), None);
    draw_texture(piece, 1500.0, 225.0, WHITE);
    piece = Texture2D::from_file_with_format(include_bytes!(r".\images\black_rook.png"), None);
    draw_texture(piece, 1300.0, 350.0, WHITE);
    piece = Texture2D::from_file_with_format(include_bytes!(r".\images\black_bishop.png"), None);
    draw_texture(piece, 1700.0, 475.0, WHITE);
    piece = Texture2D::from_file_with_format(include_bytes!(r".\images\black_knight.png"), None);
    draw_texture(piece, 1500.0, 600.0, WHITE);
    piece = Texture2D::from_file_with_format(include_bytes!(r".\images\black_pawn.png"), None);
    draw_texture(piece, 1300.0, 725.0, WHITE);
    piece = Texture2D::from_file_with_format(include_bytes!(r".\images\crown.png"), None);
    draw_texture(piece, 100.0, 65.0, WHITE);
    piece = Texture2D::from_file_with_format(include_bytes!(r".\images\crown.png"), None);
    draw_texture(piece, 1700.0, 65.0, WHITE);
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum BlackWhite {
    Black,
    White,
    Random,
}
#[derive(PartialEq)]
enum Pv {
    Pvp,
    Pvai,
}
//game endings: Draw by Insufficient Material, Draw by Threefold Repetition, Draw by Fifty-Move Rule, Time Forfeit (also make them useful in a game)

//todo list:
//undo promotions, attacks maps, pins
//game endings- timers, threefold, insufficient
//better ai
//magic bitboards?

//bugs go here:
//an passant didnt take pawn- prob does now idfk
//horsey cant take checking piece? (pawns can)
//bishop pin sucks ass
//i hate my life

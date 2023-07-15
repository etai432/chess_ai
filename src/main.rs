mod ai;
mod bitboard;
mod chess;
mod game_manager;
use game_manager::GameManager;
use macroquad::prelude::*;

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
    menu().await
}

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
                pvp(time, additional_time_per_move).await;
                return;
            } else {
                pvai(time, additional_time_per_move, blackwhite, depth_ai).await;
                return;
            }
        }
        egui_macroquad::draw();
        next_frame().await;
    }
}

async fn pvp(game_time: f32, additional: f32) {
    clear_background(BLACK);
    let mut game = GameManager::new(game_time, additional, None);
    while game.is_ending() == 2 {
        game.draw(Some(game.chess.black_pins));
        if is_mouse_button_pressed(MouseButton::Left) {
            game.get_mouse_pos();
            game.player_turn().await;
        }
        next_frame().await;
    }
}

async fn pvai(game_time: f32, additional: f32, mut player: BlackWhite, depth_ai: i32) {
    clear_background(BLACK);
    let mut game = GameManager::new(game_time, additional, Some(depth_ai));
    if player == BlackWhite::Random {
        player = if rand::gen_range(0, 1) == 0 {
            BlackWhite::White
        } else {
            BlackWhite::Black
        };
    }
    if player == BlackWhite::Black {
        game.ai_turn();
    }
    while game.is_ending() == 2 {
        game.draw(None);
        if is_mouse_button_pressed(MouseButton::Left) {
            game.get_mouse_pos();
            game.player_turn().await;
            game.ai_turn();
        }
        next_frame().await;
    }
}

#[derive(PartialEq)]
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
//todo:
//chess: done for now
//ai: best move
//game:
//ai turn (after ai)
//game endings: Draw by Insufficient Material, Draw by Threefold Repetition, Draw by Fifty-Move Rule, Time Forfeit (also make them useful in a game)
//make timers functional
//graphics:
//add pieces to the side of the menu
//make a game title that indicates whos turn it is and the winner. after a game won. back to menu screen (maybe restart)
//crown for the winner (funny little bonus)

//todo list:
//optimazing
//color last move
//title
//ai timer
//pieces in menu
//game endings- should prob be handled by the game manager
//better ai
//crown
//maybe if ai not good enough, try magic bitboards

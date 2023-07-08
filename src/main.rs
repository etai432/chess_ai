mod ai;
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
    pvp().await;
    pvai().await;
}

async fn pvp() {
    let mut game = GameManager::new();
    loop {
        game.draw();
        if is_mouse_button_pressed(MouseButton::Left) {
            game.get_mouse_pos();
            game.player_turn().await;
            game.is_ending();
        }
        next_frame().await;
    }
}

async fn pvai() {
    let mut game = GameManager::new();
    loop {
        game.draw();
        if is_mouse_button_pressed(MouseButton::Left) {
            game.get_mouse_pos();
            game.player_turn().await;
            game.ai_turn();
        }
        next_frame().await;
    }
}

//todo:
//chess: done for now
//ai: best move
//game: ai turn, game endings: Checkmate, Stalemate, Draw by Insufficient Material, Draw by Threefold Repetition, Draw by Fifty-Move Rule, Draw by Agreement, Resignation, Time Forfeit
//graphics:
//make a menu: pvp, pvai
//make a game title that indicates whos turn it is and the winner
//make time clocks and ai thinking time using egui
//crown for the winner

//simple timer using egui
// struct Timer {
//     start_time: std::time::Instant,
// }
// impl Timer {
//     fn new() -> Self {
//         Self {
//             start_time: std::time::Instant::now(),
//         }
//     }
//     fn get_elapsed_time(&self) -> std::time::Duration {
//         self.start_time.elapsed()
//     }
//     fn reset(&mut self) {
//         self.start_time = std::time::Instant::now();
//     }
// }
// let mut timer = Timer::new();
// loop {
//     egui_macroquad::ui(|egui_ctx| {
//         egui::Window::new("Timer").show(egui_ctx, |ui| {
//             ui.horizontal(|ui| {
//                 let elapsed_time = timer.get_elapsed_time();
//                 let elapsed_seconds = elapsed_time.as_secs();
//                 ui.label(format!("Elapsed Time: {} seconds", elapsed_seconds));
//                 if ui.button("Reset").clicked() {
//                     timer.reset();
//                 }
//             });
//         });
//     });
//     egui_macroquad::draw();
//     next_frame().await;
// }

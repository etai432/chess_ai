mod ai;
mod chess;
mod game_manager;
use game_manager::GameManager;
use macroquad::prelude::*;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "chess".to_owned(),
        window_height: 800,
        window_width: 800,
        window_resizable: false,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
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
//chess: en passant, game endings
//ai: ai turn

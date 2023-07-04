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
        // fullscreen: true,
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
//chess: done for now
//ai: best move
//game: ai turn, game endings: Checkmate, Stalemate, Draw by Insufficient Material, Draw by Threefold Repetition, Draw by Fifty-Move Rule, Draw by Agreement, Resignation, Time Forfeit
//graphics: make game full screen and make the expirience better, pvp, pvai, time!

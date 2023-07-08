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
    let mut game = GameManager::new(0, None);
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
    let mut game = GameManager::new(0, None);
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
//game:
//ai turn (after ai)
//game endings: Draw by Insufficient Material, Draw by Threefold Repetition, Draw by Fifty-Move Rule, Draw by Agreement, Resignation, Time Forfeit (also make them useful in a game)
//make timers functional
//graphics:
//make a menu: pvp, pvai, timers
//make a game title that indicates whos turn it is and the winner
//crown for the winner (funny little bonus)

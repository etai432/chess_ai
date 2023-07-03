mod chess;
use chess::Chess;
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
    let mut chess = Chess::new();
    loop {
        chess.draw();
        if is_mouse_button_pressed(MouseButton::Left) {
            chess.get_mouse_pos();
            chess.player_turn(chess.is_white_turn).await;
            chess.is_white_turn = !chess.is_white_turn;
        }
        next_frame().await;
    }
}

//todo:
//pawns, knights, en passant, castling (including rooks coming back to place), turns, game endings

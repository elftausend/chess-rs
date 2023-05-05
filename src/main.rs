use chess_rs::{sprites, Chess};
use macroquad::prelude::*;

#[macroquad::main("Chess")]
async fn main() {
    let mut chess: Chess = Chess::new(Some(sprites().await));

    loop {
        clear_background(DARKGRAY);

        chess.draw();

        if is_mouse_button_pressed(MouseButton::Left) {
            let field = chess.has_clicked_field(mouse_position());
            if let Some(clicked) = field {
                chess.select_or_move(clicked);
            }
        }

        next_frame().await;
    }
}

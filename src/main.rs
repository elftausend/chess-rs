use chess_rs::{sprites, Chess, State};
use macroquad::prelude::*;

#[macroquad::main("Chess")]
async fn main() {
    let mut chess: Chess = Chess::new(Some(sprites().await));

    loop {
        clear_background(DARKGRAY);

        chess.draw();

        match chess.state {
            State::Promote(to_promote) => {
                chess.draw_promote_selection(to_promote);
                if is_mouse_button_pressed(MouseButton::Left) {
                    if let Some(figure) = chess.has_clicked_promotion(to_promote, mouse_position())
                    {
                        chess.handle_promote_selection((to_promote.row, to_promote.col), figure)
                    }
                }
                // self.handle_promote_selection(field.0);
            }
            State::Select => {
                if is_mouse_button_pressed(MouseButton::Left) {
                    let field = chess.has_clicked_field(mouse_position());
                    if let Some(clicked) = field {
                        chess.select_or_move(clicked);
                    }
                }
            }
        }

        next_frame().await;
    }
}

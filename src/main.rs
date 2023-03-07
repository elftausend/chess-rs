mod valid_moves;
mod figure;
mod chess;
use chess::Chess;
mod field;

pub use field::*;
pub use figure::*;


use macroquad::prelude::*;

const SIZE: f32 = 60.;
const X_DIST: f32 = 20.;
const Y_DIST: f32 = 20.;

const ROWS: usize = 8;
const COLS: usize = 8;

#[derive(Debug, Default)]
pub struct Selection {
    selected_field: Option<(usize, usize)>,
    moves: Vec<(usize, usize)>,
}

impl Selection {
    pub fn draw(&self) {
        if self.selected_field.is_none() {
            return;
        }

        // draw selection border
        /*if let Some((row, col)) = self.selected_field {
            let x = col as f32 * SIZE + X_DIST;
            let y = row as f32 * SIZE + Y_DIST;
            draw_rectangle(x, y, SIZE, SIZE, GREEN);
        }*/

        for (row, col) in &self.moves {
            let x = *col as f32 * SIZE + X_DIST;
            let y = *row as f32 * SIZE + Y_DIST;
            draw_circle(x + SIZE / 2., y + SIZE / 2., 12., YELLOW);
        }
    }

    pub fn unselect_field(&mut self) {
        self.selected_field = None;
        self.moves.clear();
    }
}

#[macroquad::main("Chess")]
async fn main() {
    let sprites = [
        Texture2D::from_image(&load_image("./Figures/whitePawn.png").await.unwrap()),
        Texture2D::from_image(&load_image("./Figures/whiteKing.png").await.unwrap()),
        Texture2D::from_image(&load_image("./Figures/whiteQueen.png").await.unwrap()),
        Texture2D::from_image(&load_image("./Figures/whiteKnight.png").await.unwrap()),
        Texture2D::from_image(&load_image("./Figures/whiteRook.png").await.unwrap()),
        Texture2D::from_image(&load_image("./Figures/whiteBishop.png").await.unwrap()),
        Texture2D::from_image(&load_image("./Figures/blackPawn.png").await.unwrap()),
        Texture2D::from_image(&load_image("./Figures/blackKing.png").await.unwrap()),
        Texture2D::from_image(&load_image("./Figures/blackQueen.png").await.unwrap()),
        Texture2D::from_image(&load_image("./Figures/blackKnight.png").await.unwrap()),
        Texture2D::from_image(&load_image("./Figures/blackRook.png").await.unwrap()),
        Texture2D::from_image(&load_image("./Figures/blackBishop.png").await.unwrap()),
    ];

    let mut chess = Chess::new(sprites);

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

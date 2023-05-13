use macroquad::{prelude::YELLOW, shapes::draw_circle};

use crate::{SIZE, X_DIST, Y_DIST, FigureType};

#[derive(Debug, Default)]
pub struct Selection {
    pub selected_field: Option<(usize, usize)>,
    pub moves: Vec<(usize, usize)>,
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

pub fn calc_promote_x(col: usize) -> f32 {
    (SIZE * 0.5) + (col+1) as f32 * SIZE
}

pub fn calc_promote_y(row: usize, height: usize) -> f32 {
    height as f32 * SIZE * 0.75 + Y_DIST + row as f32 * SIZE
}

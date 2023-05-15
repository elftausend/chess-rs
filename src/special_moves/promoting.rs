use macroquad::prelude::*;

use crate::{
    calc_promote_x, calc_promote_y, Chess, Figure, FigureType, Position, State, Team, SIZE,
};

impl Chess {
    pub fn promote_pawn_at(&mut self, pos: (usize, usize), figure: FigureType) {
        self.field_mut(pos).figure = Some(Figure {
            figure,
            team: self.player,
            first_move: false,
        });
    }

    pub fn draw_promote_selection(&mut self, Position { mut row, col }: Position) {
        let x = calc_promote_x(col);

        let figures = [
            FigureType::Queen,
            FigureType::Rook,
            FigureType::Bishop,
            FigureType::Knight,
        ];

        if self.player == Team::Black {
            row -= 2;
        }

        for (idx, figure) in figures.into_iter().enumerate() {
            let y = calc_promote_y(row, idx);

            draw_rectangle(x, y, SIZE * 0.75, SIZE * 0.75, WHITE);
            let sprite = self.sprites.unwrap()[figure as usize + self.player as usize * 6];

            let mut params = DrawTextureParams::default();
            params.dest_size = Some(vec2(SIZE * 0.75, SIZE * 0.75));

            draw_texture_ex(sprite, x, y, WHITE, params);
        }
    }

    pub fn has_clicked_promotion(
        &self,
        Position { mut row, col }: Position,
        (mouse_x, mouse_y): (f32, f32),
    ) -> Option<FigureType> {
        if self.player == Team::Black {
            row -= 2;
        }

        let start_x = calc_promote_x(col);
        let start_y = calc_promote_y(row, 0);

        let x = ((mouse_x - start_x) / (SIZE * 0.75)).floor();
        let y = ((mouse_y - start_y) / (SIZE * 0.75)).floor();

        if x as usize != 0 && y as usize > 3 {
            return None;
        }

        Some(
            [
                FigureType::Queen,
                FigureType::Rook,
                FigureType::Bishop,
                FigureType::Knight,
            ][y as usize],
        )
    }

    pub fn handle_promote_selection(&mut self, to_promote: (usize, usize), figure: FigureType) {
        self.promote_pawn_at(to_promote, figure);

        self.player = !self.player;
        self.state = State::Select;
    }
}

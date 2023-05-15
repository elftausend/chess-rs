use crate::{Chess, FigureType, Team, COLS, ROWS};

impl Chess {
    pub fn tried_rochade(
        &self,
        clicked: (usize, usize),
    ) -> Option<((usize, usize), (usize, usize))> {
        let Some(clicked_figure) = self.field(clicked).figure else {
            return None;
        };

        let Some(previous) = self.field(self.selection.selected_field?).figure else {
            return None;
        };

        if !clicked_figure.first_move || !previous.first_move {
            return None;
        }

        if clicked_figure.team != previous.team {
            return None;
        }

        match (clicked_figure.figure, previous.figure) {
            (FigureType::King, FigureType::Rook) | (FigureType::Rook, FigureType::King) => {
                Some((self.selection.selected_field?, clicked))
            }
            (_, _) => None,
        }
    }

    pub fn is_rochade_valid(
        &self,
        (previous, clicked): ((usize, usize), (usize, usize)),
    ) -> Option<bool> {
        let dist = previous.1 as i32 - clicked.1 as i32;
        for mut modify in 1..dist.abs() {
            if dist.is_positive() {
                modify = -modify;
            }

            let next_col = (previous.1 as i32 + modify) as usize;
            if self.field((previous.0, next_col)).figure.is_some() {
                return None;
            }
        }

        // true on short rochade
        Some(dist.abs() == 3)
    }

    pub fn rochade_swap(&mut self, dir: i32, rook_move: i32) {
        let row = if self.player == Team::Black {
            0
        } else {
            ROWS - 1
        };
        let old_king_field = self.field_mut((row, 4));
        let mut king = old_king_field.figure.expect("King should be there");
        king.first_move = false;
        old_king_field.figure = None;

        let new_king_field = self.field_mut((row, (4 + dir) as usize));
        new_king_field.figure = Some(king);

        let rook_col = if dir.is_negative() { 0 } else { COLS - 1 };
        let old_rook_field = self.field_mut((row, rook_col));
        let mut rook = old_rook_field.figure.expect("Rook should be there");
        rook.first_move = false;
        old_rook_field.figure = None;

        let new_rook_field = self.field_mut((row, (4 + dir + rook_move) as usize));
        new_rook_field.figure = Some(rook);
    }
}

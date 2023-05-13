use std::collections::HashSet;

use macroquad::prelude::*;

use crate::{figure::Figure, Field, FigureType, Selection, Team, COLS, ROWS, SIZE, X_DIST, Y_DIST, ROWS_MAX_IDX, calc_promote_y, calc_promote_x};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Move {
    pub start_row: usize,
    pub start_col: usize,
    pub end_row: usize,
    pub end_col: usize,
}

#[derive(Debug)]
pub struct Chess {
    pub fields: [[Field; COLS]; ROWS],
    pub sprites: Option<[Texture2D; 12]>,
    pub selection: Selection,
    pub player: Team,
    pub latest_move: Option<Move>,
    pub state: State,
}

unsafe impl Send for Chess {}
unsafe impl Sync for Chess {}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Select,
    Promote((usize, usize))
}

pub fn spawn_figure(fields: &mut [[Field; COLS]; ROWS], col: usize, figure_type: FigureType) {
    let black_field = &mut fields[0][col];
    black_field.figure = Some(Figure {
        figure: figure_type,
        team: Team::Black,
        first_move: true,
    });

    let white_field = &mut fields[ROWS - 1][col];
    white_field.figure = Some(Figure {
        figure: figure_type,
        team: Team::White,
        first_move: true,
    });
}

impl Chess {
    pub fn new(sprites: Option<[Texture2D; 12]>) -> Chess {
        let mut fields = [[Field::default(); COLS]; ROWS];

        for row in 0..ROWS {
            for col in 0..COLS {
                let x = col as f32 * SIZE + X_DIST;
                let y = row as f32 * SIZE + Y_DIST;

                let field = &mut fields[row][col];
                field.idxs = (row, col);
                field.x = x;
                field.y = y;
            }
        }

        spawn_figure(&mut fields, 0, FigureType::Rook);
        spawn_figure(&mut fields, ROWS - 1, FigureType::Rook);

        spawn_figure(&mut fields, 1, FigureType::Knight);
        spawn_figure(&mut fields, ROWS - 2, FigureType::Knight);

        spawn_figure(&mut fields, 2, FigureType::Bishop);
        spawn_figure(&mut fields, ROWS - 3, FigureType::Bishop);

        spawn_figure(&mut fields, 3, FigureType::Queen);
        spawn_figure(&mut fields, 4, FigureType::King);

        for col in 0..COLS {
            let field = &mut fields[1][col];
            field.figure = Some(Figure {
                figure: FigureType::Pawn,
                team: Team::Black,
                first_move: true,
            });

            let field = &mut fields[6][col];
            field.figure = Some(Figure {
                figure: FigureType::Pawn,
                team: Team::White,
                first_move: true,
            })
        }

        Chess {
            fields,
            selection: Default::default(),
            sprites,
            player: Team::White,
            latest_move: None,
            state: State::Select
        }
    }

    pub fn draw(&self) {
        draw_rectangle_lines(
            X_DIST - 7. / 2.,
            Y_DIST - 7. / 2.,
            COLS as f32 * SIZE + 7.,
            ROWS as f32 * SIZE + 7.,
            7.,
            BLACK,
        );
        for row in 0..ROWS {
            for col in 0..COLS {
                let field = self.fields[row][col];

                let field_color = if (row + col) % 2 == 0 {
                    Color::new(166. / 255., 181. / 255., 181. / 255., 1.)
                } else {
                    Color::new(71. / 255., 135. / 255., 48. / 255., 1.)
                };

                field.draw(
                    field_color,
                    &self.sprites.expect("Sprites should be set at this moment"),
                );
            }
        }
        self.selection.draw();
    }

    pub fn check_check(&self, team: Team) -> Option<()> {
        let unique_valid_moves = self
            .fields
            .iter()
            .flatten()
            .filter_map(|field| Some((field, field.figure?)))
            .filter(|(_, figure)| figure.team != team)
            .flat_map(|(field, figure)| figure.valid_moves(field.idxs, &self.fields))
            .collect::<HashSet<(usize, usize)>>();

        let (king_field, _) = self
            .fields
            .iter()
            .flatten()
            .filter_map(|field| Some((field, field.figure?)))
            .find(|(_, figure)| figure.team == team && figure.figure == FigureType::King)?;

        if unique_valid_moves.contains(&king_field.idxs) {
            Some(())
        } else {
            None
        }
    }

    pub fn has_clicked_field(&mut self, (mouse_x, mouse_y): (f32, f32)) -> Option<(usize, usize)> {
        let row = ((mouse_y - Y_DIST) / SIZE).floor();
        let col = ((mouse_x - X_DIST) / SIZE).floor();

        if col >= 0. && row >= 0. && col < COLS as f32 && row < ROWS as f32 {
            return Some((row as usize, col as usize));
        }

        None
    }

    #[inline]
    pub fn field(&self, (row, col): (usize, usize)) -> &Field {
        &self.fields[row][col]
    }

    #[inline]
    pub fn field_mut(&mut self, (row, col): (usize, usize)) -> &mut Field {
        &mut self.fields[row][col]
    }

    pub fn select_field(&mut self, field_idx: (usize, usize)) {
        self.selection.selected_field = Some(field_idx);
    }

    pub fn move_figure(&mut self, from: (usize, usize), (row_to, col_to): (usize, usize)) {
        self.latest_move = Some(Move {
            start_row: from.0,
            start_col: from.1,
            end_row: row_to,
            end_col: col_to,
        });

        let mut figure = &mut self.field_mut(from).figure;
        if let Some(figure) = &mut figure {
            figure.first_move = false;
        }

        self.fields[row_to][col_to].figure = *figure;
        self.field_mut(from).figure = None;
    }

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
            ROWS-1
        };
        let old_king_field = self.field_mut((row, 4));
        let mut king = old_king_field.figure.expect("King should be there");
        king.first_move = false;
        old_king_field.figure = None;
        
        let new_king_field = self.field_mut((row, (4 + dir) as usize));
        new_king_field.figure = Some(king);

        let rook_col = if dir.is_negative() { 0 } else { COLS-1 };
        let old_rook_field = self.field_mut((row, rook_col));
        let mut rook = old_rook_field.figure.expect("Rook should be there");
        rook.first_move = false;
        old_rook_field.figure = None;

        let new_rook_field = self.field_mut((row, (4 + dir + rook_move) as usize));
        new_rook_field.figure = Some(rook);

    }

    pub fn promote_pawn_at(&mut self, (row, col): (usize, usize), figure: FigureType) {
        self.field_mut((row, col)).figure = Some(Figure {
            figure,
            team: self.player,
            first_move: false,
        });    
    }

    pub fn draw_promote_selection(&mut self, (mut row, col): (usize, usize)) {

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
            // draw_texture(sprite, x, y, WHITE);
            
            let mut params = DrawTextureParams::default();
            params.dest_size = Some(vec2(SIZE * 0.75, SIZE * 0.75));

            draw_texture_ex(sprite, x, y, WHITE, params);
        }
    }

    pub fn has_clicked_promotion(&self, (mut row, col): (usize, usize), (mouse_x, mouse_y): (f32, f32)) -> Option<FigureType> {

        if self.player == Team::Black {
            row -= 2;
        }

        let start_x = calc_promote_x(col);
        let start_y = calc_promote_y(row, 0);

        let x = ((mouse_x - start_x) / (SIZE * 0.75)).floor();
        let y = ((mouse_y - start_y) / (SIZE * 0.75)).floor();

        if x as usize != 0 && y as usize > 3 {
            return None
        }


        Some([
            FigureType::Queen,
            FigureType::Rook,
            FigureType::Bishop,
            FigureType::Knight,
        ][y as usize])    
    }

    pub fn handle_promote_selection(&mut self, to_promote: (usize, usize), figure: FigureType) {
        self.promote_pawn_at(to_promote, figure);
        
        self.player = !self.player;
        self.state = State::Select;
    }

    pub fn select_or_move(&mut self, clicked: (usize, usize)) {
        
        
        // unselect field if same field was clicked
        if self.selection.selected_field == Some(clicked) {
            self.selection.unselect_field();
            return;
        }

        if let Some(figures) = self.tried_rochade(clicked) {
            match (self.is_rochade_valid(figures), self.check_check(self.player)) {
                (Some(dir), None) => {
                    let rook_move = 1 + dir as i32 * -2;
                    let dir = -2 + dir as i32 * 4;
                    self.rochade_swap(dir, rook_move)
                }
                _ => {}
            }
            

            self.selection.unselect_field();
            return;
        }

        // check if a valid move was selected
        if self.selection.moves.contains(&clicked) {
            let selected_field = self.selection.selected_field.unwrap();
            
            self.move_figure(selected_field, clicked);
        
            match (clicked.0, self.player)  {
                (0, Team::White) | (ROWS_MAX_IDX, Team::Black)  => {
                    self.state = State::Promote(clicked);        
                    self.selection.unselect_field();
                    return;
                }
                _ => {}
            }

            self.selection.unselect_field();
            self.player = !self.player;
            return;
        }

        self.selection.unselect_field();

        let field = *self.field(clicked);
        if let Some(figure) = field.figure {
            // only select figures of the current player
            if figure.team != self.player {
                return;
            }
            self.selection.moves = figure.valid_moves(field.idxs, &self.fields);
        }

        self.select_field(clicked);
    }
}

#[cfg(test)]
mod tests {
    use crate::Chess;

    #[test]
    #[should_panic]
    fn test_is_king_in_check() {
        let chess = Chess::new(None);
        chess.check_check(crate::Team::Black).unwrap();
    }

    #[test]
    fn test_is_king_in_check_two() {
        let mut chess = Chess::new(None);
        let figure = chess.fields[0][3].figure.unwrap();
        chess.fields[0][3].figure = None;

        chess.fields[3][4].figure = Some(figure);
        chess.fields[6][4].figure = None;

        chess.check_check(crate::Team::White).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_is_king_in_check_two_blocked() {
        let mut chess = Chess::new(None);
        let figure = chess.fields[0][3].figure.unwrap();
        chess.fields[0][3].figure = None;

        chess.fields[3][4].figure = Some(figure);
        // chess.fields[6][4].figure = None;

        chess.check_check(crate::Team::White).unwrap();
    }

    #[test]
    #[should_panic]
    fn test_is_king_in_check_wrong_team() {
        let mut chess = Chess::new(None);
        let figure = chess.fields[0][3].figure.unwrap();
        chess.fields[0][3].figure = None;

        chess.fields[3][4].figure = Some(figure);
        // chess.fields[6][4].figure = None;

        chess.check_check(crate::Team::Black).unwrap();
    }
}

use macroquad::prelude::*;

use crate::{figure::Figure, Field, FigureType, Selection, Team, COLS, ROWS, SIZE, X_DIST, Y_DIST};

#[derive(Debug)]
pub struct Chess {
    pub fields: [[Field; COLS]; ROWS],
    pub sprites: [Texture2D; 12],
    pub selection: Selection,
    pub player: Team,
}

unsafe impl Send for Chess {}
unsafe impl Sync for Chess {}

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
    pub fn new(sprites: [Texture2D; 12]) -> Chess {
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

                field.draw(field_color, &self.sprites);
            }
        }
        self.selection.draw();
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
        let mut figure = &mut self.field_mut(from).figure;
        if let Some(figure) = &mut figure {
            figure.first_move = false;
        }

        self.fields[row_to][col_to].figure = *figure;
        self.field_mut(from).figure = None;
    }

    pub fn tried_rochade(&self, clicked: (usize, usize)) -> Option<((usize, usize), (usize, usize))> {
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
                return Some((self.selection.selected_field?, clicked))
            }
            (_, _) => return None,
        }
    }

    pub fn is_rochade_valid(&self, (previous, clicked): ((usize, usize), (usize, usize))) -> bool {
        let dist = previous.1 as i32 - clicked.1 as i32;

        for mut modify in 1..dist.abs() {
            if dist.is_positive() {
                modify = -modify;
            }

            let next_col = (previous.1 as i32 + modify) as usize;
            if self.field((previous.0, next_col)).figure.is_some() {
                return false;
            }
        }

        true
    }

    pub fn select_or_move(&mut self, clicked: (usize, usize)) {
        // unselect field if same field was clicked
        if self.selection.selected_field == Some(clicked) {
            self.selection.unselect_field();
            return;
        }

        if let Some(figures) = self.tried_rochade(clicked) {
            println!("tried rochade");
            if self.is_rochade_valid(figures) {

            }
            self.selection.unselect_field();
            return;
        }

        // check if a valid move was selected
        if self.selection.moves.contains(&clicked) {
            let selected_field = self.selection.selected_field.unwrap();
            self.player = !self.player;
            self.move_figure(selected_field, clicked);
            self.selection.unselect_field();
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

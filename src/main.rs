use std::io::Chain;

use macroquad::prelude::*;

const SIZE: f32 = 60.;
const X_DIST: f32 = 20.;
const Y_DIST: f32 = 20.;

const ROWS: usize = 8;
const COLS: usize = 8;

const COLORS: [Color; 2] = [WHITE, BLACK];

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum Team {
    White,
    Black,
}

struct MoveInfo {}

#[derive(Debug, Clone, Copy)]
enum Figure {
    Pawn(Team, bool),
    King(Team),
    Queen(Team),
    Knight(Team),
    Rook(Team),
    Bishop(Team),
}

impl Figure {
    pub fn valid_moves(
        &self,
        (row, col): (usize, usize),
        fields: &[[Field; 8]; 8],
    ) -> Vec<(usize, usize)> {
        match self {
            Figure::Pawn(team, first_move) => match team {
                Team::White => {
                    let mut moves = vec![(row - 1, col)];
                    if *first_move {
                        moves.push((row - 2, col))
                    }
                    moves
                }
                Team::Black => {
                    let mut moves = vec![(row + 1, col)];
                    if *first_move {
                        moves.push((row + 2, col))
                    }
                    moves
                }
            },
            Figure::King(_) => {
                vec![
                    (row + 1, col),
                    (row, col + 1),
                    (row - 1, col),
                    (row, col - 1),
                ]
            }
            Figure::Queen(_) => todo!(),
            Figure::Knight(_) => vec![
                is_move_valid((row + 2, col + 1), fields),
                is_move_valid((row + 2, col - 1), fields),
                is_move_valid((row - 2, col + 1), fields),
                is_move_valid((row - 2, col - 1), fields),
                is_move_valid((row - 1, col - 2), fields),
                is_move_valid((row - 1, col + 2), fields),
                is_move_valid((row + 1, col + 2), fields),
                is_move_valid((row + 1, col - 2), fields),
            ]
            .into_iter()
            .flatten()
            .collect(),
            Figure::Rook(_) => (1..8)
                .map_while(|add| is_move_valid((row, col + add), fields))
                .chain((1..8).map_while(|add| is_move_valid((row, col - add), fields)))
                .chain((1..8).map_while(|add| is_move_valid((row + add, col), fields)))
                .chain((1..8).map_while(|add| is_move_valid((row - add, col), fields)))
                .collect::<Vec<(usize, usize)>>(),
            Figure::Bishop(_) => (1..8)
                .map_while(|add| is_move_valid((row + add, col + add), fields))
                .chain((1..8).map_while(|sub| is_move_valid((row - sub, col - sub), fields)))
                .chain((1..8).map_while(|add| is_move_valid((row - add, col + add), fields)))
                .chain((1..8).map_while(|add| is_move_valid((row + add, col - add), fields)))
                .collect::<Vec<(usize, usize)>>(),
        }
    }
}


fn is_move_valid((row, col): (usize, usize), fields: &[[Field; 8]; 8]) -> Option<(usize, usize)> {
    if row >= ROWS || col >= COLS {
        return None;
    }

    match fields[row][col].figure {
        Some(_) => None,
        None => Some((row, col)),
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Field {
    figure: Option<Figure>,
    selected: bool,
    idxs: (usize, usize),
    x: f32,
    y: f32,
}

impl Field {
    pub fn draw(&self, field_color: Color) {
        draw_rectangle(self.x, self.y, SIZE, SIZE, field_color);

        if let Some(figure) = self.figure {
            match figure {
                Figure::Pawn(team, _) => draw_rectangle(
                    self.x + 15.,
                    self.y + 15.,
                    SIZE / 2.,
                    SIZE / 2.,
                    COLORS[team as usize],
                ),
                Figure::King(team) => match team {
                    Team::White => todo!(),
                    Team::Black => todo!(),
                },
                Figure::Queen(team) => match team {
                    Team::White => todo!(),
                    Team::Black => todo!(),
                },
                Figure::Knight(team) => draw_rectangle(
                    self.x + 15.,
                    self.y + 15.,
                    SIZE / 2.,
                    SIZE / 2.,
                    COLORS[team as usize],
                ),
    
                Figure::Rook(team) => draw_rectangle(
                    self.x + 15.,
                    self.y + 15.,
                    SIZE / 2.,
                    SIZE / 2.,
                    COLORS[team as usize],
                ),
                Figure::Bishop(team) => draw_rectangle(
                    self.x + 15.,
                    self.y + 15.,
                    SIZE / 2.,
                    SIZE / 2.,
                    COLORS[team as usize],
                ),
            }

            if self.selected {
                draw_rectangle_lines(
                    self.x + 15.,
                    self.y + 15.,
                    SIZE / 2.,
                    SIZE / 2.,
                    6.,
                    DARKGREEN,
                );
            }
        }
    }
}

#[derive(Debug)]
pub struct Chess {
    fields: [[Field; COLS]; ROWS],
}

impl Chess {
    pub fn new() -> Chess {
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

        let field = &mut fields[3][3];
        field.figure = Some(Figure::Bishop(Team::White));

        for col in 0..COLS {
            let field = &mut fields[1][col];
            field.figure = Some(Figure::Pawn(Team::Black, true));
        }

        for col in 0..COLS {
            let field = &mut fields[6][col];
            field.figure = Some(Figure::Pawn(Team::White, true));
        }

        Chess { fields }
    }

    pub fn draw(&self) {
        let mut white = true;
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

                let field_color = if white {
                    Color::new(166. / 255., 181. / 255., 181. / 255., 1.)
                } else {
                    Color::new(71. / 255., 135. / 255., 48. / 255., 1.)
                };

                field.draw(field_color);

                if col == COLS - 1 {
                    if row % 2 == 0 {
                        white = false;
                    } else {
                        white = true;
                    }
                } else {
                    white = !white;
                }
            }
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
}

#[macroquad::main("Chess")]
async fn main() {
    let mut chess = Chess::new();

    let mut selected_field: Option<(usize, usize)> = None;

    let mut moves = vec![];

    loop {
        clear_background(WHITE);

        chess.draw();

        if is_mouse_button_pressed(MouseButton::Left) {
            let field = chess.has_clicked_field(mouse_position());
            if let Some((row, col)) = field {
                if let Some((row, col)) = selected_field {
                    chess.fields[row][col].selected = false;
                    moves = vec![];
                }

                let field = &mut chess.fields[row][col];
                field.selected = true;
                println!("field: {field:?}");
                selected_field = Some(field.idxs);

                if let Some(figure) = field.figure {
                    moves = figure.valid_moves(field.idxs, &chess.fields);
                }
            }
        }

        if let Some((row, col)) = selected_field {
            let field = chess.fields[row][col];
            // show valid moves
            for (row, col) in &moves {
                let field = chess.fields[*row][*col];
                draw_circle(field.x + SIZE / 2., field.y + SIZE / 2., 12., YELLOW);
            }
        }

        next_frame().await;
    }
}

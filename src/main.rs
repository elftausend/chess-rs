use macroquad::prelude::*;

const SIZE: f32 = 60.;
const X_DIST: f32 = 20.;
const Y_DIST: f32 = 20.;

const ROWS: usize = 8;
const COLS: usize = 8;

#[derive(Debug, Clone, Copy)]
enum Team {
    White,
    Black
}

#[derive(Debug, Clone, Copy)]
enum Figure {
    Pawn(Team),
    King(Team),
    Queen(Team),
    Knight(Team),
    Rook(Team),
    Bishop(Team)
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Field {
    figure: Option<Figure>,
    selected: bool,
    idxs: (usize, usize),
    x: f32,
    y: f32
}

#[derive(Debug)]
pub struct Chess {
    fields: [[Field; COLS]; ROWS]
}

impl Chess {
    pub fn new() -> Chess {
        let mut fields = [[Field::default(); COLS]; ROWS];
        
        for row in 0..ROWS {
            for col in 0..COLS {
                let x = row as f32 * SIZE + X_DIST;
                let y = col as f32 * SIZE + Y_DIST;

                let field = &mut fields[row][col];
                field.idxs = (row, col);
                field.x = x;
                field.y = y;
            }
        }

        for col in 0..COLS {
            let field = &mut fields[1][col];
            //field.figure = Some(Fig);
        }

        Chess {
            fields
        }
    }

    fn draw_figure(&self, figure: Figure) {
        match figure {
            Figure::Pawn(team) => match team {
                Team::White => todo!(),
                Team::Black => todo!(),
            },
            Figure::King(team) => match team {
                Team::White => todo!(),
                Team::Black => todo!(),
            },
            Figure::Queen(team) => match team {
                Team::White => todo!(),
                Team::Black => todo!(),
            },
            Figure::Knight(team) => match team {
                Team::White => todo!(),
                Team::Black => todo!(),
            },
            Figure::Rook(team) => match team {
                Team::White => todo!(),
                Team::Black => todo!(),
            },
            Figure::Bishop(team) => match team {
                Team::White => todo!(),
                Team::Black => todo!(),
            },
        }
    }

    pub fn draw(&self) {
        let mut white = true;
        draw_rectangle_lines(X_DIST-7. / 2., Y_DIST-7. / 2., COLS as f32*SIZE+7., ROWS as f32*SIZE+7., 7., BLACK);
        for (row_idx, row) in self.fields.iter().enumerate() {
            for (idx, field) in row.iter().enumerate() {
                let x = field.x;
                let y = field.y;


                let color = if white {
                    Color::new(166. / 255., 181. / 255., 181. / 255., 1.)
                } else {
                    Color::new(71. / 255., 135. / 255., 48. / 255., 1.)
                };
                draw_rectangle(x, y, SIZE, SIZE, color);

                if let Some(figure) = field.figure {
                    self.draw_figure(figure);
                }
                draw_rectangle(x + 15., y + 15., SIZE / 2., SIZE / 2., GREEN);
                
                if field.selected {
                    draw_rectangle_lines(x + 15., y + 15., SIZE / 2., SIZE / 2.,  6., DARKGREEN);
                }

                if idx == COLS-1 {
                    if row_idx % 2 == 0 {
                        white = false;
                    } else {
                        white = true;
                    }
                }
        
                else {
                    white = !white;
                }
                
            }
        }
    }

    pub fn has_clicked_field(&mut self, (mouse_x, mouse_y): (f32, f32)) -> Option<(usize, usize)> {
        let row = ((mouse_y - Y_DIST) / SIZE).floor();
        let col = ((mouse_x - X_DIST) / SIZE).floor();

        if col >= 0. && row >= 0. && col < COLS as f32 && row < ROWS as f32 {
            return Some((col as usize, row as usize));
        }
        
        None
    }
}

#[macroquad::main("Chess")]
async fn main() {
    let mut chess = Chess::new();

    let mut selected_field: Option<(usize, usize)> = None;

    loop {
        clear_background(WHITE);
        
        chess.draw();

        if is_mouse_button_pressed(MouseButton::Left) {
            let field = chess.has_clicked_field(mouse_position());
            if let Some((row, col)) = field {

                if let Some((row, col)) = selected_field {
                    chess.fields[row][col].selected = false;  
                }
                
                let field = &mut chess.fields[row][col];
                field.selected = true;
                println!("field: {field:?}");
                selected_field = Some(field.idxs);
            }
        }

        next_frame().await;
    }
}

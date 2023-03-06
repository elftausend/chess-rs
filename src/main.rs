mod valid_moves;

use lazy_static::lazy_static;
use macroquad::prelude::*;
use valid_moves::{bishop_moves, is_move_valid, rook_moves, ValidMoves};

const SIZE: f32 = 60.;
const X_DIST: f32 = 20.;
const Y_DIST: f32 = 20.;

const ROWS: usize = 8;
const COLS: usize = 8;

const COLORS: [Color; 2] = [WHITE, BLACK];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum Team {
    White,
    Black,
}
impl ToString for Team {
    fn to_string(&self) -> String {
        match self {
            Team::White => "white".to_string(),
            Team::Black => "black".to_string(),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Figure {
    figure: FigureType,
    team: Team,
    first_move: bool,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum FigureType {
    Pawn,
    King,
    Queen,
    Knight,
    Rook,
    Bishop,
}

impl Figure {
    pub fn valid_moves(
        &self,
        (row, col): (usize, usize),
        fields: &[[Field; 8]; 8],
    ) -> Vec<(usize, usize)> {
        match self.figure {
            FigureType::Pawn => match self.team {
                Team::White => {
                    let mut moves = vec![];
                    if let Some(mv) = is_move_valid((row - 1, col), fields) {
                        moves.push(mv)
                    } else {
                        return vec![];
                    }

                    if self.first_move {
                        if let Some(mv) = is_move_valid((row - 2, col), fields) {
                            moves.push(mv)
                        }
                    }
                    moves
                }
                Team::Black => {
                    let mut moves = vec![(row + 1, col)];
                    if self.first_move {
                        if let Some(mv) = is_move_valid((row + 2, col), fields) {
                            moves.push(mv)
                        }
                    }
                    moves
                }
            },
            FigureType::King => {
                vec![
                    (row + 1, col),
                    (row, col + 1),
                    (row - 1, col),
                    (row, col - 1),
                ]
            }
            FigureType::Queen => bishop_moves((row, col), fields, self.team)
                .into_iter()
                .chain(rook_moves((row, col), fields, self.team))
                .collect(),
            FigureType::Knight => [
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
            FigureType::Bishop => bishop_moves((row, col), fields, self.team).into_iter().collect(),
            FigureType::Rook => rook_moves((row, col), fields, self.team).into_iter().collect(),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Field {
    figure: Option<Figure>,
    idxs: (usize, usize),
    x: f32,
    y: f32,
}

impl Field {

    pub fn draw_sprite(&self, sprite: Texture2D) {
        draw_texture(
            sprite,
            self.x - 3.,
            self.y,
            WHITE,
        );
    }

    pub async fn draw(&self, field_color: Color, sprites: &[Texture2D; 12]) {
        draw_rectangle(self.x, self.y, SIZE, SIZE, field_color);
        
        if let Some(figure) = self.figure {
            self.draw_sprite(sprites[figure.figure as usize + figure.team as usize * 6]);
        }
    }
}

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
            draw_rectangle_lines(x + 15., y + 15., SIZE / 2., SIZE / 2., 6., DARKGREEN);
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

#[derive(Debug)]
pub struct Chess {
    fields: [[Field; COLS]; ROWS],
    sprites: [Texture2D; 12],
    selection: Selection,
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

        let field = &mut fields[3][3];
        field.figure = Some(Figure { figure: FigureType::Bishop, team: Team::White, first_move: true });
        
        let field = &mut fields[4][4];
        field.figure = Some(Figure { figure: FigureType::Queen, team: Team::Black, first_move: true });

        for col in 0..COLS {
            let field = &mut fields[1][col];
            field.figure = Some(Figure { figure: FigureType::Pawn, team: Team::Black, first_move: true })
        }

        for col in 0..COLS {
            let field = &mut fields[6][col];
            field.figure = Some(Figure { figure: FigureType::Pawn, team: Team::White, first_move: true })
        }

        Chess {
            fields,
            selection: Default::default(),
            sprites
        }
    }

    pub async fn draw(&self) {
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

                field.draw(field_color, &self.sprites).await;
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

    pub fn select_field(&mut self, field_idx: (usize, usize)) -> &mut Field {
        self.selection.selected_field = Some(field_idx);
        let field = self.field_mut(field_idx);
        field
    }

    pub fn move_figure(&mut self, from: (usize, usize), (row_to, col_to): (usize, usize)) {
        let mut figure = &mut self.field_mut(from).figure;
        if let Some(figure) = &mut figure {
            figure.first_move = false;
        }

        self.fields[row_to][col_to].figure = *figure;
        self.field_mut(from).figure = None;
    }

    pub fn select_or_move(&mut self, clicked: (usize, usize)) {
        if self.selection.moves.contains(&clicked) {
            let selected_field = self.selection.selected_field.unwrap();

            self.move_figure(selected_field, clicked);
            self.selection.unselect_field();
            return;
        }

        self.selection.unselect_field();
        let field = self.select_field(clicked);

        if let Some(figure) = field.figure {
            self.selection.moves = figure.valid_moves(field.idxs, &self.fields);
        }
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
        clear_background(WHITE);

        chess.draw().await;

        if is_mouse_button_pressed(MouseButton::Left) {
            let field = chess.has_clicked_field(mouse_position());
            if let Some(clicked) = field {
                chess.select_or_move(clicked);
            }
        }

        next_frame().await;
    }
}

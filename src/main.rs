use macroquad::prelude::*;

const SIZE: f32 = 60.;
const X_DIST: f32 = 20.;
const Y_DIST: f32 = 20.;

#[derive(Debug, Default, Clone, Copy)]
pub struct Field {
    selected: bool,
    x: f32,
    y: f32
}

#[derive(Debug)]
pub struct Chess {
    fields: [[Field; 8]; 8]
}

impl Chess {
    pub fn new() -> Chess {
        let mut fields = [[Field::default(); 8]; 8];
        
        for row in 0..8 {
            for col in 0..8 {
                let x = row as f32 * SIZE + X_DIST;
                let y = col as f32 * SIZE + Y_DIST;

                fields[row][col].x = x;
                fields[row][col].y = y;
            }
        }
        Chess {
            fields
        }
    }

    pub fn draw(&self) {
        let mut white = true;
        draw_rectangle_lines(X_DIST-7. / 2., Y_DIST-7. / 2., 8.*SIZE+7., 8.*SIZE+7., 7., BLACK);
        for (row_idx, row) in self.fields.iter().enumerate() {
            for (idx, field) in row.iter().enumerate() {
                let x = field.x;
                let y = field.y;

                if white {
                    draw_rectangle(x, y, SIZE, SIZE, WHITE);   
                } else {
                    draw_rectangle(x, y, SIZE, SIZE, BLACK);
                }

                draw_rectangle(x + 15., y + 15., SIZE / 2., SIZE / 2., GREEN);
                
                if field.selected {
                    draw_rectangle_lines(x + 15., y + 15., SIZE / 2., SIZE / 2.,  6., DARKGREEN);
                }

                if idx == 7 {
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

    pub fn has_clicked_field(&mut self, click_pos: (f32, f32)) -> Option<&mut Field> {
        for row in 0..8 {
            for col in 0..8 {
                let field = &mut self.fields[row][col];

                if click_pos.0 >= field.x && click_pos.0 <= field.x+SIZE
                    && click_pos.1 >= field.y && click_pos.1 <= field.y+SIZE
                {
                    return Some(&mut self.fields[row][col])
                }
            }
        }
        None
    }
}

#[macroquad::main("Chess")]
async fn main() {
    let mut chess = Chess::new();

    loop {
        clear_background(WHITE);
        
        chess.draw();

        if is_mouse_button_pressed(MouseButton::Left) {
            let field = chess.has_clicked_field(mouse_position());
            if let Some(field) = field {
                field.selected = true;
                println!("field: {field:?}")
            }
        }

        next_frame().await;
    }
}

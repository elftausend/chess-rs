mod chess;
mod figure;
mod valid_moves;
pub use chess::Chess;
mod field;
mod selection;

use macroquad::{texture::{Texture2D, load_image}, window::{clear_background, next_frame}, prelude::{DARKGRAY, is_mouse_button_pressed, MouseButton, mouse_position}};
pub use selection::*;
pub use field::*;
pub use figure::*;

const SIZE: f32 = 60.;
const X_DIST: f32 = 20.;
const Y_DIST: f32 = 20.;

const ROWS: usize = 8;
const COLS: usize = 8;

pub async fn sprites() -> [Texture2D; 12] {
    [
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
    ]
}


#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ChessCreationWrapper(*mut *mut Chess);

unsafe impl Send for ChessCreationWrapper {}
unsafe impl Sync for ChessCreationWrapper {}

#[no_mangle]
pub extern fn chess_create(chess: ChessCreationWrapper) {
    tokio::task::spawn(async move {
        let chess = chess;
        unsafe {
            **chess.0 = Chess::new(sprites().await);
        }
    });
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ChessWrapper(*mut Chess);

unsafe impl Send for ChessWrapper {}
unsafe impl Sync for ChessWrapper {}

#[no_mangle]
pub extern fn chess_run(chess: ChessWrapper) {
    tokio::task::spawn(async move {
        let chess = chess;

        // let chess = unsafe {&mut *chess.0};
        loop {
            unsafe {&mut *chess.0}.draw();
            
            if is_mouse_button_pressed(MouseButton::Left) {
                let field = unsafe {&mut *chess.0}.has_clicked_field(mouse_position());
                if let Some(clicked) = field {
                    unsafe {&mut *chess.0}.select_or_move(clicked)
                }
            }

            next_frame().await;
        }
    });
}

#[no_mangle]
pub extern fn chess_move(chess: ChessWrapper, start_row: usize, start_col: usize, end_row: usize, end_col: usize) {
    let chess = unsafe {&mut *chess.0};
    chess.move_figure((start_row, start_col), (end_row, end_col));
    
    chess.player = !chess.player; 
}

// pub extern fn chess_last_sele

#[no_mangle]
pub extern fn free_chess(chess: *mut Chess) {
    //
}

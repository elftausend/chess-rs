mod chess;
mod figure;
mod valid_moves;
pub use chess::Chess;
mod field;
mod selection;

pub use chess::*;
pub use field::*;
pub use figure::*;
use macroquad::prelude::*;
pub use selection::*;
use tokio::runtime::Runtime;

const SIZE: f32 = 60.;
const X_DIST: f32 = 20.;
const Y_DIST: f32 = 20.;

const ROWS: usize = 8;
const ROWS_MAX_IDX: usize = 7;
const COLS: usize = 8;

// pub static RT: Lazy<Runtime> = Lazy::new(|| Runtime::new().unwrap());

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

#[no_mangle]
pub extern "C" fn chess_create(chess: *mut *mut Chess) {
    unsafe {
        *chess = Box::into_raw(Box::new(Chess::new(None)));
    }
}

#[derive(Debug, Clone, Copy)]
#[repr(C)]
pub struct ChessWrapper(pub *mut Chess);

unsafe impl Send for ChessWrapper {}
unsafe impl Sync for ChessWrapper {}

#[no_mangle]
pub extern "C" fn chess_get_current_team(chess: *mut Chess) -> Team {
    unsafe { (*chess).player }
}

#[no_mangle]
pub extern "C" fn chess_set_current_team(chess: *mut Chess, team: Team) {
    unsafe { (*chess).player = team }
}

#[no_mangle]
pub extern "C" fn chess_get_latest_move(chess: *mut Chess) -> Move {
    unsafe { (*chess).latest_move.expect("No move is available.") }
}

#[no_mangle]
pub extern "C" fn chess_get_state(chess: *mut Chess) -> State {
    unsafe { (*chess).state }
}

#[no_mangle]
pub extern "C" fn chess_run(chess: ChessWrapper) {
    Runtime::new().unwrap().block_on(async {
        //tokio::task::spawn(async move {
        macroquad::Window::from_config(Conf::default(), async move {
            let chess = chess;

            unsafe { &mut *chess.0 }.sprites = Some(sprites().await);

            // let chess = unsafe {&mut *chess.0};
            loop {
                unsafe { &mut *chess.0 }.draw();

                if is_mouse_button_pressed(MouseButton::Left) {
                    let field = unsafe { &mut *chess.0 }.has_clicked_field(mouse_position());
                    if let Some(clicked) = field {
                        unsafe { &mut *chess.0 }.select_or_move(clicked)
                    }
                }

                next_frame().await;
            }
        });
        //})
    });
}

#[no_mangle]
pub extern "C" fn chess_move(
    chess: ChessWrapper,
    start_row: usize,
    start_col: usize,
    end_row: usize,
    end_col: usize,
) {
    let chess = unsafe { &mut *chess.0 };
    chess.move_figure((start_row, start_col), (end_row, end_col));

    chess.player = !chess.player;
}

// pub extern fn chess_last_sele

#[no_mangle]
pub extern "C" fn chess_free(chess: *mut Chess) {
    unsafe {
        drop(Box::from_raw(chess));
    };
}

#[cfg(test)]
mod tests {
    use std::ptr::null_mut;

    use crate::{chess_create, chess_run, Chess, ChessWrapper};

    #[test]
    #[ignore = ""]
    fn test_raw_lib() {
        let mut chess: *mut Chess = null_mut();
        chess_create(&mut chess);

        chess_run(ChessWrapper(chess));
        loop {}
    }
}

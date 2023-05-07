use std::ptr::null_mut;

use chess_rs::{chess_create, chess_run, ChessWrapper, ChessCreationWrapper, Chess};


fn main() {
    let mut chess: *mut Chess = null_mut();
    chess_create(ChessCreationWrapper(&mut chess));
    println!("chess: {chess:?}");

    chess_run(ChessWrapper(chess));
    loop {
         
    }
}
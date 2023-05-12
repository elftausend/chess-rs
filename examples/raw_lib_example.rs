use chess_rs::{chess_create, chess_get_latest_move, chess_move, chess_run, ChessWrapper, Move};
use std::ptr::null_mut;

fn main() {
    let mut chess: ChessWrapper = ChessWrapper(null_mut());

    chess_create(&mut chess.0);

    std::thread::spawn(move || {
        let chess = chess;
        std::thread::sleep(std::time::Duration::from_millis(2000));

        chess_move(chess, 1, 1, 2, 1);

        let mv = chess_get_latest_move(chess.0);
        assert_eq!(
            mv,
            Move {
                start_row: 1,
                start_col: 1,
                end_row: 2,
                end_col: 1
            }
        );
    });

    chess_run(ChessWrapper(chess.0));
}

use std::ops::Not;

use crate::{
    valid_moves::{bishop_moves, is_move_valid, pawn_moves, rook_moves},
    Field,
};

#[derive(Debug, Clone, Copy)]
pub struct Figure {
    pub figure: FigureType,
    pub team: Team,
    pub first_move: bool,
}

impl Figure {
    pub fn valid_moves(
        &self,
        (row, col): (usize, usize),
        fields: &[[Field; 8]; 8],
    ) -> Vec<(usize, usize)> {
        match self.figure {
            FigureType::Pawn => pawn_moves((row, col), fields, self.team, self.first_move),
            FigureType::King => vec![
                is_move_valid((row + 1, col), fields, self.team),
                is_move_valid((row + 1, col - 1), fields, self.team),
                is_move_valid((row + 1, col + 1), fields, self.team),
                is_move_valid((row, col + 1), fields, self.team),
                is_move_valid((row - 1, col), fields, self.team),
                is_move_valid((row - 1, col + 1), fields, self.team),
                is_move_valid((row - 1, col - 1), fields, self.team),
                is_move_valid((row, col - 1), fields, self.team),
            ]
            .into_iter()
            .flatten()
            .collect(),
            FigureType::Queen => bishop_moves((row, col), fields, self.team)
                .into_iter()
                .chain(rook_moves((row, col), fields, self.team))
                .collect(),
            FigureType::Knight => [
                is_move_valid((row + 2, col + 1), fields, self.team),
                is_move_valid((row + 2, col - 1), fields, self.team),
                is_move_valid((row - 2, col + 1), fields, self.team),
                is_move_valid((row - 2, col - 1), fields, self.team),
                is_move_valid((row - 1, col - 2), fields, self.team),
                is_move_valid((row - 1, col + 2), fields, self.team),
                is_move_valid((row + 1, col + 2), fields, self.team),
                is_move_valid((row + 1, col - 2), fields, self.team),
            ]
            .into_iter()
            .flatten()
            .collect(),
            FigureType::Bishop => bishop_moves((row, col), fields, self.team)
                .into_iter()
                .collect(),
            FigureType::Rook => rook_moves((row, col), fields, self.team)
                .into_iter()
                .collect(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
pub enum Team {
    White = 0,
    Black = 1,
}

impl Not for Team {
    type Output = Team;

    fn not(self) -> Self::Output {
        match self {
            Team::White => Team::Black,
            Team::Black => Team::White,
        }
    }
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
#[repr(u8)]
pub enum FigureType {
    Pawn,
    King,
    Queen,
    Knight,
    Rook,
    Bishop,
}

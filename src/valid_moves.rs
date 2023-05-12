use crate::{Field, Team, COLS, ROWS};

pub struct ValidMovesIter<'a> {
    row: usize,
    col: usize,
    fadds: std::vec::IntoIter<fn(i16, i16, i16) -> (i16, i16)>,
    add: i16,
    fields: &'a [[Field; 8]; 8],
    started_invalid: bool,
    fadd: Option<fn(i16, i16, i16) -> (i16, i16)>,
    team: Team,
}

impl<'a> ValidMovesIter<'a> {
    pub fn next_fn(&mut self) {
        self.add = 1;
        self.fadd = self.fadds.next();
        self.started_invalid = false;
    }
}

impl<'a> Iterator for ValidMovesIter<'a> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.add > 8 {
            self.next_fn();
            // return None;
        }

        if self.started_invalid {
            self.next_fn();
        }

        self.fadd?;

        let mv = (self.fadd.as_ref().unwrap())(self.row as i16, self.col as i16, self.add);

        if is_out_of_bounds(mv) {
            self.next_fn();
            return self.next();
        }
        let mv = (mv.0 as usize, mv.1 as usize);

        if let Some(figure) = self.fields[mv.0][mv.1].figure {
            if self.team == figure.team {
                self.next_fn();
                return self.next();
            }
        }

        self.started_invalid = is_figure_on_field(mv, self.fields);

        self.add += 1;
        Some(mv)
    }
}

pub struct ValidMoves<'a> {
    row: usize,
    col: usize,
    f: Vec<fn(i16, i16, i16) -> (i16, i16)>,
    fields: &'a [[Field; 8]; 8],
    team: Team,
}

impl<'a> ValidMoves<'a> {
    pub fn new(
        row: usize,
        col: usize,
        f: Vec<fn(i16, i16, i16) -> (i16, i16)>,
        fields: &'a [[Field; 8]; 8],
        team: Team,
    ) -> Self {
        Self {
            row,
            col,
            f,
            fields,
            team,
        }
    }
}

impl<'a> IntoIterator for ValidMoves<'a> {
    type Item = (usize, usize);

    type IntoIter = ValidMovesIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        let mut valid_moves_iter = ValidMovesIter {
            row: self.row,
            col: self.col,
            fadds: self.f.into_iter(),
            add: 1,
            fields: self.fields,
            started_invalid: false,
            fadd: None,
            team: self.team,
        };
        valid_moves_iter.next_fn();
        valid_moves_iter
    }
}

pub fn is_move_valid(
    (row, col): (i16, i16),
    fields: &[[Field; 8]; 8],
    team: Team,
) -> Option<(usize, usize)> {
    let (row, col) = (row as usize, col as usize);
    if row >= ROWS || col >= COLS {
        return None;
    }

    match fields[row][col].figure {
        Some(figure) => {
            if team == figure.team {
                None
            } else {
                Some((row, col))
            }
        }
        None => Some((row, col)),
    }
}

pub fn is_pawn_move_valid(
    (row, col): (usize, usize),
    fields: &[[Field; 8]; 8],
) -> Option<(usize, usize)> {
    if row >= ROWS || col >= COLS {
        return None;
    }

    match fields[row][col].figure {
        Some(_) => None,
        None => Some((row, col)),
    }
}

#[inline]
fn is_out_of_bounds((row, col): (i16, i16)) -> bool {
    row as usize >= ROWS || col as usize >= COLS
}

#[inline]
fn is_figure_on_field((row, col): (usize, usize), fields: &[[Field; 8]; 8]) -> bool {
    fields[row][col].figure.is_some()
}

pub fn bishop_moves<'a>(
    (row, col): (usize, usize),
    fields: &'a [[Field; 8]; 8],
    team: Team,
) -> ValidMoves<'_> {
    let (row, col) = (row as i16, col as i16);
    ValidMoves::new(
        row as usize,
        col as usize,
        vec![
            |row, col, add| (row + add, col + add),
            |row, col, add| (row - add, col.overflowing_sub(add).0),
            |row, col, add| (row - add, col + add),
            |row, col, add| (row + add, col - add),
        ],
        fields,
        team,
    )
}

pub fn rook_moves<'a>(
    (row, col): (usize, usize),
    fields: &'a [[Field; 8]; 8],
    team: Team,
) -> ValidMoves<'_> {
    ValidMoves::new(
        row,
        col,
        vec![
            |row, col, add| (row, col + add),
            |row, col, add| (row, col - add),
            |row, col, add| (row + add, col),
            |row, col, add| (row - add, col),
        ],
        fields,
        team,
    )
}

pub fn pawn_moves(
    (row, col): (usize, usize),
    fields: &[[Field; 8]; 8],
    team: Team,
    first_move: bool,
) -> Vec<(usize, usize)> {
    let (row, first_move_row) = match team {
        Team::White => (row - 1, row - 2),
        Team::Black => (row + 1, row + 2),
    };

    let mut moves = vec![];

    if col != 0 {
        if let Some(figure) = fields[row][col - 1].figure {
            if team != figure.team {
                moves.push((row, col - 1));
            }
        }
    }
    if col + 1 < COLS {
        if let Some(figure) = fields[row][col + 1].figure {
            if team != figure.team {
                moves.push((row, col + 1));
            }
        }
    }

    if let Some(mv) = is_pawn_move_valid((row, col), fields) {
        moves.push(mv)
    } else {
        return moves;
    }

    if first_move {
        let mv = (first_move_row, col);
        if !is_out_of_bounds((mv.0 as i16, mv.1 as i16)) && !is_figure_on_field(mv, fields) {
            moves.push(mv);
        }
        if let Some(mv) = is_pawn_move_valid((first_move_row, col), fields) {
            moves.push(mv)
        }
    }
    moves
}

#[cfg(test)]
mod tests {
    /*
    #[test]
    fn test_valid_move_iter() {
        let chess = Chess::new();

        let valid_moves = ValidMoves {
            row: 4,
            col: 4,
            f: vec![|row, col, add| (row, col + add), |row, col, add| {
                (row - add, col + add)
            }],
            fields: &chess.fields,
        };

        for mv in valid_moves.into_iter() {
            println!("mv: {mv:?}");
        }
    }
    */
}

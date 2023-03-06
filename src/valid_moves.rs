use crate::{Field, COLS, ROWS};


pub struct ValidMovesIter<'a> {
    row: usize,
    col: usize,
    fadds: std::vec::IntoIter<fn(usize, usize, usize) -> (usize, usize)>,
    add: usize,
    fields: &'a [[Field; 8]; 8],
    started_invalid: bool,
    fadd: Option<fn(usize, usize, usize) -> (usize, usize)>,
}

impl<'a> ValidMovesIter<'a> {
    pub fn next_fn(&mut self) {
        self.add = 1;
        self.fadd = self.fadds.next();
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

        if self.fadd.is_none() {
            return None;
        }

        let mv = (self.fadd.as_ref().unwrap())(self.row, self.col, self.add);

        if is_out_of_bounds(mv) {
            self.next_fn();
            return self.next();
        }

        self.started_invalid = is_figure_on_field(mv, self.fields);

        self.add += 1;
        Some(mv)
    }
}

pub struct ValidMoves<'a> {
    row: usize,
    col: usize,
    f: Vec<fn(usize, usize, usize) -> (usize, usize)>,
    fields: &'a [[Field; 8]; 8],
}

impl<'a> ValidMoves<'a> {
    pub fn new(
        row: usize,
        col: usize,
        f: Vec<fn(usize, usize, usize) -> (usize, usize)>,
        fields: &'a [[Field; 8]; 8],
    ) -> Self {
        Self {
            row,
            col,
            f,
            fields,
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
        };
        valid_moves_iter.next_fn();
        valid_moves_iter
    }
}

pub fn is_move_valid((row, col): (usize, usize), fields: &[[Field; 8]; 8]) -> Option<(usize, usize)> {
    if row >= ROWS || col >= COLS {
        return None;
    }

    match fields[row][col].figure {
        Some(_) => None,
        None => Some((row, col)),
    }
}

fn is_out_of_bounds((row, col): (usize, usize)) -> bool {
    if row >= ROWS || col >= COLS {
        true
    } else {
        false
    }
}

fn is_figure_on_field((row, col): (usize, usize), fields: &[[Field; 8]; 8]) -> bool {
    match fields[row][col].figure {
        Some(_) => true,
        None => false,
    }
}

pub fn bishop_moves<'a>((row, col): (usize, usize), fields: &'a [[Field; 8]; 8]) -> ValidMoves<'a> {
    ValidMoves::new(
        row,
        col,
        vec![
            |row, col, add| (row + add, col + add),
            |row, col, add| (row - add, col - add),
            |row, col, add| (row - add, col + add),
            |row, col, add| (row + add, col - add),
        ],
        fields,
    )
}

pub fn rook_moves<'a>((row, col): (usize, usize), fields: &'a [[Field; 8]; 8]) -> ValidMoves<'a> {
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
    )
}

#[cfg(test)]
mod tests {
    use crate::{Chess, ValidMoves};

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
}

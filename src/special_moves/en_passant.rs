use crate::{Chess, FigureType, Team};

// ilhan:

impl Chess {
    pub fn check_en_passant(&self, mover_team: Team, pos: (usize, usize)) -> bool {
        let Some(figure) = self.field(pos).figure else {
            return false;
        };
        if figure.team == mover_team && figure.figure != FigureType::Pawn {
            return false;
        }

        true
    }

    pub fn add_to_en_passant_if_checked(
        &mut self,
        mover_team: Team,
        check_pos: (usize, usize),
        moved_pawn_pos: (usize, usize),
    ) {
        if self.check_en_passant(mover_team, check_pos) {
            self.en_passants
                .insert((check_pos.0, check_pos.1, !mover_team), moved_pawn_pos);
        }
    }

    pub fn remove_if_en_passant_pawn(&mut self, mover_team: Team, origin: (usize, usize), moved_to: (usize, usize)) {
        let Some(en_passant_pawn) = self.en_passants.get(&(origin.0, origin.1, mover_team)) else {
            return;
        };

        let maybe_en_passant_pos = if mover_team == Team::White {
            (moved_to.0 + 1, moved_to.1)
        } else {
            (moved_to.0 - 1, moved_to.1)
        };

        if *en_passant_pawn == maybe_en_passant_pos {
            self.field_mut(*en_passant_pawn).figure = None;
        }

    }

    pub fn invalidate_en_passants(&mut self) {
        self.en_passants.clear();
    }
}

use crate::piece::Piece;
use crate::types::{Color, Position};

#[derive(Clone,Debug,Default)]
pub struct Board {
    squares: [[Option<Piece>;8]; 8],
    pub turn: Color,
}

impl Board {
    pub fn set (&mut self, pos: Position, piece: Option<Piece>) {
        self.squares[pos.row][pos.col] = match Position::is_valid(pos.row as i8, pos.col as i8) {
            true => piece,
            false => return
        }
    }

    pub fn take (&mut self, pos: Position) -> Option<Piece> {
        match Position::is_valid(pos.row as i8, pos.col as i8) {
            true => Some(self.squares[pos.row][pos.col].take()?),
            false => return None
        }
    }

    pub fn apply_move (&mut self, from: Position, to: Position) -> () {
        let mut piece = match self.take(from) {
            Some(p) => p,
            None => return,
        };

        {
            let moves = piece.data.legal_moves(from, piece.color, self);
            if !moves.contains(&to) {
                self.set(from, Some(piece));
                return;
            }
        }

        piece.data.on_move(from, to, self);

        piece.pos = to;
        self.set(to, Some(piece));
    }
}

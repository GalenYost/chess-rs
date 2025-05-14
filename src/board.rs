use crate::piece::{Piece, pawn::PawnData};
use crate::types::{Color, Name, Position};

#[derive(Clone,Debug,Default)]
pub struct Board {
    squares: [[Option<Piece>;8]; 8],
    pub turn: Color,
}

impl Board {
    pub fn set (&mut self, pos: Position, piece: Option<Piece>) {
        if Position::is_valid(pos.row as i8, pos.col as i8) {
            self.squares[pos.row][pos.col] = piece;
        }
    }

    pub fn get (&self, pos: Position) -> Option<&Piece> {
        if Position::is_valid(pos.row as i8, pos.col as i8) {
            self.squares[pos.row][pos.col].as_ref()
        } else {
            None
        }
    }

    pub fn get_mut (&mut self, pos: Position) -> Option<&mut Piece> {
        match Position::is_valid(pos.row as i8, pos.col as i8) {
            true => self.squares[pos.row][pos.col].as_mut(),
            false => None,
        }
    }

    pub fn take (&mut self, pos: Position) -> Option<Piece> {
        match Position::is_valid(pos.row as i8, pos.col as i8) {
            true => Some(self.squares[pos.row][pos.col].take()?),
            false => None
        }
    }

    pub fn switch_turn (&mut self) -> () {
        self.turn = match self.turn {
            Color::White => Color::Black,
            Color::Black => Color::White,
        }
    }

    pub fn is_enemy_cell (&self, pos: Position, color: Color) -> bool {
        self.get(pos).map_or(false, |p| p.color != color)
    }

    pub fn is_empty_cell (&self, pos: Position) -> bool {
        self.get(pos).is_none()
    }

    pub fn apply_move (&mut self, from: Position, to: Position) -> () {
        let mut piece = match self.take(from) {
            Some(p) => p,
            None => return,
        };

        let moves = piece.data.legal_moves(from, piece.color, self);
        if !moves.contains(&to) {
            self.set(from, Some(piece));
            return;
        }

        piece.data.on_move(from, to, piece.color, self);

        piece.pos = to;
        self.set(to, Some(piece));

        self.switch_turn();
    }

    pub fn get_king_pos (&self, color: Color) -> Option<Position> {
        self.all_positions()
            .find(|&pos| {
                self.get(pos)
                    .map_or(false, |p| p.name == Name::King && p.color == color)
            })
    }

    // todo: fix fn
    pub fn is_in_check (&self, color: Color) -> bool {
        let king_pos = match self.get_king_pos(color) {
            Some(k) => k,
            None    => panic!("No {:?} king on board", color),
        };

        self.all_positions()
            .filter_map(|pos| self.get(pos).map(|p| (pos, p)))
            .any(|(pos, p)| {
                p.color != color
                    && p
                        .data
                        .legal_moves(pos, p.color, self)
                        .contains(&king_pos)
            })
    }

    pub fn is_checkmate (&mut self, color: Color) -> bool {
        self.is_in_check(color) &&
        self
            .all_positions()
            .filter_map(|from| self.get(from))
            .filter(|p| p.color == color)
            .all(|p| {
                let mut m = p.data.legal_moves(p.pos, p.color, self);
                self.exclude_king_exposure(&mut m, p.pos, color);
                m.is_empty()
            })
    }

    pub fn is_stalemate (&mut self, color: Color) -> bool {
        !self.is_in_check(color) &&
        self
            .all_positions()
            .filter_map(|from| self.get(from))
            .filter(|p| p.color == color)
            .all(|p| {
                let mut m = p.data.legal_moves(p.pos, p.color, self);
                self.exclude_king_exposure(&mut m, p.pos, color);
                m.is_empty()
            })
    }

    pub fn exclude_king_exposure (&self, available_moves: &mut Vec<Position>, from: Position, color: Color) -> () {
        available_moves.retain(|&to| {
            let mut b = self.clone();
            b.apply_move(from, to);
            !b.is_in_check(color)
        });
    }

    pub fn reset_passants (&mut self, color: Color) -> () {
        let positions: Vec<Position> = self.all_positions().collect();

        positions.into_iter().for_each(|pos| {
            if let Some(piece) = self.get_mut(pos) {
                if piece.name == Name::Pawn && piece.color == color {
                    if let Some(pawn_data) =
                        piece.data.as_any_mut().downcast_mut::<PawnData>()
                    {
                        pawn_data.passant_target = None;
                    }
                }
            }
        });
    }

    pub fn all_positions(&self) -> impl Iterator<Item = Position> {
        (0..8).flat_map(|row| (0..8).map(move |col| Position { row, col }))
    }
}

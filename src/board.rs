use crate::piece::{Piece, pawn::PawnData};
use crate::types::{Color, Name, Position};

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

    pub fn get (&self, pos: Position) -> Option<&Piece> {
        match Position::is_valid(pos.row as i8, pos.col as i8) {
            true => self.squares[pos.row][pos.col].as_ref(),
            false => None,
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
        // self.exclude_king_exposure(&mut moves, from, piece.color);
        if !moves.contains(&to) {
            self.set(from, Some(piece));
            return;
        }

        piece.data.on_move(from, to, piece.color, self);

        piece.pos = to;
        self.set(to, Some(piece));

        self.switch_turn();
    }

    pub fn get_king_pos (&mut self, color: Color) -> Option<Position> {
        for row in 0..8 {
            for col in 0..8 {
                if let Some(p) = self.get(Position { row, col }) {
                    if p.color == color && p.name == Name::King {
                        return Some(p.pos);
                    }
                    else { continue; }
                } else {
                    continue;
                }
            }
        }
        None
    }

    pub fn is_in_check (&mut self, color: Color) -> bool {
        let king_pos = match self.get_king_pos(color) {
            Some(pos) => pos,
            None => panic!("No king on board"),
        };

        for row in 0..8 {
            for col in 0..8 {
                if let Some(p) = self.get(Position { row, col }) {
                    if p.color == color { continue; }
                    if p.data.legal_moves(Position { row, col }, p.color, self).contains(&king_pos) {
                        return true;
                    }
                } else { continue; }
            }
        }
        false
    }

    pub fn exclude_king_exposure (&self, available_moves: &mut Vec<Position>, from: Position, color: Color) -> () {
        available_moves.retain(|&to| {
            let mut b = self.clone();
            b.apply_move(from, to);
            !b.is_in_check(color)
        });
    }

    pub fn reset_passants (&mut self, color: Color) -> () {
        for row in 0..8 {
            for col in 0..8 {
                let pos = Position { row, col };
                if let Some(piece) = self.get_mut(pos) {
                    if piece.name == Name::Pawn && piece.color == color {
                        if let Some(pawn_data) = piece.data.as_any_mut().downcast_mut::<PawnData>() {
                            pawn_data.passant_target = None;
                        }
                    }
                }
            }
        }
    }
}

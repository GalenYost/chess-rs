use crate::utils::step_moves;
use crate::types::{Position, Name, Color};
use crate::board::Board;
use crate::piece::{Piece, PieceData};

use std::any::Any;

static STEPS: &[(i8, i8)] = &[
    ( 2,  1), ( 2, -1),
    (-2,  1), (-2, -1),
    ( 1,  2), ( 1, -2),
    (-1,  2), (-1, -2),
];

#[derive(Clone,Debug)]
pub struct KnightData {
    pub has_moved: bool,
}

impl PieceData for KnightData {
    fn as_any (&self) -> &dyn Any { self }
    fn as_any_mut (&mut self) -> &mut dyn Any { self }

    fn legal_moves (&self, pos: Position, color: Color, board: &Board) -> Vec<Position> {
        let mut moves = step_moves(board, pos, STEPS, color);
        board.exclude_king_exposure(&mut moves, pos, color);
        moves
    }

    fn on_move (&mut self, _from: Position, _to: Position, _color: Color, _board: &mut Board) -> () {
        self.moved();
    }
}

impl KnightData {
    pub fn moved (&mut self) -> () {
        self.has_moved = true;
    }
}

pub fn new (pos: Position, color: Color) -> Piece {
    Piece {
        name: Name::Knight,
        color,
        pos,
        data: Box::new(KnightData {
            has_moved: false,
        })
    }
}

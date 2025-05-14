use crate::utils::sliding_moves;
use crate::types::{Position, Name, Color};
use crate::board::Board;
use crate::piece::{Piece, PieceData};

use std::any::Any;

static DIRS: &[(i8, i8)] = &[
    (1, 0),
    (-1, 0),
    (0, 1),
    (0, -1),
    (1, 1),
    (-1, -1),
    (-1, 1),
    (1, -1),
];


#[derive(Clone,Debug)]
pub struct QueenData {
    pub has_moved: bool,
}

impl PieceData for QueenData {
    fn as_any (&self) -> &dyn Any { self }
    fn as_any_mut (&mut self) -> &mut dyn Any { self }

    fn legal_moves (&self, pos: Position, color: Color, board: &Board) -> Vec<Position> {
        let mut moves = sliding_moves(board, pos, DIRS, color);
        board.exclude_king_exposure(&mut moves, pos, color);
        moves
    }

    fn on_move (&mut self, _from: Position, _to: Position, _color: Color, _board: &mut Board) -> () {
        self.moved();
    }
}

impl QueenData {
    pub fn moved (&mut self) -> () {
        self.has_moved = true;
    }
}

pub fn new (pos: Position, color: Color) -> Piece {
    Piece {
        name: Name::Queen,
        color,
        pos,
        data: Box::new(QueenData {
            has_moved: false,
        })
    }
}

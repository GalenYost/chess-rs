use std::fmt::Debug;

use crate::types::{Position, Color, Name};
use crate::board::Board;

pub trait PieceData: Debug {
    fn legal_moves (&self, pos: Position, color: Color, board: &Board) -> Vec<Position>;
    fn on_move (&mut self, from: Position, to: Position, board: &mut Board);
    fn box_clone (&self) -> Box<dyn PieceData>;
}

impl Clone for Box<dyn PieceData> {
    fn clone (&self) -> Box<dyn PieceData> {
        self.box_clone()
    }
}

#[derive(Debug,Clone)]
pub struct Piece {
    pub name: Name,
    pub color: Color,
    pub pos: Position,
    pub data: Box<dyn PieceData>,
}

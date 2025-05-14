pub mod pawn; pub mod king;
pub mod queen; pub mod rook;
pub mod bishop; pub mod knight;

use std::fmt::Debug;
use std::any::Any;

use crate::types::{Position, Color, Name};
use crate::board::Board;

pub trait PieceData: PieceDataClone + Debug + Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn legal_moves(&self, pos: Position, color: Color, board: &Board) -> Vec<Position>;
    fn on_move(&mut self, from: Position, to: Position, color: Color, board: &mut Board);
}

pub trait PieceDataClone {
    fn box_clone(&self) -> Box<dyn PieceData>;
}

impl<T> PieceDataClone for T
where
    T: PieceData + Clone + 'static,
{
    fn box_clone(&self) -> Box<dyn PieceData> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn PieceData> {
    fn clone(&self) -> Box<dyn PieceData> {
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

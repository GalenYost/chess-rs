use crate::types::{Position, Color};
use crate::board::Board;

pub fn sliding_moves (board: &Board, from: Position, dirs: &[(i8, i8)], color: Color) -> Vec<Position> {
    let mut moves = Vec::new();

    for &(dx, dy) in dirs {
        let mut current = from;

        loop {
            if let Some(next_pos) = current.shifted(dx, dy) {
                if board.is_empty_cell(next_pos) {
                    moves.push(next_pos);
                    current = next_pos;
                    continue;
                }
                if board.is_enemy_cell(next_pos, color) {
                    moves.push(next_pos);
                }
                break;
            } else {
                break;
            }
        }
    }

    moves
}

pub fn step_moves (board: &Board, from: Position, deltas: &[(i8, i8)], color: Color) -> Vec<Position> {
    let mut moves = Vec::new();

    for &(dx, dy) in deltas {
        if let Some(to) = from.shifted(dx, dy) {
            if board.is_empty_cell(to) || board.is_enemy_cell(to, color) {
                moves.push(to);
            }
        }
    }

    moves
}

mod types; mod piece; mod board; mod ui;

use board::Board;
use types::{Position, Color};
use piece::pawn;

fn main() {
    let mut board: Board = Board::default();

    let w_pawn_pos = Position { row: 6, col: 1 };
    let b_pawn_pos = Position { row: 4, col: 2 };

    board.set(w_pawn_pos, Some(pawn::new(w_pawn_pos, Color::White)));
    board.set(b_pawn_pos, Some(pawn::new(b_pawn_pos, Color::Black)));

    board.apply_move(w_pawn_pos, Position { row: 4, col: 1 });
    board.apply_move(b_pawn_pos, Position { row: 5, col: 1 });
}

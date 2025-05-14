mod types; mod piece; mod board; mod ui; mod utils;

use board::Board;
use types::{Position, Color};
use piece::{king, pawn, queen};

fn main() {
    let mut board: Board = Board::default();

    let w_king_pos = Position { row: 1, col: 1 };
    let b_king_pos = Position { row: 6, col: 5 };
    let b_queen_pos = Position { row: 3, col: 1 };

    board.set(w_king_pos, Some(king::new(w_king_pos, Color::White)));
    board.set(b_king_pos, Some(king::new(b_king_pos, Color::Black)));
    board.set(b_queen_pos, Some(queen::new(b_queen_pos, Color::Black)));

    println!("king found at: {:?}", board.get_king_pos(Color::White));

    println!("{:?}", board.is_in_check(Color::White));
}

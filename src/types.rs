#[derive(Default,Clone,Copy,PartialEq,Eq,Debug)]
pub enum Color {
    #[default]
    White,
    Black
}

#[derive(Clone,Copy,PartialEq,Eq,Debug)]
pub enum Name {
    Pawn, Knight, Rook, Bishop, King, Queen
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct Position {
    pub row: usize,
    pub col: usize
}

impl Position {
    pub fn is_valid (row: i8, col: i8) -> bool {
        (0..8).contains(&row) && (0..8).contains(&col)
    }

    pub fn shifted (&self, dx: i8, dy: i8) -> Option<Position> {
        let ri = self.row as i8 + dy;
        let ci = self.col as i8 + dx;

        if Position::is_valid(ri, ci) {
            Some(Position {
                row: ri as usize,
                col: ci as usize,
            })
        } else {
            None
        }
    }
}

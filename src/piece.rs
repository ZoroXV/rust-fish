#[derive(Copy, Clone)]
pub struct Piece {
    pub color: PieceColor,
    pub kind: PieceKind,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PieceColor {
    White,
    Black,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum PieceKind {
    Empty,
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl Default for Piece {
    fn default() -> Piece {
        Piece {
            color: PieceColor::White,
            kind: PieceKind::Empty,
        }
    }
}

impl Piece {
    pub fn get_char(self) -> &'static str {
        match self.kind {
            PieceKind::Pawn => "\u{265F}",
            PieceKind::Knight => "\u{265E}",
            PieceKind::Bishop => "\u{265D}",
            PieceKind::Rook => "\u{265C}",
            PieceKind::Queen => "\u{265B}",
            PieceKind::King => "\u{265A}",
            _ => " ",
        }
    }
}

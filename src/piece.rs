use std::fmt;

#[derive(Debug)]
pub struct Piece {
    color: PieceColor,
    kind: PieceKind,
}

#[derive(Debug, PartialEq)]
enum PieceColor {
    Empty,
    White,
    Black,
}

#[derive(Debug, PartialEq)]
enum PieceKind {
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
            color: PieceColor::Empty,
            kind: PieceKind::Pawn,
        }
    }
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c = match &self.kind {
            pawn => "P",
            knight => "N",
            bishop => "B",
            rook => "R",
            queen => "Q",
            king => "K",
        };

        if self.color == PieceColor::Empty {
            write!(f, ".")
        } else {
            write!(f, "{}", c)
        }
    }
}

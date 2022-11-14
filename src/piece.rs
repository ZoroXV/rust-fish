#[derive(Debug)]
pub struct Piece {
    color: PieceColor,
    kind: PieceKind
}

#[derive(Debug)]
enum PieceColor {
    NONE,
    WHITE,
    BLACK
}

#[derive(Debug)]
enum PieceKind {
    PAWN,
    KNIGHT,
    BISHOP,
    ROOK,
    QUEEN,
    KING
}

impl Default for Piece {
    fn default() -> Piece {
        Piece {
            color: PieceColor::NONE,
            kind: PieceKind::PAWN
        }
    }
}

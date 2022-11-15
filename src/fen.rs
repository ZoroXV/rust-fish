pub use crate::piece::*;

pub fn get_piece_from_fen_char(c: char) -> Result<Piece, &'static str> {
    let piece = match c {
        'p' => Piece {
            color: PieceColor::White,
            kind: PieceKind::Pawn,
        },
        'n' => Piece {
            color: PieceColor::White,
            kind: PieceKind::Knight,
        },
        'b' => Piece {
            color: PieceColor::White,
            kind: PieceKind::Bishop,
        },
        'r' => Piece {
            color: PieceColor::White,
            kind: PieceKind::Rook,
        },
        'q' => Piece {
            color: PieceColor::White,
            kind: PieceKind::Queen,
        },
        'k' => Piece {
            color: PieceColor::White,
            kind: PieceKind::King,
        },
        'P' => Piece {
            color: PieceColor::Black,
            kind: PieceKind::Pawn,
        },
        'N' => Piece {
            color: PieceColor::Black,
            kind: PieceKind::Knight,
        },
        'B' => Piece {
            color: PieceColor::Black,
            kind: PieceKind::Bishop,
        },
        'R' => Piece {
            color: PieceColor::Black,
            kind: PieceKind::Rook,
        },
        'Q' => Piece {
            color: PieceColor::Black,
            kind: PieceKind::Queen,
        },
        'K' => Piece {
            color: PieceColor::Black,
            kind: PieceKind::King,
        },
        _ => return Err("Invalid FEN String."),
    };

    Ok(piece)
}

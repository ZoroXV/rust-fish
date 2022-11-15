pub use crate::fen::*;
pub use crate::piece::*;

use bit_set::BitSet;
use colored::Colorize;

pub struct Board {
    white_pawns: BitSet,
    white_knights: BitSet,
    white_bishops: BitSet,
    white_rooks: BitSet,
    white_queens: BitSet,
    white_king: BitSet,

    black_pawns: BitSet,
    black_knights: BitSet,
    black_bishops: BitSet,
    black_rooks: BitSet,
    black_queens: BitSet,
    black_king: BitSet,
}

impl Default for Board {
    fn default() -> Board {
        Board {
            white_pawns: BitSet::with_capacity(64),
            white_knights: BitSet::with_capacity(64),
            white_bishops: BitSet::with_capacity(64),
            white_rooks: BitSet::with_capacity(64),
            white_queens: BitSet::with_capacity(64),
            white_king: BitSet::with_capacity(64),

            black_pawns: BitSet::with_capacity(64),
            black_knights: BitSet::with_capacity(64),
            black_bishops: BitSet::with_capacity(64),
            black_rooks: BitSet::with_capacity(64),
            black_queens: BitSet::with_capacity(64),
            black_king: BitSet::with_capacity(64),
        }
    }
}

impl Board {
    pub fn from_fen(fen_string: &str) -> Result<Self, &'static str> {
        let fen_blocks: Vec<&str> = fen_string.split(' ').collect();
        if fen_blocks.len() != 6 {
            return Err("Invalid FEN String: Missing informations");
        }

        println!("Next Player to play: {}", fen_blocks[1]);
        println!("Castling Infos: {}", fen_blocks[2]);
        println!("En passant cell: {}", fen_blocks[3]);
        println!("Half-move count: {}", fen_blocks[4]);
        println!("Full-move count: {}", fen_blocks[5]);

        let board_rows: Vec<&str> = fen_blocks[0].split('/').collect();

        Ok(Self::default())
    }

    pub fn print_board(&self) {
        println!("A B C D E F G H");
        for i in (0..8).rev() {
            for j in (0..8).rev() {
                let piece = self.index(i * 8 + j);

                let cell = format!("{} ", piece.get_char());
                let cell = if piece.color == PieceColor::White {
                    cell.white()
                } else {
                    cell.black()
                };
                let cell = if (i + j) % 2 == 0 {
                    cell.on_blue()
                } else {
                    cell.on_bright_white()
                };

                print!("{}", cell);
            }
            println!(" {}", 8 - i);
        }
    }

    fn index(&self, idx: usize) -> Piece {
        let color: PieceColor;
        let kind: PieceKind;
        if self.white_pawns.contains(idx) {
            color = PieceColor::White;
            kind = PieceKind::Pawn;
        } else if self.white_knights.contains(idx) {
            color = PieceColor::White;
            kind = PieceKind::Knight;
        } else if self.white_bishops.contains(idx) {
            color = PieceColor::White;
            kind = PieceKind::Bishop;
        } else if self.white_rooks.contains(idx) {
            color = PieceColor::White;
            kind = PieceKind::Rook;
        } else if self.white_queens.contains(idx) {
            color = PieceColor::White;
            kind = PieceKind::Queen;
        } else if self.white_king.contains(idx) {
            color = PieceColor::White;
            kind = PieceKind::King;
        } else if self.black_pawns.contains(idx) {
            color = PieceColor::Black;
            kind = PieceKind::Pawn;
        } else if self.black_knights.contains(idx) {
            color = PieceColor::Black;
            kind = PieceKind::Knight;
        } else if self.black_bishops.contains(idx) {
            color = PieceColor::Black;
            kind = PieceKind::Bishop;
        } else if self.black_rooks.contains(idx) {
            color = PieceColor::Black;
            kind = PieceKind::Rook;
        } else if self.black_queens.contains(idx) {
            color = PieceColor::Black;
            kind = PieceKind::Queen;
        } else if self.black_king.contains(idx) {
            color = PieceColor::Black;
            kind = PieceKind::King;
        } else {
            color = PieceColor::White;
            kind = PieceKind::Empty;
        }

        Piece { color, kind }
    }
}

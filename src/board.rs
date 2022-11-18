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
        let mut board = Self::default();

        let fen_blocks: Vec<&str> = fen_string.split(' ').collect();
        if fen_blocks.len() != 6 {
            return Err("Invalid FEN String: Missing informations");
        }

        println!("Next Player to play: {}", fen_blocks[1]);
        println!("Castling Infos: {}", fen_blocks[2]);
        println!("En passant cell: {}", fen_blocks[3]);
        println!("Half-move count: {}", fen_blocks[4]);
        println!("Full-move count: {}", fen_blocks[5]);

        let rows: Vec<&str> = fen_blocks[0].split('/').collect();
        if rows.len() != 8 {
            return Err("Invalid FEN String. Board must have exactly.");
        }

        for (i, row) in rows.into_iter().enumerate() {
            let mut j = 0;

            for c in row.chars() {
                if c.is_ascii_digit() {
                    let empty_cells = c.to_digit(10).unwrap();
                    j += empty_cells;
                } else {
                    match c {
                        'P' => board.white_pawns.insert(i * 8 + j as usize),
                        'N' => board.white_knights.insert(i * 8 + j as usize),
                        'B' => board.white_bishops.insert(i * 8 + j as usize),
                        'R' => board.white_rooks.insert(i * 8 + j as usize),
                        'Q' => board.white_queens.insert(i * 8 + j as usize),
                        'K' => board.white_king.insert(i * 8 + j as usize),
                        'p' => board.black_pawns.insert(i * 8 + j as usize),
                        'n' => board.black_knights.insert(i * 8 + j as usize),
                        'b' => board.black_bishops.insert(i * 8 + j as usize),
                        'r' => board.black_rooks.insert(i * 8 + j as usize),
                        'q' => board.black_queens.insert(i * 8 + j as usize),
                        'k' => board.black_king.insert(i * 8 + j as usize),
                        _ => return Err("Invalid FEN String."),
                    };
                    j += 1;
                }
            }
        }

        Ok(board)
    }

    pub fn print_board(&self) {
        println!("A B C D E F G H");
        for i in 0..8 {
            for j in 0..8 {
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
            return Piece::default();
        }

        Piece { color, kind }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fen_empty_board() {
        let b = Board::from_fen("8/8/8/8/8/8/8/8 w - - 0 1").unwrap();
        for i in 0..64 {
            assert_eq!(b.index(i), Piece::default());
        }
    }
}

pub use crate::piece::*;

use bit_set::BitSet;

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
        let mut b = Board {
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
        };

        for i in 8..16 {
            b.white_pawns.insert(i);
        }
        b.white_knights.insert(1);
        b.white_knights.insert(6);
        b.white_bishops.insert(2);
        b.white_bishops.insert(5);
        b.white_rooks.insert(0);
        b.white_rooks.insert(7);
        b.white_queens.insert(3);
        b.white_king.insert(4);

        for i in 48..56 {
            b.black_pawns.insert(i);
        }
        b.black_knights.insert(57);
        b.black_knights.insert(62);
        b.black_bishops.insert(58);
        b.black_bishops.insert(61);
        b.black_rooks.insert(56);
        b.black_rooks.insert(63);
        b.black_queens.insert(59);
        b.black_king.insert(60);

        b
    }
}

impl Board {
    pub fn print_board(&self) {
        println!("A B C D E F G H");
        for i in 0..8 {
            for j in 0..8 {
                let piece = Piece::default();
                println!("{:?}", piece);
            }
            println!(" {}", 8-i);
        }
    }
}

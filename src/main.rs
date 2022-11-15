mod board;
mod fen;
mod piece;

const DEFAULT_FEN_STRING: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

fn main() {
    println!("FEN String: {}", DEFAULT_FEN_STRING);
    let board = board::Board::from_fen(DEFAULT_FEN_STRING);
    match board {
        Ok(board) => board.print_board(),
        Err(e) => panic!("{}", e),
    }

    let fen_example = "8/8/8/4p1K1/2k1P3/8/8/8 b - - 0 1";
    println!("FEN String: {}", fen_example);
    let board = board::Board::from_fen(fen_example);
    match board {
        Ok(board) => board.print_board(),
        Err(e) => panic!("{}", e),
    }
}

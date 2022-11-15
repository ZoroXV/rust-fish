mod board;
mod piece;

fn main() {
    let b = board::Board::default();

    b.print_board();
}

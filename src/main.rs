mod board;


fn main() {
    
    let mut my_board = board::Board::new();
    
    my_board.set_mutable(0, 0, 8);
    my_board.set_unmutable(0, 1, 9);

    my_board.print();
}

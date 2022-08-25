mod read_file;
mod board;

fn main() {
    
    let mut my_board = board::Board::new();

    read_file::read_input("./data/easy_sudoku.txt");
    
    my_board.set_mutable(0, 0, 8);
    my_board.set_unmutable(0, 1, 9);
    my_board.print();
}

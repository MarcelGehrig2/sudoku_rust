mod read_file;
mod board;

fn main() {
    
    let mut my_board = board::Board::new();

    let file = "./data/easy_sudoku.txt";
    let boards = read_file::read_input(file);
    
    println!("Found {} Sudoku Board(s) in {}",boards.len(), file);

    for board in boards {
        //let mut cur_board = board::Board::from_string(board);
        println!("Solving Board:");
        //cur_board.print();
        //solve...
        println!("Solved Board:");
        //cur_board.print();
    }

    my_board.set_mutable(0, 0, 8);
    my_board.set_unmutable(0, 1, 9);
    my_board.print();
}

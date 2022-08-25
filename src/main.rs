mod read_file;
mod board;

fn main() {
    let file = "./data/easy_sudoku.txt";
    let boards = read_file::read_input(file);
    
    println!("Found {} Sudoku Board(s) in {}",boards.len(), file);

    for board in boards {
        let cur_board = board::Board::from_string(board);
        println!("Solving Board:");
        cur_board.print();
        //solve...
        println!("Solved Board:");
        //cur_board.print();
    }
}

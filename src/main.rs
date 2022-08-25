mod read_file;
mod board;
mod solver;

fn main() {
    println!("##############################################################################################");
    println!(r"______ _   _ _____ _____   _____           _       _            _____       _                ");
    println!(r"| ___ \ | | /  ___|_   _| /  ___|         | |     | |          /  ___|     | |               ");
    println!(r"| |_/ / | | \ `--.  | |   \ `--. _   _  __| | ___ | | ___   _  \ `--.  ___ | |_   _____ _ __ ");
    println!(r"|    /| | | |`--. \ | |    `--. \ | | |/ _` |/ _ \| |/ / | | |  `--. \/ _ \| \ \ / / _ \ '__|");
    println!(r"| |\ \| |_| /\__/ / | |   /\__/ / |_| | (_| | (_) |   <| |_| | /\__/ / (_) | |\ V /  __/ |   ");
    println!(r"\_| \_|\___/\____/  \_/   \____/ \__,_|\__,_|\___/|_|\_\\__,_| \____/ \___/|_| \_/ \___|_|   "); 
    println!("##############################################################################################");

    let file = "./data/easy_sudoku.txt";
    let boards = read_file::read_input(file);    
                                                                                                
    println!("Found {} Sudoku Board(s) in {}",boards.len(), file);
    
    for board in boards {
        let mut cur_board = board::Board::from_string(board);

        println!(r"**********************************************************************************************");
        println!("Solving Board:");
        solver::Solver::solve(&mut cur_board);
        cur_board.print();
        //solve...
        println!("Solved Board:");
        cur_board.print();
    }

}

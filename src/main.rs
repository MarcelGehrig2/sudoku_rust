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

    let file = "./data/sudoku.txt";
    let boards = read_file::read_input(file);
    let num_of_boards = boards.len();
    let mut solved_boards = 0;
    let mut handled_boards = 0;
                                     
    println!("Found {} Sudoku Board(s) in {}",num_of_boards, file);
    
    for board in boards {
        handled_boards += 1;
        let mut cur_board = board::Board::from_string(board);
        println!(r"**********************************************************************************************");
        println!("Solving Board {} of {}:",handled_boards,num_of_boards);
        cur_board.print();
        if solver::Solver::solve(&mut cur_board) {
            println!("Solved Board:");
            cur_board.print();
            solved_boards += 1;
        }
        else{
            println!("");
        }
    }

    println!(r"**********************************************************************************************");
    println!(r"Done... =)");
    println!("Could solve {} out of {}!",solved_boards,num_of_boards)

}

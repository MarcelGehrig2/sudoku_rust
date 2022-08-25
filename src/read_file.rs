use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


pub fn read_input(file_path: &str) -> Vec<String> {

    let mut board_strings = Vec::new();

    if let Ok(lines) = read_lines(&file_path) {
                
        let mut board = String::from("");
        
        for line in lines {
            if let Ok(current_line) = line {

                if current_line.contains("Grid") {
                    if board.len() == 81 {
                        board_strings.push(board);
                    }
                    board = String::from("");
                }
                else {
                    board.push_str(&current_line);
                    
                }
            }
        }
        if board.len() == 81 {
            board_strings.push(board);
        }
    }

    board_strings
    
}

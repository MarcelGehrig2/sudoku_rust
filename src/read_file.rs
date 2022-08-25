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


pub fn read_input(file_path: &str) {

    if let Ok(lines) = read_lines(&file_path) {
        
        let mut cur_grid_nr = 0;
        let mut cur_row = 0;
        for line in lines {
            if let Ok(current_line) = line {

                if current_line.contains("Grid") {

                    let mut iter = current_line.split_ascii_whitespace();
                    iter.next();
                    cur_grid_nr = iter.next().unwrap_or_default().parse::<u32>().unwrap();
                    cur_row = 0;
                    println!("New GRID Nr {}", cur_grid_nr);
                }
                else {
                    //println!("Grid data (Grid Nr {}):", cur_grid_nr);

                    let count = current_line.chars().count();
                    assert_eq!(9, count);


                    let digits = current_line.chars();

                    let mut cur_col = 0;
                    //println!("{}", digits.next().unwrap());
                    for digit in digits {
                        assert!(digit.is_numeric());
                        let digit_nr = digit.to_digit(10).unwrap();
                        if digit_nr > 0 {
                            //TODO: Set Digit in Board
                            //println!("row {} col {} is {}", cur_row, cur_col, digit);
                        }
                        cur_col = cur_col + 1;
                    }

                    cur_row = cur_row + 1;
                }
            }
        }
    }
}

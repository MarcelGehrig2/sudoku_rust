pub use super::board::Board;

#[derive(Debug)]
pub struct Solver {

}

impl Solver {
    pub fn solve(board: &mut Board) {

        
        println!("--- starting solving ----");

        let mut valid_found = true;
        let mut board_solved = false;
        let mut back_tracking = false;
        let mut counter = 0;
        let printout_period = 1;
        let mut i_column = 0;
        let mut i_row = 0;
        
        Board::get_prev_index(&mut i_row, &mut i_column);

        while !board_solved {
            valid_found = false;

            if !back_tracking {
                Board::get_next_index(&mut i_row, &mut i_column);
            } else {
                Board::get_prev_index(&mut i_row, &mut i_column);
            }

            // cant modify unmutable cells
            if !board.is_cell_mutable(i_row, i_column) {
                continue;
            }
            
            let mut initial_value = board.get_cell_value(i_row, i_column);

            // setting 0s to 1
            if initial_value == 0 {
                initial_value = 1;
            }

            // current value may be valid
            //   if we are back tracking, we don't want to use the current value
            if !back_tracking {
                if board.is_cell_valid(i_row, i_column, initial_value) {
                    board.set_mutable(i_row, i_column, initial_value);
                    valid_found = true;
                    continue;
                }
            }

            // if the existing value is not valid, we check if the other 8 numbers have a valid candidate
            for offset in initial_value + 1..9 {
                // let test_value = (initial_value + offset) % 9 + 1;
                let test_value = offset;
                if board.is_cell_valid(i_row, i_column, test_value) {
                    valid_found = true;
                    board.set_mutable(i_row, i_column, test_value);
                    break
                }
            }

            if !valid_found {
                board.set_mutable(i_row, i_column, 0);
            }

            // if we dont found a valid soulution we go back
            back_tracking = !valid_found;
            
            // Check if finished
            if valid_found {
                if i_column == 8 && i_row == 8 {
                    board_solved = true;
                    break;
                }
            } else {
                if i_column == 0 && i_row == 0 {
                    println!("XXXXXXX not possible");
                    break
                }
            }

            if back_tracking {
            }

            // Printout
            counter = counter + 1;
            if counter % printout_period == 0 {
                board.print();
                counter = counter;
            }

        }

    }


}
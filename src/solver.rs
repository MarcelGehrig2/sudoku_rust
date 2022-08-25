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

        while !board_solved {
            valid_found = false;

            // cant modify unmutable cells
            if !board.is_cell_mutable(i_row, i_column) {
                continue;
            }
            
            // setting 0s to 1
            if board.get_cell_value(i_row, i_column) == 0 {
                board.set_mutable(i_row, i_column, 1)
            }
            
            let initial_value = board.get_cell_value(i_row, i_column);

            // current value may be valid
            //   if we are back tracking, we don't want to use the current value
            if !back_tracking {
                if board.is_cell_valid(i_row, i_column, initial_value) {
                    valid_found = true;
                    continue;
                }
            }

            // if the existing value is not valid, we check if the other 8 numbers have a valid candidate
            for offset in 0..7 {
                let test_value = (initial_value + offset) % 9 + 1;
                if board.is_cell_valid(i_row, i_column, test_value) {
                    valid_found = true;
                    board.set_mutable(i_row, i_column, test_value);
                    break
                }
            }

            // if we dont found a valid soulution we go back
            back_tracking = !valid_found;

            if valid_found {
                if i_column == 8 && i_row == 8 {
                    board_solved = true;
                    break;
                }
                Board::get_next_index(&mut i_row, &mut i_column);
            } else {
                if i_column == 0 && i_row == 0 {
                    println!("XXXXXXX not possible");
                    break
                }
                Board::get_prev_index(&mut i_row, &mut i_column);
            }

            // Printout
            counter = counter + 1;
            if counter % printout_period == 0 {
                board.print();
            }

        }

    }


}
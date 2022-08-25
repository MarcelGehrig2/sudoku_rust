pub const HEIGHT: usize = 9;
pub const LENGTH: usize = 9;

#[derive(Debug, Copy, Clone)]
struct Cell {
    pub value: u32,
    pub mutable: bool
}

#[derive(Debug)]
pub struct Board {
    cells: [[Cell; LENGTH]; HEIGHT],
}

impl Board {
    pub fn new() -> Self {
        let empty_cell: Cell = Cell {value: 0, mutable: true};
        Self{cells: [[empty_cell; 9]; 9]}
    }

    pub fn set_unmutable(&mut self, x: usize, y: usize, value: u32) {
        self.cells[x][y].value = value;
        self.cells[x][y].mutable = false;
    }

    pub fn set_mutable(&mut self, x: usize, y: usize, value: u32) {
        self.cells[x][y].value = value;
        self.cells[x][y].mutable = true;
    }

    pub fn from_string(board : String) -> Board{
        let mut my_board = Board::new();

        let lenght = board.chars().count();
        assert_eq!(lenght, 9*9);
        
        let mut digits = board.chars();
        let mut cur_row = 0;
        let mut cur_col = 0;
        for _n in 0..81{
            let digit : char = digits.next().unwrap();
            assert!(digit.is_numeric());
            let digit_nr = digit.to_digit(10).unwrap();

            if digit_nr > 0 {
                my_board.set_unmutable(cur_row,cur_col,digit_nr);
            }
            Board::get_next_index(&mut cur_row,&mut cur_col);
        }

         my_board
    }

    pub fn is_cell_mutable(&self, i_row: usize, i_col: usize) -> bool {
        self.cells[i_row][i_col].mutable
    }

    pub fn get_cell_value(&self, i_row: usize, i_col: usize) -> u32 {
        self.cells[i_row][i_col].value
    }

    pub fn is_cell_valid(&self, i_row: usize, i_col: usize, val: u32) -> bool {
        if !self.is_cell_mutable(i_row, i_col) {
            return false;
        }

        //Check Row
        for col in 0..8 {
            if self.get_cell_value(i_row, col) == val {
                return false;
            }
        }

        //Check Column
        for row in 0..8 {
            if self.get_cell_value(row, i_col) == val {
                return false;
            }
        }

        //Check Box
        let start_col_box = i_col / 3;
        let start_row_box = i_row / 3;

        for row in (3 * start_row_box)..((3 * start_row_box) + 2) {
            for col in (3 * start_col_box)..((3 * start_col_box) + 2) {
                if self.get_cell_value(row, col) == val {
                    return false;
                }
            }
        }

        true
    }


    pub fn print(&self){
        println!(" -------+-------+------- ");
        for (i, row) in self.cells.iter().enumerate() {
        let mut line : String = String::from("| ");
        for (j, col) in row.iter().enumerate() {
			
            if col.value > 0 {
                line.push(char::from_digit(col.value, 10).unwrap());
            }
            else{
                line.push(' ');
            }
            line.push(' ');
            if j == 2 || j == 5 {
                line.push('|');
                line.push(' ');
            }
        }
        line.push('|');
        println!("{}",line);
        if i == 2 || i == 5 {
            println!("+-------+-------+-------+");
        }
    }
    println!("+-------+-------+-------+");
    }

    pub fn get_next_index(i_row: &mut usize, i_column: &mut usize){
        if *i_column < 8{
            *i_column += 1;
        }
        else{
            *i_column = 0;
            if *i_row < 8 {
                *i_row += 1;
            }
            else{
                *i_row = 0;
            }
        }
    }

    pub fn get_prev_index(i_row: &mut usize, i_column: &mut usize){
        if *i_column > 0{
            *i_column -= 1;
        }
        else{
            *i_column = 8;
            if *i_row > 0 {
                *i_row -= 1;
            }
            else{
                *i_row = 8;
            }
        }
    }
}
pub const HEIGHT: usize = 9;
pub const LENGTH: usize = 9;

#[derive(Debug, Copy, Clone)]
struct Cell {
    pub value: u8,
    pub mutable: bool
}

#[derive(Debug)]
pub struct Board {
    cells: [[Cell; LENGTH]; HEIGHT],
}

impl Board {
    pub fn new() -> Self {
        let empty_cell: Cell = Cell {value: 0, mutable: false};
        Self{cells: [[empty_cell; 9]; 9]}
    }

    pub fn set_unmutable(&mut self, x: usize, y: usize, value: u8) {
        self.cells[x][y].value = value;
        self.cells[x][y].mutable = false;
    }

    pub fn set_mutable(&mut self, x: usize, y: usize, value: u8) {
        self.cells[x][y].value = value;
        self.cells[x][y].mutable = true;
    }

    pub fn is_cell_mutable(&self, i_row: usize, i_col: usize) -> bool {
        self.cells[i_row][i_col].mutable
    }

    pub fn get_cell_value(&self, i_row: usize, i_col: usize) -> u8 {
        self.cells[i_row][i_col].value
    }

    pub fn is_cell_valid(&self, i_row: usize, i_col: usize, val: u8) -> bool {
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
        println!("  ---------------------------  ");
        for (i, row) in self.cells.iter().enumerate() {
        let mut line : String = String::from("|");
        for (j, col) in row.iter().enumerate() {
            line.push(' ');
			
            line.push(char::from_digit(col.value as u32, 10).unwrap());
            line.push(' ');
            if j == 2 || j == 5 {
                line.push('|');
            }
        }
        line.push('|');
        println!("{}",line);
        if i == 2 || i == 5 {
            println!("  ---------------------------  ");
        }
    }

    println!("  ---------------------------  ");
    }
}
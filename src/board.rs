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
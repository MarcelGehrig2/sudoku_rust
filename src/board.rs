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
        let empty_cell: Cell = Cell {value: 0, mutable: false};
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
        for n in 0..81{
            let cur_col = n % 9;

            let digit : char = digits.next().unwrap();
            assert!(digit.is_numeric());
            let digit_nr = digit.to_digit(10).unwrap();

            if digit_nr > 0 {
                my_board.set_unmutable(cur_row,cur_col,digit_nr);
            }

            if cur_col == 8 {
                cur_row += 1;
            }
        }

         my_board
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
}
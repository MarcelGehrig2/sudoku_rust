

pub mod types {

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

        pub fn SetUnmutable(&mut self, x: usize, y: usize, value: u8) {
            self.cells[x][y].value = value;
            self.cells[x][y].mutable = false;
       }

       pub fn SetMutable(&mut self, x: usize, y: usize, value: u8) {
           self.cells[x][y].value = value;
           self.cells[x][y].mutable = true;
      }
   }

}

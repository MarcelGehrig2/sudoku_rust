

pub mod types {

    pub const HEIGHT: usize = 9;
    pub const LENGTH: usize = 9;

    #[derive(Debug)]
    pub struct Cell {
       pub value: u8,
       pub fixed: bool
    }

    #[derive(Debug)]
    pub struct Board {
      //  pub state: [[u8; 9]; 9]
        pub state: [[u8; LENGTH]; HEIGHT],
    //    pub cell: Cell,
    }

    impl Board {
        fn SetUnmuatable(&self, x: usize, y: usize, value: u8) {
            self.state[x][y] = value;
        }
    }

}

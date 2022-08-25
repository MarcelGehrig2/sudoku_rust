use std::fs;

pub fn read_input() {

   
    let contents = fs::read_to_string("./data/sudoku.txt")
        .expect("Should have been able to read the file");



    let mut lines = contents.lines();

    println!("{:?}", lines.next());

    println!("{:?}", lines.next());
}

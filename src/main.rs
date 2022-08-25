mod types;


fn main() {
    
  //  let mut board = types::types::Cell {value: 10, fixed: false};
    
    let mut board = types::types::Board {state: [[0; 9]; 9]};
    
    board.state[1][5] = 1;

 //   board.state[0][0].cell.value = 0;

    println!("Hello, world! {:?}", board);
}

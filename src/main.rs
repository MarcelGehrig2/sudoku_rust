mod types;


fn main() {
    
    
    let mut board = types::types::Board::new();
    
    board.SetMutable(0, 0, 8);
    board.SetUnmutable(0, 1, 9);


    println!("Hello, world! {:?}", board);
}

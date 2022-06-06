use crate::prelude::*;

mod board;
mod prelude {
    pub use rand::Rng;
    pub use std::collections::HashMap;
    pub use eframe::egui::*;
    pub use crate::board::*;
    pub use crate::CellValue::*;
}


fn main() {
    
    let board = Board::new(10, 10, 5);

    for i in 0..10 {
        for j in 0..10 {
            println!("({}, {}): {:?}", i, j, board.mines.get(&(i,j)))
        }
    }

}

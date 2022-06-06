use crate::prelude::*;

mod board;
mod prelude {
    pub const WIDTH: usize = 5;
    pub const HEIGHT: usize = 10;
    pub const NUM_MINES: usize = 5;
    
    pub use rand::Rng;
    pub use std::collections::HashMap;
    pub use eframe::egui::*;
    pub use crate::board::*;
}


fn main() {
    let test = Board::new();

    println!("{:#?}", &test);
}
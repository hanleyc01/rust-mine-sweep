use crate::prelude::*;

mod board;
mod prelude {
    pub use rand::Rng;
    pub use std::collections::HashMap;
    pub use eframe::egui::*;
    pub use crate::board::*;
}


fn main() {

    let x = initialize_board(10, 10, 5);
    
    for x_h in 0..10 {
        for y_h in 0..10{
            println!("({}, {}) is: {:?}", x_h, y_h, x.get(&(x_h, y_h)))
        }
    }

}

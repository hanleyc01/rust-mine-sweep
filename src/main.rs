use crate::prelude::*;

mod board;
mod prelude {
    pub const WIDTH: usize = 50;
    pub const HEIGHT: usize = 75;
    
    pub use rand::Rng;
    pub use std::collections::HashMap;
    pub use eframe::egui::*;
    pub use crate::board::*;
}


fn main() {
    todo!();
}



use crate::prelude::*;

mod board;
mod app;

mod prelude {
    pub const WIDTH: usize = 5;
    pub const HEIGHT: usize = 10;
    pub const NUM_MINES: usize = 5;
    
    pub use rand::Rng;
    pub use std::collections::HashMap;
    pub use eframe::*;
    pub use crate::board::*;
    pub use crate::app::*;
}


fn main() {
    let native_options = eframe::NativeOptions::default();

    //run_native("mine-sweep", native_options, app_creator)
}
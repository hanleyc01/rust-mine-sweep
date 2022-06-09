use crate::prelude::*;

mod board;
mod app;

mod prelude {
    pub const WIDTH: usize = 5;
    pub const HEIGHT: usize = 10;
    pub const NUM_MINES: usize = 5;

    pub use crate::board::*;
    pub use crate::app::*;
    pub use eframe::egui;
    pub use rand::Rng;
    pub use std::collections::HashMap;
}

fn main() {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "rust-mine-sweep",
        native_options,
        Box::new(|cc| Box::new(MineSweep::new(cc))),
    );
}


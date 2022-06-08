use crate::prelude::*;
use eframe::egui::{CentralPanel, Ui};

/// Our application, contains a score, a timer, and the gameboard itself
pub struct MineSweep {
    score: usize,
    time: f32,
    game: Board,
}

impl MineSweep {
    /// Initialization of our gameboard
    pub fn new() -> Self {
        Self {
            score: 0,
            time: 0.,
            game: Board::new(),
        }
    }
}

impl App for MineSweep {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        CentralPanel::default().show(ctx, |Ui| Ui.label("MineSweep"));
    }

    fn save(&mut self, _storage: &mut dyn Storage) {
        todo!()
    }
}

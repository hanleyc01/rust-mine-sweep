use eframe::egui::Visuals;

use crate::prelude::*;

#[derive(Default)]
pub struct MineSweep {}

impl MineSweep {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        Self::default()
    }
}

impl eframe::App for MineSweep {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark()); 
        egui::TopBottomPanel::top("Menu Panel").show(ctx, |ui| {
            ui.label("Welcome to Rust Mine Sweeper!");
         });
         egui::CentralPanel::default().show(ctx, |ui| {
            if ui.add(egui::Button::new("⚪")).clicked() {
            }
            
         });
    }
}
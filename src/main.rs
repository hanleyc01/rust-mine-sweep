use std::{collections::HashMap};
//use eframe::egui::*;

enum CellValue {
    Covered(bool),
    Mine(bool),
    Number(u32),
    Flag(bool),
}

/// This will be the main abstraction to represent our game board, containing
/// all important information about the current game state.
struct Board {
    width: u32,
    height: u32,
    playing_board: HashMap<(u32, u32), [CellValue; 5]>,
}

impl Board {
    fn new(height: u32, width: u32) -> Self {
        Self {
            width,
            height,
            playing_board: initialize_board(width, height),
        }
    }
}

/// Helper function which ought to intialize our playing board with three goals: 
/// 1) Describe all cells as being `Covered(bool)`; 2) Randomize the placement of `Mine(bool)` within the cells;
/// 3) Describe the cells surrounding the cells with mines with a specific `Number(u32)`, where `u32` is defined as the amount of cells
/// nearby which contain a `Mine(true)`; 4) set all cells to `Flag(false)`.
fn initialize_board(width: u32, height: u32) -> HashMap<(u32, u32), [CellValue; 5]> {
    use crate::CellValue::*;
    let mut playing_board: HashMap<(u32, u32), [CellValue; 5]> = HashMap::new();

    let mine_cells: [(u32, u32); 10] = get_random_pairs(width, height);

    for x in 0..width {
        for y in 0..height {
            let mut cell_descriptions: [CellValue; 5] = [Covered(true), todo!(), todo!(), todo!(), todo!(),];
        }
    }

    playing_board
}

/// Generates a list of `(u32, u32)` pairs which ought to contain the (x,y) coordinates of cells
/// which have mines in them.
fn get_random_pairs(width: u32, height: u32) -> [(u32, u32); 10] {
    todo!();
}

fn main() {
    println!("Hello, world!");
}

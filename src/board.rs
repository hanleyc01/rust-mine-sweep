use crate::prelude::*;

/// The `CellValue` enumerated type provides a series of descriptions which describe a particular cell on the board.
/// Using an enumerated type allows for easy reasoning and pattern-matching.
#[derive(Debug)]
pub enum CellValue {
    Covered(bool),
    Mine(bool),
    Number(u32),
    Flag(bool),
}

/// This will be the main abstraction to represent our game board, containing
/// all important information about the current game state.
#[derive(Debug)]
pub struct Board {
    width: u32,
    height: u32,
    difficulty: u32,
    playing_board: HashMap<(u32, u32), [CellValue; 4]>,
}

impl Board {
    
    /// Initializes a board of paramterized width, height, difficulty (number of mines).
    /// This simply sets the board as being `[Covered(true), Mine(false), Number(0), Flag(false)]
    pub fn new(height: u32, width: u32, difficulty: u32) -> Self {
        Self {
            width,
            height,
            difficulty,
            playing_board: initialize_board(width, height, difficulty),
        }
    }
    
}

/// Helper function which ought to intialize our playing board with three goals:
/// 1. Describe all cells as being `Covered(bool)`;
/// 2. Randomize the placement of `Mine(bool)` within the cells;
/// 3. Describe the cells surrounding the cells with mines with a specific `Number(u32)`, where `u32` is defined as the amount of cells
/// nearby which contain a `Mine(true)`;
/// 4. set all cells to `Flag(false)`.
pub fn initialize_board(width: u32, height: u32, diff: u32) -> HashMap<(u32, u32), [CellValue; 4]> {
    use crate::CellValue::*;
    let mut playing_board: HashMap<(u32, u32), [CellValue; 4]> = HashMap::new();

    for x in 0..width {
        for y in 0..height {
            let mut cell_descriptions: [CellValue; 4] = [
                Covered(true),
                Mine(false),
                Number(0),
                Flag(false),
            ];
            playing_board.insert((x,y), cell_descriptions);
        }
    }

    playing_board
}

use std::{cell::Cell, hash::Hash};

use crate::prelude::*;

/// The `CellValue` enumerated type provides a series of descriptions which describe a particular cell on the board.
/// Using an enumerated type allows for easy reasoning and pattern-matching.
#[derive(Debug, Copy, Clone)]
pub enum CellValue {
    Uncovered(bool),
    Mine(bool),
    Number(u32),
    Flag(bool),
}

/// This will be the main abstraction to represent our game board, containing
/// all important information about the current game state.
#[derive(Debug)]
pub struct Board {
    pub width: u32,
    pub height: u32,
    pub difficulty: u32,
    pub covered: HashMap<(u32, u32), CellValue>,
    pub mines: HashMap<(u32, u32), CellValue>,
    pub numbers: HashMap<(u32, u32), CellValue>,
    pub flagged: HashMap<(u32, u32), CellValue>,
}

impl Board {
    /// Initializes a board of paramterized width, height, difficulty (number of mines).
    /// This simply sets the board as being `[Covered(true), Mine(false), Number(0), Flag(false)]
    pub fn new(width: u32, height: u32, difficulty: u32) -> Self {
        Self {
            width,
            height,
            difficulty,
            covered: initialize_cells(width, height, Uncovered(false)),
            mines: initialize_cells(width, height, Mine(false)),
            numbers: HashMap::new(),
            flagged: HashMap::new(),
        }
    }
      
}

/// Randomizes the placesment of mines within the playing board
fn place_mines(w: u32, h: u32, diff: u32) -> HashMap<(u32, u32), CellValue> {
    let mut board = HashMap::new();

    let mut mine_vec = get_random_pairs(w, h, diff);

    for i in 0..w {
        for j in 0..h {
            
            if mine_vec.contains(&(i,j)) {
                board.insert((i, j), Mine(true));
                let mut mine_vec = mine_vec.retain(|&x| x != (i, j));
            } else {
                board.insert((i,j), Mine(false));
            }
        
        }
    }

    board
}

/// Parameterized way to initialize all cell values with some value.
fn initialize_cells(w:u32, h:u32, cell_val: CellValue) -> HashMap<(u32, u32), CellValue> {
    let mut hash: HashMap<(u32, u32), CellValue> = HashMap::new();

    for i in 0..w {
        for j in 0..h {
            hash.insert((i, j), cell_val);
        }
    }

    hash
}

/// Takes self.difficulty to determine how many pairs will have a `Mine(true)`.
/// Note that this does have the possibility of generating pairs which are identical; however, it has a very
/// low chance beacuse of the `pairs.contains()`. Not perfect, but it'll do.
fn get_random_pairs(w: u32, h: u32, diff: u32) -> Vec<(u32, u32)> {
    let mut pairs: Vec<(u32, u32)> = Vec::new();

    let mut rng = rand::thread_rng();
    
    for _i in 0..diff {
        
        let mut x_y: (u32, u32) = (rng.gen_range(0..w), rng.gen_range(0..h));
        
        if pairs.contains(&x_y) {
            let x_y = (rng.gen_range(0..w), rng.gen_range(0..h));
            pairs.push(x_y); 
        } else {
            pairs.push(x_y);
        }

    }

    pairs
}
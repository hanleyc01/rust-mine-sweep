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
            playing_board: {
                use crate::board::CellValue::*;

                let mut playing_board: HashMap<(u32, u32), [CellValue; 4]> = HashMap::new();

                for x in 0..width {
                    for y in 0..height {
                        let mut cell_descriptions: [CellValue; 4] =
                            [Covered(true), Mine(false), Number(0), Flag(false)];
                        playing_board.insert((x, y), cell_descriptions);
                    }
                }

                playing_board
            },
        }
    }

    /// Randomizes the placement of mines depending on the the Board's difficulty
    pub fn place_mines(&mut self) {
        // Difficulty is the number of mines placed on the board
        let difficulty = self.difficulty; 

        // Storage vector to contain all the pairs determined by rng to have the mine
        let mut mine_pairs: Vec<(u32, u32)> = get_random_pairs(self.width, self.height, difficulty); 

    }
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
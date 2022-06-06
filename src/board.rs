use crate::prelude::*;

/// Main abstraction used for representing the game board; note how `game_board` is represented by constants
/// defined in the prelude.
#[derive(Debug, PartialEq, Clone)]
pub struct Board {
    game_board: [[Tile; HEIGHT]; WIDTH],
}

/// This is the main abstraction used to represent the cells within the playing board.
#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Tile {
    num: MineProx,
    visibility: Visibility,
}

/// Describes the visibility of a specific `Tile` within the playingboard
#[derive(Debug, PartialEq, Clone, Copy)]
pub(super) enum Visibility {
    Covered,
    Uncovered,
    Flag,
}

/// Abstraction to represent the proximity of nearby mines in other `Tiles`, this will be ultimately calculated
/// on an analysis of `Board`.
#[derive(Debug, PartialEq, Clone, Copy)]
pub(super) enum MineProx {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Mine,
}

impl Board {
    /// Initializes the board using the `get_random_pairs()` helper function to determine where the mines will be on
    /// the board.
    pub fn new() -> Self {
        Self {
            game_board: {
                
                let mines: [(usize, usize) ; NUM_MINES] = get_random_pairs();
                
                let mut board: [[Tile; HEIGHT]; WIDTH] = [[Tile::init_no_mine(); HEIGHT] ; WIDTH];
                for i in 0..WIDTH {
                    for j in 0..HEIGHT {
                        
                        let pair = (i,j);
                        
                        if mines.contains(&pair) {
                            board[i][j] = Tile::init_mine();
                        }

                    }
                }

                board
            },
        }
    }
}

impl Tile {
    /// Initializes a Tile with no mine.
    fn init_no_mine() -> Self {
        Self {
            num: MineProx::Zero,
            visibility: Visibility::Covered,
        }
    }

    /// Initializes a cell a mine.
    fn init_mine() -> Self {
        Self {
            num: MineProx::Mine,
            visibility: Visibility::Covered,
        }
    }
}

/// Helper function which generates an `array` of `(u32, u32)`, size `const NUM_MINES`,
///  which stores the `Tile`'s which contain a mine
fn get_random_pairs() -> [(usize, usize); NUM_MINES] {
    let mut arr: [(usize, usize); NUM_MINES] = [(0, 0); NUM_MINES];

    let mut rng = rand::thread_rng();

    for i in 0..NUM_MINES {
        let x = rng.gen_range(0..WIDTH);
        let y = rng.gen_range(0..HEIGHT);

        arr[i] = (x, y);
    }

    arr
}

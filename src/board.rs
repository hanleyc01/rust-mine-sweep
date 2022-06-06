use crate::prelude::*;

/// Main abstraction used for representing the game board; note how `game_board` is represented by constants 
/// defined in the prelude.
pub struct Board {
    num_mines: u32,
    game_board: [[Tile; WIDTH] ; HEIGHT],
}

/// This is the main abstraction used to represent the cells within the playing board.
pub struct Tile {
    num: MineProx,
    visibility: Visibility,
}

/// Describes the visibility of a specific `Tile` within the playingboard
pub(super) enum Visibility {
    Covered,
    Uncovered,
    Flag,
}

/// Abstraction to represent the proximity of nearby mines in other `Tiles`, this will be ultimately calculated
/// on an analysis of `Board`
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
}
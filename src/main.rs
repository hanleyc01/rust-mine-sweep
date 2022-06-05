use rand::Rng;
use std::collections::HashMap;
//use eframe::egui::*;

/// The `CellValue` enumerated type provides a series of descriptions which describe a particular cell on the board.
/// Using an enumerated type allows for easy reasoning and pattern-matching.
#[derive(Debug)]
enum CellValue {
    Covered(bool),
    Mine(bool),
    Number(u32),
    Flag(bool),
}

/// This will be the main abstraction to represent our game board, containing
/// all important information about the current game state.
#[derive(Debug)]
struct Board {
    width: u32,
    height: u32,
    difficulty: u32,
    playing_board: HashMap<(u32, u32), [CellValue; 4]>,
}

impl Board {
    fn new(height: u32, width: u32, difficulty: u32) -> Self {
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
fn initialize_board(width: u32, height: u32, diff: u32) -> HashMap<(u32, u32), [CellValue; 4]> {
    use crate::CellValue::*;
    let mut playing_board: HashMap<(u32, u32), [CellValue; 4]> = HashMap::new();

    let mut mine_cells: Vec<(u32, u32)> = get_random_pairs(width, height, diff);

    for x in 0..width {
        for y in 0..height {
            let mut cell_descriptions: [CellValue; 4] = [
                Covered(true),
                Mine(check_identity(x, y, &mut mine_cells)),
                Number(0),
                Flag(false),
            ];

            playing_board.insert((x,y), cell_descriptions);
        }
    }

    playing_board
}

/// Generates a vector of `(u32, u32)` pairs which ought to contain the (x,y) coordinates of cells
/// which have mines in them; vector should be of size `diff`, or difficulty.
fn get_random_pairs(width: u32, height: u32, diff: u32) -> Vec<(u32, u32)> {
    let mut mine_arr: Vec<(u32, u32)> = Vec::new();
    let mut rng = rand::thread_rng();

    for i in 0..diff {
        let x = rng.gen_range(0..width);
        let y = rng.gen_range(0..height);
        mine_arr.push((x, y));
    }

    mine_arr
}

/// Testing out an rng which does not duplicate an of the values (relative to the `x` and `y` of the cell).
fn non_dup_rand_pairs(width: u32, height: u32, diff: u32) -> Vec<(u32, u32)> {
    use rand::seq::SliceRandom;
    use rand::thread_rng;
    

    let mut rng = thread_rng();
    let mut x: Vec<u32> = (0..width).collect();
    let mut y: Vec<u32> = (0..height).collect();

    x.shuffle(&mut rng);
    y.shuffle(&mut rng);

    let mut x = x.split_at(diff as usize);
    let mut y = y.split_at(diff as usize);
    
    let non_dup_pairs = x.1.into_iter().zip(y.1.into_iter()).collect::<Vec<_>>();

    non_dup_pairs
}

/// Checks the head of the list of randomly generated mines in the playing field against the x and y
/// looped over; and if `(x,y)` == `(fst head mines, snd head mines)` => it drops the head, and returns true. Otherwise, it returns false
fn check_identity(x: u32, y: u32, mines: &mut Vec<(u32, u32)>) -> bool {
    let head = &mines[0];

    if (head.0, head.1) == (x, y) {
        mines.remove(0);
        true
    } else {
        false
    }
}

fn main() {

    let x = initialize_board(10, 10, 5);
    
    for x_h in 0..10 {
        for y_h in 0..10{
            println!("({}, {}) is: {:?}", x_h, y_h, x.get(&(x_h, y_h)))
        }
    }

}

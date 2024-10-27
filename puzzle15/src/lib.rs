use std::collections::HashSet;

/// Holds information about which tile is in which position.
/// Should be fairly compact and easy to copy.
#[derive(Debug, Clone)]
pub struct GameState {
    // TODO
    // Initialize an array of arrays (to represent a 4 * 4 matrix)
    // Option: to cater for the changing empty tile which is filled
    tile_array: [[Option<u8> ; 4]; 4],
}

/// Creates the default position of tiles, starting with 1 in the top left corner.
impl Default for GameState {
    fn default() -> Self {
        //todo!()
        let mut tile = [[None ; 4]; 4]; // Initially set all tiles as None (empty)
        
        // Matrix definition
        //                 Columns
        //      Index   0    1    2    3
        //         0 |  1 |  2 |  3 |  4 |
        // Rows    1 |  5 |  6 |  7 |  8 |
        //         2 |  9 | 10 | 11 | 12 |
        //         3 | 13 | 14 | 15 |    |
        //
        // [3][0] = 13, [3][1] = 14, [3][2] = 15
        // x(3) + y(0) + z = 13, 3x + z = 13        --> eqn 1
        // x(3) + y(1) + z = 14, 3x + y + z = 14    --> eqn 2
        // x(3) + y(2) + z = 15, 3x + 2y + z = 15   --> eqn 3
        // Solve simultaneously, x = 4, y = 1 and z = 1
        // Formula for each cell = 4 * (row_index) + (column_index) + 1
        
        for i in 0..4{
            for j in 0..4{
                let tile_value = 4 * (j) + i + 1;
                
                if tile_value == 16 {
                    tile[i][j] = None;
                } else {
                    tile[i][j] = Some(tile_value as u8);
                }
            }
        }
        GameState{
            tile_array: tile
        }
    }
}

/// Generates a human-readable representation of the game state.
impl std::fmt::Display for GameState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //todo!()

        for i in 0..4{
            for j in 0..4{
                let tile = self.get(j,i);

                match tile{
                    Some(tile) => write!(f, "| {:>2} ", tile)?, //Right align
                    None => write!(f, "|    ")?,
                }
            }
            writeln!(f, "|")?;
        }
        Ok(())
        
    }
}

/// Checks whether two game states are the same,.
impl PartialEq for GameState {
    fn eq(&self, other: &Self) -> bool {
        // todo!()
        self.tile_array == other.tile_array
    }
}

/// Feel free to ignore this. (but do not remove)
impl Eq for GameState {}

impl GameState {
    // /// Updates a position with a new tile.
    pub fn set(&mut self, x: u8, y: u8, tile: Option<u8>) {
        //todo!()
        self.tile_array[x as usize][y as usize] = tile;
    }

    /// Returns the tile at position x,y.
    pub fn get(&self, x: u8, y: u8) -> Option<u8> {
        //todo!()
        
        self.tile_array[x as usize][y as usize]

    }

    /// Returns false if there is a duplicate tile in this game state.
    pub fn all_tiles_unique(&self) -> bool {
        //todo!()

        let mut set= HashSet::new();

        for i in 0..4{
            for j in 0..4{
                set.insert(self.get(i,j));
            }
        }
        
        set.len() == 16
    }

    /// Swaps the tile from (x1,y1) with the tile from (x2,y2)
    pub fn swap(&mut self, x1: u8, y1: u8, x2: u8, y2: u8) {
        //todo!()
        
        let first_value = self.get(x1, y1);
        let second_value = self.get(x2, y2);
        self.set(x1, y1, second_value);
        self.set(x2, y2, first_value);

    }

    /// Locates the position of the empty tile [None] in GameStruct
    /// Returns a tuple of its coordinates
    pub fn empty_tile(&self) -> Option<(u8, u8)> {

        for i in 0..4 {
            for j in 0..4 {
                if self.get(i as u8, j as u8) == None {
                    return Some((i, j));
                }
            }
        }
        None
    }

    /// Updates the state to reflect the move that was performed. Returns false if the move was
    /// not possible.
    pub fn perform_move(&mut self, m: Move) -> bool {
        //todo!()

        if let Some((none_tile_col_index, none_tile_row_index)) = self.empty_tile() {

            match m {
                Move::LeftToRight => {
                    if none_tile_col_index > 0 {
                        self.swap(none_tile_col_index, none_tile_row_index, none_tile_col_index - 1, none_tile_row_index);
                        true
                    } else {
                        false
                    }
                }
                
                Move::RightToLeft => {
                    if none_tile_col_index < 3 {
                        self.swap(none_tile_col_index, none_tile_row_index, none_tile_col_index + 1, none_tile_row_index);
                        true
                    } else {
                        false
                    }
                }
                Move::TopToBottom => {
                    if none_tile_row_index > 0 {
                        self.swap(none_tile_col_index, none_tile_row_index, none_tile_col_index, none_tile_row_index - 1);
                        true
                    } else {
                        false
                    }
                }
                Move::BottomToTop => {
                    if none_tile_row_index < 3 {
                        self.swap(none_tile_col_index, none_tile_row_index, none_tile_col_index, none_tile_row_index + 1);
                        true
                    } else {
                        false
                    }
                }
            }
    } else {
        false
    }
        
    }

    /// Performs a series of moves. Returns the number of moves that were successful.
    pub fn perform_moves(&mut self, moves: &[Move]) -> usize {
        //todo!()

        let mut number_of_successful_moves = 0;

        for &i in moves{
            if self.perform_move(i){
                number_of_successful_moves += 1;
            }
        }
        number_of_successful_moves
    }

    /// Tries to parse a game state from the provided string.
    /// Returns None if parsing is not possible, or if the parsed game state would contain
    /// duplicate or invalid tiles.
    /// Ignores whitespace.
    pub fn from_str(s: &str) -> Option<Self> {
        // todo!()

        let mut state = GameState::default();
        let mut all_tiles_stringed = s.split_whitespace();

        for i in 0..4 {
            for j in 0..4 {
                if let Some(individual_tile) = all_tiles_stringed.next() {
                    if individual_tile == "" {
                        state.tile_array[i][j] = None
                    } else if let Ok(n) = individual_tile.parse(){
                        state.tile_array[i][j] = Some(n)
                    } else {
                        return None
                    }
                }
            }
        }


        if state.all_tiles_unique(){
            Some(state)
        } else {
            None
        }
    }
}

// /// Finds the minimal number of moves needed to get from one state to the other.
// /// Might run forever if there is no path, so use with caution!
// pub fn find_shortest_path(from: GameState, to: GameState) -> Vec<Move> {
//     todo!()
// }

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
#[repr(u8)]
pub enum Move {
                // Inference from test cases
    LeftToRight,// Swap empty tile with tile to its left
    RightToLeft,// Swap empty tile with tile to its right
    TopToBottom,// Swap empty tile with tile above it
    BottomToTop,// Swap empty tile with tile below it
}


#[cfg(test)]
mod tests;
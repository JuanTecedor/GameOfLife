type Cell = bool;
type GridType = [[Cell; GAME_SIZE]; GAME_SIZE];

const GAME_SIZE : usize = 32;

pub struct Game {
    grid : GridType,
}

impl Game {
    pub fn grid(&self) -> &GridType {
        &self.grid
    }

    pub fn new_test_1() -> Self {
        let mut grid = [[false; GAME_SIZE]; GAME_SIZE];
        grid[0][0] = true;
        grid[0][1] = true;
        grid[1][0] = true;
        grid[1][1] = true;
        grid[10][11] = true;
        grid[10][12] = true;
        grid[10][13] = true;
        Self {
            grid
        }
    }
}
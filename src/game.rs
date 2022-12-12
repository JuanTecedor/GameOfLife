type Cell = bool;
type GridType = [[Cell; GAME_SIZE]; GAME_SIZE];

pub const GAME_SIZE : usize = 32;

#[derive(Copy, Clone)]
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

    fn neighbours(&self, x: usize, y: usize) -> u32 {
        let mut count : u32 = 0;
        for i in -1..2_i32 {
            for j in -1..2_i32 {
                if !(i == 0 && j == 0) {
                    let xx = x as i32 + i;
                    let yy = y as i32 + j;
                    if xx > 0 && xx < GAME_SIZE as i32
                        && yy > 0 && yy < GAME_SIZE as i32 {
                        if self.grid[xx as usize][yy as usize] {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    pub fn next_cell_status(&self, x: usize, y: usize) -> bool {
        let neighbours = self.neighbours(x, y);
        if neighbours == 3 {
            true
        }
        else if neighbours == 2 {
            self.grid[x][y]
        }
        else {
            false
        }
    }

    pub fn set_cell_status(&mut self, new_status: Cell, x: usize, y: usize) {
        self.grid[x][y] = new_status;
    }
}
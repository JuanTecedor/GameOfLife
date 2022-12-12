pub const GAME_SIZE : usize = 32;

#[derive(Copy, Clone)]
pub struct Cell {
    pub current_state: bool,
    pub next_state: bool,
}

type GridType = [[Cell; GAME_SIZE]; GAME_SIZE];

pub struct Game {
    grid : GridType,
}

impl Game {
    pub fn grid(&self) -> &GridType {
        &self.grid
    }

    pub fn new_test_1() -> Self {
        let mut grid = [[Cell{current_state: false, next_state: false}; GAME_SIZE]; GAME_SIZE];
        grid[1][1] = Cell{current_state: true, next_state: false};
        grid[1][2] = Cell{current_state: true, next_state: false};
        grid[2][1] = Cell{current_state: true, next_state: false};
        grid[2][2] = Cell{current_state: true, next_state: false};
        grid[10][11] = Cell{current_state: true, next_state: false};
        grid[10][12] = Cell{current_state: true, next_state: false};
        grid[10][13] = Cell{current_state: true, next_state: false};
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
                        if self.grid[xx as usize][yy as usize].current_state {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    pub fn calculate_next_cell_status(&mut self) {
        for i in 0..GAME_SIZE {
            for j in 0..GAME_SIZE {
                let neighbours = self.neighbours(i, j);
                if neighbours == 3 {
                    self.grid[i][j].next_state = true
                }
                else if neighbours == 2 {
                    let mut cell = &mut self.grid[i][j];
                    cell.next_state = cell.current_state
                }
                else {
                    self.grid[i][j].next_state = false
                }
            }
        }
    }

    pub fn update_cells(&mut self) {
        for i in 0..GAME_SIZE {
            for j in 0..GAME_SIZE {
                let mut cell = &mut self.grid[i][j];
                cell.current_state = cell.next_state;
            }
        }
    }
}
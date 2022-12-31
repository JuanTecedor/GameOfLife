use crate::{cell::Cell, cell_status::CellStatus};

pub type GridType = Vec<Vec<Cell>>;

pub struct Game {
    grid: GridType,
}

impl Game {
    const DEFAULT_GAME_SIZE: usize = 128;

    pub fn new(grid: GridType) -> Self {
        for col in &grid {
            debug_assert!(col.len() == grid.len());
        }
        Self { grid: grid }
    }

    pub fn new_empty_default_size() -> Self {
        Self {
            grid: vec![
                vec![Cell::new(CellStatus::DEAD); Game::DEFAULT_GAME_SIZE];
                Game::DEFAULT_GAME_SIZE
            ],
        }
    }

    pub fn new_random_default_size() -> Self {
        Self::new_random(Game::DEFAULT_GAME_SIZE)
    }

    pub fn new_random(size: usize) -> Self {
        let mut grid: Vec<Vec<Cell>> = Vec::with_capacity(size);
        for row in 0..size {
            grid.push(Vec::with_capacity(size));
            for _ in 0..size {
                grid[row].push(Cell::new_random());
            }
        }
        Self::new(grid)
    }

    pub fn grid(&self) -> &GridType {
        &self.grid
    }

    fn count_neighbours(&self, x: usize, y: usize) -> u32 {
        let mut count: u32 = 0;
        for i in -1..2_i32 {
            for j in -1..2_i32 {
                if !(i == 0 && j == 0) {
                    let xx = x as i32 + i;
                    let yy = y as i32 + j;
                    if xx > 0
                        && xx < self.grid.len() as i32
                        && yy > 0
                        && yy < self.grid.len() as i32
                    {
                        if self.grid[xx as usize][yy as usize].current_state() == CellStatus::ALIVE
                        {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    fn calculate_next_cell_status(&mut self) {
        for i in 0..self.grid.len() {
            for j in 0..self.grid.len() {
                let neighbours = self.count_neighbours(i, j);
                let cell = &mut self.grid[i][j];
                if neighbours == 3 {
                    cell.set_next_state(CellStatus::ALIVE);
                } else if neighbours == 2 {
                    cell.set_next_state(cell.current_state());
                } else {
                    cell.set_next_state(CellStatus::DEAD);
                }
            }
        }
    }

    pub fn update_cells(&mut self) {
        self.calculate_next_cell_status();
        for i in 0..self.grid.len() {
            for j in 0..self.grid.len() {
                self.grid[i][j].update();
            }
        }
    }

    pub fn set_current_state(&mut self, i: usize, j: usize, next_state: CellStatus) {
        if i < self.game_side() && j < self.game_side() {
            self.grid[i][j].set_current_state(next_state);
        }
    }

    pub fn game_side(&self) -> usize {
        self.grid().len()
    }
}

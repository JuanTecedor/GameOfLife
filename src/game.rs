use crate::cell::{Cell, CellStatus};

type GridType = Vec<Vec<Cell>>;

pub struct Game {
    grid: GridType,
}

impl Game {
    pub fn new(grid: GridType) -> Self {
        for col in &grid {
            debug_assert!(col.len() == grid.len());
        }
        Self { grid: grid }
    }

    pub fn new_random(size: usize) -> Self {
        let mut grid = vec![
            vec![
                Cell {
                    current_state: CellStatus::DEAD,
                    next_state: CellStatus::DEAD
                };
                size
            ];
            size
        ];
        for col in &mut grid {
            for cell in col {
                cell.current_state = if rand::random() {
                    CellStatus::DEAD
                } else {
                    CellStatus::ALIVE
                };
            }
        }
        Self::new(grid)
    }

    pub fn grid(&self) -> &GridType {
        &self.grid
    }

    fn neighbours(&self, x: usize, y: usize) -> u32 {
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
                        if let CellStatus::ALIVE = self.grid[xx as usize][yy as usize].current_state
                        {
                            count += 1;
                        }
                    }
                }
            }
        }
        count
    }

    pub fn calculate_next_cell_status(&mut self) {
        for i in 0..self.grid.len() {
            for j in 0..self.grid.len() {
                let neighbours = self.neighbours(i, j);
                let mut cell = &mut self.grid[i][j];
                if neighbours == 3 {
                    cell.next_state = CellStatus::ALIVE
                } else if neighbours == 2 {
                    cell.next_state = cell.current_state
                } else {
                    cell.next_state = CellStatus::DEAD
                }
            }
        }
    }

    pub fn update_cells(&mut self) {
        for i in 0..self.grid.len() {
            for j in 0..self.grid.len() {
                let mut cell = &mut self.grid[i][j];
                cell.current_state = cell.next_state;
            }
        }
    }
}

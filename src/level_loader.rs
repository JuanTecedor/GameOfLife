use std::{
    fs::File,
    io::{self, BufRead, Error},
    path::Path,
};

use crate::{cell::Cell, cell_status::CellStatus, game::GridType};

pub fn load_level() -> Result<GridType, Error> {
    let path = Path::new("data").join("example.txt");
    match File::open(path) {
        Err(e) => return Result::Err(e),
        Ok(file) => {
            let mut grid: GridType = vec![];
            let lines = io::BufReader::new(file).lines();
            for line in lines {
                if let Ok(line_str) = line {
                    let mut row: Vec<Cell> = vec![];
                    for char in line_str.chars() {
                        if char != '\n' {
                            row.push(Cell::new(if char == '.' {
                                CellStatus::DEAD
                            } else {
                                CellStatus::ALIVE
                            }))
                        }
                    }
                    grid.push(row);
                }
            }
            return Ok(grid);
        }
    }
}

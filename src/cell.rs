#[derive(Copy, Clone)]
pub enum CellStatus {
    ALIVE,
    DEAD,
}

#[derive(Copy, Clone)]
pub struct Cell {
    pub current_state: CellStatus,
    pub next_state: CellStatus,
}

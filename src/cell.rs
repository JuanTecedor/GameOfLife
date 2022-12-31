use crate::cell_status::CellStatus;

#[derive(Copy, Clone)]
pub struct Cell {
    current_state: CellStatus,
    next_state: CellStatus,
}

impl Cell {
    pub fn new(cell_status: CellStatus) -> Self {
        Self {
            current_state: cell_status,
            next_state: cell_status,
        }
    }

    pub fn new_random() -> Self {
        Self::new(if rand::random() {
            CellStatus::ALIVE
        } else {
            CellStatus::DEAD
        })
    }

    pub fn set_current_state(&mut self, current_state: CellStatus) {
        self.current_state = current_state;
    }

    pub fn set_next_state(&mut self, next_state: CellStatus) {
        self.next_state = next_state;
    }

    pub fn current_state(&self) -> CellStatus {
        self.current_state
    }

    pub fn update(&mut self) {
        self.current_state = self.next_state;
    }
}

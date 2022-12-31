use crate::cell_status::CellStatus;

struct SetCellCommand {
    new_status: CellStatus,
    grid_x: usize,
    grid_y: usize,
}

pub enum Command {
    Quit,
    Step,
    ToggleAutostep,
    LoadRandomGame,
    LoadEmptyGame,
    LoadExample,
    SetCellCommand,
}
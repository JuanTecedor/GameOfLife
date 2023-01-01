use crate::cell_status::CellStatus;

pub enum Command {
    Quit,
    Step,
    ToggleAutostep,
    LoadRandomGame,
    LoadEmptyGame,
    LoadExample,
    SetCellCommand {
        new_status: CellStatus,
        grid_x: usize,
        grid_y: usize,
    },
}

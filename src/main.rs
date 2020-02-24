mod cell;
mod cell_grid;
mod ui;

use cell_grid::CellGrid;
use std::{
    io::{self, Error},
    thread,
    time::Duration,
};
use termion::raw::IntoRawMode;
use ui::TUI;

fn main() -> Result<(), Error> {
    let mut grid = CellGrid::new(8);
    grid.toggle(0, 3);
    grid.toggle(5, 4);
    grid.toggle(3, 3);
    grid.toggle(5, 6);
    grid.toggle(7, 3);
    grid.toggle(7, 7);
    grid.toggle(4, 7);
    grid.toggle(3, 0);
    grid.toggle(3, 1);
    grid.toggle(3, 2);

    let mut ui = TUI::new(io::stdout().into_raw_mode()?)?;

    loop {
        grid.update();
        ui.render(grid.get_alive_cells(), grid.get_dim())?;
        thread::sleep(Duration::from_secs(1));
    }
}

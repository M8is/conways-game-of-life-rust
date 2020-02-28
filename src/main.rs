mod cell;
mod cell_grid;
mod ui;

use cell_grid::CellGrid;
use std::{
    convert::TryFrom,
    io::{self, Stdout},
    thread,
    time::Duration,
};
use ui::TUI;

fn main() {
    let mut game = Game::new().unwrap();
    game.run().unwrap();
}

struct Game {
    grid: CellGrid,
    ui: TUI<Stdout>,
}

impl Game {
    /// Creates a new game
    pub fn new() -> Result<Game, Box<dyn std::error::Error>> {
        let grid = CellGrid::new(16);
        let ui = TUI::new(io::stdout(), u16::try_from(grid.get_dim())?)?;
        Ok(Game { grid, ui })
    }

    /// Starts the game
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.set_cell_active(0, 3)?;
        self.set_cell_active(5, 4)?;
        self.set_cell_active(3, 3)?;
        self.set_cell_active(5, 6)?;
        self.set_cell_active(7, 3)?;
        self.set_cell_active(7, 7)?;
        self.set_cell_active(4, 7)?;
        self.set_cell_active(3, 0)?;
        self.set_cell_active(3, 1)?;
        self.set_cell_active(3, 2)?;
        self.set_cell_active(7, 15)?;
        self.set_cell_active(4, 15)?;
        self.set_cell_active(3, 10)?;
        self.set_cell_active(3, 11)?;
        self.set_cell_active(3, 12)?;
        self.set_cell_active(12, 12)?;
        self.set_cell_active(13, 12)?;
        self.set_cell_active(14, 14)?;
        self.set_cell_active(13, 15)?;
        self.set_cell_active(12, 13)?;

        for _ in 1..16 {
            self.grid.update();
            self.ui.clear()?;
            for (x, y) in self.grid.get_alive_cells() {
                self.ui
                    .set_point(u16::try_from(x)?, u16::try_from(y)?, true)?;
            }
            self.ui.flush()?;
            thread::sleep(Duration::from_secs(1));
        }
        self.ui.clear()?;

        Ok(())
    }

    fn set_cell_active(&mut self, x: u16, y: u16) -> Result<(), io::Error> {
        self.grid.toggle(x as usize, y as usize);
        Ok(self.ui.set_point(x, y, true)?)
    }
}

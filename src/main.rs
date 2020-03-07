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
    game.run();
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
    pub fn run(&mut self) {
        let alive_cells = [
            (3, 0),
            (3, 1),
            (3, 2),
            (3, 3),
            (0, 3),
            (7, 3),
            (5, 4),
            (5, 6),
            (4, 7),
            (7, 7),
            (3, 10),
            (3, 11),
            (3, 12),
            (12, 12),
            (13, 12),
            (12, 13),
            (14, 14),
            (4, 15),
            (7, 15),
            (13, 15),
        ];

        for (x, y) in alive_cells.iter() {
            self.grid.toggle(*x, *y).unwrap();
        }

        for _ in 1..16 {
            self.grid.update();
            self.ui.draw(self.grid.get_states()).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    }
}

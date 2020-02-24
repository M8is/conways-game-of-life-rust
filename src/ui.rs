use std::io::{Error, Write};
use tui::{
    backend::TermionBackend,
    layout::{Constraint, Direction, Layout},
    widgets::{Block, Borders, Dataset, Widget},
    Terminal,
};

pub struct TUI<W>
where
    W: Write,
{
    terminal: Terminal<TermionBackend<W>>,
}

impl<W> TUI<W>
where
    W: Write,
{
    /// Creates a new TUI
    pub fn new(out: W) -> Result<TUI<W>, Error> {
        let terminal = Terminal::new(TermionBackend::new(out))?;
        Ok(TUI { terminal })
    }

    pub fn render(&mut self, grid: Vec<(usize, usize)>, n: usize) -> Result<(), Error> {
        self.terminal.draw(|mut f| {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)].as_ref())
                .split(f.size());
            Dataset::default().name("Alive Cells").data(grid);
            Block::default()
                .title("Conways Game of Life")
                .borders(Borders::ALL)
                .render(&mut f, chunks[1]);
        })
    }
}

#[cfg(test)]
mod tui_tests {
    use super::*;
    use std::io::{Cursor, Error};

    #[test]
    fn initializes() -> Result<(), Error> {
        let c = Cursor::new(Vec::new());
        let _tui = TUI::new(c)?;
        Ok(())
    }
}

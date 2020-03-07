use std::io::{Error, Write};
use termion::{
    clear, cursor,
    raw::{IntoRawMode, RawTerminal},
    style,
};

pub struct TUI<W>
where
    W: Write,
{
    terminal: RawTerminal<W>,
    size: u16,
}

impl<W> TUI<W>
where
    W: Write,
{
    /// Creates a new TUI with a squared grid of the given size
    pub fn new(out: W, size: u16) -> Result<TUI<W>, Error> {
        let mut terminal = out.into_raw_mode()?;
        write!(terminal, "{}", clear::All)?;
        let tui = TUI { terminal, size };
        Ok(tui)
    }

    /// Draws the grid
    pub fn draw(&mut self, grid: Vec<bool>) -> Result<(), Error> {
        let alive_repr = format!("{}{}{}", style::Bold, "O", style::Reset);
        let dead_repr = " ";
        let repr = grid
            .iter()
            .enumerate()
            .fold(String::new(), |acc, (i, alive)| {
                let value_repr = if *alive {
                    acc + &alive_repr
                } else {
                    acc + dead_repr
                };
                if i > 0 && i % self.size as usize == 0 {
                    format!("{}{}", value_repr, "\r\n")
                } else {
                    value_repr
                }
            });

        write!(self.terminal, "{}{}", cursor::Goto(1, 1), repr)?;
        self.terminal.flush()
    }
}

impl<W> Drop for TUI<W>
where
    W: Write,
{
    fn drop(&mut self) {
        write!(self.terminal, "{}{}", cursor::Goto(1, 1), clear::All).unwrap();
    }
}

#[cfg(test)]
mod tui_tests {
    use super::*;
    use std::io::{Cursor, Error};

    #[test]
    fn initializes() -> Result<(), Error> {
        let c = Cursor::new(Vec::new());
        let _tui = TUI::new(c, 16)?;
        Ok(())
    }
}

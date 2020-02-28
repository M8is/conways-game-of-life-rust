use std::io::{Error, Write};
use termion::{
    clear, color, cursor,
    raw::{IntoRawMode, RawTerminal},
    style,
};

pub struct TUI<W>
where
    W: Write,
{
    terminal: RawTerminal<W>,
    pub size: u16,
}

impl<W> TUI<W>
where
    W: Write,
{
    /// Creates a new TUI with a squared grid of the given size
    pub fn new(out: W, size: u16) -> Result<TUI<W>, Error> {
        let terminal = out.into_raw_mode()?;
        let mut tui = TUI { terminal, size };
        tui.clear()?;
        Ok(tui)
    }

    pub fn clear(&mut self) -> Result<(), Error> {
        write!(self.terminal, "{}", clear::All)?;
        for x in 0..self.size {
            for y in 0..self.size {
                self.set_point(x, y, false)?;
            }
        }
        self.flush()?;
        Ok(())
    }

    /// Ensure the UI is up-to-date
    pub fn flush(&mut self) -> Result<(), Error> {
        write!(
            self.terminal,
            "{}",
            cursor::Goto(self.size + 1, self.size + 1)
        )?;
        self.terminal.flush()
    }

    /// Shows the given point as active or inactive
    pub fn set_point(&mut self, x: u16, y: u16, active: bool) -> Result<(), Error> {
        if active {
            write!(
                self.terminal,
                "{}{}{}{}{}",
                cursor::Goto(x + 1, y + 1),
                color::Fg(color::White),
                style::Bold,
                "O",
                style::Reset,
            )
        } else {
            write!(
                self.terminal,
                "{}{}{}{}",
                cursor::Goto(x + 1, y + 1),
                color::Fg(color::Black),
                "X",
                style::Reset,
            )
        }
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

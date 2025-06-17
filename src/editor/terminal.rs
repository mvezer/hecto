use crossterm::cursor::{Hide, MoveTo, Show};
use crossterm::queue;
use crossterm::style::Print;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType};
use std::io::{stdout, Error, Write};

pub struct Terminal {}
pub struct Size {
    pub rows: u16,
    pub cols: u16,
}
pub struct Position {
    pub x: u16,
    pub y: u16,
}

impl Terminal {
    pub fn hide_cursor() -> Result<(), Error> {
        let mut stdout = stdout();
        queue!(stdout, Hide)?;
        Ok(())
    }
    pub fn show_cursor() -> Result<(), Error> {
        let mut stdout = stdout();
        queue!(stdout, Show)?;
        Ok(())
    }
    pub fn move_cursor_to(position: Position) -> Result<(), Error> {
        let mut stdout = stdout();
        queue!(stdout, MoveTo(position.x, position.y))?;
        Ok(())
    }
    pub fn size() -> Result<Size, Error> {
        let s = size()?;
        Ok(Size {
            rows: s.0,
            cols: s.1,
        })
    }
    pub fn print(s: &str) -> Result<(), Error> {
        let mut stdout = stdout();
        queue!(stdout, Print(s))?;
        Ok(())
    }
    pub fn clear_line() -> Result<(), Error> {
        let mut stdout = stdout();
        queue!(stdout, Clear(ClearType::CurrentLine))?;
        Ok(())
    }
    pub fn clear_screen() -> Result<(), Error> {
        let mut stdout = stdout();
        queue!(stdout, Clear(ClearType::All))?;
        Ok(())
    }

    pub fn flush() -> Result<(), Error> {
        stdout().flush()
    }

    pub fn initilaize() -> Result<(), Error> {
        enable_raw_mode()?;
        let _ = Self::clear_screen();
        Self::move_cursor_to(Position { x: 0, y: 0 })?;
        Ok(())
    }
    pub fn terminate() -> Result<(), Error> {
        disable_raw_mode()?;
        Ok(())
    }
}

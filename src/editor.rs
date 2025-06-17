use crossterm::event::{read, Event, Event::Key, KeyCode::Char, KeyEvent, KeyModifiers};
mod terminal;
use std::io::Error;
use terminal::{Position, Size, Terminal};

pub struct Editor {
    should_quit: bool,
}

impl Editor {
    pub const fn default() -> Self {
        Self { should_quit: false }
    }
    pub fn run(&mut self) {
        Terminal::initilaize().unwrap();
        let _ = Self::display_welcome_message();
        let result = self.repl();
        Terminal::terminate().unwrap();
        result.unwrap();
    }
    pub fn repl(&mut self) -> Result<(), Error> {
        loop {
            self.refresh_screen()?;
            if self.should_quit {
                break;
            }
            let event = read()?;
            self.evaluate_event(&event);
        }
        Ok(())
    }
    fn evaluate_event(&mut self, event: &Event) {
        if let Key(KeyEvent {
            code, modifiers, ..
        }) = event
        {
            match code {
                Char('q') if *modifiers == KeyModifiers::CONTROL => {
                    self.should_quit = true;
                }
                _ => (),
            }
        }
    }
    fn draw_rows() -> Result<(), Error> {
        let Size { rows, .. } = Terminal::size().unwrap();
        Terminal::hide_cursor()?;
        for r in 0..rows {
            Terminal::clear_line()?;
            Terminal::print("~")?;
            if r + 1 < rows {
                Terminal::print("\r\n")?;
            }
        }
        Terminal::flush()?;
        Terminal::show_cursor()?;
        let _ = Terminal::move_cursor_to(Position { x: 0, y: 0 });
        Ok(())
    }
    fn refresh_screen(&self) -> Result<(), Error> {
        if self.should_quit {
            Terminal::clear_screen()?;
            println!("Bye!");
        } else {
            Self::draw_rows()?;
        }
        Ok(())
    }
    fn display_welcome_message() -> Result<(), Error> {
        let message = "awesome hecto terminal editor v0.1.0";
        let s = Terminal::size()?;
        Terminal::move_cursor_to(Position {
            x: s.cols / 2 - message.len() as u16 / 2,
            y: s.rows / 3,
        })?;
        Terminal::print(message)?;
        Terminal::flush()?;
        Ok(())
    }
}

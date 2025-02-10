use crossterm::execute;
use std::fmt::Debug;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cursor {
    Move(u16, u16),
}

impl Debug for Cursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cursor::Move(x, y) => {
                write!(f, "Cursor::Move(x: {}, y: {})", x, y)
            }
        }
    }
}

impl Cursor {
    pub fn move_cursor(moveto: Self) -> anyhow::Result<()> {
        match moveto {
            Cursor::Move(x, y) => {
                execute!(std::io::stdout(), crossterm::cursor::MoveTo(x, y))?;
            }
        }

        Ok(())
    }
}

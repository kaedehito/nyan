use crossterm::execute;
use std::fmt::Debug;

/// `Cursor` represents cursor movement operations.
///
/// Currently, it supports moving the cursor to a specific `(x, y)` position.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cursor {
    /// Moves the cursor to the specified `(x, y)` coordinates.
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
    /// Moves the cursor to the specified position.
    ///
    /// # Arguments
    /// * `moveto` - A `Cursor` enum variant specifying the target position.
    ///
    /// # Returns
    /// * `Ok(())` on success.
    /// * `Err(anyhow::Error)` if an error occurs while executing the movement.
    pub fn move_cursor(moveto: Self) -> anyhow::Result<()> {
        match moveto {
            Cursor::Move(x, y) => {
                execute!(std::io::stdout(), crossterm::cursor::MoveTo(x, y))?;
            }
        }
        Ok(())
    }
}

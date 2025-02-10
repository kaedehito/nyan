//! This module defines the `Cursor` enum and provides functions for controlling cursor movement in the terminal.
//!
//! The `Cursor` enum currently supports moving the cursor to a specified `(x, y)` position. This can be used for precise control over where content is drawn in the terminal.
//!
//! # Enum
//!
//! - `Cursor`: Represents cursor movement operations. It includes a variant `Move(u16, u16)` for moving the cursor to specific coordinates `(x, y)`.
//!
//! # Methods
//!
//! - `move_cursor(moveto: Cursor)`: Moves the cursor to the specified position. The position is defined by the `Cursor::Move(x, y)` variant. This method returns a result indicating success or failure.

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

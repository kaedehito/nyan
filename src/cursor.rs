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

/// The `Cursor` enum represents cursor movement operations.
///
/// Currently, it supports various cursor movements, such as moving the cursor to a specific `(x, y)` position,
/// moving left, right, up, down, and moving to the next line.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Cursor {
    /// Moves the cursor to the specified `(x, y)` coordinates.
    Move(u16, u16),
    /// Moves the cursor to the specified number of units to the left.
    MoveLeft(u16),
    /// Moves the cursor to the specified number of units to the right.
    MoveRight(u16),
    /// Moves the cursor to the specified number of units upward.
    MoveUp(u16),
    /// Moves the cursor to the specified number of units downward.
    MoveDown(u16),
    /// Moves the cursor to the next line by the specified number of units.
    MoveToNextLine(u16),
}

impl Debug for Cursor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cursor::Move(x, y) => {
                write!(f, "Cursor::Move(x: {x}, y: {y})")
            }
            Cursor::MoveLeft(x) => {
                write!(f, "Cursor::MoveLeft({x})")
            }
            Cursor::MoveRight(x) => {
                write!(f, "Cursor::MoveRight({x})")
            }
            Cursor::MoveUp(y) => {
                write!(f, "Cursor::MoveUp({y})")
            }
            Cursor::MoveDown(y) => {
                write!(f, "Cursor::MoveDown({y})")
            }
            Cursor::MoveToNextLine(next) => {
                write!(f, "Cursor::MoveToNextLine({next})")
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
    ///
    /// # Example
    /// ```ignore
    /// Cursor::move_cursor(Cursor::Move(10, 5));
    /// ```
    ///
    /// This function executes the specified cursor movement operation.
    pub fn move_cursor(moveto: Self) -> anyhow::Result<()> {
        match moveto {
            Cursor::Move(x, y) => {
                execute!(std::io::stdout(), crossterm::cursor::MoveTo(x, y))?;
            }
            Cursor::MoveLeft(x) => {
                execute!(std::io::stdout(), crossterm::cursor::MoveLeft(x))?;
            }
            Cursor::MoveRight(x) => {
                execute!(std::io::stdout(), crossterm::cursor::MoveRight(x))?;
            }
            Cursor::MoveUp(y) => {
                execute!(std::io::stdout(), crossterm::cursor::MoveUp(y))?;
            }
            Cursor::MoveDown(y) => {
                execute!(std::io::stdout(), crossterm::cursor::MoveDown(y))?;
            }
            Cursor::MoveToNextLine(next) => {
                execute!(std::io::stdout(), crossterm::cursor::MoveToNextLine(next))?;
            }
        }
        Ok(())
    }
}

//! This module provides the `App` struct, which manages terminal control and rendering functionalities.
//!
//! The `App` struct supports features like enabling alternate screens, clearing the terminal, enabling raw mode, controlling the cursor visibility, and managing the frames per second (FPS) for terminal updates.
//!
//! # Structs
//!
//! - `App`: A struct that controls various terminal settings and allows drawing content to the terminal with the specified configurations.
//!
//! # Methods
//!
//! - `new(fps: u64)`: Creates a new `App` instance with a specified frames per second (FPS) value. The FPS cannot be 0, as it will default to 1.
//! - `alternate_screen()`: Enables the alternate screen (similar to full-screen mode) for the terminal.
//! - `clear()`: Enables the feature to clear the terminal screen on each frame.
//! - `raw_mode()`: Enables raw mode, which disables input buffering and line editing.
//! - `cursor()`: Controls the visibility of the terminal cursor.
//! - `fps(fps: u64)`: Sets the frames per second for terminal updates.
//! - `draw(func: F)`: Executes the drawing function (`func`), managing terminal settings like alternate screen, raw mode, cursor visibility, clearing the screen, and enforcing the FPS.
//! - `exit()`: Exits the terminal drawing mode, restoring the original screen and cursor visibility.

use anyhow::Result;
use crossterm::{cursor, execute, terminal};

use std::{fmt::Debug, io, thread, time::Duration};

/// `NyanTerminal` is a struct that handles terminal control and drawing.
/// It supports functionalities like enabling alternate screens, clearing the terminal,
/// enabling raw mode, and controlling the cursor visibility and FPS.
pub struct App {
    stdout: io::Stdout,
    alternatescreen: bool,
    clear: bool,
    rawmode: bool,
    cursor: bool,
    fps: u64,
    looped: bool,
}

impl Debug for App {
    /// Provides a custom debug implementation for `NyanTerminal`, showing its current settings.
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut cursor_state = "Show";
        if self.cursor {
            cursor_state = "Hide";
        }
        fmt.debug_struct("NyanTerminal")
            .field("alternate_screen", &self.alternatescreen)
            .field("clear", &self.clear)
            .field("raw_mode", &self.rawmode)
            .field("cursor", &cursor_state)
            .finish()
    }
}

impl io::Write for App {
    /// Writes bytes to the terminal output.
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.stdout.write(buf)
    }

    /// Flushes the output buffer to ensure all data is written.
    fn flush(&mut self) -> io::Result<()> {
        self.stdout.flush()
    }
}

impl App {
    /// Creates a new `NyanTerminal` instance with the specified frames per second (FPS).
    /// The FPS must be at least 1, as 0 would cause an error.
    ///
    /// # Arguments
    /// - `fps`: The frames per second for the terminal refresh rate.
    ///
    /// # Returns
    /// A new `NyanTerminal` instance.
    pub fn new(fps: u64) -> Self {
        Self {
            stdout: io::stdout(),
            alternatescreen: false,
            clear: false,
            rawmode: false,
            cursor: false,
            fps: fps.max(1), // Prevents FPS from being 0
            looped: false,
        }
    }

    /// Enables the alternate screen (like entering a full-screen mode).
    ///
    /// # Returns
    /// A new `NyanTerminal` instance with the alternate screen enabled.
    pub fn alternate_screen(self) -> Self {
        let mut nyan = self;
        nyan.alternatescreen = true;
        nyan
    }

    /// Enables the terminal clearing feature.
    ///
    /// # Returns
    /// A new `NyanTerminal` instance with the clear flag set.
    pub fn clear(self) -> Self {
        let mut nyan = self;
        nyan.clear = true;
        nyan
    }

    /// Enables raw mode (disables input buffering and line editing).
    ///
    /// # Returns
    /// A new `NyanTerminal` instance with raw mode enabled.
    pub fn raw_mode(self) -> Self {
        let mut nyan = self;
        nyan.rawmode = true;
        nyan
    }

    /// Hides the cursor.
    ///
    /// This method hides the cursor, regardless of the provided flag.
    ///
    /// # Returns
    /// A new `NyanTerminal` instance with the cursor hidden.
    pub fn hide_cursor(self) -> Self {
        let mut nyan = self;
        nyan.cursor = true;
        nyan
    }

    /// Chenge fps.
    ///
    /// # Returns
    /// A new `NyanTerminal` instance with the fps set.
    pub fn fps(self, fps: u64) -> Self {
        let mut nyan = self;
        nyan.fps = fps;
        nyan
    }

    /// Retrieves the current size of the terminal window.
    ///
    /// This function uses `crossterm` to get the terminal's width and height
    /// in character cells.
    ///
    /// # Returns
    /// - `Ok((u16, u16))`: A tuple containing the terminal's width and height.
    /// - `Err(anyhow::Error)`: If retrieving the terminal size fails.
    ///
    /// # Example
    /// ```
    /// use nyan::app::App;
    /// use std::error::Error;
    ///
    /// fn main() -> Result<(), Box<dyn Error>>{
    ///     let (width, height) = App::get_terminal_size()?;
    ///     println!("Terminal size: {}x{}", width, height);
    ///     Ok(())
    /// }
    /// ```
    ///
    /// # Errors
    /// This function will return an error if the terminal size cannot be determined.
    pub fn get_terminal_size() -> anyhow::Result<(u16, u16)> {
        let (x, y) = crossterm::terminal::size()?;
        Ok((x, y))
    }

    /// Executes a function to draw the terminal content, handling setup and cleanup for terminal settings.
    /// It can manage alternate screens, raw mode, cursor visibility, clearing the terminal, and FPS control.
    ///
    /// # Arguments
    /// - `func`: A closure that handles the terminal drawing logic.
    ///
    /// # Returns
    /// A `Result` indicating success or failure of the operation.
    pub fn draw<F: FnOnce()>(&mut self, func: F) -> Result<()> {
        execute!(&self.stdout, cursor::MoveTo(0, 0))?;

        if self.alternatescreen && !self.looped {
            execute!(&self.stdout, terminal::EnterAlternateScreen)?;
        }

        if self.rawmode && !self.looped {
            terminal::enable_raw_mode()?;
        }

        if !self.cursor {
            execute!(&self.stdout, cursor::Show)?;
        } else {
            execute!(&self.stdout, cursor::Hide)?;
        }

        if self.clear {
            execute!(&self.stdout, terminal::Clear(terminal::ClearType::All))?
        }

        self.looped = true;

        func();

        // Convert FPS to milliseconds and sleep to maintain the FPS rate
        let frame_duration = Duration::from_millis(1000 / self.fps);
        thread::sleep(frame_duration);

        Ok(())
    }

    /// Exits the terminal drawing mode, restoring the original screen and cursor visibility.
    ///
    /// # Returns
    /// A `Result` indicating success or failure of the operation.
    pub fn exit(self) -> Result<()> {
        execute!(
            &self.stdout,
            cursor::MoveTo(0, 0),
            cursor::Show,
            terminal::LeaveAlternateScreen
        )?;

        if self.rawmode {
            terminal::disable_raw_mode()?;
        }

        Ok(())
    }
}

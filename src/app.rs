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
}

impl Debug for App {
    /// Provides a custom debug implementation for `NyanTerminal`, showing its current settings.
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fmt.debug_struct("NyanTerminal")
            .field("alternate_screen", &self.alternatescreen)
            .field("clear", &self.clear)
            .field("raw_mode", &self.rawmode)
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

    /// Hides or shows the cursor based on the flag.
    ///
    /// # Returns
    /// A new `NyanTerminal` instance with the cursor visibility set.
    pub fn cursor(self) -> Self {
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

    /// Executes a function to draw the terminal content, handling setup and cleanup for terminal settings.
    /// It can manage alternate screens, raw mode, cursor visibility, clearing the terminal, and FPS control.
    ///
    /// # Arguments
    /// - `func`: A closure that handles the terminal drawing logic.
    ///
    /// # Returns
    /// A `Result` indicating success or failure of the operation.
    pub fn draw<F: FnOnce()>(&self, func: F) -> Result<()> {
        execute!(&self.stdout, cursor::MoveTo(0, 0))?;

        if self.rawmode {
            terminal::enable_raw_mode()?;
        }

        if self.alternatescreen {
            execute!(&self.stdout, terminal::EnterAlternateScreen)?;
        }

        if !self.cursor {
            execute!(&self.stdout, cursor::Show)?;
        } else {
            execute!(&self.stdout, cursor::Hide)?;
        }

        if self.clear {
            execute!(&self.stdout, terminal::Clear(terminal::ClearType::All))?
        }

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
        execute!(&self.stdout, cursor::Show, terminal::LeaveAlternateScreen)?;

        if self.rawmode {
            terminal::disable_raw_mode()?;
        }

        Ok(())
    }
}

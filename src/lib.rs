//! # nyan
//!
//! A Rust library for creating dynamic Terminal User Interfaces (TUI). Nyan provides a simple and intuitive way to manage and draw objects in the terminal, built on top of Ratatui and Crossterm.
//!
//!
//!
//! ## Features
//!
//! - Simple object management system for terminal UI elements
//! - Built-in support for common terminal operations
//! - Event handling and input management
//! - Flexible rendering system
//! - Cross-platform compatibility
//! - Easy integration with existing Rust TUI applications
//!
//! ## Dependencies
//!
//! - Rust 1.70 or higher
//! - [ratatui](https://github.com/ratatui-org/ratatui) - TUI library
//! - [crossterm](https://github.com/crossterm-rs/crossterm) - Terminal manipulation
//!
//!
//! ## Basic Usage
//!
//! Here's a simple example of how to use nyan:
//!
//! ```rust
//! use nyan::{app::App, nyan_obj::NyanObj, objects::Objects, input::{NyanInput, NyanKey}};
//! use std::error::Error;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     // Initialize the application (with 30 FPS)
//!     let mut nyan = App::new(30).clear().raw_mode().alternate_screen().hide_cursor();
//!    
//!    let mut obj = NyanObj::new();
//!    
//!    // Create and add objects to the application
//!    obj.add_object("text", Objects::new_text("Hello world!"), (0, 0));
//!    
//!    // Run the main event loop
//!    loop {
//!        // Draw the object
//!        nyan.draw(|| {
//!            obj.draw_object("text").unwrap(); // will display "Hello world!"
//!        })?;
//!
//!        let key = NyanInput::get_input();
//!
//!        match key?{
//!            // Ctrl + C to exit
//!            NyanInput::Ctrl(NyanKey::C) => {
//!                break;
//!            }
//!
//!            _ => {}
//!        }
//!
//!     }
//!
//!     nyan.exit()?;
//!    
//!     Ok(())
//! }
//!
//! ```

pub mod app;
pub mod cursor;
pub mod errors;
pub mod input;
pub mod nyan_obj;
pub mod objects;

#[cfg(test)]
mod tests {
    use crate::{
        app::App,
        cursor::Cursor,
        input::{NyanInput, NyanKey},
        nyan_obj::NyanObj,
        objects::Objects,
    };

    #[test]
    fn test() {
        let mut nyan = App::new(60)
            .clear()
            .raw_mode()
            .alternate_screen()
            .hide_cursor();

        let mut obj = NyanObj::new();

        obj.add_object("hello world", Objects::new_text("Hello world!"), (0, 1));

        loop {
            let (_, height) = App::get_terminal_size().unwrap();

            nyan.draw(|| {
                obj.draw_object("hello world").unwrap();
                obj.draw_object("hi").unwrap_or_else(|e| {
                    Cursor::move_cursor(Cursor::MoveToNextLine(1)).unwrap();
                    println!("{:?}", e);
                });
            })
            .unwrap();

            let p = NyanInput::get_input();
            match p.unwrap() {
                NyanInput::Ctrl(NyanKey::C) => {
                    break;
                }

                NyanInput::Shift(NyanInput::Key(NyanKey::Q)) => {
                    break;
                }

                NyanInput::Key(NyanKey::OtherKey('/')) => {}

                NyanInput::Key(NyanKey::A) => {}

                NyanInput::UpAllow => {}

                _ => {}
            }
        }

        nyan.exit().unwrap();
    }
}

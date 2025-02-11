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
//! use nyan::{app::App, nyanobj::NyanObj, objects::Objects, input::{NyanInput, NyanInputKey}};
//! use std::error::Error;
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     // Initialize the application (with 30 FPS)
//!     let mut nyan = App::new(30).clear().raw_mode().alternate_screen().hide_cursor();
//!    
//!    let mut obj = NyanObj::new();
//!    
//!    // Create and add objects to the application
//!    obj.add_object("text", Objects::new_text("Hello world!"));
//!    
//!    // Run the main event loop
//!    loop {
//!        // Draw the object
//!        nyan.draw(|| {
//!            obj.draw_object("text"); // will display "Hello world!"
//!        })?;
//!
//!        let key = NyanInput::get_input();
//!
//!        match key?{
//!            // Ctrl + C to exit
//!            NyanInput::Ctrl(NyanInputKey::C) => {
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
pub mod nyanobj;
pub mod objects;

#[cfg(test)]
mod tests {
    use crate::{
        app::App,
        cursor::Cursor,
        input::{NyanInput, NyanInputKey},
        nyanobj::NyanObj,
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

        obj.add_object("hello_world", Objects::new_text("Hello world!"));
        let dbged = format!("{:?}", &nyan);

        obj.add_object("Test_NyanTerm_dbg", Objects::new_text(&dbged));
        obj.add_object("input_key", Objects::Air);
        obj.add_object("surash", Objects::Air);
        obj.add_object("frame", Objects::Air);
        obj.add_object("Character", Objects::new_text("□"));

        let mut frame = 0u64;

        loop {
            frame += 1;
            obj.update_object("frame", Objects::new_text(frame.to_string()));

            let (_, height) = App::get_terminal_size().unwrap();

            nyan.draw(|| {
                obj.draw_object("hello_world").unwrap();

                obj.draw_with_move("Test_NyanTerm_dbg", Cursor::MoveToNextLine(1))
                    .unwrap();

                obj.draw_with_move("surash", Cursor::MoveToNextLine(1))
                    .unwrap();

                obj.draw_with_move("Character", Cursor::MoveToNextLine(1))
                    .unwrap();

                obj.draw_with_move("frame", Cursor::new(0, height)).unwrap();
            })
            .unwrap();

            let p = NyanInput::get_input();
            match p.unwrap() {
                NyanInput::Ctrl(NyanInputKey::C) => {
                    break;
                }

                NyanInput::Shift(NyanInput::Key(NyanInputKey::Q)) => {
                    break;
                }

                NyanInput::Key(NyanInputKey::OtherKey('/')) => {
                    obj.update_object("surash", Objects::new_text("inputed"));
                }

                NyanInput::Key(NyanInputKey::A) => {
                    obj.update_object("hello_world", Objects::new_text("You Plessed \"A\"!"));
                }

                NyanInput::UpAllow => {
                    obj.update_object("Character", Objects::new_text("□ < なに？"));
                }

                _ => {}
            }
        }

        nyan.exit().unwrap();
    }
}

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
//!     // Initialize the application (with 10 FPS)
//!     let mut nyan = App::new(10).clear().raw_mode().alternate_screen();
//!    
//!    let mut obj = NyanObj::new();
//!    
//!    // Create and add objects to the application
//!    obj.add_object("text", Objects::Text("Hello world!"));
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
pub mod input;
pub mod nyanobj;
pub mod objects;

#[cfg(test)]
mod tests {
    use crate::{
        app::App,
        cursor::{self, Cursor},
        input::{NyanInput, NyanInputKey},
        nyanobj::NyanObj,
        objects::Objects,
    };

    #[test]
    fn test() {
        let mut nyan = App::new(60).clear().raw_mode().alternate_screen();

        let mut obj = NyanObj::new();

        obj.add_object("hello_world", Objects::Text("Hello world!"));
        let dbged = format!("{:?}", &nyan);
        obj.add_object("Test_NyanTerm_dbg", Objects::Text(&dbged));

        obj.add_object("surash", Objects::Air);

        loop {
            nyan.draw(|| {
                obj.draw_object("hello_world").unwrap();
                cursor::Cursor::move_cursor(Cursor::Move(0, 1)).unwrap();
                obj.draw_object("Test_NyanTerm_dbg").unwrap();
                cursor::Cursor::move_cursor(Cursor::Move(0, 2)).unwrap();
                obj.draw_object("surash").unwrap();
            })
            .unwrap();

            let p = NyanInput::get_input();
            match p.unwrap() {
                NyanInput::Ctrl(NyanInputKey::C) => {
                    break;
                }

                NyanInput::Key(NyanInputKey::OtherKey('/')) => {
                    obj.update_object("surash", Objects::Text("inputed"));
                }

                NyanInput::Key(NyanInputKey::A) => {
                    obj.update_object("hello_world", Objects::Text("You Plessed \"A\"!"));
                }
                _ => {}
            }
        }

        nyan.exit().unwrap();
    }
}

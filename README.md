# nyan

A Rust library for creating dynamic Terminal User Interfaces (TUI). Nyan provides a simple and intuitive way to manage and draw objects in the terminal, built on top of Ratatui and Crossterm.

## Features

- Simple object management system for terminal UI elements
- Built-in support for common terminal operations
- Event handling and input management
- Flexible rendering system
- Cross-platform compatibility
- Easy integration with existing Rust TUI applications

## Dependencies

- Rust 1.70 or higher
- [ratatui](https://github.com/ratatui-org/ratatui) - TUI library
- [crossterm](https://github.com/crossterm-rs/crossterm) - Terminal manipulation

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
nyan = { git = "https://github.com/kaedehito/nyan" }
```

## Basic Usage

Here's a simple example of how to use nyan:

```rust
use nyan::{app::App, nyan_obj::NyanObj, objects::Objects, input::{NyanInput, NyanKey}};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the application
    // In this case, it sets 10 FPS, clears the screen, enables raw mode, and switches to alternate screen mode
    let mut nyan = App::new(10)
        .clear()       // Clears the screen
        .raw_mode()    // Enables raw mode (receives key inputs as is)
        .alternate_screen(); // Enables alternate screen mode (uses a screen different from the default terminal screen)

    // Create a new NyanObj object
    let mut obj = NyanObj::new();
   
    // Add an object with the key "text" and assign it a text object
    // Here, we are creating an object to display the text "Hello world!"
    obj.add_object("text", Objects::new_text("Hello world!"), (0, 0));

    // Start the main event loop
    loop {
        // Draw the object on the screen
        nyan.draw(|| {
            // Draw the object with the key "text"
            obj.draw_object("text").unwrap(); // Displays "Hello world!" on the screen
        })?;

        // Get user input
        let key = NyanInput::get_input();

        // Handle the key input
        match key? {
            // If Ctrl + C is pressed, exit the loop and terminate the application
            NyanInput::Ctrl(NyanKey::C) => {
                break; // Exit the loop and terminate the app
            }

            // Do nothing for other key inputs
            _ => {}
        }
    }

    // Exit the application
    nyan.exit()?;
    
    // Return Ok to indicate normal termination
    Ok(())
}

```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

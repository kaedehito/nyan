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
use nyan::{init::App, nyanterm::NyanTermObjs, objects::Objects};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the application (with 10 FPS)
    let nyan = App::new(10).clear().alternate_screen();
    
    let mut obj = NyanTermObjs::new();
    
    // Create and add objects to the application
    obj.add_object("text", Objects::Text("Hello world!"));
    
    // Run the main event loop
    loop {
        // Draw the object
        nyan.draw(|| {
            obj.draw_object("text"); // will display "Hello world!"
        })?;
    }
    
    Ok(())
}

```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

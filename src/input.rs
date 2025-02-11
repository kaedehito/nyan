//! This module defines types and functions for handling keyboard inputs in the Nyan application.
//!
//! The `NyanKey` enum represents individual keyboard keys, including alphabetic keys and unrecognized keys. It also includes a variant for undefined keys (`NoKeys(char)`).
//!
//! The `NyanInput` enum represents various types of keyboard inputs, including key presses with modifier keys such as Shift, Ctrl, and Alt. It also defines special keys like Enter, Backspace, Arrow keys, and Function keys. Additionally, it can handle key presses for regular and invalid keys.
//!
//! This module includes the `get_input` function, which asynchronously retrieves keyboard input by polling for events and returning a corresponding `NyanInput` enum value. It supports detecting key presses with different modifiers (Shift, Ctrl, Alt), as well as special and function keys.
//!
//! # Enums
//!
//! - `NyanKey`: Represents individual keyboard keys, including alphabetic keys (A-Z) and undefined keys.
//! - `NyanInput`: Represents various types of keyboard inputs, including keys with modifiers, special keys, function keys, and regular key presses.
//!
//! # Methods
//!
//! - `get_input`: Asynchronously retrieves the keyboard input. It waits for 16 milliseconds using `poll` and returns a `NyanInput` value representing the key pressed.

use std::{fmt::Debug, time::Duration};

use crossterm::event::{self, KeyCode, KeyModifiers};

/// `NyanKey` represents individual keyboard keys.
///
/// It includes alphabet keys (`A-Z`) and unrecognized keys (`NoKeys(char)`).
#[allow(unused)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NyanKey {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    OtherKey(char),
}

impl Debug for NyanKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => write!(f, "NyanKey::A"),
            Self::B => write!(f, "NyanKey::B"),
            Self::C => write!(f, "NyanKey::C"),
            Self::D => write!(f, "NyanKey::D"),
            Self::E => write!(f, "NyanKey::E"),
            Self::F => write!(f, "NyanKey::F"),
            Self::G => write!(f, "NyanKey::G"),
            Self::H => write!(f, "NyanKey::H"),
            Self::I => write!(f, "NyanKey::I"),
            Self::J => write!(f, "NyanKey::J"),
            Self::K => write!(f, "NyanKey::K"),
            Self::L => write!(f, "NyanKey::L"),
            Self::M => write!(f, "NyanKey::M"),
            Self::N => write!(f, "NyanKey::N"),
            Self::O => write!(f, "NyanKey::O"),
            Self::P => write!(f, "NyanKey::P"),
            Self::Q => write!(f, "NyanKey::Q"),
            Self::R => write!(f, "NyanKey::R"),
            Self::S => write!(f, "NyanKey::S"),
            Self::T => write!(f, "NyanKey::T"),
            Self::U => write!(f, "NyanKey::U"),
            Self::V => write!(f, "NyanKey::V"),
            Self::W => write!(f, "NyanKey::W"),
            Self::X => write!(f, "NyanKey::X"),
            Self::Y => write!(f, "NyanKey::Y"),
            Self::Z => write!(f, "NyanKey::Z"),
            Self::OtherKey(c) => write!(f, "NyanKey::({})", c),
        }
    }
}

/// `NyanInput` represents keyboard inputs.
///
/// It supports special keys and modifier keys (`Shift`, `Ctrl`, `Alt`).
#[allow(unused)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NyanInput<'a> {
    /// Input with Shift modifier
    Shift(&'a NyanInput<'a>),
    /// Input with Ctrl modifier
    Ctrl(NyanKey),
    /// Input with Alt modifier
    Alt(NyanKey),
    /// Arrow keys
    UpAllow,
    DownAllow,
    LeftAllow,
    RightAllow,
    /// Other special keys
    Enter,
    BackSpace,
    Tab,
    Esc,
    End,
    CapsLock,
    Insert,
    Home,
    PageUp,
    PageDown,
    Delete,
    /// Function keys
    FunctionKey(u8),
    /// Regular key
    Key(NyanKey),
    /// Invalid key input
    Null,
}

impl<'a> Debug for NyanInput<'a> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Alt(o) => write!(fmt, "NyanInput::Alt({:?})", o),
            Self::Ctrl(o) => write!(fmt, "NyanInput::Ctrl({:?})", o),
            Self::Shift(o) => write!(fmt, "NyanInput::Shift({:?})", o),
            Self::UpAllow => write!(fmt, "NyanInput::UpAllow"),
            Self::DownAllow => write!(fmt, "NyanInput::DownAllow"),
            Self::RightAllow => write!(fmt, "NyanInput::RightAllow"),
            Self::LeftAllow => write!(fmt, "NyanInput::LeftAllow"),
            Self::Enter => write!(fmt, "NyanInput::Enter"),
            Self::BackSpace => write!(fmt, "NyanInput::BackSpace"),
            Self::Tab => write!(fmt, "NyanInput::Tab"),
            Self::Esc => write!(fmt, "NyanInput::Esc"),
            Self::End => write!(fmt, "NyanInput::End"),
            Self::CapsLock => write!(fmt, "NyanInput::CapsLock"),
            Self::Insert => write!(fmt, "NyanInput::Insert"),
            Self::Home => write!(fmt, "NyanInput::Home"),
            Self::PageUp => write!(fmt, "NyanInput::PageUp"),
            Self::PageDown => write!(fmt, "NyanInput::PageDown"),
            Self::Delete => write!(fmt, "NyanInput::Delete"),
            Self::FunctionKey(f) => write!(fmt, "NyanInput::FunctionKey(F{})", f),
            Self::Key(k) => write!(fmt, "NyanInput::Key({:?})", k),
            Self::Null => write!(fmt, "NyanInput::Null"),
        }
    }
}

impl<'a> NyanInput<'a> {
    /// `get_input` asynchronously retrieves keyboard input.
    ///
    /// Waits for 16 milliseconds using `poll` and returns `NyanInput` if a key is pressed.
    ///
    /// # Returns
    /// * `Ok(NyanInput)` - on success
    /// * `Err(anyhow::Error)` - if reading input fails
    #[allow(unused)]
    pub fn get_input() -> anyhow::Result<Self> {
        if event::poll(Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                let nyan_input = match key.code {
                    KeyCode::Char(ch) => {
                        let nyan_key = match ch.to_ascii_lowercase() {
                            'a' => NyanKey::A,
                            'b' => NyanKey::B,
                            'c' => NyanKey::C,
                            'd' => NyanKey::D,
                            'e' => NyanKey::E,
                            'f' => NyanKey::F,
                            'g' => NyanKey::G,
                            'h' => NyanKey::H,
                            'i' => NyanKey::I,
                            'j' => NyanKey::J,
                            'k' => NyanKey::K,
                            'l' => NyanKey::L,
                            'm' => NyanKey::M,
                            'n' => NyanKey::N,
                            'o' => NyanKey::O,
                            'p' => NyanKey::P,
                            'q' => NyanKey::Q,
                            'r' => NyanKey::R,
                            's' => NyanKey::S,
                            't' => NyanKey::T,
                            'u' => NyanKey::U,
                            'v' => NyanKey::V,
                            'w' => NyanKey::W,
                            'x' => NyanKey::X,
                            'y' => NyanKey::Y,
                            'z' => NyanKey::Z,
                            p => NyanKey::OtherKey(p),
                        };
                        if key.modifiers.contains(KeyModifiers::CONTROL) {
                            Self::Ctrl(nyan_key)
                        } else if key.modifiers.contains(KeyModifiers::ALT) {
                            Self::Alt(nyan_key)
                        } else if key.modifiers.contains(KeyModifiers::SHIFT) {
                            Self::Shift(Box::leak(Box::new(NyanInput::Key(nyan_key))))
                        } else {
                            Self::Key(nyan_key)
                        }
                    }
                    KeyCode::Left => Self::LeftAllow,
                    KeyCode::Right => Self::RightAllow,
                    KeyCode::Up => Self::UpAllow,
                    KeyCode::Down => Self::DownAllow,
                    KeyCode::Enter => Self::Enter,
                    KeyCode::Backspace => Self::BackSpace,
                    KeyCode::Tab => Self::Tab,
                    KeyCode::Esc => Self::Esc,
                    KeyCode::End => Self::End,
                    KeyCode::Insert => Self::Insert,
                    KeyCode::CapsLock => Self::CapsLock,
                    KeyCode::Home => Self::Home,
                    KeyCode::PageUp => Self::PageUp,
                    KeyCode::PageDown => Self::PageDown,
                    KeyCode::Delete => Self::Delete,
                    KeyCode::F(f) => Self::FunctionKey(f),
                    KeyCode::Null => Self::Null,
                    _ => return Ok(Self::Null),
                };
                return Ok(nyan_input);
            }
        }
        Ok(Self::Null)
    }
}

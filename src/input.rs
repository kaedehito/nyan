use std::{fmt::Debug, time::Duration};

use crossterm::event::{self, KeyCode, KeyModifiers};

/// `NyanInputKey` represents individual keyboard keys.
///
/// It includes alphabet keys (`A-Z`) and unrecognized keys (`NoKeys(char)`).
#[allow(unused)]
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum NyanInputKey {
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
    NoKeys(char),
}

impl Debug for NyanInputKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::A => write!(f, "NyanInputKey::A"),
            Self::B => write!(f, "NyanInputKey::B"),
            Self::C => write!(f, "NyanInputKey::C"),
            Self::D => write!(f, "NyanInputKey::D"),
            Self::E => write!(f, "NyanInputKey::E"),
            Self::F => write!(f, "NyanInputKey::F"),
            Self::G => write!(f, "NyanInputKey::G"),
            Self::H => write!(f, "NyanInputKey::H"),
            Self::I => write!(f, "NyanInputKey::I"),
            Self::J => write!(f, "NyanInputKey::J"),
            Self::K => write!(f, "NyanInputKey::K"),
            Self::L => write!(f, "NyanInputKey::L"),
            Self::M => write!(f, "NyanInputKey::M"),
            Self::N => write!(f, "NyanInputKey::N"),
            Self::O => write!(f, "NyanInputKey::O"),
            Self::P => write!(f, "NyanInputKey::P"),
            Self::Q => write!(f, "NyanInputKey::Q"),
            Self::R => write!(f, "NyanInputKey::R"),
            Self::S => write!(f, "NyanInputKey::S"),
            Self::T => write!(f, "NyanInputKey::T"),
            Self::U => write!(f, "NyanInputKey::U"),
            Self::V => write!(f, "NyanInputKey::V"),
            Self::W => write!(f, "NyanInputKey::W"),
            Self::X => write!(f, "NyanInputKey::X"),
            Self::Y => write!(f, "NyanInputKey::Y"),
            Self::Z => write!(f, "NyanInputKey::Z"),
            Self::NoKeys(c) => write!(f, "NyanInputKey::({})", c),
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
    Ctrl(NyanInputKey),
    /// Input with Alt modifier
    Alt(NyanInputKey),
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
    Key(NyanInputKey),
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
                            'a' => NyanInputKey::A,
                            'b' => NyanInputKey::B,
                            'c' => NyanInputKey::C,
                            'd' => NyanInputKey::D,
                            'e' => NyanInputKey::E,
                            'f' => NyanInputKey::F,
                            'g' => NyanInputKey::G,
                            'h' => NyanInputKey::H,
                            'i' => NyanInputKey::I,
                            'j' => NyanInputKey::J,
                            'k' => NyanInputKey::K,
                            'l' => NyanInputKey::L,
                            'm' => NyanInputKey::M,
                            'n' => NyanInputKey::N,
                            'o' => NyanInputKey::O,
                            'p' => NyanInputKey::P,
                            'q' => NyanInputKey::Q,
                            'r' => NyanInputKey::R,
                            's' => NyanInputKey::S,
                            't' => NyanInputKey::T,
                            'u' => NyanInputKey::U,
                            'v' => NyanInputKey::V,
                            'w' => NyanInputKey::W,
                            'x' => NyanInputKey::X,
                            'y' => NyanInputKey::Y,
                            'z' => NyanInputKey::Z,
                            _ => return Ok(Self::UpAllow),
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

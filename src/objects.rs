//! This module defines the `Objects` enum, which represents different types of objects that can be used in the terminal display.
//!
//! The `Objects` enum can represent a `Block`, an `Air`, or a `Text` object containing a string slice (`&'a str`). These objects can be displayed or manipulated within the terminal context.
//!
//! # Enum
//!
//! - `Objects`: Represents various types of objects. It includes the following variants:
//!     - `Block`: A block object (potentially used for drawing a visual element).
//!     - `Air`: An air object, representing an empty or invisible entity.
//!     - `Text`: A text object, containing a string slice (`&'a str`), used for displaying text in the terminal.
//!
//! # Methods
//!
//! - `Debug`: Provides a custom debug implementation for the `Objects` enum. It formats the enum variants in a human-readable way, displaying the respective type and data (if applicable).

use std::borrow::Cow;
use std::fmt::Debug;

#[derive(PartialEq, Eq, Hash)]
/// The `Objects` enum represents different types of objects.
/// It can be a `Block`, `Air`, or a `Text` object containing a `AsRef<str>`.
pub enum Objects<'a> {
    /// Represents a block object.
    Block,

    /// Represents an air object (an empty or non-visible object).
    Air,

    /// Represents a text object that contains a string.
    Text(Cow<'a, str>),
}

impl<'a> Debug for Objects<'a> {
    /// Provides a custom debug implementation for `Objects`, printing a string representation of each variant.
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            // Formats the Air variant
            Objects::Air => {
                write!(fmt, "Objects::Air")
            }

            // Formats the Block variant
            Objects::Block => {
                write!(fmt, "Objects::Block")
            }

            // Formats the Text variant, displaying the contained text
            Objects::Text(t) => {
                write!(fmt, "Objects::Text({})", t.as_ref())
            }
        }
    }
}

impl<'a> Objects<'a> {
    pub fn new_text<T: Into<Cow<'a, str>>>(text: T) -> Self {
        Self::Text(text.into())
    }
}

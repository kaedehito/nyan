use std::fmt::Debug;

#[derive(PartialEq, Eq, Hash)]
/// The `Objects` enum represents different types of objects.
/// It can be a `Block`, `Air`, or a `Text` object containing a string slice (`&'a str`).
pub enum Objects<'a> {
    /// Represents a block object.
    Block,

    /// Represents an air object (an empty or non-visible object).
    Air,

    /// Represents a text object that contains a string.
    Text(&'a str),
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
                write!(fmt, "Objects::Text({})", t)
            }
        }
    }
}

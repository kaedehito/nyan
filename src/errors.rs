use std::borrow::Cow;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum NyanError<'a> {
    #[error("Failed to draw {0}")]
    DrawFailed(Cow<'a, str>),

    #[error("Failed to move cursor: {0}")]
    Cursor(Cow<'a, str>),

    #[error("Object with ID \"{0}\" is not found")]
    ObjectNotFound(Cow<'a, str>),
}

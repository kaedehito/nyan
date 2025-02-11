/*!
A module for managing a collection of drawable objects identified by unique IDs.

# Overview

This module defines a collection type, [`NyanObj`], which stores objects that can be drawn on the screen.
Each object is represented by the [`Objects`] enum, which supports various types such as:
- **Text:** A textual object that prints a string.
- **Air:** An empty (non-visible) object.
- **Block:** A block object (drawing functionality is not yet implemented).

Objects are stored along with a unique identifier (as a `Cow<str>`) and display coordinates. The module provides methods to add, remove, update, and draw these objects.

# Examples

```rust
use nyan::nyan_obj::NyanObj;
use nyan::objects::Objects;
use nyan::cursor::Cursor;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>>{
    let mut collection = NyanObj::new();

    // Add a text object with an explicit coordinate.
    collection.add_object("text1", Objects::new_text("Hello, world!"), (10, 5));

    // Draw the object using its stored coordinate.
    collection.draw_object("text1")?;

    // Alternatively, draw the object after moving the cursor to a new position.
    let new_cursor = Cursor::new(20, 10);
    collection.draw_with_move("text1", new_cursor)?;

    Ok(())
}
```
*/

use crate::cursor::{self, Cursor};
use crate::errors::{self, NyanError};
use crate::objects::Objects;
use std::borrow::Cow;

/// Internal structure representing a single object entry in the collection.
///
/// Each `NyanObjs` holds:
/// - An object of type [`Objects`].
/// - A unique identifier stored as a `Cow<str>`.
/// - The display coordinate as a tuple `(x, y)`.
struct NyanObjs<'a> {
    object: Objects<'a>,
    id: Cow<'a, str>,
    coordinate: (u16, u16),
}

impl<'a> NyanObjs<'a> {
    /// Creates a new `NyanObjs` instance.
    ///
    /// # Parameters
    ///
    /// - `object`: The object to store (of type [`Objects`]).
    /// - `id`: A unique identifier for the object.
    /// - `coordinate`: A tuple `(x, y)` indicating where the object should be drawn.
    ///
    /// # Returns
    ///
    /// A new instance of `NyanObjs`.
    pub fn new(object: Objects<'a>, id: Cow<'a, str>, coordinate: (u16, u16)) -> Self {
        Self {
            object,
            id,
            coordinate,
        }
    }
}

/// A collection of drawable objects identified by unique string IDs.
///
/// The [`NyanObj`] struct manages an internal list of objects (stored as [`NyanObjs`]).
/// Each stored object includes its type (defined by the [`Objects`] enum), a unique ID, and
/// a coordinate for drawing.
///
/// # Type Parameters
///
/// - `'a`: Lifetime for the stored object data.
pub struct NyanObj<'a> {
    /// Internal storage for objects.
    ///
    /// Each element holds the object, its unique identifier, and its drawing coordinate.
    inner: Vec<NyanObjs<'a>>,
}

impl<'a> NyanObj<'a> {
    /// Creates an empty `NyanObj` collection.
    ///
    /// # Returns
    ///
    /// A new instance of [`NyanObj`] with no stored objects.
    pub fn new() -> Self {
        Self { inner: Vec::new() }
    }

    /// Adds a new object to the collection with a specified coordinate.
    ///
    /// # Parameters
    ///
    /// - `id`: The unique identifier for the object (any type convertible into a `Cow<str>`).
    /// - `object`: The object to add, represented by the [`Objects`] enum.
    /// - `coordinate`: A tuple `(x, y)` specifying the object's drawing position.
    pub fn add_object<P: Into<Cow<'a, str>>>(
        &mut self,
        id: P,
        object: Objects<'a>,
        coordinate: (u16, u16),
    ) {
        self.inner
            .push(NyanObjs::new(object, id.into(), coordinate));
    }

    /// Adds a new object to the collection with a default coordinate of `(0, 0)`.
    ///
    /// This method is useful when the drawing position is not yet determined.
    ///
    /// # Parameters
    ///
    /// - `id`: The unique identifier for the object.
    /// - `object`: The object to add.
    pub fn add_object_with_default<P: Into<Cow<'a, str>> + Clone>(
        &mut self,
        id: P,
        object: Objects<'a>,
    ) {
        self.inner.push(NyanObjs::new(object, id.into(), (0, 0)));
    }

    /// Removes an object from the collection by its unique identifier.
    ///
    /// # Parameters
    ///
    /// - `id`: The identifier of the object to remove.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the object was found and removed.
    /// - An error of type [`NyanError::ObjectNotFound`] if no object with the given ID exists.
    pub fn remove_object<P: Into<Cow<'a, str>> + Clone>(
        &'static mut self,
        id: P,
    ) -> anyhow::Result<()> {
        let cid = id.clone().into();

        // Find the index of the object with the specified ID.
        if let Some(o) = self.inner.iter().position(|f| f.id == cid) {
            self.inner.remove(o);
            Ok(())
        } else {
            Err(NyanError::ObjectNotFound(id.into()).into())
        }
    }

    /// Updates an existing object in the collection.
    ///
    /// **Note:** Currently, this method only removes the existing object with the given ID.
    /// It is expected that the caller will add a new object afterward if an update is intended.
    ///
    /// # Parameters
    ///
    /// - `id`: The identifier of the object to update.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the object was successfully removed.
    /// - An error if the object with the given ID does not exist.
    pub fn update_object<P: Into<Cow<'a, str>>>(&'static mut self, id: P) -> anyhow::Result<()> {
        let cid = id.into();
        self.remove_object(cid)?;
        Ok(())
    }

    /// Retrieves the index of an object in the collection by its unique identifier.
    ///
    /// This is an internal helper method.
    ///
    /// # Parameters
    ///
    /// - `id`: The identifier of the object to find.
    ///
    /// # Returns
    ///
    /// - `Some(index)` if the object is found.
    /// - `None` if no object with the given ID exists.
    pub(self) fn get<P: Into<Cow<'a, str>>>(&self, id: P) -> Option<usize> {
        let id = id.into();
        self.inner.iter().position(|f| f.id == id)
    }

    /// Draws the object associated with the given ID at its stored coordinate.
    ///
    /// The method performs the following steps:
    /// 1. Retrieves the object by its unique ID.
    /// 2. Moves the cursor to the object's stored coordinate.
    /// 3. Draws the object based on its type:
    ///    - **Text:** Prints the text to the console.
    ///    - **Air:** Does nothing.
    ///    - **Block:** Not yet implemented (invokes `todo!()`).
    ///
    /// # Parameters
    ///
    /// - `id`: The identifier of the object to draw.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the object was successfully drawn.
    /// - An error if the object is not found or if moving the cursor fails.
    pub fn draw_object<P: Into<Cow<'static, str>>>(&self, id: P) -> anyhow::Result<()> {
        let id = id.into();
        if let Some(object_index) = self.get(id.clone()) {
            let obj = &self.inner[object_index];

            // Attempt to move the cursor to the object's coordinate.
            if let Err(e) =
                cursor::Cursor::move_cursor(Cursor::Move(obj.coordinate.0, obj.coordinate.1))
            {
                return Err(errors::NyanError::Cursor(e.to_string().into()).into());
            }

            // Draw the object based on its type.
            match &obj.object {
                // For a Text object, print its content.
                Objects::Text(t) => {
                    println!("{}", t.as_ref());
                }
                // For an Air object, no drawing is performed.
                Objects::Air => {}
                // For a Block object, drawing functionality is not yet implemented.
                Objects::Block => {
                    todo!()
                }
            }
            Ok(())
        } else {
            // Object not found.
            Err(NyanError::ObjectNotFound(id).into())
        }
    }

    /// Draws an object at a specified cursor position.
    ///
    /// Unlike [`draw_object`], this method moves the cursor to a provided position rather than
    /// using the object's stored coordinate. After moving the cursor, it draws the object according
    /// to its type:
    /// - **Text:** Prints the text.
    /// - **Air:** Does nothing.
    /// - **Block:** Not yet implemented.
    ///
    /// # Parameters
    ///
    /// - `id`: The unique identifier of the object to draw.
    /// - `moveto`: A [`Cursor`] specifying the new cursor position.
    ///
    /// # Returns
    ///
    /// - `Ok(())` if the object was successfully drawn.
    /// - An error if the object is not found or if moving the cursor fails.
    ///
    /// # Example
    /// ```ignore
    /// let cursor_pos = Cursor::new(10, 5);
    /// nyan.draw_with_move("text_object", cursor_pos)?;
    /// ```
    pub fn draw_with_move<P: Into<Cow<'static, str>> + Clone>(
        &self,
        id: P,
        moveto: Cursor,
    ) -> anyhow::Result<()> {
        let cid = id.clone().into();

        if let Some(object_index) = self.get(cid) {
            // Move the cursor to the specified position.
            Cursor::move_cursor(moveto)?;

            // Draw the object based on its type.
            match &self.inner[object_index].object {
                Objects::Text(t) => {
                    println!("{}", t.as_ref());
                }
                Objects::Air => {}
                Objects::Block => {
                    todo!()
                }
            }
        } else {
            return Err(errors::NyanError::ObjectNotFound(id.into()).into());
        }

        Ok(())
    }
}

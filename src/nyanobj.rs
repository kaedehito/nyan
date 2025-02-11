//! A module that provides a collection of objects identified by a unique ID, where each object is represented by the `Objects` enum.
//!
//! This module defines the `NyanObj` struct, which stores a collection of objects. The `NyanObj` struct is parameterized by a unique identifier type `P`, which is expected to be convertible into a `String`.
//! The collection allows adding, removing, updating, and drawing objects by their associated IDs.
//!
//! The `Objects` enum can represent various types of objects, such as text, air, or blocks. Drawing functionality is provided for each type of object.
//!
//! # Structs
//!
//! - `NyanObj`: A struct representing a collection of objects identified by a unique ID (`P`). It includes methods for managing and interacting with the collection.
//!
//! # Enums
//!
//! - `Objects`: An enum that represents the possible types of objects in the collection. These types include `Text`, `Air`, and `Block`.
//!
//! # Methods
//!
//! - `new()`: Creates a new `NyanObj` instance with an empty object map.
//! - `add_object(id: P, object: Objects<'a>)`: Adds a new object to the collection, associating it with the given ID.
//! - `remove_object(id: P)`: Removes an object from the collection by its ID.
//! - `update_object(id: P, object: Objects<'a>)`: Updates an existing object in the collection with a new value, using the given ID.
//! - `draw_object(id: P)`: Draws the object associated with the given ID. If the object is not found, an error message is returned.

use crate::cursor::Cursor;
use crate::errors::NyanError;
use crate::objects::Objects;
use std::borrow::Cow;
use std::collections::HashMap;

/// A struct representing a collection of objects identified by a unique ID of type `P`.
///
/// `P` is expected to be convertible into a `String`, and each object in the collection is represented by the `Objects<'a>` enum.
/// The `Objects` enum represents various types of objects, such as `Text`, `Air`, or `Block`.
pub struct NyanObj<'a> {
    /// A hashmap that stores objects, with the object ID (`P`) as the key and the object (`Objects<'a>`) as the value.
    /// The ID (`P`) is used to uniquely identify each object in the collection.
    objects: HashMap<Cow<'a, str>, Objects<'a>>,
}

impl<'a> NyanObj<'a> {
    /// Creates a new `NyanObj` instance with an empty object map.
    ///
    /// # Returns
    /// A new instance of `NyanObj` with an empty object map.
    pub fn new() -> Self {
        Self {
            objects: HashMap::new(),
        }
    }

    /// Adds a new object to the collection, associating it with the given ID.
    ///
    /// # Arguments
    /// - `id`: The ID of the object.
    /// - `object`: The object to add to the collection.
    pub fn add_object<P: Into<Cow<'a, str>>>(&mut self, id: P, object: Objects<'a>) {
        self.objects.insert(id.into(), object);
    }

    /// Removes an object from the collection, identified by the given ID.
    ///
    /// # Arguments
    /// - `id`: The ID of the object to remove.
    pub fn remove_object<P: Into<Cow<'a, str>>>(&mut self, id: P) {
        self.objects.remove(&id.into());
    }

    /// Updates an existing object in the collection, replacing the object associated with the given ID.
    ///
    /// # Arguments
    /// - `id`: The ID of the object to update.
    /// - `object`: The new object to associate with the given ID.
    pub fn update_object<P: Into<Cow<'a, str>>>(&mut self, id: P, object: Objects<'a>) {
        let id = id.into();
        self.objects.remove(&id);
        self.objects.insert(id, object.into());
    }

    /// Draws the object associated with the given ID.
    /// If the object is not found, an error message is returned.
    ///
    /// # Arguments
    /// - `id`: The ID of the object to draw.
    ///
    /// # Returns
    /// - `Result<(), String>`: Returns `Ok(())` if the object is successfully drawn, or an error message if the object is not found.
    pub fn draw_object<P: Into<Cow<'static, str>>>(&self, id: P) -> anyhow::Result<()> {
        let id = id.into();
        if let Some(object) = self.objects.get(&id) {
            match object {
                // Draws a Text object
                Objects::Text(t) => {
                    println!("{}", t.as_ref());
                }

                // Does nothing for Air objects
                Objects::Air => {}

                // Block object drawing is not yet implemented
                Objects::Block => {
                    todo!()
                }
            }
            Ok(())
        } else {
            // Returns an error message if the object is not found
            Err(NyanError::ObjectNotFound(id).into())
        }
    }

    /// Draws an object at the specified cursor position.
    ///
    /// This method moves the cursor to the given position and then draws the object
    /// associated with the provided `id`. The behavior depends on the object type:
    /// - **Text**: Prints the text at the new position.
    /// - **Air**: Does nothing.
    /// - **Block**: Not yet implemented (`todo!()`).
    ///
    /// # Type Parameters
    /// - `P`: A type that can be converted into a `Cow<'a, str>` (e.g., `&str` or `String`).
    ///
    /// # Parameters
    /// - `id`: The identifier of the object to be drawn.
    /// - `moveto`: The `Cursor` position where the object should be drawn.
    ///
    /// # Returns
    /// - `Ok(())` on success.
    /// - `Err(anyhow::Error)` if moving the cursor fails.
    ///
    /// # Example
    /// ```ignore
    /// let cursor_pos = Cursor::new(10, 5);
    /// nyan.draw_with_move("text_object", cursor_pos)?;
    /// ```
    ///
    /// # Errors
    /// This function may return an error if moving the cursor fails.
    pub fn draw_with_move<P: Into<Cow<'a, str>>>(
        &self,
        id: P,
        moveto: Cursor,
    ) -> anyhow::Result<()> {
        let id = id.into();

        if let Some(object) = self.objects.get(&id) {
            // Move cursor to the specified position
            Cursor::move_cursor(moveto)?;

            match object {
                // Draws a Text object by printing it
                Objects::Text(t) => {
                    println!("{}", t.as_ref());
                }

                // Air objects do nothing
                Objects::Air => {}

                // Block object drawing is not yet implemented
                Objects::Block => {
                    todo!()
                }
            }
        }

        Ok(())
    }
}

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

use crate::objects::Objects;
use std::{collections::HashMap, fmt::Debug, hash::Hash};

/// A struct representing a collection of objects identified by a unique ID of type `P`.
///
/// `P` is expected to be convertible into a `String`, and each object in the collection is represented by the `Objects<'a>` enum.
/// The `Objects` enum represents various types of objects, such as `Text`, `Air`, or `Block`.
pub struct NyanObj<P = String, T = String>
where
    P: AsRef<str> + Clone, // `Clone` added to allow duplicating IDs
    T: AsRef<str>,
{
    /// A hashmap that stores objects, with the object ID (`P`) as the key and the object (`Objects<'a>`) as the value.
    /// The ID (`P`) is used to uniquely identify each object in the collection.
    objects: HashMap<P, Objects<T>>,
}

impl<P, T: AsRef<str>> NyanObj<P, T>
where
    P: AsRef<str> + Eq + Hash + Debug + Clone,
{
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
    pub fn add_object(&mut self, id: P, object: Objects<T>) {
        self.objects.insert(id, object);
    }

    /// Removes an object from the collection, identified by the given ID.
    ///
    /// # Arguments
    /// - `id`: The ID of the object to remove.
    pub fn remove_object(&mut self, id: P) {
        self.objects.remove(&id);
    }

    /// Updates an existing object in the collection, replacing the object associated with the given ID.
    ///
    /// # Arguments
    /// - `id`: The ID of the object to update.
    /// - `object`: The new object to associate with the given ID.
    pub fn update_object(&mut self, id: P, object: Objects<T>) {
        self.objects.remove(&id);
        self.objects.insert(id, object);
    }

    /// Draws the object associated with the given ID.
    /// If the object is not found, an error message is returned.
    ///
    /// # Arguments
    /// - `id`: The ID of the object to draw.
    ///
    /// # Returns
    /// - `Result<(), String>`: Returns `Ok(())` if the object is successfully drawn, or an error message if the object is not found.
    pub fn draw_object(&self, id: P) -> Result<(), String> {
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
            Err(format!("Object with ID {:?} not found!", id))
        }
    }
}

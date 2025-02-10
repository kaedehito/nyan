use crate::objects::Objects;
use std::{collections::HashMap, fmt::Debug, hash::Hash};

/// A struct representing a collection of objects identified by a unique ID of type `P`.
///
/// `P` is expected to be convertible into a `String`, and each object in the collection is represented by `Objects<'a>`.
/// `Objects` is an enum that represents various types of objects, such as text, air, or block.
pub struct NyanObj<'a, P = String>
where
    P: AsRef<str> + Clone, // Added Clone to allow duplicating IDs
{
    /// A hashmap that stores objects, with the object ID (`P`) as the key and the object (`Objects<'a>`) as the value.
    /// The ID (`P`) is used to uniquely identify each object in the collection.
    objects: HashMap<P, Objects<'a>>,
}

impl<'a, P> NyanObj<'a, P>
where
    P: AsRef<str> + Eq + Hash + Debug + Clone,
{
    /// Creates a new `NyanTermObjs` instance with an empty object map.
    ///
    /// # Returns
    /// A new instance of `NyanTermObjs` with an empty object map.
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
    pub fn add_object(&mut self, id: P, object: Objects<'a>) {
        self.objects.insert(id, object);
    }

    pub fn remove_object(&mut self, id: P) {
        self.objects.remove(&id);
    }

    pub fn update_object(&mut self, id: P, object: Objects<'a>) {
        self.objects.remove(&id);
        self.objects.insert(id, object);
    }

    /// Draws the object associated with the given ID.
    /// If the object is not found, an error message is printed.
    ///
    /// # Arguments
    /// - `id`: The ID of the object to draw.
    ///
    /// # Returns
    /// - `Result<(), String>`: Ok if successful, or an error message if the object is not found.
    pub fn draw_object(&self, id: P) -> Result<(), String> {
        if let Some(object) = self.objects.get(&id) {
            match object {
                // Draws a Text object
                Objects::Text(t) => {
                    println!("{t}");
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

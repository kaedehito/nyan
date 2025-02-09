use crate::objects::Objects;
use std::{collections::HashMap, fmt::Debug, hash::Hash};

/// `NyanTermObjs` manages a map of objects, with the object ID (of type `P`) as the key and the objects (of type `Objects`) as the value.
/// It provides methods to add and draw objects.
pub struct NyanTermObjs<'a, P>
where
    P: Into<String>,
{
    /// A hashmap that stores objects, with the object ID (`P`) as the key and the object (`Objects`) as the value
    objects: HashMap<P, Objects<'a>>, // ID(P) → Objects is managed by ID
}

impl<'a, P> NyanTermObjs<'a, P>
where
    P: Into<String> + Eq + Hash + Debug,
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

    /// Draws the object associated with the given ID.
    /// If the object is not found, an error message is printed.
    ///
    /// # Arguments
    /// - `id`: The ID of the object to draw.
    pub fn draw_object(&self, id: P) {
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
        } else {
            // Prints an error message if the object is not found
            panic!("Object with ID {:?} not found!", id);
        }
    }
}

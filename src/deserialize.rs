//! Deserialize module which houses the Deserialize and Deserializer traits
//! used to handle the deserialization process. Also houses the implementation
//! of Deserialize on supported core items.

mod json;

use crate::error::Result;
pub use json::Json;

/// Trait to implement on deserializable items. Defines how the item is
/// deserialized.
pub trait Deserialize {
    /// Accept a deserializer, allowing it to deserialize this item. Note that
    /// this is an internal method used to deserialize from the Deserializer and
    /// is uncommon to use outside this library.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn accept<S>(deserializer: &S, input: &S::Input) -> Result<Self>
    where
        S: Deserializer,
        Self: Sized;
}

impl Deserialize for () {
    /// Accept a deserializer, allowing it to deserialize this item. Note that
    /// this is an internal method used to deserialize from the Deserializer and is
    /// uncommon to use outside this library.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn accept<S>(deserializer: &S, input: &S::Input) -> Result<Self>
    where
        S: Deserializer,
    {
        deserializer.visit_unit(input)
    }
}

/// Trait to implement on an item that conducts the deserialization, and
/// defines how data is deserialized. Interaction with this should be done
/// using the deserialize method, which in turn calls the required visit
/// methods to convert from the input type into the specified output type.
pub trait Deserializer {
    /// The input type for this Deserializer.
    type Input;

    /// Deserialize the input into the required output type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn deserialize<S>(&self, input: &Self::Input) -> Result<S>
    where
        S: Deserialize;

    /// Visit and deserialize a unit type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn visit_unit(&self, input: &Self::Input) -> Result<()>;
}

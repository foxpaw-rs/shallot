//! Deserialize module which houses the Deserialize and Deserializer traits
//! used to handle the deserialization process. Also houses the implementation
//! of Deserialize on supported core items.

mod json;
pub use json::Json;

/// Trait to implement on deserializable items. Defines how the item is
/// deserialized.
pub trait Deserialize {}

/// Trait to implement on an item that conducts the deserialization, and
/// defines how data is deserialized. Interaction with this should be done
/// using the deserialize method, which in turn calls the required visit
/// methods to convert from the input type into the specified output type.
pub trait Deserializer {
    /// The input type for this Deserializer.
    type Input;
}

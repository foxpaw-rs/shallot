//! Serialize module which houses the Serialize and Serializer traits used to
//! handle the serialization process. Also houses the implementation of
//! Serialize on supported core items.

mod json;
pub use json::Json;

/// Trait to implement on serializable items. Defines how the item is
/// serialized.
pub trait Serialize {}

/// Trait to implement on an item that conducts the serialization, and defines
/// how data is serialized. Interaction with this should be done using the
/// serialize method, which in turn calls the required visit methods to
/// convert into the output type.
pub trait Serializer {
    /// The output type for this Serializer.
    type Output;
}

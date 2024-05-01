//! Serialize module which houses the Serialize and Serializer traits used to
//! handle the serialization process. Also houses the implementation of
//! Serialize on supported core items.

mod json;
pub use json::Json;

/// Trait to implement on serializable items. Defines how the item is
/// serialized.
pub trait Serialize {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer;
}

impl Serialize for () {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_unit()
    }
}

impl Serialize for bool {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_bool(self)
    }
}

/// Trait to implement on an item that conducts the serialization, and defines
/// how data is serialized. Interaction with this should be done using the
/// serialize method, which in turn calls the required visit methods to
/// convert into the output type.
pub trait Serializer {
    /// The output type for this Serializer.
    type Output;

    /// Serialize the input into the required output type.
    fn serialize<S>(&self, input: &S) -> Self::Output
    where
        S: Serialize;

    /// Visit and serialize a bool type.
    fn visit_bool(&self, input: &bool) -> Self::Output;

    /// Visit and serialize a unit type.
    fn visit_unit(&self) -> Self::Output;
}

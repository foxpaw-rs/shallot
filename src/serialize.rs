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

impl Serialize for i8 {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_i8(self)
    }
}

impl Serialize for i16 {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_i16(self)
    }
}

impl Serialize for i32 {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_i32(self)
    }
}

impl Serialize for i64 {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_i64(self)
    }
}

impl Serialize for i128 {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_i128(self)
    }
}

impl Serialize for isize {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_isize(self)
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

    /// Visit and serialize an i8 type.
    fn visit_i8(&self, input: &i8) -> Self::Output;

    /// Visit and serialize an i16 type.
    fn visit_i16(&self, input: &i16) -> Self::Output;

    /// Visit and serialize an i32 type.
    fn visit_i32(&self, input: &i32) -> Self::Output;

    /// Visit and serialize an i64 type.
    fn visit_i64(&self, input: &i64) -> Self::Output;

    /// Visit and serialize an i128 type.
    fn visit_i128(&self, input: &i128) -> Self::Output;

    /// Visit and serialize an isize type.
    fn visit_isize(&self, input: &isize) -> Self::Output;

    /// Visit and serialize a unit type.
    fn visit_unit(&self) -> Self::Output;
}

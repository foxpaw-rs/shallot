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

impl Deserialize for bool {
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
        deserializer.visit_bool(input)
    }
}

impl Deserialize for i8 {
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
        deserializer.visit_i8(input)
    }
}

impl Deserialize for i16 {
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
        deserializer.visit_i16(input)
    }
}

impl Deserialize for i32 {
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
        deserializer.visit_i32(input)
    }
}

impl Deserialize for i64 {
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
        deserializer.visit_i64(input)
    }
}

impl Deserialize for i128 {
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
        deserializer.visit_i128(input)
    }
}

impl Deserialize for isize {
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
        deserializer.visit_isize(input)
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

    /// Visit and deserialize a bool type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn visit_bool(&self, input: &Self::Input) -> Result<bool>;

    /// Visit and deserialize a i8 type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn visit_i8(&self, input: &Self::Input) -> Result<i8>;

    /// Visit and deserialize a i16 type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn visit_i16(&self, input: &Self::Input) -> Result<i16>;

    /// Visit and deserialize a i32 type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn visit_i32(&self, input: &Self::Input) -> Result<i32>;

    /// Visit and deserialize a i64 type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn visit_i64(&self, input: &Self::Input) -> Result<i64>;

    /// Visit and deserialize a i128 type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn visit_i128(&self, input: &Self::Input) -> Result<i128>;

    /// Visit and deserialize a isize type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn visit_isize(&self, input: &Self::Input) -> Result<isize>;

    /// Visit and deserialize a unit type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    fn visit_unit(&self, input: &Self::Input) -> Result<()>;
}

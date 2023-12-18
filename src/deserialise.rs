//! Deserialise
//!
//! Deserialise module which houses the Deserialise and Deserialiser trait used
//! to handle the deserialisation process. Also houses the implementation of
//! Deserialise on supported core items.

mod json;

use crate::error::Error;
pub use json::Json;

/// Deserialise
///
/// Trait to implement on deserialisable items. Single accept method which
/// supports the visitor pattern deserialisation process.
pub trait Deserialise {
    /// Accept
    ///
    /// Accept a deserialiser, allowing it to deserailise this item. Note that this
    /// is an internal method used to deserialise from the Deserialiser and is
    /// uncommon to use outside this library.
    ///
    /// # Errors
    /// Will return a Syntax error if invalid input detected by the Deserialiser.
    /// Will return an Overflow error if the input overflows the specified type.
    fn accept<D>(deserialiser: &D, input: D::Input) -> Result<Self, Error>
    where
        D: Deserialiser,
        Self: Sized;
}

impl Deserialise for () {
    /// Accept
    ///
    /// Accept a deserialiser, allowing it to deserailise this item. Note that this
    /// is an internal method used to deserialise from the Deserialiser and is
    /// uncommon to use outside this library.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::deserialise::{Json, Deserialise};
    ///
    /// let deserialiser = Json::new();
    /// let output = <()>::accept(&deserialiser, "()");
    /// ```
    ///
    /// # Errors
    /// Will return a Syntax error if invalid input detected by the Deserialiser.
    fn accept<D>(deserialiser: &D, input: D::Input) -> Result<Self, Error>
    where
        D: Deserialiser,
        Self: Sized,
    {
        deserialiser.visit_unit(input)
    }
}

/// Deserialiser
///
/// Trait to implement on a struct that conducts the actual deserialisation and
/// defines how data is deserialised. Implements a deserialise method to
/// traverse the item and deserialise it, along with numerous visit methods to
/// actually convert the item into requested type.
pub trait Deserialiser: Sized {
    /// The input type for the Deerialiser.
    type Input;

    /// Deserialise the input.
    ///
    /// Request this deserialiser deserialise the provided input.
    ///
    /// # Errors
    /// Will return a Syntax error if invalid input is supplied.
    fn deserialise<T>(&self, input: Self::Input) -> Result<T, Error>
    where
        T: Deserialise;

    /// Visit a unit type.
    ///
    /// Visit and deserialise a unit type.
    ///
    /// # Errors
    /// Will return a Syntax error if invalid input is supplied.
    fn visit_unit(&self, input: Self::Input) -> Result<(), Error>;
}

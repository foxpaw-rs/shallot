//! Serialise
//!
//! Serialise module which houses the Serialise and Serialiser trait used to
//! handle the serialisation process. Also houses the implementation of
//! Serialise on supported core items.
mod json;

pub use json::Json;

/// Serialise
///
/// Trait to implement on serialisable items. Single accept method which
/// supports the visitor pattern serialisation process.
pub trait Serialise {
    /// Accept
    ///
    /// Accept a serialiser, allowing it to serailise this item. Note that this
    /// is an internal method used to serialise from the Serialiser and is
    /// uncommon to use outside this library.
    fn accept<S>(&self, serialiser: &S) -> S::Output
    where
        S: Serialiser;
}

impl Serialise for () {
    /// Accept
    ///
    /// Accept a serialiser, allowing it to serailise this item. Note that this
    /// is an internal method used to serialise from the Serialiser and is
    /// uncommon to use outside this library.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::serialise::{Json, Serialise};
    ///
    /// let serialiser = Json::new();
    /// let output = ().accept(&serialiser);
    /// ```
    fn accept<S>(&self, serialiser: &S) -> S::Output
    where
        S: Serialiser,
    {
        serialiser.visit_unit()
    }
}

/// Serialiser
///
/// Trait to implement on a struct that conducts the actual serialisation and
/// defines how data is serialised. Implements a serialise method to traverse
/// the item and serialise it, along with numerous visit methods to actually
/// convert the item into the Output type.
pub trait Serialiser: Sized {
    /// The output type for the Serialiser.
    type Output;

    /// Serialise the input.
    ///
    /// Request this serialiser serialise the provided input.
    fn serialise<T>(&self, input: &T) -> Self::Output
    where
        T: Serialise;

    /// Visit a unit type.
    ///
    /// Visit and serialise a unit type.
    fn visit_unit(&self) -> Self::Output;

}

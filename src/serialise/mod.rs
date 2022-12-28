pub mod json;
pub mod table;
pub mod toml;

pub use json::Json;
use std::fmt::Display;
pub use table::Table;
pub use toml::Toml;

/// Serialise.
///
/// The Serialise trait marks that this type can be serialised. No methods are
/// required to be implemented on the type, however, all the contained datan in
/// the type must also be Serialise.
pub trait Serialise {}

/// Serialiser.
///
/// The Serialiser trait defines required methods for a type intended to
/// serialise data.
pub trait Serialiser: Display {
    /// The expected output type.
    type Ok;
}

impl Serialise for u8 {}

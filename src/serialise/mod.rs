//! Serialise Module
//!
//! Contains the Serialise and Serialiser traits. Also implements the Serialise
//! trait on types supported by Shallot.
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
pub trait Serialise {
    /// Accept a Serialiser.
    ///
    /// Accept a Serialiser that will serialise this object.
    fn accept(&self, serialiser: &dyn Serialiser) -> String;
}

impl Serialise for u8 {
    /// Accept a Serialiser.
    ///
    /// Accept a Serialiser that will serialise this object.
    fn accept(&self, serialiser: &dyn Serialiser) -> String {
        serialiser.visit_u8(self)
    }
}

/// Serialiser.
///
/// The Serialiser trait defines required methods for a type intended to
/// serialise data.
pub trait Serialiser: Display {
    /// Serialise.
    ///
    /// Serialise the current serialiser.
    fn serialise(&self) -> String;

    /// Visit U8.
    ///
    /// Visit a u8 node and serialise it.
    fn visit_u8(&self, value: &u8) -> String;
}

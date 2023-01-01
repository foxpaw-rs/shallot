//! Deserialise Module
//!
//! Contains the Deserialise and Deserialiser traits. Also implements the
//! Deserialise trait on types supported by Shallot.
pub mod error;
pub mod json;
pub mod table;
pub mod toml;

pub use error::ParseError;
pub use json::Json;
pub use table::Table;
pub use toml::Toml;

/// Deserialise.
///
/// The Deserialise trait marks that this type can be deserialised. Contains
/// methods to deserialise the provided content, into the concrete implementing
/// type.
pub trait Deserialise {
    /// Accept a Deerialiser.
    ///
    /// Accept a Deerialiser that will parse the provided content for
    /// deserialisation.
    fn accept<'de, T>(
        deserialiser: &'de dyn Deserialiser<'de, T>,
        content: &'de str,
    ) -> Result<Self, ParseError>
    where
        Self: Sized;

    /// Generate a ParseError.
    ///
    /// Will generate a generic ParseError given the content that caused the error
    /// and the stringified type attempting to deserialise.
    fn parse_error(content: &str, concrete: &str) -> ParseError {
        ParseError::new(&format!("Invalid value '{}' for type '{}'.", content, concrete))
    }
}

impl Deserialise for u8 {
    /// Accept a Deerialiser.
    ///
    /// Accept a Deerialiser that will parse the provided content for
    /// deserialisation.
    fn accept<'de, T>(
        deserialiser: &'de dyn Deserialiser<'de, T>,
        content: &'de str,
    ) -> Result<Self, ParseError>
    where
        Self: Sized,
    {
        let content = deserialiser.visit_u8(content)?;
        content
            .parse::<Self>()
            .map_err(|_| Self::parse_error(content, "u8"))
    }
}

/// Deserialiser.
///
/// The Deserialiser trait defines required methods for a type intended to
/// deserialise data.
pub trait Deserialiser<'de, T> {
    /// Deserialise.
    ///
    /// Deserialise the current deserialiser.
    fn deserialise(&self) -> Result<T, ParseError>
    where
        T: Deserialise;

    /// Visit U8.
    ///
    /// Visit a u8 node and parse it for deserialisation.
    fn visit_u8(&'de self, content: &'de str) -> Result<&'de str, ParseError>;
}

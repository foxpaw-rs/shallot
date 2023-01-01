//! Error Module.
//!
//! This module houses the deserialisation Error types used in the library.
//! These types are to be used throughout the library for error handling.

use core::fmt::{self, Display, Formatter};
use std::error::Error;

/// Parse Error.
///
/// Typed error for the library. Utilised for all errors raised from this
/// library. Uses a provided String as the internal error message. Can be used
/// in a core::result::Result, however, for convenience a Result type is
/// provided in this module.
///
/// Example
/// ```rust
/// use shallot::deserialise::ParseError;
///
/// fn is_not_zero(number: u8) -> Result<(), ParseError> {
///     if number == 0 { return Err(ParseError::new("Zero!")); }
///     Ok(())
/// }
/// ```
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ParseError {
    /// The error message.
    message: String,
}

impl ParseError {
    /// Create a new Parse Error.
    ///
    /// Construct a new Parse Error with the provided message.
    ///
    /// # Example
    /// ```rust
    /// use shallot::deserialise::ParseError;
    ///
    /// ParseError::new("Something went wrong...");
    /// ```
    pub fn new(message: &str) -> Self {
        Self {
            message: String::from(message),
        }
    }
}

impl Display for ParseError {
    /// Format an Parse Error for display.
    ///
    /// Formats the error for display and pretty printing.
    ///
    /// # Example
    /// ```
    /// use shallot::deserialise::ParseError;
    ///
    /// let error = ParseError::new("Message");
    /// eprintln!("{}", error);
    /// ```
    ///
    /// # Error
    /// Will error if the underlying write macro fails.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "Error: {}", self.message)
    }
}

impl Error for ParseError {}

#[cfg(test)]
mod tests {

    use super::*;

    /// ParseError::new must create as per struct initialisation.
    ///
    /// The new method on Parse Error must create an object as per the struct
    /// initialiser syntax.
    #[test]
    fn error_new() {
        let expected = ParseError {
            message: String::from("Message"),
        };
        let actual = ParseError::new("Message");
        assert_eq!(expected, actual);
    }

    /// ParseError::fmt must display the Parse Error.
    ///
    /// The Display trait provides a to_string method which allows testing that the
    /// Parse Error formats correctly when displayed or converted to string types.
    #[test]
    fn error_fmt() {
        let expected = "Error: Message";
        let actual = ParseError::new("Message").to_string();

        assert_eq!(expected, actual);
    }
}

//! Error
//!
//! Error module to house the Shallot error types.

mod syntax;

use std::cmp::PartialEq;
use std::fmt::{Debug, Display, Formatter, Result};
pub use syntax::Syntax;

/// Error.
///
/// The Shallot Error type, encompassing all the internal error types which
/// provie builder style syntax. This Error is the only error type presented
/// external to the library.
#[derive(Debug)]
pub struct Error {
    /// The type of error.
    kind: Kind,
    /// The error message.
    message: String,
}

impl Error {
    /// Create a new Error.
    ///
    /// Creates a new Error based off the supplied error constucted.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::{Error, Syntax};
    ///
    /// let error = Error::new(&Syntax::new(0, 0));
    /// ```
    pub fn new(source: &impl Marker) -> Self {
        Self {
            kind: source.kind(),
            message: source.to_string(),
        }
    }
}

impl Display for Error {
    /// Format this Error error.
    ///
    /// Formats the Error error, taking into account the option expected and
    /// unexpected values.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::{Error, Syntax};
    ///
    /// let error: Error = Syntax::new(0, 0).unexpected("(").into();
    /// println!("{}", error);
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.message)
    }
}

impl From<Syntax> for Error {
    /// Transform Syntax into an Error
    ///
    /// Transforms Syntax into an Error for error handling in the calling
    /// application
    ///
    /// # Example
    /// ```rust
    /// use shallot::error::{Error, Syntax};
    ///
    /// let error: Error = Syntax::new(1, 1).into();
    /// ```
    fn from(syntax: Syntax) -> Self {
        Self::new(&syntax)
    }
}

impl PartialEq for Error {
    /// Compare the two Errors.
    ///
    /// Compares the two error types, signalling they are equal if the kind and
    /// message are the same
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::{Error, Syntax};
    ///
    /// let left: Error = Syntax::new(0, 0).unexpected("(").into();
    /// let right: Error = Syntax::new(0, 0).unexpected("(").into();
    /// println!("{}", left == right);
    /// ```
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.message == other.message
    }
}

impl std::error::Error for Error {}

/// Kind.
///
/// The Error kinds available.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Kind {
    /// A Syntax error type.
    Syntax,
}

/// Marker.
///
/// A Marker trait to signify that the implementor is a Shallot Error type.
pub trait Marker: Display + Debug + std::error::Error {
    /// Get the kind of error.
    ///
    /// Retrieves the kind of error.
    fn kind(&self) -> Kind;
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Error::new() creates a Error as expected.
    ///
    /// Test that Error::new() creates a Error as expected.
    #[test]
    fn error_new() {
        let actual = Error::new(&Syntax::new(0, 1));
        let inner = Syntax::new(0, 1);
        let expected = Error {
            kind: Kind::Syntax,
            message: inner.to_string(),
        };
        assert_eq!(expected, actual);
    }

    /// Error::fmt correctly formats.
    ///
    /// Test that Error::fmt correctly formats.
    #[test]
    fn error_fmt() {
        let actual = Error::new(&Syntax::new(0, 1));
        let expected = "Error: Syntax error [Line: 0, Col: 1]";
        assert_eq!(expected, actual.to_string());
    }

    /// Error::eq correctly compares.
    ///
    /// Test that Error::eq correctly compares.
    #[test]
    fn error_eq() {
        let left = Error::new(&Syntax::new(0, 1));
        let right = Error::new(&Syntax::new(0, 1));
        assert!(left == right);

        let left = Error::new(&Syntax::new(0, 1));
        let right = Error::new(&Syntax::new(0, 2));
        assert!(left != right);
    }
}

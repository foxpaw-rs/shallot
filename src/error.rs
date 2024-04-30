//! Error module to house the base error type exported by Shallot. Additionally
//! houses the error types supplied by the library.
//!
//! # Examples
//! ```rust
//! use shallot::error::{Error, Kind};
//!
//! let error = Error::new("Whoops, something went wrong!");
//! ```

mod syntax;

use std::convert::From;
use std::{error, fmt, result};
pub use syntax::Syntax;

pub type Result<T> = result::Result<T, Error>;

/// Generic error which is used when providing an error from the library.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Error {
    /// The error message.
    message: String,

    /// The error kind.
    kind: Kind,
}

impl Error {
    /// Create a new Error.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::{Error, Kind};
    ///
    /// let error = Error::new("Whoops, something went wrong!");
    /// ```
    #[must_use]
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_owned(),
            kind: Kind::General,
        }
    }
}

impl fmt::Display for Error {
    /// Format the error for displaying.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::{Error, Kind};
    ///
    /// let error = Error::new("Whoops, something went wrong!");
    /// println!("{error}");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Error]: {}", self.message)
    }
}

impl error::Error for Error {}

impl From<Syntax> for Error {
    /// Convert from a syntax error into a shallot Error.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::{Error, Syntax};
    ///
    /// let error: Error = Syntax::new(1, 1).unexpected("b").expected("a").into();
    /// ```
    fn from(item: Syntax) -> Self {
        let mut error = Self::new(&item.to_string());
        error.kind = Kind::Syntax;
        error
    }
}

/// The available error types. These represent all the error types encountered
/// through the Shallot library.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Kind {
    /// A general error.
    General,

    /// A syntax error.
    Syntax,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test Error::new creates a Error as expected.
    #[test]
    fn new_correct() {
        let expected = Error {
            message: "Whoops, something went wrong!".to_owned(),
            kind: Kind::General,
        };
        let actual = Error::new("Whoops, something went wrong!");
        assert_eq!(expected, actual);
    }

    /// Test Error::fmt functions correctly.
    #[test]
    fn fmt_correct() {
        let expected = "[Error]: Whoops, something went wrong!";
        let actual = Error::new("Whoops, something went wrong!").to_string();
        assert_eq!(expected, actual);
    }

    /// Test Error::from functions correctly from Syntax.
    #[test]
    fn from_syntax_correct() {
        let mut expected = Error::new("Syntax error at (1, 1)");
        expected.kind = Kind::Syntax;
        let actual = Syntax::new(1, 1).into();
        assert_eq!(expected, actual);
    }
}

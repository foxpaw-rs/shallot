//! Error module to house the base error type exported by Shallot. Additionally
//! houses the error types supplied by the library.
//!
//! # Examples
//! ```rust
//! use shallot::error::{Error, Kind};
//!
//! let error = Error::new("Whoops, something went wrong!", Kind::General);
//! ```

use std::error;
use std::fmt;

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
    /// let error = Error::new("Whoops, something went wrong!", Kind::General);
    /// ```
    #[must_use]
    pub fn new(message: &str, kind: Kind) -> Self {
        Self {
            message: message.to_owned(),
            kind,
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
    /// let error = Error::new("Whoops, something went wrong!", Kind::General);
    /// println!("{error}");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[Error]: {}", self.message)
    }
}

impl error::Error for Error {}

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
    fn error_new_correct() {
        let expected = Error {
            message: "Whoops, something went wrong!".to_owned(),
            kind: Kind::General,
        };
        let actual = Error::new("Whoops, something went wrong!", Kind::General);
        assert_eq!(expected, actual);
    }

    /// Test Error::fmt functions correctly.
    #[test]
    fn error_fmt_correct() {
        let expected = "[Error]: Whoops, something went wrong!";
        let actual = Error::new("Whoops, something went wrong!", Kind::General).to_string();
        assert_eq!(expected, actual);
    }
}

//! Syntax module to house the syntax error type. Used to signify when a syntax
//! error has occurred
//!
//! # Examples
//! ```rust
//! use shallot::error::Syntax;
//!
//! let error = Syntax::new(1, 1).unexpected("b").expected("a");
//! ```

use std::error::Error;
use std::fmt;

/// Syntax error to signify that invalid syntax was located.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Syntax {
    /// The column where the syntax error occurs.
    col: usize,

    /// What was expected.
    expected: Option<String>,

    /// The row where the syntax error occurs.
    row: usize,

    /// What was found that was unexpected and caused the error.
    unexpected: Option<String>,
}

impl Syntax {
    /// Create a new Syntax error.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Syntax;
    ///
    /// let error = Syntax::new(1, 1);
    /// ```
    #[must_use]
    pub fn new(row: usize, col: usize) -> Self {
        Self {
            col,
            expected: None,
            row,
            unexpected: None,
        }
    }

    /// Set the expected value, to notify the user what was expected in the input.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Syntax;
    ///
    /// let error = Syntax::new(1, 1).expected("a");
    /// ```
    #[must_use]
    pub fn expected(mut self, expected: &str) -> Self {
        self.expected = Some(expected.to_owned());
        self
    }

    /// Set the unexpected value, to notify the user what was unexpected in the input.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Syntax;
    ///
    /// let error = Syntax::new(1, 1).unexpected("b");
    /// ```
    #[must_use]
    pub fn unexpected(mut self, unexpected: &str) -> Self {
        self.unexpected = Some(unexpected.to_owned());
        self
    }
}

impl fmt::Display for Syntax {
    /// Format the error for displaying.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Syntax;
    ///
    /// let error = Syntax::new(1, 1).unexpected("b").expected("a");
    /// println!("{error}");
    /// ```
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (&self.expected, &self.unexpected) {
            (None, None) => write!(f, "Syntax error at ({}, {})", self.row, self.col),
            (None, Some(u)) => write!(
                f,
                "Syntax error, unexpected \"{u}\" at ({}, {})",
                self.row, self.col
            ),
            (Some(e), None) => write!(
                f,
                "Syntax error, expected \"{e}\" at ({}, {})",
                self.row, self.col
            ),
            (Some(e), Some(u)) => write!(
                f,
                "Syntax error, unexpected \"{u}\", expected \"{e}\" at ({}, {})",
                self.row, self.col
            ),
        }
    }
}

impl Error for Syntax {}

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

    /// Test Syntax::new creates a Syntax as expected.
    #[test]
    fn error_new_correct() {
        let expected = Syntax {
            col: 1,
            expected: None,
            row: 1,
            unexpected: None,
        };
        let actual = Syntax::new(1, 1);
        assert_eq!(expected, actual);
    }

    /// Test Syntax::expected sets the expected value.
    #[test]
    fn error_expected_correct() {
        let expected = Some("a".to_owned());
        let actual = Syntax::new(1, 1).expected("a").expected;
        assert_eq!(expected, actual);
    }

    /// Test Syntax::unexpected sets the unexpected value.
    #[test]
    fn error_unexpected_correct() {
        let expected = Some("a".to_owned());
        let actual = Syntax::new(1, 1).unexpected("a").unexpected;
        assert_eq!(expected, actual);
    }

    /// Test Syntax::fmt formats with no expected or unexpected values.
    #[test]
    fn error_fmt_none() {
        let expected = "Syntax error at (1, 1)".to_owned();
        let actual = Syntax::new(1, 1).to_string();
        assert_eq!(expected, actual);
    }

    /// Test Syntax::fmt formats with an expected value.
    #[test]
    fn error_fmt_expected() {
        let expected = "Syntax error, expected \"a\" at (1, 1)".to_owned();
        let actual = Syntax::new(1, 1).expected("a").to_string();
        assert_eq!(expected, actual);
    }

    /// Test Syntax::fmt formats with an unexpected value.
    #[test]
    fn error_fmt_unexpected() {
        let expected = "Syntax error, unexpected \"b\" at (1, 1)".to_owned();
        let actual = Syntax::new(1, 1).unexpected("b").to_string();
        assert_eq!(expected, actual);
    }

    /// Test Syntax::fmt formats with expected and unexpected values.
    #[test]
    fn error_fmt_both() {
        let expected = "Syntax error, unexpected \"b\", expected \"a\" at (1, 1)".to_owned();
        let actual = Syntax::new(1, 1).unexpected("b").expected("a").to_string();
        assert_eq!(expected, actual);
    }
}

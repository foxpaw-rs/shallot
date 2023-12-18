//! Syntax
//!
//! Syntax error module to house the Syntax error type.

use crate::error::{Kind, Marker};
use std::error::Error;
use std::fmt::{Display, Formatter, Result};

/// Syntax.
///
/// The Syntax error type, used when a syntax error is encountered.
///
/// # Exampples
/// ```rust
/// use shallot::error::Syntax;
///
/// fn convert_to_unit(val: &str) -> Result<(), Syntax> {
///     if(val != "()") {
///         Err(Syntax::new(0, 0).expected("()"))
///     } else {
///         Ok(())
///     }
/// }
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Syntax {
    /// The column where the error occurred.
    col: usize,

    /// The expected value.
    expected: Option<String>,

    /// The line where the error occurred.
    line: usize,

    /// The unexpected value
    unexpected: Option<String>,
}

impl Syntax {
    /// Create a new Syntax error.
    ///
    /// Creates a new Syntax error at the specified location.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Syntax;
    ///
    /// let error = Syntax::new(0, 0);
    /// ```
    #[must_use]
    pub const fn new(line: usize, col: usize) -> Self {
        Self {
            col,
            expected: None,
            line,
            unexpected: None,
        }
    }

    /// Set the expected value.
    ///
    /// Sets the expected value for the Syntax error. Note that this method consumes
    /// the Syntax and returns it to allow for chaining.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Syntax;
    ///
    /// let error = Syntax::new(0, 0).expected("(");
    /// ```
    #[must_use]
    pub fn expected(mut self, expected: &str) -> Self {
        self.expected = Some(expected.to_owned());
        self
    }

    /// Set the unexpected value.
    ///
    /// Sets the unexpected value for the Syntax error. Note that this method
    /// consumes the Syntax and returns it to allow for chaining.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Syntax;
    ///
    /// let error = Syntax::new(0, 0).unexpected("(");
    /// ```
    #[must_use]
    pub fn unexpected(mut self, unexpected: &str) -> Self {
        self.unexpected = Some(unexpected.to_owned());
        self
    }
}

impl Display for Syntax {
    /// Format this Syntax error.
    ///
    /// Formats the Syntax error, taking into account the option expected and
    /// unexpected values.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Syntax;
    ///
    /// println!("{}", Syntax::new(0, 0).unexpected("("));
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        match self {
            Self {
                line,
                col,
                expected: Some(e),
                unexpected: Some(u),
            } => write!(
                f,
                "Error: Unexpected '{u}', expected '{e}' [Line: {line}, Col: {col}]"
            ),
            Self {
                line,
                col,
                expected: Some(e),
                unexpected: None,
            } => write!(f, "Error: Expected '{e}' [Line: {line}, Col: {col}]"),
            Self {
                line,
                col,
                expected: None,
                unexpected: Some(u),
            } => write!(f, "Error: Unexpected '{u}' [Line: {line}, Col: {col}]",),
            Self {
                line,
                col,
                expected: None,
                unexpected: None,
            } => write!(f, "Error: Syntax error [Line: {line}, Col: {col}]"),
        }
    }
}

impl Error for Syntax {}

impl Marker for Syntax {
    /// Get the kind of error.
    ///
    /// Retrieves the kind of error.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::{Syntax, Marker};
    ///
    /// let kind = Syntax::new(0, 1).kind();
    /// ```
    fn kind(&self) -> Kind {
        Kind::Syntax
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Syntax::new() creates a Syntax as expected.
    ///
    /// Test that Syntax::new() creates a Syntax as expected.
    #[test]
    fn syntax_new() {
        let actual = Syntax::new(0, 1);
        let expected = Syntax {
            col: 1,
            expected: None,
            line: 0,
            unexpected: None,
        };
        assert_eq!(expected, actual);
    }

    /// Syntax::expected correctly sets the expected value.
    ///
    /// Test that Syntax::expected correctly sets the expected value.
    #[test]
    fn syntax_expected() {
        let actual = Syntax::new(0, 1).expected("e");
        let expected = Some("e".to_owned());
        assert_eq!(expected, actual.expected);
    }

    /// Syntax::unexpected correctly sets the unexpected value.
    ///
    /// Test that Syntax::unexpected correctly sets the unexpected value.
    #[test]
    fn syntax_unexpected() {
        let actual = Syntax::new(0, 1).unexpected("u");
        let expected = Some("u".to_owned());
        assert_eq!(expected, actual.unexpected);
    }

    /// Syntax::fmt correctly formats with all internals set.
    ///
    /// Test that Syntax::fmt correctly formats with all internals set.
    #[test]
    fn syntax_fmt_all() {
        let actual = Syntax::new(0, 1).expected("e").unexpected("u");
        let expected = "Error: Unexpected 'u', expected 'e' [Line: 0, Col: 1]";
        assert_eq!(expected, actual.to_string());
    }

    /// Syntax::fmt correctly formats with expected set.
    ///
    /// Test that Syntax::fmt correctly formats with expected set.
    #[test]
    fn syntax_fmt_expected() {
        let actual = Syntax::new(0, 1).expected("e");
        let expected = "Error: Expected 'e' [Line: 0, Col: 1]";
        assert_eq!(expected, actual.to_string());
    }

    /// Syntax::fmt correctly formats with unexpected set.
    ///
    /// Test that Syntax::fmt correctly formats with unexpected set.
    #[test]
    fn syntax_fmt_unexpected() {
        let actual = Syntax::new(0, 1).unexpected("u");
        let expected = "Error: Unexpected 'u' [Line: 0, Col: 1]";
        assert_eq!(expected, actual.to_string());
    }

    /// Syntax::fmt correctly formats with neither expected or unexpected.
    ///
    /// Test that Syntax::fmt correctly formats with neither expected or unexpected.
    #[test]
    fn syntax_fmt_none() {
        let actual = Syntax::new(0, 1);
        let expected = "Error: Syntax error [Line: 0, Col: 1]";
        assert_eq!(expected, actual.to_string());
    }

    /// Syntax::kind correctly returns.
    ///
    /// Test that Syntax::kind correctly returns.
    #[test]
    fn syntax_kind() {
        let actual = Syntax::new(0, 1);
        let expected = Kind::Syntax;
        assert_eq!(expected, actual.kind());
    }
}

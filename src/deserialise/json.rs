//! Json
//!
//! Json module which houses the Json deserialiser. Defined to the JSON
//! specification located at <https://www.json.org/json-en.html>.

use crate::deserialise::{Deserialise, Deserialiser};
use crate::error::{Error, Syntax};
use core::cell::Cell;
use core::marker::PhantomData;
use core::str::Chars;

/// Json
///
/// Json deserialiser, will deserialise data into Json formatting.
///
/// # Examples
/// ```rust
/// use shallot::deserialise::{Json, Deserialiser};
/// # use shallot::error;
///
/// # fn main() -> Result<(), error::Error> {
/// let output: () = Json::new().deserialise("()")?;
/// # Ok(output)
/// # }
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Json<'a> {
    /// The current column number.
    col: Cell<usize>,

    /// The current line number.
    line: Cell<usize>,

    /// Phantom to hold the lifetime of this object for the Input associated type.
    phantom: PhantomData<&'a ()>,
}

impl<'a> Json<'a> {
    /// Create a new Json.
    ///
    /// Creates a new Json deserialiser.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::deserialise::Json;
    ///
    /// let json = Json::new();
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            col: Cell::new(0),
            line: Cell::new(1),
            phantom: PhantomData,
        }
    }

    /// Retrieve the next value.
    ///
    /// Retrieve the next character from the Chars iterator.
    fn next(&self, chars: &mut Chars) -> Option<char> {
        let c = chars.next();
        match c {
            Some(c) if c == '\n' => {
                self.line.set(self.line.get() + 1);
                self.col.set(1);
            }
            _ => self.col.set(self.col.get() + 1),
        }
        c
    }

    /// Retrieve the next value while the predicate is met.
    ///
    /// Retrieve the next character from the Chars iterator while the predicate is met.
    fn next_while(&self, chars: &mut Chars, predicate: fn(char) -> bool) -> Option<char> {
        while let Some(c) = self.next(chars) {
            if !predicate(c) {
                return Some(c);
            }
        }
        None
    }
}

impl<'a> Deserialiser for Json<'a> {
    /// The input type for a Json deserialiser.
    type Input = &'a str;

    /// Deserialise the input.
    ///
    /// Request this deserialiser deserialise the provided input.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::deserialise::{Json, Deserialiser};
    /// # use shallot::error;
    ///
    /// # fn main() -> Result<(), error::Error> {
    /// let raw = "()";
    /// let data = Json::new().deserialise(raw)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// Will return a Syntax error if invalid input is supplied.
    /// Will return an Overflow error if the input overflows the specified type.
    fn deserialise<T>(&self, input: Self::Input) -> Result<T, Error>
    where
        T: Deserialise,
    {
        self.col.set(0);
        self.line.set(1);
        T::accept(self, input)
    }

    /// Visit a unit type.
    ///
    /// Visit and deserialise a unit type.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::deserialise::{Json, Deserialiser};
    /// # use shallot::error;
    ///
    /// # fn main() -> Result<(), error::Error> {
    /// let raw = "()";
    /// let data = Json::new().visit_unit(raw)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// # Errors
    /// Will error if invalid syntax is encountered.
    fn visit_unit(&self, input: Self::Input) -> Result<(), Error> {
        let mut chars = input.chars();

        // Check first character is open brace.
        match self.next_while(&mut chars, char::is_whitespace) {
            Some(c) if c != '(' => {
                return Err(Syntax::new(self.line.get(), self.col.get())
                    .expected("(")
                    .unexpected(c.encode_utf8(&mut [0u8; 4]))
                    .into())
            }
            None => {
                return Err(Syntax::new(self.line.get(), self.col.get())
                    .expected("(")
                    .into())
            }
            _ => (),
        };

        // Check next character is open brace.
        match self.next_while(&mut chars, char::is_whitespace) {
            Some(c) if c != ')' => {
                return Err(Syntax::new(self.line.get(), self.col.get())
                    .expected(")")
                    .unexpected(c.encode_utf8(&mut [0u8; 4]))
                    .into())
            }
            None => {
                return Err(Syntax::new(self.line.get(), self.col.get())
                    .expected(")")
                    .into())
            }
            _ => (),
        };

        // Check no other characters.
        match self.next_while(&mut chars, char::is_whitespace) {
            Some(c) => {
                return Err(Syntax::new(self.line.get(), self.col.get())
                    .unexpected(c.encode_utf8(&mut [0u8; 4]))
                    .into())
            }
            None => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    /// Json::new() creates a Json as expected.
    ///
    /// Test that Json::new() creates a Json as expected.
    #[test]
    fn json_new() {
        let expected = Json {
            col: Cell::new(0),
            line: Cell::new(1),
            phantom: PhantomData,
        };
        let actual = Json::new();
        assert_eq!(expected, actual);
    }

    /// Json::visit_unit correctly deserialises a unit.
    ///
    /// Test that Json::visit_unit correctly deserialises a unit.
    #[test]
    fn json_visit_unit() {
        let expected = ();
        let actual = Json::new().visit_unit("()").unwrap();
        assert_eq!(expected, actual);

        let actual = Json::new().deserialise::<()>("()").unwrap();
        assert_eq!(expected, actual);
    }

    /// Json::visit_unit correctly deserialises a unit with whitespace.
    ///
    /// Test that Json::visit_unit correctly deserialises a unit with whitespace.
    #[test]
    fn json_visit_unit_whitespace() {
        let expected = ();
        let actual = Json::new().visit_unit(" ( \n ) ").unwrap();
        assert_eq!(expected, actual);

        let actual = Json::new().deserialise::<()>("( \n ) ").unwrap();
        assert_eq!(expected, actual);
    }

    /// Json::visit_unit error upon missing start parenthesis.
    ///
    /// Test that Json::visit_unit error upon missing start parenthesis.
    #[test]
    fn json_visit_unit_missing_start_parenthesis() {
        let expected: Error = Syntax::new(1, 1).expected("(").into();
        let actual = Json::new().visit_unit("").unwrap_err();
        assert_eq!(expected, actual);

        let actual = Json::new().deserialise::<()>("").unwrap_err();
        assert_eq!(expected, actual);
    }

    /// Json::visit_unit error upon unexpected start parenthesis.
    ///
    /// Test that Json::visit_unit error upon unexpected start parenthesis.
    #[test]
    fn json_visit_unit_unexpected_start_parenthesis() {
        let expected: Error = Syntax::new(1, 1).expected("(").unexpected("a").into();
        let actual = Json::new().visit_unit("a)").unwrap_err();
        assert_eq!(expected, actual);

        let actual = Json::new().deserialise::<()>("a)").unwrap_err();
        assert_eq!(expected, actual);
    }

    /// Json::visit_unit error upon missing end parenthesis.
    ///
    /// Test that Json::visit_unit error upon missing end parenthesis.
    #[test]
    fn json_visit_unit_missing_end_parenthesis() {
        let expected: Error = Syntax::new(1, 2).expected(")").into();
        let actual = Json::new().visit_unit("(").unwrap_err();
        assert_eq!(expected, actual);

        let actual = Json::new().deserialise::<()>("(").unwrap_err();
        assert_eq!(expected, actual);
    }

    /// Json::visit_unit error upon unexpected end parenthesis.
    ///
    /// Test that Json::visit_unit error upon unexpected end parenthesis.
    #[test]
    fn json_visit_unit_unexpected_end_parenthesis() {
        let expected: Error = Syntax::new(1, 2).expected(")").unexpected("a").into();
        let actual = Json::new().visit_unit("(a").unwrap_err();
        assert_eq!(expected, actual);

        let actual = Json::new().deserialise::<()>("(a").unwrap_err();
        assert_eq!(expected, actual);
    }

    /// Json::visit_unit error upon extra characters.
    ///
    /// Test that Json::visit_unit error upon extra characters.
    #[test]
    fn json_visit_unit_extra_characters() {
        let expected: Error = Syntax::new(1, 3).unexpected("a").into();
        let actual = Json::new().visit_unit("()a").unwrap_err();
        assert_eq!(expected, actual);

        let actual = Json::new().deserialise::<()>("()a").unwrap_err();
        assert_eq!(expected, actual);
    }
}

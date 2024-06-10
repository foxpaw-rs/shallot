//! Json module which houses the Json deserializer.

use crate::deserialize::{Deserialize, Deserializer};
use crate::error::{Error, Overflow, Result, Syntax};
use std::cell::Cell;
use std::marker::PhantomData;
use std::num::{IntErrorKind, ParseIntError};

/// Json deserializer which converts JSON strings into deserialize items.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Json<'a> {
    /// The current column number.
    col: Cell<usize>,

    /// The current row number.
    row: Cell<usize>,

    /// Phantomdata to hold the lifetime of the Input &str.
    phantom: PhantomData<&'a ()>,
}

impl<'a> Json<'a> {
    /// Create a new Json deserializer.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::Json;
    ///
    /// let json = Json::new();
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            col: Cell::new(1),
            row: Cell::new(1),
            phantom: PhantomData,
        }
    }

    /// Consume all the remaining tokens.
    fn consume_all(&self, input: &'a str) -> (&'a str, &'a str) {
        let parts = input.split('\n').collect::<Vec<_>>();
        if parts.len() > 1 {
            self.row.set(self.row.get() + parts.len() - 1);
            self.col.set(parts.last().get_or_insert(&"").len());
        } else {
            self.col
                .set(self.col.get() + parts.last().get_or_insert(&"").len());
        }
        (input, "")
    }

    /// Consume the expected value.
    fn consume_expected(&self, input: &'a str, expected: &'a str) -> Result<(&'a str, &'a str)> {
        let taken = self.take_expected(input, expected)?;
        self.consume_all(taken.0);
        Ok(taken)
    }

    /// consume from the input until the delimiter is reached, considering
    /// delimiters included within quotes.
    fn consume_until(&self, input: &'a str, until: char) -> Result<(&'a str, &'a str)> {
        let taken = self.take_until(input, until)?;
        self.consume_all(taken.0);
        Ok(taken)
    }

    /// Consume whitespace in the input string.
    fn consume_whitespace(&self, input: &'a str) -> (&'a str, &'a str) {
        let mut found = None;
        for (n, c) in input.chars().enumerate() {
            match c {
                '\n' => {
                    self.row.set(self.row.get() + 1);
                    self.col.set(1);
                }
                c if c.is_whitespace() => self.col.set(self.col.get() + 1),
                _ => {
                    found = Some(n);
                    break;
                }
            }
        }

        found.map_or((input, ""), |f| (&input[..f], &input[f..]))
    }

    /// Convert a float errors into library error types.
    fn convert_float_error(&self, input: &<Self as Deserializer>::Input, kind: &str) -> Error {
        self.syntax_error_number(input, kind)
    }

    /// Convert a integer errors into library error types.
    fn convert_int_error(
        &self,
        err: &ParseIntError,
        input: &<Self as Deserializer>::Input,
        kind: &str,
    ) -> Error {
        match err.kind() {
            IntErrorKind::Empty => Syntax::new(self.row.get(), self.col.get())
                .expected(kind)
                .into(),
            IntErrorKind::InvalidDigit => self.syntax_error_number(input, kind),
            IntErrorKind::PosOverflow | IntErrorKind::NegOverflow => {
                Overflow::new(self.row.get(), self.col.get())
                    .kind(kind)
                    .into()
            }
            _ => Syntax::new(self.row.get(), self.col.get()).into(),
        }
    }

    /// Decode a string, taking into consideration escaped characters.
    fn decode_string(&self, input: &<Self as Deserializer>::Input) -> Result<String> {
        let (_, stripped) = self.take_expected(input, "\"")?;
        let (result, _) = self.take_until(stripped, '\"')?;
        Ok(result.replace("\\\"", "\"").replace("\\\\", "\\"))
    }

    /// Create a syntax error for numeric types.
    fn syntax_error_number(&self, input: &str, kind: &str) -> Error {
        let mut first = true;
        let mut dot = false;
        for c in input.chars() {
            match c {
                '0'..='9' => self.col.set(self.col.get() + 1),
                '-' if first && !kind.starts_with('u') => self.col.set(self.col.get() + 1),
                '.' if !dot && kind.starts_with('f') => {
                    self.col.set(self.col.get() + 1);
                    dot = true;
                }
                c if c.is_whitespace() => {
                    return Syntax::new(self.row.get(), self.col.get())
                        .unexpected(c.encode_utf8(&mut [0_u8; 4]))
                        .into()
                }
                _ => {
                    return Syntax::new(self.row.get(), self.col.get())
                        .unexpected(&c.to_string())
                        .into()
                }
            }

            if first {
                first = false;
            }
        }

        Syntax::new(self.row.get(), self.col.get())
            .expected(kind)
            .into()
    }

    /// Take an expected string.
    fn take_expected(&self, input: &'a str, expected: &'a str) -> Result<(&'a str, &'a str)> {
        Ok((
            expected,
            input.strip_prefix(expected).ok_or_else(|| {
                let e: Error = match input.chars().next() {
                    Some(f) => Syntax::new(self.row.get(), self.col.get())
                        .unexpected(f.encode_utf8(&mut [0_u8; 4]))
                        .expected(expected)
                        .into(),
                    None => Syntax::new(self.row.get(), self.col.get())
                        .expected(expected)
                        .into(),
                };
                e
            })?,
        ))
    }

    /// Take from the input until the delimiter is reached, considering
    /// delimiters included within quotes.
    fn take_until(&self, input: &'a str, until: char) -> Result<(&'a str, &'a str)> {
        let mut quote = false;
        let mut backslash = false;
        let mut found = None;
        for (n, c) in input.chars().enumerate() {
            match c {
                c if !(quote || (until == '\"' && backslash)) && c == until => {
                    found = Some(n);
                    break;
                }
                '\"' if !backslash => quote = !quote,
                '\\' if !backslash => {
                    backslash = true;
                    continue;
                }
                _ => (),
            }
            backslash = false;
        }

        found.map(|n| (&input[..n], &input[n..])).ok_or_else(|| {
            self.consume_all(input);
            let e: Error = match input.chars().last() {
                Some(f) => Syntax::new(self.row.get(), self.col.get())
                    .unexpected(f.encode_utf8(&mut [0_u8; 4]))
                    .expected(until.encode_utf8(&mut [0_u8; 4]))
                    .into(),
                None => Syntax::new(self.row.get(), self.col.get())
                    .expected(until.encode_utf8(&mut [0_u8; 4]))
                    .into(),
            };
            e
        })
    }
}

impl<'a> Default for Json<'a> {
    /// Create a new default Json deserializer.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::Json;
    ///
    /// let json = Json::default();
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> Deserializer for Json<'a> {
    /// The input type for this Deserializer.
    type Input = &'a str;

    /// Deserialize the input into the required output type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: () = json.deserialize(&"null")?;
    ///     Ok(())
    /// }
    /// ```
    fn deserialize<S>(&self, input: &Self::Input) -> Result<S>
    where
        S: Deserialize,
    {
        S::accept(self, input)
    }

    /// Visit and deserialize a bool type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: bool = json.deserialize(&"true")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_bool(&self, input: &Self::Input) -> Result<bool> {
        let (_, trim) = self.consume_whitespace(input);
        let result = match trim.trim() {
            "true" => true,
            "false" => false,
            _ => {
                return Err(Syntax::new(self.row.get(), self.col.get())
                    .unexpected(input)
                    .expected("bool")
                    .into())
            }
        };
        self.consume_all(trim);
        Ok(result)
    }

    /// Visit and deserialize a char type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: char = json.deserialize(&"\"a\"")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_char(&self, input: &Self::Input) -> Result<char> {
        let (_, trim) = self.consume_whitespace(input);
        let string = self.decode_string(&trim.trim())?;

        let result = if string.len() > 1 {
            let e: Error = Overflow::new(self.row.get(), self.col.get())
                .kind("char")
                .into();
            Err(e)
        } else {
            string.chars().next().ok_or_else(|| {
                Syntax::new(self.row.get(), self.col.get() + 1)
                    .unexpected("\"")
                    .into()
            })
        }?;

        let (_, remainder) = self.consume_expected(trim, "\"")?;
        let (_, remainder) = self.consume_until(remainder, '\"')?;
        let (_, remainder) = self.consume_expected(remainder, "\"")?;
        let (_, remainder) = self.consume_whitespace(remainder);
        if let Some(c) = remainder.chars().next() {
            Err(Syntax::new(self.row.get(), self.col.get())
                .unexpected(c.encode_utf8(&mut [0_u8; 4]))
                .into())
        } else {
            Ok(result)
        }
    }

    /// Visit and deserialize an f32 type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: f32 = json.deserialize(&"1")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_f32(&self, input: &Self::Input) -> Result<f32> {
        let (_, trim) = self.consume_whitespace(input);
        let result = trim
            .trim()
            .parse::<f32>()
            .map_err(|_| self.convert_float_error(&trim.trim(), "f32"))?;

        if !result.is_finite() {
            return Err(Overflow::new(self.row.get(), self.col.get())
                .kind("f32")
                .into());
        }

        self.consume_all(trim);
        Ok(result)
    }

    /// Visit and deserialize an f64 type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: f64 = json.deserialize(&"1")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_f64(&self, input: &Self::Input) -> Result<f64> {
        let (_, trim) = self.consume_whitespace(input);
        let result = trim
            .trim()
            .parse::<f64>()
            .map_err(|_| self.convert_float_error(&trim.trim(), "f64"))?;

        if !result.is_finite() {
            return Err(Overflow::new(self.row.get(), self.col.get())
                .kind("f64")
                .into());
        }

        self.consume_all(trim);
        Ok(result)
    }

    /// Visit and deserialize an i8 type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: i8 = json.deserialize(&"1")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_i8(&self, input: &Self::Input) -> Result<i8> {
        let (_, trim) = self.consume_whitespace(input);
        let result = trim
            .trim()
            .parse::<i8>()
            .map_err(|err| self.convert_int_error(&err, &trim.trim(), "i8"))?;
        self.consume_all(trim);
        Ok(result)
    }

    /// Visit and deserialize an i16 type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: i16 = json.deserialize(&"1")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_i16(&self, input: &Self::Input) -> Result<i16> {
        let (_, trim) = self.consume_whitespace(input);
        let result = trim
            .trim()
            .parse::<i16>()
            .map_err(|err| self.convert_int_error(&err, &trim.trim(), "i16"))?;
        self.consume_all(trim);
        Ok(result)
    }

    /// Visit and deserialize an i32 type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: i32 = json.deserialize(&"1")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_i32(&self, input: &Self::Input) -> Result<i32> {
        let (_, trim) = self.consume_whitespace(input);
        let result = trim
            .trim()
            .parse::<i32>()
            .map_err(|err| self.convert_int_error(&err, &trim.trim(), "i32"))?;
        self.consume_all(trim);
        Ok(result)
    }

    /// Visit and deserialize an i64 type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: i64 = json.deserialize(&"1")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_i64(&self, input: &Self::Input) -> Result<i64> {
        let (_, trim) = self.consume_whitespace(input);
        let result = trim
            .trim()
            .parse::<i64>()
            .map_err(|err| self.convert_int_error(&err, &trim.trim(), "i64"))?;
        self.consume_all(trim);
        Ok(result)
    }

    /// Visit and deserialize an i128 type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: i128 = json.deserialize(&"1")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_i128(&self, input: &Self::Input) -> Result<i128> {
        let (_, trim) = self.consume_whitespace(input);
        let result = trim
            .trim()
            .parse::<i128>()
            .map_err(|err| self.convert_int_error(&err, &trim.trim(), "i128"))?;
        self.consume_all(trim);
        Ok(result)
    }

    /// Visit and deserialize an isize type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: isize = json.deserialize(&"1")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_isize(&self, input: &Self::Input) -> Result<isize> {
        let (_, trim) = self.consume_whitespace(input);
        let result = trim
            .trim()
            .parse::<isize>()
            .map_err(|err| self.convert_int_error(&err, &trim.trim(), "isize"))?;
        self.consume_all(trim);
        Ok(result)
    }

    /// Visit and deserialize a String type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: String = json.deserialize(&"\"abc\"")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_string(&self, input: &Self::Input) -> Result<String> {
        let (_, trim) = self.consume_whitespace(input);
        let result = self.decode_string(&trim.trim())?;

        let (_, remainder) = self.consume_expected(trim, "\"")?;
        let (_, remainder) = self.consume_until(remainder, '\"')?;
        let (_, remainder) = self.consume_expected(remainder, "\"")?;
        let (_, remainder) = self.consume_whitespace(remainder);
        if let Some(c) = remainder.chars().next() {
            Err(Syntax::new(self.row.get(), self.col.get())
                .unexpected(c.encode_utf8(&mut [0_u8; 4]))
                .into())
        } else {
            Ok(result)
        }
    }

    /// Visit and deserialize a tuple type of size 1.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: (u8,) = json.deserialize(&"[1]")?;
    ///     Ok(())
    /// }
    /// ```
    #[allow(clippy::many_single_char_names)]
    fn visit_tuple_1<A>(&self, input: &Self::Input) -> Result<(A,)>
    where
        A: Deserialize,
    {
        let (_, trim) = self.consume_whitespace(input);
        let (_, trim) = self.consume_expected(trim, "[")?;

        let (a, remainder) = self.take_until(trim, ']')?;
        let a = self.deserialize::<A>(&a)?;
        let (_, remainder) = self.consume_expected(remainder, "]")?;

        let result = (a,);

        let (_, remainder) = self.consume_whitespace(remainder);
        if let Some(c) = remainder.chars().next() {
            Err(Syntax::new(self.row.get(), self.col.get())
                .unexpected(c.encode_utf8(&mut [0_u8; 4]))
                .into())
        } else {
            Ok(result)
        }
    }

    /// Visit and deserialize a tuple type of size 2.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: (u8, u8) = json.deserialize(&"[1, 2]")?;
    ///     Ok(())
    /// }
    /// ```
    #[allow(clippy::many_single_char_names)]
    fn visit_tuple_2<A, B>(&self, input: &Self::Input) -> Result<(A, B)>
    where
        A: Deserialize,
        B: Deserialize,
    {
        let (_, trim) = self.consume_whitespace(input);
        let (_, trim) = self.consume_expected(trim, "[")?;

        let (a, remainder) = self.take_until(trim, ',')?;
        let a = self.deserialize::<A>(&a)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (b, remainder) = self.take_until(remainder, ']')?;
        let b = self.deserialize::<B>(&b)?;
        let (_, remainder) = self.consume_expected(remainder, "]")?;

        let result = (a, b);

        let (_, remainder) = self.consume_whitespace(remainder);
        if let Some(c) = remainder.chars().next() {
            Err(Syntax::new(self.row.get(), self.col.get())
                .unexpected(c.encode_utf8(&mut [0_u8; 4]))
                .into())
        } else {
            Ok(result)
        }
    }

    /// Visit and deserialize a tuple type of size 3.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: (u8, u8, u8) = json.deserialize(&"[1, 2, 3]")?;
    ///     Ok(())
    /// }
    /// ```
    #[allow(clippy::many_single_char_names)]
    fn visit_tuple_3<A, B, C>(&self, input: &Self::Input) -> Result<(A, B, C)>
    where
        A: Deserialize,
        B: Deserialize,
        C: Deserialize,
    {
        let (_, trim) = self.consume_whitespace(input);
        let (_, trim) = self.consume_expected(trim, "[")?;

        let (a, remainder) = self.take_until(trim, ',')?;
        let a = self.deserialize::<A>(&a)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (b, remainder) = self.take_until(remainder, ',')?;
        let b = self.deserialize::<B>(&b)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (c, remainder) = self.take_until(remainder, ']')?;
        let c = self.deserialize::<C>(&c)?;
        let (_, remainder) = self.consume_expected(remainder, "]")?;

        let result = (a, b, c);

        let (_, remainder) = self.consume_whitespace(remainder);
        if let Some(c) = remainder.chars().next() {
            Err(Syntax::new(self.row.get(), self.col.get())
                .unexpected(c.encode_utf8(&mut [0_u8; 4]))
                .into())
        } else {
            Ok(result)
        }
    }

    /// Visit and deserialize a tuple type of size 4.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: (u8, u8, u8, u8) = json.deserialize(&"[1, 2, 3, 4]")?;
    ///     Ok(())
    /// }
    /// ```
    #[allow(clippy::many_single_char_names)]
    fn visit_tuple_4<A, B, C, D>(&self, input: &Self::Input) -> Result<(A, B, C, D)>
    where
        A: Deserialize,
        B: Deserialize,
        C: Deserialize,
        D: Deserialize,
    {
        let (_, trim) = self.consume_whitespace(input);
        let (_, trim) = self.consume_expected(trim, "[")?;

        let (a, remainder) = self.take_until(trim, ',')?;
        let a = self.deserialize::<A>(&a)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (b, remainder) = self.take_until(remainder, ',')?;
        let b = self.deserialize::<B>(&b)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (c, remainder) = self.take_until(remainder, ',')?;
        let c = self.deserialize::<C>(&c)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (d, remainder) = self.take_until(remainder, ']')?;
        let d = self.deserialize::<D>(&d)?;
        let (_, remainder) = self.consume_expected(remainder, "]")?;

        let result = (a, b, c, d);

        let (_, remainder) = self.consume_whitespace(remainder);
        if let Some(c) = remainder.chars().next() {
            Err(Syntax::new(self.row.get(), self.col.get())
                .unexpected(c.encode_utf8(&mut [0_u8; 4]))
                .into())
        } else {
            Ok(result)
        }
    }

    /// Visit and deserialize a tuple type of size 5.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: (u8, u8, u8, u8, u8) = json.deserialize(&"[
    ///         1, 2, 3, 4, 5
    ///     ]")?;
    ///     Ok(())
    /// }
    /// ```
    #[allow(clippy::many_single_char_names)]
    fn visit_tuple_5<A, B, C, D, E>(&self, input: &Self::Input) -> Result<(A, B, C, D, E)>
    where
        A: Deserialize,
        B: Deserialize,
        C: Deserialize,
        D: Deserialize,
        E: Deserialize,
    {
        let (_, trim) = self.consume_whitespace(input);
        let (_, trim) = self.consume_expected(trim, "[")?;

        let (a, remainder) = self.take_until(trim, ',')?;
        let a = self.deserialize::<A>(&a)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (b, remainder) = self.take_until(remainder, ',')?;
        let b = self.deserialize::<B>(&b)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (c, remainder) = self.take_until(remainder, ',')?;
        let c = self.deserialize::<C>(&c)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (d, remainder) = self.take_until(remainder, ',')?;
        let d = self.deserialize::<D>(&d)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (e, remainder) = self.take_until(remainder, ']')?;
        let e = self.deserialize::<E>(&e)?;
        let (_, remainder) = self.consume_expected(remainder, "]")?;

        let result = (a, b, c, d, e);

        let (_, remainder) = self.consume_whitespace(remainder);
        if let Some(c) = remainder.chars().next() {
            Err(Syntax::new(self.row.get(), self.col.get())
                .unexpected(c.encode_utf8(&mut [0_u8; 4]))
                .into())
        } else {
            Ok(result)
        }
    }

    /// Visit and deserialize a tuple type of size 6.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: (u8, u8, u8, u8, u8, u8) = json.deserialize(&"[
    ///         1, 2, 3, 4, 5, 6
    ///     ]")?;
    ///     Ok(())
    /// }
    /// ```
    #[allow(clippy::many_single_char_names)]
    fn visit_tuple_6<A, B, C, D, E, F>(&self, input: &Self::Input) -> Result<(A, B, C, D, E, F)>
    where
        A: Deserialize,
        B: Deserialize,
        C: Deserialize,
        D: Deserialize,
        E: Deserialize,
        F: Deserialize,
    {
        let (_, trim) = self.consume_whitespace(input);
        let (_, trim) = self.consume_expected(trim, "[")?;

        let (a, remainder) = self.take_until(trim, ',')?;
        let a = self.deserialize::<A>(&a)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (b, remainder) = self.take_until(remainder, ',')?;
        let b = self.deserialize::<B>(&b)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (c, remainder) = self.take_until(remainder, ',')?;
        let c = self.deserialize::<C>(&c)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (d, remainder) = self.take_until(remainder, ',')?;
        let d = self.deserialize::<D>(&d)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (e, remainder) = self.take_until(remainder, ',')?;
        let e = self.deserialize::<E>(&e)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (f, remainder) = self.take_until(remainder, ']')?;
        let f = self.deserialize::<F>(&f)?;
        let (_, remainder) = self.consume_expected(remainder, "]")?;

        let result = (a, b, c, d, e, f);

        let (_, remainder) = self.consume_whitespace(remainder);
        if let Some(c) = remainder.chars().next() {
            Err(Syntax::new(self.row.get(), self.col.get())
                .unexpected(c.encode_utf8(&mut [0_u8; 4]))
                .into())
        } else {
            Ok(result)
        }
    }

    /// Visit and deserialize a tuple type of size 7.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: (u8, u8, u8, u8, u8, u8, u8) = json.deserialize(&"[
    ///         1, 2, 3, 4, 5, 6, 7
    ///     ]")?;
    ///     Ok(())
    /// }
    /// ```
    #[allow(clippy::many_single_char_names)]
    fn visit_tuple_7<A, B, C, D, E, F, G>(
        &self,
        input: &Self::Input,
    ) -> Result<(A, B, C, D, E, F, G)>
    where
        A: Deserialize,
        B: Deserialize,
        C: Deserialize,
        D: Deserialize,
        E: Deserialize,
        F: Deserialize,
        G: Deserialize,
    {
        let (_, trim) = self.consume_whitespace(input);
        let (_, trim) = self.consume_expected(trim, "[")?;

        let (a, remainder) = self.take_until(trim, ',')?;
        let a = self.deserialize::<A>(&a)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (b, remainder) = self.take_until(remainder, ',')?;
        let b = self.deserialize::<B>(&b)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (c, remainder) = self.take_until(remainder, ',')?;
        let c = self.deserialize::<C>(&c)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (d, remainder) = self.take_until(remainder, ',')?;
        let d = self.deserialize::<D>(&d)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (e, remainder) = self.take_until(remainder, ',')?;
        let e = self.deserialize::<E>(&e)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (f, remainder) = self.take_until(remainder, ',')?;
        let f = self.deserialize::<F>(&f)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (g, remainder) = self.take_until(remainder, ']')?;
        let g = self.deserialize::<G>(&g)?;
        let (_, remainder) = self.consume_expected(remainder, "]")?;

        let result = (a, b, c, d, e, f, g);

        let (_, remainder) = self.consume_whitespace(remainder);
        if let Some(c) = remainder.chars().next() {
            Err(Syntax::new(self.row.get(), self.col.get())
                .unexpected(c.encode_utf8(&mut [0_u8; 4]))
                .into())
        } else {
            Ok(result)
        }
    }

    /// Visit and deserialize a tuple type of size 8.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: (u8, u8, u8, u8, u8, u8, u8, u8) = json.deserialize(&"[
    ///         1, 2, 3, 4, 5, 6, 7, 8
    ///     ]")?;
    ///     Ok(())
    /// }
    /// ```
    #[allow(clippy::many_single_char_names)]
    fn visit_tuple_8<A, B, C, D, E, F, G, H>(
        &self,
        input: &Self::Input,
    ) -> Result<(A, B, C, D, E, F, G, H)>
    where
        A: Deserialize,
        B: Deserialize,
        C: Deserialize,
        D: Deserialize,
        E: Deserialize,
        F: Deserialize,
        G: Deserialize,
        H: Deserialize,
    {
        let (_, trim) = self.consume_whitespace(input);
        let (_, trim) = self.consume_expected(trim, "[")?;

        let (a, remainder) = self.take_until(trim, ',')?;
        let a = self.deserialize::<A>(&a)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (b, remainder) = self.take_until(remainder, ',')?;
        let b = self.deserialize::<B>(&b)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (c, remainder) = self.take_until(remainder, ',')?;
        let c = self.deserialize::<C>(&c)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (d, remainder) = self.take_until(remainder, ',')?;
        let d = self.deserialize::<D>(&d)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (e, remainder) = self.take_until(remainder, ',')?;
        let e = self.deserialize::<E>(&e)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (f, remainder) = self.take_until(remainder, ',')?;
        let f = self.deserialize::<F>(&f)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (g, remainder) = self.take_until(remainder, ',')?;
        let g = self.deserialize::<G>(&g)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (h, remainder) = self.take_until(remainder, ']')?;
        let h = self.deserialize::<H>(&h)?;
        let (_, remainder) = self.consume_expected(remainder, "]")?;

        let result = (a, b, c, d, e, f, g, h);

        let (_, remainder) = self.consume_whitespace(remainder);
        if let Some(c) = remainder.chars().next() {
            Err(Syntax::new(self.row.get(), self.col.get())
                .unexpected(c.encode_utf8(&mut [0_u8; 4]))
                .into())
        } else {
            Ok(result)
        }
    }

    /// Visit and deserialize a tuple type of size 9.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: (u8, u8, u8, u8, u8, u8, u8, u8, u8) = json.deserialize(&"[
    ///         1, 2, 3, 4, 5, 6, 7, 8, 9
    ///     ]")?;
    ///     Ok(())
    /// }
    /// ```
    #[allow(clippy::many_single_char_names)]
    fn visit_tuple_9<A, B, C, D, E, F, G, H, I>(
        &self,
        input: &Self::Input,
    ) -> Result<(A, B, C, D, E, F, G, H, I)>
    where
        A: Deserialize,
        B: Deserialize,
        C: Deserialize,
        D: Deserialize,
        E: Deserialize,
        F: Deserialize,
        G: Deserialize,
        H: Deserialize,
        I: Deserialize,
    {
        let (_, trim) = self.consume_whitespace(input);
        let (_, trim) = self.consume_expected(trim, "[")?;

        let (a, remainder) = self.take_until(trim, ',')?;
        let a = self.deserialize::<A>(&a)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (b, remainder) = self.take_until(remainder, ',')?;
        let b = self.deserialize::<B>(&b)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (c, remainder) = self.take_until(remainder, ',')?;
        let c = self.deserialize::<C>(&c)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (d, remainder) = self.take_until(remainder, ',')?;
        let d = self.deserialize::<D>(&d)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (e, remainder) = self.take_until(remainder, ',')?;
        let e = self.deserialize::<E>(&e)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (f, remainder) = self.take_until(remainder, ',')?;
        let f = self.deserialize::<F>(&f)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (g, remainder) = self.take_until(remainder, ',')?;
        let g = self.deserialize::<G>(&g)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (h, remainder) = self.take_until(remainder, ',')?;
        let h = self.deserialize::<H>(&h)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (i, remainder) = self.take_until(remainder, ']')?;
        let i = self.deserialize::<I>(&i)?;
        let (_, remainder) = self.consume_expected(remainder, "]")?;

        let result = (a, b, c, d, e, f, g, h, i);

        let (_, remainder) = self.consume_whitespace(remainder);
        if let Some(c) = remainder.chars().next() {
            Err(Syntax::new(self.row.get(), self.col.get())
                .unexpected(c.encode_utf8(&mut [0_u8; 4]))
                .into())
        } else {
            Ok(result)
        }
    }

    /// Visit and deserialize a tuple type of size 10.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) =
    ///         json.deserialize(&"[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]")?;
    ///     Ok(())
    /// }
    /// ```
    #[allow(clippy::many_single_char_names)]
    fn visit_tuple_10<A, B, C, D, E, F, G, H, I, J>(
        &self,
        input: &Self::Input,
    ) -> Result<(A, B, C, D, E, F, G, H, I, J)>
    where
        A: Deserialize,
        B: Deserialize,
        C: Deserialize,
        D: Deserialize,
        E: Deserialize,
        F: Deserialize,
        G: Deserialize,
        H: Deserialize,
        I: Deserialize,
        J: Deserialize,
    {
        let (_, trim) = self.consume_whitespace(input);
        let (_, trim) = self.consume_expected(trim, "[")?;

        let (a, remainder) = self.take_until(trim, ',')?;
        let a = self.deserialize::<A>(&a)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (b, remainder) = self.take_until(remainder, ',')?;
        let b = self.deserialize::<B>(&b)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (c, remainder) = self.take_until(remainder, ',')?;
        let c = self.deserialize::<C>(&c)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (d, remainder) = self.take_until(remainder, ',')?;
        let d = self.deserialize::<D>(&d)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (e, remainder) = self.take_until(remainder, ',')?;
        let e = self.deserialize::<E>(&e)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (f, remainder) = self.take_until(remainder, ',')?;
        let f = self.deserialize::<F>(&f)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (g, remainder) = self.take_until(remainder, ',')?;
        let g = self.deserialize::<G>(&g)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (h, remainder) = self.take_until(remainder, ',')?;
        let h = self.deserialize::<H>(&h)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (i, remainder) = self.take_until(remainder, ',')?;
        let i = self.deserialize::<I>(&i)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (j, remainder) = self.take_until(remainder, ']')?;
        let j = self.deserialize::<J>(&j)?;
        let (_, remainder) = self.consume_expected(remainder, "]")?;

        let result = (a, b, c, d, e, f, g, h, i, j);

        let (_, remainder) = self.consume_whitespace(remainder);
        if let Some(c) = remainder.chars().next() {
            Err(Syntax::new(self.row.get(), self.col.get())
                .unexpected(c.encode_utf8(&mut [0_u8; 4]))
                .into())
        } else {
            Ok(result)
        }
    }

    /// Visit and deserialize a tuple type of size 11.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) =
    ///         json.deserialize(&"[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]")?;
    ///     Ok(())
    /// }
    /// ```
    #[allow(clippy::many_single_char_names)]
    fn visit_tuple_11<A, B, C, D, E, F, G, H, I, J, K>(
        &self,
        input: &Self::Input,
    ) -> Result<(A, B, C, D, E, F, G, H, I, J, K)>
    where
        A: Deserialize,
        B: Deserialize,
        C: Deserialize,
        D: Deserialize,
        E: Deserialize,
        F: Deserialize,
        G: Deserialize,
        H: Deserialize,
        I: Deserialize,
        J: Deserialize,
        K: Deserialize,
    {
        let (_, trim) = self.consume_whitespace(input);
        let (_, trim) = self.consume_expected(trim, "[")?;

        let (a, remainder) = self.take_until(trim, ',')?;
        let a = self.deserialize::<A>(&a)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (b, remainder) = self.take_until(remainder, ',')?;
        let b = self.deserialize::<B>(&b)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (c, remainder) = self.take_until(remainder, ',')?;
        let c = self.deserialize::<C>(&c)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (d, remainder) = self.take_until(remainder, ',')?;
        let d = self.deserialize::<D>(&d)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (e, remainder) = self.take_until(remainder, ',')?;
        let e = self.deserialize::<E>(&e)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (f, remainder) = self.take_until(remainder, ',')?;
        let f = self.deserialize::<F>(&f)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (g, remainder) = self.take_until(remainder, ',')?;
        let g = self.deserialize::<G>(&g)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (h, remainder) = self.take_until(remainder, ',')?;
        let h = self.deserialize::<H>(&h)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (i, remainder) = self.take_until(remainder, ',')?;
        let i = self.deserialize::<I>(&i)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (j, remainder) = self.take_until(remainder, ',')?;
        let j = self.deserialize::<J>(&j)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (k, remainder) = self.take_until(remainder, ']')?;
        let k = self.deserialize::<K>(&k)?;
        let (_, remainder) = self.consume_expected(remainder, "]")?;

        let result = (a, b, c, d, e, f, g, h, i, j, k);

        let (_, remainder) = self.consume_whitespace(remainder);
        if let Some(c) = remainder.chars().next() {
            Err(Syntax::new(self.row.get(), self.col.get())
                .unexpected(c.encode_utf8(&mut [0_u8; 4]))
                .into())
        } else {
            Ok(result)
        }
    }

    /// Visit and deserialize a tuple type of size 12.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: (u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8) =
    ///         json.deserialize(&"[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]")?;
    ///     Ok(())
    /// }
    /// ```
    #[allow(clippy::many_single_char_names)]
    fn visit_tuple_12<A, B, C, D, E, F, G, H, I, J, K, L>(
        &self,
        input: &Self::Input,
    ) -> Result<(A, B, C, D, E, F, G, H, I, J, K, L)>
    where
        A: Deserialize,
        B: Deserialize,
        C: Deserialize,
        D: Deserialize,
        E: Deserialize,
        F: Deserialize,
        G: Deserialize,
        H: Deserialize,
        I: Deserialize,
        J: Deserialize,
        K: Deserialize,
        L: Deserialize,
    {
        let (_, trim) = self.consume_whitespace(input);
        let (_, trim) = self.consume_expected(trim, "[")?;

        let (a, remainder) = self.take_until(trim, ',')?;
        let a = self.deserialize::<A>(&a)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (b, remainder) = self.take_until(remainder, ',')?;
        let b = self.deserialize::<B>(&b)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (c, remainder) = self.take_until(remainder, ',')?;
        let c = self.deserialize::<C>(&c)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (d, remainder) = self.take_until(remainder, ',')?;
        let d = self.deserialize::<D>(&d)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (e, remainder) = self.take_until(remainder, ',')?;
        let e = self.deserialize::<E>(&e)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (f, remainder) = self.take_until(remainder, ',')?;
        let f = self.deserialize::<F>(&f)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (g, remainder) = self.take_until(remainder, ',')?;
        let g = self.deserialize::<G>(&g)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (h, remainder) = self.take_until(remainder, ',')?;
        let h = self.deserialize::<H>(&h)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (i, remainder) = self.take_until(remainder, ',')?;
        let i = self.deserialize::<I>(&i)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (j, remainder) = self.take_until(remainder, ',')?;
        let j = self.deserialize::<J>(&j)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (k, remainder) = self.take_until(remainder, ',')?;
        let k = self.deserialize::<K>(&k)?;
        let (_, remainder) = self.consume_expected(remainder, ",")?;

        let (l, remainder) = self.take_until(remainder, ']')?;
        let l = self.deserialize::<L>(&l)?;
        let (_, remainder) = self.consume_expected(remainder, "]")?;

        let result = (a, b, c, d, e, f, g, h, i, j, k, l);

        let (_, remainder) = self.consume_whitespace(remainder);
        if let Some(c) = remainder.chars().next() {
            Err(Syntax::new(self.row.get(), self.col.get())
                .unexpected(c.encode_utf8(&mut [0_u8; 4]))
                .into())
        } else {
            Ok(result)
        }
    }

    /// Visit and deserialize an u8 type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///z
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: u8 = json.deserialize(&"1")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_u8(&self, input: &Self::Input) -> Result<u8> {
        let (_, trim) = self.consume_whitespace(input);

        let result = trim
            .trim()
            .parse::<u8>()
            .map_err(|err| self.convert_int_error(&err, &trim.trim(), "u8"))?;
        self.consume_all(trim);
        Ok(result)
    }

    /// Visit and deserialize an u16 type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: u16 = json.deserialize(&"1")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_u16(&self, input: &Self::Input) -> Result<u16> {
        let (_, trim) = self.consume_whitespace(input);

        let result = trim
            .trim()
            .parse::<u16>()
            .map_err(|err| self.convert_int_error(&err, &trim.trim(), "u16"))?;
        self.consume_all(trim);
        Ok(result)
    }

    /// Visit and deserialize an u32 type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: u32 = json.deserialize(&"1")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_u32(&self, input: &Self::Input) -> Result<u32> {
        let (_, trim) = self.consume_whitespace(input);

        let result = trim
            .trim()
            .parse::<u32>()
            .map_err(|err| self.convert_int_error(&err, &trim.trim(), "u32"))?;
        self.consume_all(trim);
        Ok(result)
    }

    /// Visit and deserialize an u64 type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: u64 = json.deserialize(&"1")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_u64(&self, input: &Self::Input) -> Result<u64> {
        let (_, trim) = self.consume_whitespace(input);

        let result = trim
            .trim()
            .parse::<u64>()
            .map_err(|err| self.convert_int_error(&err, &trim.trim(), "u64"))?;
        self.consume_all(trim);
        Ok(result)
    }

    /// Visit and deserialize an u128 type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: u128 = json.deserialize(&"1")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_u128(&self, input: &Self::Input) -> Result<u128> {
        let (_, trim) = self.consume_whitespace(input);

        let result = trim
            .trim()
            .parse::<u128>()
            .map_err(|err| self.convert_int_error(&err, &trim.trim(), "u128"))?;
        self.consume_all(trim);
        Ok(result)
    }

    /// Visit and deserialize a unit type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: () = json.deserialize(&"null")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_unit(&self, input: &Self::Input) -> Result<()> {
        let (_, trim) = self.consume_whitespace(input);

        if trim.trim() != "null" {
            return Err(Syntax::new(self.row.get(), self.col.get())
                .unexpected(input)
                .expected("null")
                .into());
        }
        self.consume_all(trim);
        Ok(())
    }

    /// Visit and deserialize an usize type.
    ///
    /// # Errors
    /// Will error if the provided input does not deserialize to the correct item.
    ///
    /// # Examples
    /// ```rust
    /// use shallot::error::Result;
    /// use shallot::deserialize::{Deserializer, Json};
    ///
    /// fn main() -> Result<()> {
    ///     let json = Json::new();
    ///     let output: usize = json.deserialize(&"1")?;
    ///     Ok(())
    /// }
    /// ```
    fn visit_usize(&self, input: &Self::Input) -> Result<usize> {
        let (_, trim) = self.consume_whitespace(input);

        let result = trim
            .trim()
            .parse::<usize>()
            .map_err(|err| self.convert_int_error(&err, &trim.trim(), "usize"))?;
        self.consume_all(trim);
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test Json::new creates a Json as expected.
    #[test]
    fn new_correct() {
        let expected = Json {
            col: Cell::new(1),
            row: Cell::new(1),
            phantom: PhantomData,
        };
        let actual = Json::new();
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_bool correctly deserializes a true bool type.
    #[test]
    fn visit_bool_true() {
        let expected = Ok(true);
        let actual = Json::new().deserialize(&"true");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_bool correctly deserializes a false bool type.
    #[test]
    fn visit_bool_false() {
        let expected = Ok(false);
        let actual = Json::new().deserialize(&"false");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_bool correctly deserializes with whitespace.
    #[test]
    fn visit_bool_whitespace() {
        let expected = Ok(false);
        let actual = Json::new().deserialize(&" \nfalse  ");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_bool correctly errors upon unexpected value.
    #[test]
    fn visit_bool_incorrect() {
        let expected: Result<bool> =
            Err(Syntax::new(1, 1).unexpected("fail").expected("bool").into());
        let actual = Json::new().deserialize(&"fail");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_char correctly deserializes a char type.
    #[test]
    fn visit_char_correct() {
        let expected = Ok('a');
        let actual = Json::new().deserialize(&"\"a\"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_char correctly deserializes a escaped backslash.
    #[test]
    fn visit_char_escape_backslash() {
        let expected = Ok('\\');
        let actual = Json::new().deserialize(&"\"\\\\\"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_char correctly deserializes a escaped quote.
    #[test]
    fn visit_char_escape_quote() {
        let expected = Ok('\"');
        let actual = Json::new().deserialize(&"\"\\\"\"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_char correctly deserializes with whitespace.
    #[test]
    fn visit_char_whitespace() {
        let expected = Ok('a');
        let actual = Json::new().deserialize(&"  \n\"a\"  ");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_char correctly errors when empty.
    #[test]
    fn visit_char_empty() {
        let expected: Result<char> = Err(Syntax::new(1, 1).expected("\"").into());
        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_char correctly errors when provided nothing.
    #[test]
    fn visit_char_nothing() {
        let expected: Result<char> = Err(Syntax::new(1, 2).unexpected("\"").into());
        let actual = Json::new().deserialize(&"\"\"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_char correctly errors on overflow.
    #[test]
    fn visit_char_overflow() {
        let expected: Result<char> = Err(Overflow::new(1, 1).kind("char").into());
        let actual = Json::new().deserialize(&"\"ab\"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_char correctly errors on missing leading quote.
    #[test]
    fn visit_char_missing_leading_quote() {
        let expected: Result<char> = Err(Syntax::new(1, 1).unexpected("a").expected("\"").into());
        let actual = Json::new().deserialize(&"a\"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_char correctly errors on missing trailing quote.
    #[test]
    fn visit_char_missing_trailing_quote() {
        let expected: Result<char> = Err(Syntax::new(1, 2).unexpected("a").expected("\"").into());
        let actual = Json::new().deserialize(&"\"a");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_char correctly errors on replaced trailing quote.
    #[test]
    fn visit_char_replaced_trailing_quote() {
        let expected: Result<char> = Err(Syntax::new(1, 3).unexpected("b").expected("\"").into());
        let actual = Json::new().deserialize(&"\"ab");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_char correctly errors on trailing chars.
    #[test]
    fn visit_char_trailing_chars() {
        let expected: Result<char> = Err(Syntax::new(1, 6).unexpected("b").into());
        let actual = Json::new().deserialize(&"\"a\"  b");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_char correctly errors on one quote.
    #[test]
    fn visit_char_one_quote() {
        let expected: Result<char> = Err(Syntax::new(1, 1).expected("\"").into());
        let actual = Json::new().deserialize(&"\"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f32 correctly deserializes an f32 type.
    #[test]
    fn visit_f32_positive() {
        let expected = Ok(1_f32);
        let actual = Json::new().deserialize(&"1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f32 correctly deserializes a negative f32 type.
    #[test]
    fn visit_f32_negative() {
        let expected = Ok(-1_f32);
        let actual = Json::new().deserialize(&"-1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f32 correctly deserializes a zero f32 type.
    #[test]
    fn visit_f32_zero() {
        let expected = Ok(0_f32);
        let actual = Json::new().deserialize(&"0");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f32 correctly deserializes an f32 with surrounding whitespace.
    #[test]
    fn visit_f32_surrounding_whitespace() {
        let expected = Ok(0_f32);
        let actual = Json::new().deserialize(&" \n0  ");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f32 correctly errors upon empty value.
    #[test]
    fn visit_f32_empty() {
        let expected: Result<f32> = Err(Syntax::new(1, 1).expected("f32").into());
        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f32 correctly errors upon an invalid character.
    #[test]
    fn visit_f32_invalid_character() {
        let expected: Result<f32> = Err(Syntax::new(1, 2).unexpected("|").into());
        let actual = Json::new().deserialize(&"1|2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f32 correctly errors upon an invalid negative.
    #[test]
    fn visit_f32_invalid_negative() {
        let expected: Result<f32> = Err(Syntax::new(1, 3).unexpected("-").into());
        let actual = Json::new().deserialize(&"-1-2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f32 correctly errors upon an invalid dot.
    #[test]
    fn visit_f32_invalid_dot() {
        let expected: Result<f32> = Err(Syntax::new(1, 3).unexpected(".").into());
        let actual = Json::new().deserialize(&".1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f32 correctly errors upon an invalid whitespace.
    #[test]
    fn visit_f32_invalid_whitespace() {
        let expected: Result<f32> = Err(Syntax::new(1, 2).unexpected(" ").into());
        let actual = Json::new().deserialize(&"1 2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f32 correctly errors upon an invalid newline.
    #[test]
    fn visit_f32_invalid_newline() {
        let expected: Result<f32> = Err(Syntax::new(1, 2).unexpected("\n").into());
        let actual = Json::new().deserialize(&"1\n2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f32 correctly errors upon overflow.
    #[test]
    fn visit_f32_overflow() {
        let value = f32::MAX.to_string() + "0";
        let expected: Result<f32> = Err(Overflow::new(1, 1).kind("f32").into());
        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f32 correctly errors upon negative overflow.
    #[test]
    fn visit_f32_negative_overflow() {
        let value = f32::MIN.to_string() + "0";
        let expected: Result<f32> = Err(Overflow::new(1, 1).kind("f32").into());
        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f64 correctly deserializes an f64 type.
    #[test]
    fn visit_f64_positive() {
        let expected = Ok(1_f64);
        let actual = Json::new().deserialize(&"1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f64 correctly deserializes a negative f64 type.
    #[test]
    fn visit_f64_negative() {
        let expected = Ok(-1_f64);
        let actual = Json::new().deserialize(&"-1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f64 correctly deserializes a zero f64 type.
    #[test]
    fn visit_f64_zero() {
        let expected = Ok(0_f64);
        let actual = Json::new().deserialize(&"0");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f64 correctly deserializes an f64 with surrounding whitespace.
    #[test]
    fn visit_f64_surrounding_whitespace() {
        let expected = Ok(0_f64);
        let actual = Json::new().deserialize(&" \n0  ");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f64 correctly errors upon empty value.
    #[test]
    fn visit_f64_empty() {
        let expected: Result<f64> = Err(Syntax::new(1, 1).expected("f64").into());
        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f64 correctly errors upon an invalid character.
    #[test]
    fn visit_f64_invalid_character() {
        let expected: Result<f64> = Err(Syntax::new(1, 2).unexpected("|").into());
        let actual = Json::new().deserialize(&"1|2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f64 correctly errors upon an invalid negative.
    #[test]
    fn visit_f64_invalid_negative() {
        let expected: Result<f64> = Err(Syntax::new(1, 3).unexpected("-").into());
        let actual = Json::new().deserialize(&"-1-2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f64 correctly errors upon an invalid dot.
    #[test]
    fn visit_f64_invalid_dot() {
        let expected: Result<f64> = Err(Syntax::new(1, 3).unexpected(".").into());
        let actual = Json::new().deserialize(&".1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f64 correctly errors upon an invalid whitespace.
    #[test]
    fn visit_f64_invalid_whitespace() {
        let expected: Result<f64> = Err(Syntax::new(1, 2).unexpected(" ").into());
        let actual = Json::new().deserialize(&"1 2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f64 correctly errors upon an invalid newline.
    #[test]
    fn visit_f64_invalid_newline() {
        let expected: Result<f64> = Err(Syntax::new(1, 2).unexpected("\n").into());
        let actual = Json::new().deserialize(&"1\n2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f64 correctly errors upon overflow.
    #[test]
    fn visit_f64_overflow() {
        let value = f64::MAX.to_string() + "0";
        let expected: Result<f64> = Err(Overflow::new(1, 1).kind("f64").into());
        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_f64 correctly errors upon negative overflow.
    #[test]
    fn visit_f64_negative_overflow() {
        let value = f64::MIN.to_string() + "0";
        let expected: Result<f64> = Err(Overflow::new(1, 1).kind("f64").into());
        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i8 correctly deserializes an i8 type.
    #[test]
    fn visit_i8_positive() {
        let expected = Ok(1_i8);
        let actual = Json::new().deserialize(&"1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i8 correctly deserializes a negative i8 type.
    #[test]
    fn visit_i8_negative() {
        let expected = Ok(-1_i8);
        let actual = Json::new().deserialize(&"-1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i8 correctly deserializes a zero i8 type.
    #[test]
    fn visit_i8_zero() {
        let expected = Ok(0_i8);
        let actual = Json::new().deserialize(&"0");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i8 correctly deserializes an i8 with surrounding whitespace.
    #[test]
    fn visit_i8_surrounding_whitespace() {
        let expected = Ok(0_i8);
        let actual = Json::new().deserialize(&" \n0  ");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i8 correctly errors upon empty value.
    #[test]
    fn visit_i8_empty() {
        let expected: Result<i8> = Err(Syntax::new(1, 1).expected("i8").into());
        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i8 correctly errors upon an invalid character.
    #[test]
    fn visit_i8_invalid_character() {
        let expected: Result<i8> = Err(Syntax::new(1, 2).unexpected(".").into());
        let actual = Json::new().deserialize(&"1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i8 correctly errors upon an invalid negative.
    #[test]
    fn visit_i8_invalid_negative() {
        let expected: Result<i8> = Err(Syntax::new(1, 3).unexpected("-").into());
        let actual = Json::new().deserialize(&"-1-2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i8 correctly errors upon an invalid whitespace.
    #[test]
    fn visit_i8_invalid_whitespace() {
        let expected: Result<i8> = Err(Syntax::new(1, 2).unexpected(" ").into());
        let actual = Json::new().deserialize(&"1 2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i8 correctly errors upon an invalid newline.
    #[test]
    fn visit_i8_invalid_newline() {
        let expected: Result<i8> = Err(Syntax::new(1, 2).unexpected("\n").into());
        let actual = Json::new().deserialize(&"1\n2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i8 correctly errors upon overflow.
    #[test]
    fn visit_i8_overflow() {
        let value = i8::MAX.to_string() + "0";
        let expected: Result<i8> = Err(Overflow::new(1, 1).kind("i8").into());
        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i8 correctly errors upon negative overflow.
    #[test]
    fn visit_i8_negative_overflow() {
        let value = i8::MIN.to_string() + "0";
        let expected: Result<i8> = Err(Overflow::new(1, 1).kind("i8").into());
        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i16 correctly deserializes an i16 type.
    #[test]
    fn visit_i16_positive() {
        let expected = Ok(1_i16);
        let actual = Json::new().deserialize(&"1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i16 correctly deserializes a negative i16 type.
    #[test]
    fn visit_i16_negative() {
        let expected = Ok(-1_i16);
        let actual = Json::new().deserialize(&"-1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i16 correctly deserializes a zero i16 type.
    #[test]
    fn visit_i16_zero() {
        let expected = Ok(0_i16);
        let actual = Json::new().deserialize(&"0");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i16 correctly deserializes an i16 with surrounding whitespace.
    #[test]
    fn visit_i16_surrounding_whitespace() {
        let expected = Ok(0_i16);
        let actual = Json::new().deserialize(&" \n0  ");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i16 correctly errors upon empty value.
    #[test]
    fn visit_i16_empty() {
        let expected: Result<i16> = Err(Syntax::new(1, 1).expected("i16").into());
        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i16 correctly errors upon an invalid character.
    #[test]
    fn visit_i16_invalid_character() {
        let expected: Result<i16> = Err(Syntax::new(1, 2).unexpected(".").into());
        let actual = Json::new().deserialize(&"1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i16 correctly errors upon an invalid negative.
    #[test]
    fn visit_i16_invalid_negative() {
        let expected: Result<i16> = Err(Syntax::new(1, 3).unexpected("-").into());
        let actual = Json::new().deserialize(&"-1-2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i16 correctly errors upon an invalid whitespace.
    #[test]
    fn visit_i16_invalid_whitespace() {
        let expected: Result<i16> = Err(Syntax::new(1, 2).unexpected(" ").into());
        let actual = Json::new().deserialize(&"1 2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i16 correctly errors upon an invalid newline.
    #[test]
    fn visit_i16_invalid_newline() {
        let expected: Result<i16> = Err(Syntax::new(1, 2).unexpected("\n").into());
        let actual = Json::new().deserialize(&"1\n2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i16 correctly errors upon overflow.
    #[test]
    fn visit_i16_overflow() {
        let value = i16::MAX.to_string() + "0";
        let expected: Result<i16> = Err(Overflow::new(1, 1).kind("i16").into());
        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i16 correctly errors upon negative overflow.
    #[test]
    fn visit_i16_negative_overflow() {
        let value = i16::MIN.to_string() + "0";
        let expected: Result<i16> = Err(Overflow::new(1, 1).kind("i16").into());
        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i32 correctly deserializes an i32 type.
    #[test]
    fn visit_i32_positive() {
        let expected = Ok(1_i32);
        let actual = Json::new().deserialize(&"1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i32 correctly deserializes a negative i32 type.
    #[test]
    fn visit_i32_negative() {
        let expected = Ok(-1_i32);
        let actual = Json::new().deserialize(&"-1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i32 correctly deserializes a zero i32 type.
    #[test]
    fn visit_i32_zero() {
        let expected = Ok(0_i32);
        let actual = Json::new().deserialize(&"0");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i32 correctly deserializes an i32 with surrounding whitespace.
    #[test]
    fn visit_i32_surrounding_whitespace() {
        let expected = Ok(0_i32);
        let actual = Json::new().deserialize(&" \n0  ");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i32 correctly errors upon empty value.
    #[test]
    fn visit_i32_empty() {
        let expected: Result<i32> = Err(Syntax::new(1, 1).expected("i32").into());
        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i32 correctly errors upon an invalid character.
    #[test]
    fn visit_i32_invalid_character() {
        let expected: Result<i32> = Err(Syntax::new(1, 2).unexpected(".").into());
        let actual = Json::new().deserialize(&"1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i32 correctly errors upon an invalid negative.
    #[test]
    fn visit_i32_invalid_negative() {
        let expected: Result<i32> = Err(Syntax::new(1, 3).unexpected("-").into());
        let actual = Json::new().deserialize(&"-1-2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i32 correctly errors upon an invalid whitespace.
    #[test]
    fn visit_i32_invalid_whitespace() {
        let expected: Result<i32> = Err(Syntax::new(1, 2).unexpected(" ").into());
        let actual = Json::new().deserialize(&"1 2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i32 correctly errors upon an invalid newline.
    #[test]
    fn visit_i32_invalid_newline() {
        let expected: Result<i32> = Err(Syntax::new(1, 2).unexpected("\n").into());
        let actual = Json::new().deserialize(&"1\n2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i32 correctly errors upon overflow.
    #[test]
    fn visit_i32_overflow() {
        let value = i32::MAX.to_string() + "0";
        let expected: Result<i32> = Err(Overflow::new(1, 1).kind("i32").into());
        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i32 correctly errors upon negative overflow.
    #[test]
    fn visit_i32_negative_overflow() {
        let value = i32::MIN.to_string() + "0";
        let expected: Result<i32> = Err(Overflow::new(1, 1).kind("i32").into());
        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i64 correctly deserializes an i64 type.
    #[test]
    fn visit_i64_positive() {
        let expected = Ok(1_i64);
        let actual = Json::new().deserialize(&"1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i64 correctly deserializes a negative i64 type.
    #[test]
    fn visit_i64_negative() {
        let expected = Ok(-1_i64);
        let actual = Json::new().deserialize(&"-1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i64 correctly deserializes a zero i64 type.
    #[test]
    fn visit_i64_zero() {
        let expected = Ok(0_i64);
        let actual = Json::new().deserialize(&"0");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i64 correctly deserializes an i64 with surrounding whitespace.
    #[test]
    fn visit_i64_surrounding_whitespace() {
        let expected = Ok(0_i64);
        let actual = Json::new().deserialize(&" \n0  ");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i64 correctly errors upon empty value.
    #[test]
    fn visit_i64_empty() {
        let expected: Result<i64> = Err(Syntax::new(1, 1).expected("i64").into());
        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i64 correctly errors upon an invalid character.
    #[test]
    fn visit_i64_invalid_character() {
        let expected: Result<i64> = Err(Syntax::new(1, 2).unexpected(".").into());
        let actual = Json::new().deserialize(&"1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i64 correctly errors upon an invalid negative.
    #[test]
    fn visit_i64_invalid_negative() {
        let expected: Result<i64> = Err(Syntax::new(1, 3).unexpected("-").into());
        let actual = Json::new().deserialize(&"-1-2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i64 correctly errors upon an invalid whitespace.
    #[test]
    fn visit_i64_invalid_whitespace() {
        let expected: Result<i64> = Err(Syntax::new(1, 2).unexpected(" ").into());
        let actual = Json::new().deserialize(&"1 2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i64 correctly errors upon an invalid newline.
    #[test]
    fn visit_i64_invalid_newline() {
        let expected: Result<i64> = Err(Syntax::new(1, 2).unexpected("\n").into());
        let actual = Json::new().deserialize(&"1\n2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i64 correctly errors upon overflow.
    #[test]
    fn visit_i64_overflow() {
        let value = i64::MAX.to_string() + "0";
        let expected: Result<i64> = Err(Overflow::new(1, 1).kind("i64").into());
        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i64 correctly errors upon negative overflow.
    #[test]
    fn visit_i64_negative_overflow() {
        let value = i64::MIN.to_string() + "0";
        let expected: Result<i64> = Err(Overflow::new(1, 1).kind("i64").into());
        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i128 correctly deserializes an i128 type.
    #[test]
    fn visit_i128_positive() {
        let expected = Ok(1_i128);
        let actual = Json::new().deserialize(&"1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i128 correctly deserializes a negative i128 type.
    #[test]
    fn visit_i128_negative() {
        let expected = Ok(-1_i128);
        let actual = Json::new().deserialize(&"-1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i128 correctly deserializes a zero i128 type.
    #[test]
    fn visit_i128_zero() {
        let expected = Ok(0_i128);
        let actual = Json::new().deserialize(&"0");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i128 correctly deserializes an i128 with surrounding whitespace.
    #[test]
    fn visit_i128_surrounding_whitespace() {
        let expected = Ok(0_i128);
        let actual = Json::new().deserialize(&" \n0  ");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i128 correctly errors upon empty value.
    #[test]
    fn visit_i128_empty() {
        let expected: Result<i128> = Err(Syntax::new(1, 1).expected("i128").into());
        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i128 correctly errors upon an invalid character.
    #[test]
    fn visit_i128_invalid_character() {
        let expected: Result<i128> = Err(Syntax::new(1, 2).unexpected(".").into());
        let actual = Json::new().deserialize(&"1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i128 correctly errors upon an invalid negative.
    #[test]
    fn visit_i128_invalid_negative() {
        let expected: Result<i128> = Err(Syntax::new(1, 3).unexpected("-").into());
        let actual = Json::new().deserialize(&"-1-2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i128 correctly errors upon an invalid whitespace.
    #[test]
    fn visit_i128_invalid_whitespace() {
        let expected: Result<i128> = Err(Syntax::new(1, 2).unexpected(" ").into());
        let actual = Json::new().deserialize(&"1 2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i128 correctly errors upon an invalid newline.
    #[test]
    fn visit_i128_invalid_newline() {
        let expected: Result<i128> = Err(Syntax::new(1, 2).unexpected("\n").into());
        let actual = Json::new().deserialize(&"1\n2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i128 correctly errors upon overflow.
    #[test]
    fn visit_i128_overflow() {
        let value = i128::MAX.to_string() + "0";
        let expected: Result<i128> = Err(Overflow::new(1, 1).kind("i128").into());
        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_i128 correctly errors upon negative overflow.
    #[test]
    fn visit_i128_negative_overflow() {
        let value = i128::MIN.to_string() + "0";
        let expected: Result<i128> = Err(Overflow::new(1, 1).kind("i128").into());
        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_isize correctly deserializes an isize type.
    #[test]
    fn visit_isize_positive() {
        let expected = Ok(1_isize);
        let actual = Json::new().deserialize(&"1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_isize correctly deserializes a negative isize type.
    #[test]
    fn visit_isize_negative() {
        let expected = Ok(-1_isize);
        let actual = Json::new().deserialize(&"-1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_isize correctly deserializes a zero isize type.
    #[test]
    fn visit_isize_zero() {
        let expected = Ok(0_isize);
        let actual = Json::new().deserialize(&"0");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_isize correctly deserializes an isize with surrounding whitespace.
    #[test]
    fn visit_isize_surrounding_whitespace() {
        let expected = Ok(0_isize);
        let actual = Json::new().deserialize(&" \n0  ");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_isize correctly errors upon empty value.
    #[test]
    fn visit_isize_empty() {
        let expected: Result<isize> = Err(Syntax::new(1, 1).expected("isize").into());
        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_isize correctly errors upon an invalid character.
    #[test]
    fn visit_isize_invalid_character() {
        let expected: Result<isize> = Err(Syntax::new(1, 2).unexpected(".").into());
        let actual = Json::new().deserialize(&"1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_isize correctly errors upon an invalid negative.
    #[test]
    fn visit_isize_invalid_negative() {
        let expected: Result<isize> = Err(Syntax::new(1, 3).unexpected("-").into());
        let actual = Json::new().deserialize(&"-1-2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_isize correctly errors upon an invalid whitespace.
    #[test]
    fn visit_isize_invalid_whitespace() {
        let expected: Result<isize> = Err(Syntax::new(1, 2).unexpected(" ").into());
        let actual = Json::new().deserialize(&"1 2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_isize correctly errors upon an invalid newline.
    #[test]
    fn visit_isize_invalid_newline() {
        let expected: Result<isize> = Err(Syntax::new(1, 2).unexpected("\n").into());
        let actual = Json::new().deserialize(&"1\n2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_isize correctly errors upon overflow.
    #[test]
    fn visit_isize_overflow() {
        let value = i128::MAX.to_string() + "0";
        let expected: Result<isize> = Err(Overflow::new(1, 1).kind("isize").into());
        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_isize correctly errors upon negative overflow.
    #[test]
    fn visit_isize_negative_overflow() {
        let value = i128::MIN.to_string() + "0";
        let expected: Result<isize> = Err(Overflow::new(1, 1).kind("isize").into());
        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Tes Json::visit_string correctly deserializes a String type.
    #[test]
    fn visit_string_correct() {
        let expected = Ok("a".to_string());
        let actual = Json::new().deserialize(&"\"a\"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_string correctly deserializes a escaped backslash.
    #[test]
    fn visit_string_escape_backslash() {
        let expected = Ok("\\".to_string());
        let actual = Json::new().deserialize(&"\"\\\\\"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_string correctly deserializes a escaped quote.
    #[test]
    fn visit_string_escape_quote() {
        let expected = Ok("\"".to_string());
        let actual = Json::new().deserialize(&"\"\\\"\"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_string correctly handles an empty string.
    #[test]
    fn visit_string_nothing() {
        let expected = Ok(String::new());
        let actual = Json::new().deserialize(&"\"\"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_string correctly deserializes with whitespace.
    #[test]
    fn visit_string_whitespace() {
        let expected = Ok("a".to_string());
        let actual = Json::new().deserialize(&"  \n\"a\"  ");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_string correctly errors when empty.
    #[test]
    fn visit_string_empty() {
        let expected: Result<String> = Err(Syntax::new(1, 1).expected("\"").into());
        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_string correctly errors on missing leading quote.
    #[test]
    fn visit_string_missing_leading_quote() {
        let expected: Result<String> = Err(Syntax::new(1, 1).unexpected("a").expected("\"").into());
        let actual = Json::new().deserialize(&"a\"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_string correctly errors on missing trailing quote.
    #[test]
    fn visit_string_missing_trailing_quote() {
        let expected: Result<String> = Err(Syntax::new(1, 2).unexpected("a").expected("\"").into());
        let actual = Json::new().deserialize(&"\"a");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_string correctly errors on one quote.
    #[test]
    fn visit_string_one_quote() {
        let expected: Result<String> = Err(Syntax::new(1, 1).expected("\"").into());
        let actual = Json::new().deserialize(&"\"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_string correctly errors on trailing chars.
    #[test]
    fn visit_string_trailing_chars() {
        let expected: Result<String> = Err(Syntax::new(1, 6).unexpected("b").into());
        let actual = Json::new().deserialize(&"\"a\"  b");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_1 correctly deserializes a tuple type of size 1.
    #[test]
    fn visit_tuple_1_correct() {
        let expected = Ok((1_u8,));
        let actual = Json::new().deserialize(&"[1]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_1 correctly deserializes a tuple type of size 1
    /// when a delimiter exists in a string.
    #[test]
    fn visit_tuple_1_delimiter() {
        let expected = Ok((',',));
        let actual = Json::new().deserialize(&"[\",\"]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_1 correctly deserializes with whitespace.
    #[test]
    fn visit_tuple_1_whitespace() {
        let expected = Ok((1_u8,));
        let actual = Json::new().deserialize(&"  \n[1]  ");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_1 correctly deserializes with internal whitespace.
    #[test]
    fn visit_tuple_1_internal_whitespace() {
        let expected = Ok((1_u8,));
        let actual = Json::new().deserialize(&"[  \n1  ]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_1 correctly errors when empty.
    #[test]
    fn visit_tuple_1_empty() {
        let expected: Result<(u8,)> = Err(Syntax::new(1, 1).expected("[").into());
        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_1 correctly errors on an empty tuple.
    #[test]
    fn visit_tuple_1_nothing() {
        let expected: Result<(u8,)> = Err(Syntax::new(1, 2).expected("u8").into());
        let actual = Json::new().deserialize(&"[]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_1 correctly errors with a missing leading bracket.
    #[test]
    fn visit_tuple_1_missing_leading_bracket() {
        let expected: Result<(u8,)> = Err(Syntax::new(1, 1).expected("[").unexpected("1").into());
        let actual = Json::new().deserialize(&"1]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_1 correctly errors with a single bracket.
    #[test]
    fn visit_tuple_1_single_bracket() {
        let expected: Result<(u8,)> = Err(Syntax::new(1, 2).expected("]").into());
        let actual = Json::new().deserialize(&"[");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_1 correctly errors with a missing trailing bracket.
    #[test]
    fn visit_tuple_1_missing_trailing_bracket() {
        let expected: Result<(u8,)> = Err(Syntax::new(1, 4).expected("]").unexpected("}").into());
        let actual = Json::new().deserialize(&"[1}");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_1 correctly errors on overflow.
    #[test]
    fn visit_tuple_1_overflow() {
        let expected: Result<(u8,)> = Err(Syntax::new(1, 3).unexpected(",").into());
        let actual = Json::new().deserialize(&"[1, 2]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_1 correctly errors on trailing chars.
    #[test]
    fn visit_tuple_1_trailing_chars() {
        let expected: Result<(u8,)> = Err(Syntax::new(1, 6).unexpected("b").into());
        let actual = Json::new().deserialize(&"[1]  b");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_2 correctly deserializes a tuple type of size 2.
    #[test]
    fn visit_tuple_2_correct() {
        let expected = Ok((1_u8, 2_u8));
        let actual = Json::new().deserialize(&"[1, 2]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_2 correctly deserializes a tuple type of size 2
    /// when a delimiter exists in a string.
    #[test]
    fn visit_tuple_2_delimiter() {
        let expected = Ok((',', 2_u8));
        let actual = Json::new().deserialize(&"[\",\", 2]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_2 correctly deserializes with whitespace.
    #[test]
    fn visit_tuple_2_whitespace() {
        let expected = Ok((1_u8, 2_u8));
        let actual = Json::new().deserialize(&"  \n[1, 2]  ");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_2 correctly deserializes with internal whitespace.
    #[test]
    fn visit_tuple_2_internal_whitespace() {
        let expected = Ok((1_u8, 2_u8));
        let actual = Json::new().deserialize(&"[  \n1, \n2  ]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_2 correctly errors when empty.
    #[test]
    fn visit_tuple_2_empty() {
        let expected: Result<(u8, u8)> = Err(Syntax::new(1, 1).expected("[").into());
        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_2 correctly errors on an empty tuple.
    #[test]
    fn visit_tuple_2_nothing() {
        let expected: Result<(u8, u8)> =
            Err(Syntax::new(1, 3).unexpected("]").expected(",").into());
        let actual = Json::new().deserialize(&"[]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_2 correctly errors with a missing leading bracket.
    #[test]
    fn visit_tuple_2_missing_leading_bracket() {
        let expected: Result<(u8, u8)> =
            Err(Syntax::new(1, 1).expected("[").unexpected("1").into());
        let actual = Json::new().deserialize(&"1, 2]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_2 correctly errors with a single bracket.
    #[test]
    fn visit_tuple_2_single_bracket() {
        let expected: Result<(u8, u8)> = Err(Syntax::new(1, 2).expected(",").into());
        let actual = Json::new().deserialize(&"[");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_2 correctly errors with a missing trailing bracket.
    #[test]
    fn visit_tuple_2_missing_trailing_bracket() {
        let expected: Result<(u8, u8)> =
            Err(Syntax::new(1, 7).expected("]").unexpected("}").into());
        let actual = Json::new().deserialize(&"[1, 2}");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_2 correctly errors on overflow.
    #[test]
    fn visit_tuple_2_overflow() {
        let expected: Result<(u8, u16)> = Err(Syntax::new(1, 6).unexpected(",").into());
        let actual = Json::new().deserialize(&"[1, 2, 3]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_2 correctly errors on underflow.
    #[test]
    fn visit_tuple_2_underflow() {
        let expected: Result<(u8, u8)> =
            Err(Syntax::new(1, 4).unexpected("]").expected(",").into());
        let actual = Json::new().deserialize(&"[1]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_2 correctly errors on trailing chars.
    #[test]
    fn visit_tuple_2_trailing_chars() {
        let expected: Result<(u8, u8)> = Err(Syntax::new(1, 9).unexpected("b").into());
        let actual = Json::new().deserialize(&"[1, 2]  b");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_3 correctly deserializes a tuple type of size 3.
    #[test]
    fn visit_tuple_3_correct() {
        let expected = Ok((1_u8, 2_u8, 3_u8));
        let actual = Json::new().deserialize(&"[1, 2, 3]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_3 correctly deserializes a tuple type of size 3
    /// when a delimiter exists in a string.
    #[test]
    fn visit_tuple_3_delimiter() {
        let expected = Ok((',', 2_u8, 3_u8));
        let actual = Json::new().deserialize(&"[\",\", 2, 3]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_3 correctly deserializes with whitespace.
    #[test]
    fn visit_tuple_3_whitespace() {
        let expected = Ok((1_u8, 2_u8, 3_u8));
        let actual = Json::new().deserialize(&"  \n[1, 2, 3]  ");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_3 correctly deserializes with internal whitespace.
    #[test]
    fn visit_tuple_3_internal_whitespace() {
        let expected = Ok((1_u8, 2_u8, 3_u8));
        let actual = Json::new().deserialize(&"[  \n1, \n2, \n3  ]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_3 correctly errors when empty.
    #[test]
    fn visit_tuple_3_empty() {
        let expected: Result<(u8, u8, u8)> = Err(Syntax::new(1, 1).expected("[").into());
        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_3 correctly errors on an empty tuple.
    #[test]
    fn visit_tuple_3_nothing() {
        let expected: Result<(u8, u8, u8)> =
            Err(Syntax::new(1, 3).unexpected("]").expected(",").into());
        let actual = Json::new().deserialize(&"[]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_3 correctly errors with a missing leading bracket.
    #[test]
    fn visit_tuple_3_missing_leading_bracket() {
        let expected: Result<(u8, u8, u8)> =
            Err(Syntax::new(1, 1).expected("[").unexpected("1").into());
        let actual = Json::new().deserialize(&"1, 2, 3]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_3 correctly errors with a single bracket.
    #[test]
    fn visit_tuple_3_single_bracket() {
        let expected: Result<(u8, u8, u8)> = Err(Syntax::new(1, 2).expected(",").into());
        let actual = Json::new().deserialize(&"[");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_3 correctly errors with a missing trailing bracket.
    #[test]
    fn visit_tuple_3_missing_trailing_bracket() {
        let expected: Result<(u8, u8, u8)> =
            Err(Syntax::new(1, 10).expected("]").unexpected("}").into());
        let actual = Json::new().deserialize(&"[1, 2, 3}");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_3 correctly errors on overflow.
    #[test]
    fn visit_tuple_3_overflow() {
        let expected: Result<(u8, u8, u8)> = Err(Syntax::new(1, 9).unexpected(",").into());
        let actual = Json::new().deserialize(&"[1, 2, 3, 4]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_3 correctly errors on underflow.
    #[test]
    fn visit_tuple_3_underflow() {
        let expected: Result<(u8, u8, u8)> =
            Err(Syntax::new(1, 4).unexpected("]").expected(",").into());
        let actual = Json::new().deserialize(&"[1]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_3 correctly errors on trailing chars.
    #[test]
    fn visit_tuple_3_trailing_chars() {
        let expected: Result<(u8, u8, u8)> = Err(Syntax::new(1, 12).unexpected("b").into());
        let actual = Json::new().deserialize(&"[1, 2, 3]  b");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_4 correctly deserializes a tuple type of size 4.
    #[test]
    fn visit_tuple_4_correct() {
        let expected = Ok((1_u8, 2_u8, 3_u8, 4_u8));
        let actual = Json::new().deserialize(&"[1, 2, 3, 4]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_4 correctly deserializes a tuple type of size 4
    /// when a delimiter exists in a string.
    #[test]
    fn visit_tuple_4_delimiter() {
        let expected = Ok((',', 2_u8, 3_u8, 4_u8));
        let actual = Json::new().deserialize(&"[\",\", 2, 3, 4]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_4 correctly deserializes with whitespace.
    #[test]
    fn visit_tuple_4_whitespace() {
        let expected = Ok((1_u8, 2_u8, 3_u8, 4_u8));
        let actual = Json::new().deserialize(&"  \n[1, 2, 3, 4]  ");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_4 correctly deserializes with internal whitespace.
    #[test]
    fn visit_tuple_4_internal_whitespace() {
        let expected = Ok((1_u8, 2_u8, 3_u8, 4_u8));
        let actual = Json::new().deserialize(&"[  \n1, \n2, \n3, \n4  ]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_4 correctly errors when empty.
    #[test]
    fn visit_tuple_4_empty() {
        let expected: Result<(u8, u8, u8, u8)> = Err(Syntax::new(1, 1).expected("[").into());
        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_4 correctly errors on an empty tuple.
    #[test]
    fn visit_tuple_4_nothing() {
        let expected: Result<(u8, u8, u8, u8)> =
            Err(Syntax::new(1, 3).unexpected("]").expected(",").into());
        let actual = Json::new().deserialize(&"[]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_4 correctly errors with a missing leading bracket.
    #[test]
    fn visit_tuple_4_missing_leading_bracket() {
        let expected: Result<(u8, u8, u8, u8)> =
            Err(Syntax::new(1, 1).expected("[").unexpected("1").into());
        let actual = Json::new().deserialize(&"1, 2, 3, 4]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_4 correctly errors with a single bracket.
    #[test]
    fn visit_tuple_4_single_bracket() {
        let expected: Result<(u8, u8, u8, u8)> = Err(Syntax::new(1, 2).expected(",").into());
        let actual = Json::new().deserialize(&"[");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_4 correctly errors with a missing trailing bracket.
    #[test]
    fn visit_tuple_4_missing_trailing_bracket() {
        let expected: Result<(u8, u8, u8, u8)> =
            Err(Syntax::new(1, 13).expected("]").unexpected("}").into());
        let actual = Json::new().deserialize(&"[1, 2, 3, 4}");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_4 correctly errors on overflow.
    #[test]
    fn visit_tuple_4_overflow() {
        let expected: Result<(u8, u8, u8, u8)> = Err(Syntax::new(1, 12).unexpected(",").into());
        let actual = Json::new().deserialize(&"[1, 2, 3, 4, 5]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_4 correctly errors on underflow.
    #[test]
    fn visit_tuple_4_underflow() {
        let expected: Result<(u8, u8, u8, u8)> =
            Err(Syntax::new(1, 4).unexpected("]").expected(",").into());
        let actual = Json::new().deserialize(&"[1]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_4 correctly errors on trailing chars.
    #[test]
    fn visit_tuple_4_trailing_chars() {
        let expected: Result<(u8, u8, u8, u8)> = Err(Syntax::new(1, 15).unexpected("b").into());
        let actual = Json::new().deserialize(&"[1, 2, 3, 4]  b");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_5 correctly deserializes a tuple type of size 5.
    #[test]
    fn visit_tuple_5_correct() {
        let expected = Ok((1_u8, 2_u8, 3_u8, 4_u8, 5_u8));
        let actual = Json::new().deserialize(&"[1, 2, 3, 4, 5]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_5 correctly deserializes a tuple type of size 5
    /// when a delimiter exists in a string.
    #[test]
    fn visit_tuple_5_delimiter() {
        let expected = Ok((',', 2_u8, 3_u8, 4_u8, 5_u8));
        let actual = Json::new().deserialize(&"[\",\", 2, 3, 4, 5]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_5 correctly deserializes with whitespace.
    #[test]
    fn visit_tuple_5_whitespace() {
        let expected = Ok((1_u8, 2_u8, 3_u8, 4_u8, 5_u8));
        let actual = Json::new().deserialize(&"  \n[1, 2, 3, 4, 5]  ");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_5 correctly deserializes with internal whitespace.
    #[test]
    fn visit_tuple_5_internal_whitespace() {
        let expected = Ok((1_u8, 2_u8, 3_u8, 4_u8, 5_u8));
        let actual = Json::new().deserialize(&"[  \n1, \n2, \n3, \n4, \n5  ]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_5 correctly errors when empty.
    #[test]
    fn visit_tuple_5_empty() {
        let expected: Result<(u8, u8, u8, u8, u8)> = Err(Syntax::new(1, 1).expected("[").into());
        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_5 correctly errors on an empty tuple.
    #[test]
    fn visit_tuple_5_nothing() {
        let expected: Result<(u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 3).unexpected("]").expected(",").into());
        let actual = Json::new().deserialize(&"[]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_5 correctly errors with a missing leading bracket.
    #[test]
    fn visit_tuple_5_missing_leading_bracket() {
        let expected: Result<(u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 1).expected("[").unexpected("1").into());
        let actual = Json::new().deserialize(&"1, 2, 3, 4, 5]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_5 correctly errors with a single bracket.
    #[test]
    fn visit_tuple_5_single_bracket() {
        let expected: Result<(u8, u8, u8, u8, u8)> = Err(Syntax::new(1, 2).expected(",").into());
        let actual = Json::new().deserialize(&"[");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_5 correctly errors with a missing trailing bracket.
    #[test]
    fn visit_tuple_5_missing_trailing_bracket() {
        let expected: Result<(u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 16).expected("]").unexpected("}").into());
        let actual = Json::new().deserialize(&"[1, 2, 3, 4, 5}");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_5 correctly errors on overflow.
    #[test]
    fn visit_tuple_5_overflow() {
        let expected: Result<(u8, u8, u8, u8, u8)> = Err(Syntax::new(1, 15).unexpected(",").into());
        let actual = Json::new().deserialize(&"[1, 2, 3, 4, 5, 6]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_5 correctly errors on underflow.
    #[test]
    fn visit_tuple_5_underflow() {
        let expected: Result<(u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 4).unexpected("]").expected(",").into());
        let actual = Json::new().deserialize(&"[1]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_5 correctly errors on trailing chars.
    #[test]
    fn visit_tuple_5_trailing_chars() {
        let expected: Result<(u8, u8, u8, u8, u8)> = Err(Syntax::new(1, 18).unexpected("b").into());
        let actual = Json::new().deserialize(&"[1, 2, 3, 4, 5]  b");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_6 correctly deserializes a tuple type of size 6.
    #[test]
    fn visit_tuple_6_correct() {
        let expected = Ok((1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8));
        let actual = Json::new().deserialize(&"[1, 2, 3, 4, 5, 6]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_6 correctly deserializes a tuple type of size 6
    /// when a delimiter exists in a string.
    #[test]
    fn visit_tuple_6_delimiter() {
        let expected = Ok((',', 2_u8, 3_u8, 4_u8, 5_u8, 6_u8));
        let actual = Json::new().deserialize(&"[\",\", 2, 3, 4, 5, 6]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_6 correctly deserializes with whitespace.
    #[test]
    fn visit_tuple_6_whitespace() {
        let expected = Ok((1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8));
        let actual = Json::new().deserialize(&"  \n[1, 2, 3, 4, 5, 6]  ");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_6 correctly deserializes with internal whitespace.
    #[test]
    fn visit_tuple_6_internal_whitespace() {
        let expected = Ok((1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8));
        let actual = Json::new().deserialize(&"[  \n1, \n2, \n3, \n4, \n5, \n6  ]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_6 correctly errors when empty.
    #[test]
    fn visit_tuple_6_empty() {
        let expected: Result<(u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 1).expected("[").into());
        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_6 correctly errors on an empty tuple.
    #[test]
    fn visit_tuple_6_nothing() {
        let expected: Result<(u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 3).unexpected("]").expected(",").into());
        let actual = Json::new().deserialize(&"[]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_6 correctly errors with a missing leading bracket.
    #[test]
    fn visit_tuple_6_missing_leading_bracket() {
        let expected: Result<(u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 1).expected("[").unexpected("1").into());
        let actual = Json::new().deserialize(&"1, 2, 3, 4, 5, 6]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_6 correctly errors with a single bracket.
    #[test]
    fn visit_tuple_6_single_bracket() {
        let expected: Result<(u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 2).expected(",").into());
        let actual = Json::new().deserialize(&"[");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_6 correctly errors with a missing trailing bracket.
    #[test]
    fn visit_tuple_6_missing_trailing_bracket() {
        let expected: Result<(u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 19).expected("]").unexpected("}").into());
        let actual = Json::new().deserialize(&"[1, 2, 3, 4, 5, 6}");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_6 correctly errors on overflow.
    #[test]
    fn visit_tuple_6_overflow() {
        let expected: Result<(u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 18).unexpected(",").into());
        let actual = Json::new().deserialize(&"[1, 2, 3, 4, 5, 6, 7]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_6 correctly errors on underflow.
    #[test]
    fn visit_tuple_6_underflow() {
        let expected: Result<(u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 4).unexpected("]").expected(",").into());
        let actual = Json::new().deserialize(&"[1]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_6 correctly errors on trailing chars.
    #[test]
    fn visit_tuple_6_trailing_chars() {
        let expected: Result<(u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 21).unexpected("b").into());
        let actual = Json::new().deserialize(&"[1, 2, 3, 4, 5, 6]  b");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_7 correctly deserializes a tuple type of size 7.
    #[test]
    fn visit_tuple_7_correct() {
        let expected = Ok((1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8));
        let actual = Json::new().deserialize(&"[1, 2, 3, 4, 5, 6, 7]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_7 correctly deserializes a tuple type of size 7
    /// when a delimiter exists in a string.
    #[test]
    fn visit_tuple_7_delimiter() {
        let expected = Ok((',', 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8));
        let actual = Json::new().deserialize(&"[\",\", 2, 3, 4, 5, 6, 7]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_7 correctly deserializes with whitespace.
    #[test]
    fn visit_tuple_7_whitespace() {
        let expected = Ok((1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8));
        let actual = Json::new().deserialize(&"  \n[1, 2, 3, 4, 5, 6, 7]  ");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_7 correctly deserializes with internal whitespace.
    #[test]
    fn visit_tuple_7_internal_whitespace() {
        let expected = Ok((1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8));
        let actual = Json::new().deserialize(&"[  \n1, \n2, \n3, \n4, \n5, \n6, \n7  ]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_7 correctly errors when empty.
    #[test]
    fn visit_tuple_7_empty() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 1).expected("[").into());
        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_7 correctly errors on an empty tuple.
    #[test]
    fn visit_tuple_7_nothing() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 3).unexpected("]").expected(",").into());
        let actual = Json::new().deserialize(&"[]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_7 correctly errors with a missing leading bracket.
    #[test]
    fn visit_tuple_7_missing_leading_bracket() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 1).expected("[").unexpected("1").into());
        let actual = Json::new().deserialize(&"1, 2, 3, 4, 5, 6, 7]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_7 correctly errors with a single bracket.
    #[test]
    fn visit_tuple_7_single_bracket() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 2).expected(",").into());
        let actual = Json::new().deserialize(&"[");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_7 correctly errors with a missing trailing bracket.
    #[test]
    fn visit_tuple_7_missing_trailing_bracket() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 22).expected("]").unexpected("}").into());
        let actual = Json::new().deserialize(&"[1, 2, 3, 4, 5, 6, 7}");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_7 correctly errors on overflow.
    #[test]
    fn visit_tuple_7_overflow() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 21).unexpected(",").into());
        let actual = Json::new().deserialize(&"[1, 2, 3, 4, 5, 6, 7, 8]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_7 correctly errors on underflow.
    #[test]
    fn visit_tuple_7_underflow() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 4).unexpected("]").expected(",").into());
        let actual = Json::new().deserialize(&"[1]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_7 correctly errors on trailing chars.
    #[test]
    fn visit_tuple_7_trailing_chars() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 24).unexpected("b").into());
        let actual = Json::new().deserialize(&"[1, 2, 3, 4, 5, 6, 7]  b");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_8 correctly deserializes a tuple type of size 8.
    #[test]
    fn visit_tuple_8_correct() {
        let expected = Ok((1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8));
        let actual = Json::new().deserialize(&"[1, 2, 3, 4, 5, 6, 7, 8]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_8 correctly deserializes a tuple type of size 8
    /// when a delimiter exists in a string.
    #[test]
    fn visit_tuple_8_delimiter() {
        let expected = Ok((',', 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8));
        let actual = Json::new().deserialize(&"[\",\", 2, 3, 4, 5, 6, 7, 8]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_8 correctly deserializes with whitespace.
    #[test]
    fn visit_tuple_8_whitespace() {
        let expected = Ok((1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8));
        let actual = Json::new().deserialize(&"  \n[1, 2, 3, 4, 5, 6, 7, 8]  ");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_8 correctly deserializes with internal whitespace.
    #[test]
    fn visit_tuple_8_internal_whitespace() {
        let expected = Ok((1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8));
        let actual = Json::new().deserialize(&"[  \n1, \n2, \n3, \n4, \n5, \n6, \n7, \n8  ]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_8 correctly errors when empty.
    #[test]
    fn visit_tuple_8_empty() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 1).expected("[").into());
        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_8 correctly errors on an empty tuple.
    #[test]
    fn visit_tuple_8_nothing() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 3).unexpected("]").expected(",").into());
        let actual = Json::new().deserialize(&"[]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_8 correctly errors with a missing leading bracket.
    #[test]
    fn visit_tuple_8_missing_leading_bracket() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 1).expected("[").unexpected("1").into());
        let actual = Json::new().deserialize(&"1, 2, 3, 4, 5, 6, 7, 8]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_8 correctly errors with a single bracket.
    #[test]
    fn visit_tuple_8_single_bracket() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 2).expected(",").into());
        let actual = Json::new().deserialize(&"[");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_8 correctly errors with a missing trailing bracket.
    #[test]
    fn visit_tuple_8_missing_trailing_bracket() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 25).expected("]").unexpected("}").into());
        let actual = Json::new().deserialize(&"[1, 2, 3, 4, 5, 6, 7, 8}");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_8 correctly errors on overflow.
    #[test]
    fn visit_tuple_8_overflow() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 24).unexpected(",").into());
        let actual = Json::new().deserialize(&"[1, 2, 3, 4, 5, 6, 7, 8, 9]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_8 correctly errors on underflow.
    #[test]
    fn visit_tuple_8_underflow() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 4).unexpected("]").expected(",").into());
        let actual = Json::new().deserialize(&"[1]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_8 correctly errors on trailing chars.
    #[test]
    fn visit_tuple_8_trailing_chars() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 27).unexpected("b").into());
        let actual = Json::new().deserialize(&"[1, 2, 3, 4, 5, 6, 7, 8]  b");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_9 correctly deserializes a tuple type of size 9.
    #[test]
    fn visit_tuple_9_correct() {
        let expected = Ok((1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8, 9_u8));
        let actual = Json::new().deserialize(&"[1, 2, 3, 4, 5, 6, 7, 8, 9]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_9 correctly deserializes a tuple type of size 9
    /// when a delimiter exists in a string.
    #[test]
    fn visit_tuple_9_delimiter() {
        let expected = Ok((',', 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8, 9_u8));
        let actual = Json::new().deserialize(&"[\",\", 2, 3, 4, 5, 6, 7, 8, 9]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_9 correctly deserializes with whitespace.
    #[test]
    fn visit_tuple_9_whitespace() {
        let expected = Ok((1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8, 9_u8));
        let actual = Json::new().deserialize(&"  \n[1, 2, 3, 4, 5, 6, 7, 8, 9]  ");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_9 correctly deserializes with internal whitespace.
    #[test]
    fn visit_tuple_9_internal_whitespace() {
        let expected = Ok((1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8, 9_u8));
        let actual = Json::new().deserialize(&"[  \n1, \n2, \n3, \n4, \n5, \n6, \n7, \n8, \n9  ]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_9 correctly errors when empty.
    #[test]
    fn visit_tuple_9_empty() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 1).expected("[").into());
        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_9 correctly errors on an empty tuple.
    #[test]
    fn visit_tuple_9_nothing() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 3).unexpected("]").expected(",").into());
        let actual = Json::new().deserialize(&"[]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_9 correctly errors with a missing leading bracket.
    #[test]
    fn visit_tuple_9_missing_leading_bracket() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 1).expected("[").unexpected("1").into());
        let actual = Json::new().deserialize(&"1, 2, 3, 4, 5, 6, 7, 8, 9]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_9 correctly errors with a single bracket.
    #[test]
    fn visit_tuple_9_single_bracket() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 2).expected(",").into());
        let actual = Json::new().deserialize(&"[");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_9 correctly errors with a missing trailing bracket.
    #[test]
    fn visit_tuple_9_missing_trailing_bracket() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 28).expected("]").unexpected("}").into());
        let actual = Json::new().deserialize(&"[1, 2, 3, 4, 5, 6, 7, 8, 9}");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_9 correctly errors on overflow.
    #[test]
    fn visit_tuple_9_overflow() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 27).unexpected(",").into());
        let actual = Json::new().deserialize(&"[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_9 correctly errors on underflow.
    #[test]
    fn visit_tuple_9_underflow() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 4).unexpected("]").expected(",").into());
        let actual = Json::new().deserialize(&"[1]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_9 correctly errors on trailing chars.
    #[test]
    fn visit_tuple_9_trailing_chars() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 30).unexpected("b").into());
        let actual = Json::new().deserialize(&"[1, 2, 3, 4, 5, 6, 7, 8, 9]  b");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_10 correctly deserializes a tuple type of size 10.
    #[test]
    fn visit_tuple_10_correct() {
        let expected = Ok((1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8, 9_u8, 10_u8));
        let actual = Json::new().deserialize(&"[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_10 correctly deserializes a tuple type of size 10
    /// when a delimiter exists in a string.
    #[test]
    fn visit_tuple_10_delimiter() {
        let expected = Ok((',', 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8, 9_u8, 10_u8));
        let actual = Json::new().deserialize(&"[\",\", 2, 3, 4, 5, 6, 7, 8, 9, 10]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_10 correctly deserializes with whitespace.
    #[test]
    fn visit_tuple_10_whitespace() {
        let expected = Ok((1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8, 9_u8, 10_u8));
        let actual = Json::new().deserialize(&"  \n[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]  ");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_10 correctly deserializes with internal whitespace.
    #[test]
    fn visit_tuple_10_internal_whitespace() {
        let expected = Ok((1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8, 9_u8, 10_u8));
        let actual =
            Json::new().deserialize(&"[  \n1, \n2, \n3, \n4, \n5, \n6, \n7, \n8, \n9, \n10  ]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_10 correctly errors when empty.
    #[test]
    fn visit_tuple_10_empty() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 1).expected("[").into());
        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_10 correctly errors on an empty tuple.
    #[test]
    fn visit_tuple_10_nothing() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 3).unexpected("]").expected(",").into());
        let actual = Json::new().deserialize(&"[]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_10 correctly errors with a missing leading bracket.
    #[test]
    fn visit_tuple_10_missing_leading_bracket() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 1).expected("[").unexpected("1").into());
        let actual = Json::new().deserialize(&"1, 2, 3, 4, 5, 6, 7, 8, 9, 10]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_10 correctly errors with a single bracket.
    #[test]
    fn visit_tuple_10_single_bracket() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 2).expected(",").into());
        let actual = Json::new().deserialize(&"[");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_10 correctly errors with a missing trailing bracket.
    #[test]
    fn visit_tuple_10_missing_trailing_bracket() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 32).expected("]").unexpected("}").into());
        let actual = Json::new().deserialize(&"[1, 2, 3, 4, 5, 6, 7, 8, 9, 10}");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_10 correctly errors on overflow.
    #[test]
    fn visit_tuple_10_overflow() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 31).unexpected(",").into());
        let actual = Json::new().deserialize(&"[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_10 correctly errors on underflow.
    #[test]
    fn visit_tuple_10_underflow() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 4).unexpected("]").expected(",").into());
        let actual = Json::new().deserialize(&"[1]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_10 correctly errors on trailing chars.
    #[test]
    fn visit_tuple_10_trailing_chars() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 34).unexpected("b").into());
        let actual = Json::new().deserialize(&"[1, 2, 3, 4, 5, 6, 7, 8, 9, 10]  b");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_11 correctly deserializes a tuple type of size 11.
    #[test]
    fn visit_tuple_11_correct() {
        let expected = Ok((
            1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8, 9_u8, 10_u8, 11_u8,
        ));
        let actual = Json::new().deserialize(&"[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_11 correctly deserializes a tuple type of size 11
    /// when a delimiter exists in a string.
    #[test]
    fn visit_tuple_11_delimiter() {
        let expected = Ok((
            ',', 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8, 9_u8, 10_u8, 11_u8,
        ));
        let actual = Json::new().deserialize(&"[\",\", 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_11 correctly deserializes with whitespace.
    #[test]
    fn visit_tuple_11_whitespace() {
        let expected = Ok((
            1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8, 9_u8, 10_u8, 11_u8,
        ));
        let actual = Json::new().deserialize(&"  \n[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]  ");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_11 correctly deserializes with internal whitespace.
    #[test]
    fn visit_tuple_11_internal_whitespace() {
        let expected = Ok((
            1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8, 9_u8, 10_u8, 11_u8,
        ));
        let actual = Json::new()
            .deserialize(&"[  \n1, \n2, \n3, \n4, \n5, \n6, \n7, \n8, \n9, \n10, \n11  ]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_11 correctly errors when empty.
    #[test]
    fn visit_tuple_11_empty() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 1).expected("[").into());
        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_11 correctly errors on an empty tuple.
    #[test]
    fn visit_tuple_11_nothing() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 3).unexpected("]").expected(",").into());
        let actual = Json::new().deserialize(&"[]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_11 correctly errors with a missing leading bracket.
    #[test]
    fn visit_tuple_11_missing_leading_bracket() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 1).expected("[").unexpected("1").into());
        let actual = Json::new().deserialize(&"1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_11 correctly errors with a single bracket.
    #[test]
    fn visit_tuple_11_single_bracket() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 2).expected(",").into());
        let actual = Json::new().deserialize(&"[");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_11 correctly errors with a missing trailing bracket.
    #[test]
    fn visit_tuple_11_missing_trailing_bracket() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 36).expected("]").unexpected("}").into());
        let actual = Json::new().deserialize(&"[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11}");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_11 correctly errors on overflow.
    #[test]
    fn visit_tuple_11_overflow() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 35).unexpected(",").into());
        let actual = Json::new().deserialize(&"[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_11 correctly errors on underflow.
    #[test]
    fn visit_tuple_11_underflow() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 4).unexpected("]").expected(",").into());
        let actual = Json::new().deserialize(&"[1]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_11 correctly errors on trailing chars.
    #[test]
    fn visit_tuple_11_trailing_chars() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 38).unexpected("b").into());
        let actual = Json::new().deserialize(&"[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]  b");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_12 correctly deserializes a tuple type of size 12.
    #[test]
    fn visit_tuple_12_correct() {
        let expected = Ok((
            1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8, 9_u8, 10_u8, 11_u8, 12_u8,
        ));
        let actual = Json::new().deserialize(&"[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_12 correctly deserializes a tuple type of size 12
    /// when a delimiter exists in a string.
    #[test]
    fn visit_tuple_12_delimiter() {
        let expected = Ok((
            ',', 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8, 9_u8, 10_u8, 11_u8, 12_u8,
        ));
        let actual = Json::new().deserialize(&"[\",\", 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_12 correctly deserializes with whitespace.
    #[test]
    fn visit_tuple_12_whitespace() {
        let expected = Ok((
            1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8, 9_u8, 10_u8, 11_u8, 12_u8,
        ));
        let actual = Json::new().deserialize(&"  \n[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]  ");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_12 correctly deserializes with internal whitespace.
    #[test]
    fn visit_tuple_12_internal_whitespace() {
        let expected = Ok((
            1_u8, 2_u8, 3_u8, 4_u8, 5_u8, 6_u8, 7_u8, 8_u8, 9_u8, 10_u8, 11_u8, 12_u8,
        ));
        let actual = Json::new()
            .deserialize(&"[  \n1, \n2, \n3, \n4, \n5, \n6, \n7, \n8, \n9, \n10, \n11, \n12  ]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_12 correctly errors when empty.
    #[test]
    fn visit_tuple_12_empty() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 1).expected("[").into());
        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_12 correctly errors on an empty tuple.
    #[test]
    fn visit_tuple_12_nothing() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 3).unexpected("]").expected(",").into());
        let actual = Json::new().deserialize(&"[]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_12 correctly errors with a missing leading bracket.
    #[test]
    fn visit_tuple_12_missing_leading_bracket() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 1).expected("[").unexpected("1").into());
        let actual = Json::new().deserialize(&"1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_12 correctly errors with a single bracket.
    #[test]
    fn visit_tuple_12_single_bracket() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 2).expected(",").into());
        let actual = Json::new().deserialize(&"[");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_12 correctly errors with a missing trailing bracket.
    #[test]
    fn visit_tuple_12_missing_trailing_bracket() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 40).expected("]").unexpected("}").into());
        let actual = Json::new().deserialize(&"[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12}");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_12 correctly errors on overflow.
    #[test]
    fn visit_tuple_12_overflow() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 39).unexpected(",").into());
        let actual = Json::new().deserialize(&"[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_12 correctly errors on underflow.
    #[test]
    fn visit_tuple_12_underflow() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 4).unexpected("]").expected(",").into());
        let actual = Json::new().deserialize(&"[1]");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_tuple_12 correctly errors on trailing chars.
    #[test]
    fn visit_tuple_12_trailing_chars() {
        let expected: Result<(u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8, u8)> =
            Err(Syntax::new(1, 42).unexpected("b").into());
        let actual = Json::new().deserialize(&"[1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]  b");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u8 correctly deserializes an u8 type.
    #[test]
    fn visit_u8_positive() {
        let expected = Ok(1_u8);
        let actual = Json::new().deserialize(&"1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u8 correctly deserializes a zero u8 type.
    #[test]
    fn visit_u8_zero() {
        let expected = Ok(0_u8);
        let actual = Json::new().deserialize(&"0");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u8 correctly deserializes an u8 with surrounding whitespace.
    #[test]
    fn visit_u8_surrounding_whitespace() {
        let expected = Ok(0_u8);
        let actual = Json::new().deserialize(&" \n0  ");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u8 correctly errors upon empty value.
    #[test]
    fn visit_u8_empty() {
        let expected: Result<u8> = Err(Syntax::new(1, 1).expected("u8").into());
        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u8 correctly errors upon an invalid character.
    #[test]
    fn visit_u8_invalid_character() {
        let expected: Result<u8> = Err(Syntax::new(1, 2).unexpected(".").into());
        let actual = Json::new().deserialize(&"1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u8 correctly errors upon an invalid whitespace.
    #[test]
    fn visit_u8_invalid_whitespace() {
        let expected: Result<u8> = Err(Syntax::new(1, 2).unexpected(" ").into());
        let actual = Json::new().deserialize(&"1 2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u8 correctly errors upon an invalid newline.
    #[test]
    fn visit_u8_invalid_newline() {
        let expected: Result<u8> = Err(Syntax::new(1, 2).unexpected("\n").into());
        let actual = Json::new().deserialize(&"1\n2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u8 correctly errors upon overflow.
    #[test]
    fn visit_u8_overflow() {
        let value = u8::MAX.to_string() + "0";
        let expected: Result<u8> = Err(Overflow::new(1, 1).kind("u8").into());
        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u8 correctly errors upon negative values.
    #[test]
    fn visit_u8_negative() {
        let expected: Result<u8> = Err(Syntax::new(1, 1).unexpected("-").into());
        let actual = Json::new().deserialize(&"-1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u16 correctly deserializes an u16 type.
    #[test]
    fn visit_u16_positive() {
        let expected = Ok(1_u16);
        let actual = Json::new().deserialize(&"1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u16 correctly deserializes a zero u16 type.
    #[test]
    fn visit_u16_zero() {
        let expected = Ok(0_u16);
        let actual = Json::new().deserialize(&"0");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u16 correctly deserializes an u16 with surrounding whitespace.
    #[test]
    fn visit_u16_surrounding_whitespace() {
        let expected = Ok(0_u16);
        let actual = Json::new().deserialize(&" \n0  ");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u16 correctly errors upon empty value.
    #[test]
    fn visit_u16_empty() {
        let expected: Result<u16> = Err(Syntax::new(1, 1).expected("u16").into());
        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u16 correctly errors upon an invalid character.
    #[test]
    fn visit_u16_invalid_character() {
        let expected: Result<u16> = Err(Syntax::new(1, 2).unexpected(".").into());
        let actual = Json::new().deserialize(&"1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u16 correctly errors upon an invalid whitespace.
    #[test]
    fn visit_u16_invalid_whitespace() {
        let expected: Result<u16> = Err(Syntax::new(1, 2).unexpected(" ").into());
        let actual = Json::new().deserialize(&"1 2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u16 correctly errors upon an invalid newline.
    #[test]
    fn visit_u16_invalid_newline() {
        let expected: Result<u16> = Err(Syntax::new(1, 2).unexpected("\n").into());
        let actual = Json::new().deserialize(&"1\n2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u16 correctly errors upon overflow.
    #[test]
    fn visit_u16_overflow() {
        let value = u16::MAX.to_string() + "0";
        let expected: Result<u16> = Err(Overflow::new(1, 1).kind("u16").into());
        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u16 correctly errors upon negative values.
    #[test]
    fn visit_u16_negative() {
        let expected: Result<u16> = Err(Syntax::new(1, 1).unexpected("-").into());
        let actual = Json::new().deserialize(&"-1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u32 correctly deserializes an u32 type.
    #[test]
    fn visit_u32_positive() {
        let expected = Ok(1_u32);
        let actual = Json::new().deserialize(&"1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u32 correctly deserializes a zero u32 type.
    #[test]
    fn visit_u32_zero() {
        let expected = Ok(0_u32);
        let actual = Json::new().deserialize(&"0");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u32 correctly deserializes an u32 with surrounding whitespace.
    #[test]
    fn visit_u32_surrounding_whitespace() {
        let expected = Ok(0_u32);
        let actual = Json::new().deserialize(&" \n0  ");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u32 correctly errors upon empty value.
    #[test]
    fn visit_u32_empty() {
        let expected: Result<u32> = Err(Syntax::new(1, 1).expected("u32").into());
        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u32 correctly errors upon an invalid character.
    #[test]
    fn visit_u32_invalid_character() {
        let expected: Result<u32> = Err(Syntax::new(1, 2).unexpected(".").into());
        let actual = Json::new().deserialize(&"1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u32 correctly errors upon an invalid whitespace.
    #[test]
    fn visit_u32_invalid_whitespace() {
        let expected: Result<u32> = Err(Syntax::new(1, 2).unexpected(" ").into());
        let actual = Json::new().deserialize(&"1 2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u32 correctly errors upon an invalid newline.
    #[test]
    fn visit_u32_invalid_newline() {
        let expected: Result<u32> = Err(Syntax::new(1, 2).unexpected("\n").into());
        let actual = Json::new().deserialize(&"1\n2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u32 correctly errors upon overflow.
    #[test]
    fn visit_u32_overflow() {
        let value = u32::MAX.to_string() + "0";
        let expected: Result<u32> = Err(Overflow::new(1, 1).kind("u32").into());
        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u32 correctly errors upon negative values.
    #[test]
    fn visit_u32_negative() {
        let expected: Result<u32> = Err(Syntax::new(1, 1).unexpected("-").into());
        let actual = Json::new().deserialize(&"-1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u64 correctly deserializes an u64 type.
    #[test]
    fn visit_u64_positive() {
        let expected = Ok(1_u64);
        let actual = Json::new().deserialize(&"1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u64 correctly deserializes a zero u64 type.
    #[test]
    fn visit_u64_zero() {
        let expected = Ok(0_u64);
        let actual = Json::new().deserialize(&"0");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u64 correctly deserializes an u64 with surrounding whitespace.
    #[test]
    fn visit_u64_surrounding_whitespace() {
        let expected = Ok(0_u64);
        let actual = Json::new().deserialize(&" \n0  ");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u64 correctly errors upon empty value.
    #[test]
    fn visit_u64_empty() {
        let expected: Result<u64> = Err(Syntax::new(1, 1).expected("u64").into());
        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u64 correctly errors upon an invalid character.
    #[test]
    fn visit_u64_invalid_character() {
        let expected: Result<u64> = Err(Syntax::new(1, 2).unexpected(".").into());
        let actual = Json::new().deserialize(&"1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u64 correctly errors upon an invalid whitespace.
    #[test]
    fn visit_u64_invalid_whitespace() {
        let expected: Result<u64> = Err(Syntax::new(1, 2).unexpected(" ").into());
        let actual = Json::new().deserialize(&"1 2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u64 correctly errors upon an invalid newline.
    #[test]
    fn visit_u64_invalid_newline() {
        let expected: Result<u64> = Err(Syntax::new(1, 2).unexpected("\n").into());
        let actual = Json::new().deserialize(&"1\n2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u64 correctly errors upon overflow.
    #[test]
    fn visit_u64_overflow() {
        let value = u64::MAX.to_string() + "0";
        let expected: Result<u64> = Err(Overflow::new(1, 1).kind("u64").into());
        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u64 correctly errors upon negative values.
    #[test]
    fn visit_u64_negative() {
        let expected: Result<u64> = Err(Syntax::new(1, 1).unexpected("-").into());
        let actual = Json::new().deserialize(&"-1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u128 correctly deserializes an u128 type.
    #[test]
    fn visit_u128_positive() {
        let expected = Ok(1_u128);
        let actual = Json::new().deserialize(&"1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u128 correctly deserializes a zero u128 type.
    #[test]
    fn visit_u128_zero() {
        let expected = Ok(0_u128);
        let actual = Json::new().deserialize(&"0");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u128 correctly deserializes an u128 with surrounding whitespace.
    #[test]
    fn visit_u128_surrounding_whitespace() {
        let expected = Ok(0_u128);
        let actual = Json::new().deserialize(&" \n0  ");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u128 correctly errors upon empty value.
    #[test]
    fn visit_u128_empty() {
        let expected: Result<u128> = Err(Syntax::new(1, 1).expected("u128").into());
        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u128 correctly errors upon an invalid character.
    #[test]
    fn visit_u128_invalid_character() {
        let expected: Result<u128> = Err(Syntax::new(1, 2).unexpected(".").into());
        let actual = Json::new().deserialize(&"1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u128 correctly errors upon an invalid whitespace.
    #[test]
    fn visit_u128_invalid_whitespace() {
        let expected: Result<u128> = Err(Syntax::new(1, 2).unexpected(" ").into());
        let actual = Json::new().deserialize(&"1 2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u128 correctly errors upon an invalid newline.
    #[test]
    fn visit_u128_invalid_newline() {
        let expected: Result<u128> = Err(Syntax::new(1, 2).unexpected("\n").into());
        let actual = Json::new().deserialize(&"1\n2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u128 correctly errors upon overflow.
    #[test]
    fn visit_u128_overflow() {
        let value = u128::MAX.to_string() + "0";
        let expected: Result<u128> = Err(Overflow::new(1, 1).kind("u128").into());
        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_u128 correctly errors upon negative values.
    #[test]
    fn visit_u128_negative() {
        let expected: Result<u128> = Err(Syntax::new(1, 1).unexpected("-").into());
        let actual = Json::new().deserialize(&"-1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_unit correctly deserializes a unit type.
    #[test]
    fn visit_unit_correct() {
        let expected = Ok(());
        let actual = Json::new().deserialize(&"null");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_unit correctly deserializes with whitespace.
    #[test]
    fn visit_unit_whitespace() {
        let expected = Ok(());
        let actual = Json::new().deserialize(&" \nnull  ");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_unit correctly errors upon unexpected value.
    #[test]
    fn visit_unit_incorrect() {
        let expected: Result<()> =
            Err(Syntax::new(1, 1).unexpected("fail").expected("null").into());
        let actual = Json::new().deserialize(&"fail");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_usize correctly deserializes an usize type.
    #[test]
    fn visit_usize_positive() {
        let expected = Ok(1_usize);
        let actual = Json::new().deserialize(&"1");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_usize correctly deserializes a zero usize type.
    #[test]
    fn visit_usize_zero() {
        let expected = Ok(0_usize);
        let actual = Json::new().deserialize(&"0");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_usize correctly deserializes an usize with surrounding whitespace.
    #[test]
    fn visit_usize_surrounding_whitespace() {
        let expected = Ok(0_usize);
        let actual = Json::new().deserialize(&" \n0  ");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_usize correctly errors upon empty value.
    #[test]
    fn visit_usize_empty() {
        let expected: Result<usize> = Err(Syntax::new(1, 1).expected("usize").into());
        let actual = Json::new().deserialize(&"");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_usize correctly errors upon an invalid character.
    #[test]
    fn visit_usize_invalid_character() {
        let expected: Result<usize> = Err(Syntax::new(1, 2).unexpected(".").into());
        let actual = Json::new().deserialize(&"1.2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_usize correctly errors upon an invalid whitespace.
    #[test]
    fn visit_usize_invalid_whitespace() {
        let expected: Result<usize> = Err(Syntax::new(1, 2).unexpected(" ").into());
        let actual = Json::new().deserialize(&"1 2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_usize correctly errors upon an invalid newline.
    #[test]
    fn visit_usize_invalid_newline() {
        let expected: Result<usize> = Err(Syntax::new(1, 2).unexpected("\n").into());
        let actual = Json::new().deserialize(&"1\n2");
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_usize correctly errors upon overflow.
    #[test]
    fn visit_usize_overflow() {
        let value = u128::MAX.to_string() + "0";
        let expected: Result<usize> = Err(Overflow::new(1, 1).kind("usize").into());
        let actual = Json::new().deserialize(&value.as_str());
        assert_eq!(expected, actual);
    }

    /// Test Json::visit_usize correctly errors upon negative values.
    #[test]
    fn visit_usize_negative() {
        let expected: Result<usize> = Err(Syntax::new(1, 1).unexpected("-").into());
        let actual = Json::new().deserialize(&"-1");
        assert_eq!(expected, actual);
    }
}

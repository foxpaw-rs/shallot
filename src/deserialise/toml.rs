//! Serialise::Toml Module
//!
//! Contains implementation for the Toml Deerialiser.
use crate::deserialise::{Deserialise, Deserialiser, ParseError};
use std::marker::PhantomData;

/// Toml.
///
/// Toml deserialiser for the Shallot library. Will deserialise Deserialise
/// implementing structs into a TOML format. Note that if a Toml object is not
/// immediately chained as shown in the example, the intended Deserialise type
/// `T` will likely need to be annotated.
///
/// # Examples
/// ```rust
/// use shallot::deserialise::{Toml, Deserialiser, ParseError};
///
/// fn main() -> Result<(), ParseError> {
///     let content = "1";
///     let deserialiser = Toml::new(content);
///     let data: u8 = deserialiser.deserialise()?;
///     Ok(())
/// }
/// ```
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Toml<'c, T>  where T: Deserialise {
    /// The content to serialise.
    content: &'c str,

    /// The PhantonData housing the type to deserialise into.
    phantom: PhantomData<T>,
}

impl<'c, T> Toml<'c, T> where T: Deserialise {
    /// Create a new Toml.
    ///
    /// Create a new Toml instance.
    ///
    /// # Example
    /// ```rust
    /// use shallot::deserialise::Toml;
    ///
    /// let content = "1";
    /// let deserialiser: Toml<'_, u8> = Toml::new(&content);
    /// ```
    pub fn new(content: &'c str) -> Self {
        Self { 
            content,
            phantom: PhantomData
        }
    }
}

impl<'c, 'de, T> Deserialiser<'de, T> for Toml<'c, T>
where
    T: Deserialise,
{
    /// Deserialise.
    ///
    /// Deserialise the current deserialiser.
    ///
    /// # Example
    /// ```rust
    /// use shallot::deserialise::{Deserialiser, Toml, ParseError};
    ///
    /// fn main() -> Result<(), ParseError> {
    ///     let content = "1";
    ///     let deserialised: u8 = Toml::new(content).deserialise()?;
    ///     Ok(())
    /// }
    /// ```
    fn deserialise(&self) -> Result<T, ParseError> {
        T::accept::<T>(self, self.content)
    }

    /// Visit U8.
    ///
    /// Visit a u8 node and parse it for deserialisation.
    ///
    /// # Example
    /// ```rust
    /// use shallot::deserialise::{Deserialiser, Toml, ParseError};
    ///
    /// fn main() -> Result<(), ParseError> {
    ///     let content = "1";
    ///     let deserialiser: Toml<'_, u8> = Toml::new(&content);
    ///     assert_eq!("1", deserialiser.visit_u8(content)?);
    ///     Ok(())
    /// }
    /// ```
    fn visit_u8(&'de self, content: &'de str) -> Result<&'de str, ParseError> {
        Ok(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Toml::new must create as per struct initialisation.
    ///
    /// The new method on Toml must create an object as per the struct
    /// initialiser syntax.
    #[test]
    fn toml_new() {
        let content = "1";
        let expected: Toml<'_, u8> = Toml { content: &content, phantom: PhantomData };
        let actual: Toml<'_, u8> = Toml::new(&content);

        assert_eq!(expected, actual);
    }

    /// Toml::visit_u8 must correctly deserialise a u8.
    ///
    /// The visit_u8 method must correctly deserialise u8 values.
    #[test]
    fn toml_visit_u8() {
        let content = "1";
        let toml: Toml<'_, u8> = Toml::new(&content);

        assert_eq!("1", toml.visit_u8(content).unwrap());
        assert_eq!(1_u8, toml.deserialise().unwrap());
    }


    /// Toml::visit_u8 must error on invalid u8.
    ///
    /// The visit_u8 method must correctly return the value, however, deserialise
    /// must correctly error on an invalid u8 token.
    #[test]
    fn toml_visit_u8_invalid() {
        let content = "a";
        let toml: Toml<'_, u8> = Toml::new(&content);

        assert_eq!("a", toml.visit_u8(content).unwrap());
        assert_eq!(ParseError::new("Invalid value 'a' for type 'u8'."), toml.deserialise().unwrap_err());
    }
}

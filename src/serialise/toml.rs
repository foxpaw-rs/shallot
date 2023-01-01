//! Serialise::Toml Module
//!
//! Contains implementation for the Toml Serialiser.
use crate::serialise::{Serialise, Serialiser};
use std::fmt::{self, Display, Formatter};

/// Toml.
///
/// Toml serialiser for the Shallot library. Will serialise Serialise
/// implementing structs into a TOML format.
///
/// # Examples
/// ```rust
/// use shallot::serialise::{Toml, Serialiser};
///
/// let content: u8 = 1;
/// let serialiser = Toml::new(&content);
/// let data = serialiser.serialise();
/// ```
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Toml<'a, C>
where
    C: 'a + Serialise,
{
    /// The content to serialise.
    content: &'a C,
}

impl<'a, C> Toml<'a, C>
where
    C: 'a + Serialise,
{
    /// Create a new Toml.
    ///
    /// Create a new Toml instance.
    ///
    /// # Example
    /// ```rust
    /// use shallot::serialise::Toml;
    ///
    /// let content: u8 = 1;
    /// let serialiser = Toml::new(&content);
    /// ```
    pub fn new(content: &'a C) -> Self {
        Self { content }
    }
}

impl<'a, C> Serialiser for Toml<'a, C>
where
    C: 'a + Serialise,
{
    /// Serialise.
    ///
    /// Serialise the current serialiser.
    ///
    /// # Example
    /// ```rust
    /// use shallot::serialise::{Toml, Serialiser};
    ///
    /// let content: u8 = 1;
    /// let serialised = Toml::new(&content).serialise();
    /// ```
    fn serialise(&self) -> String {
        self.content.accept(self)
    }

    /// Visit U8.
    ///
    /// Visit a u8 node and serialise it.
    ///
    /// # Example
    /// ```rust
    /// use shallot::serialise::{Toml, Serialiser};
    ///
    /// let content: u8 = 1;
    /// let serialiser = Toml::new(&content);
    /// assert_eq!("1", serialiser.visit_u8(&content));
    /// ```
    fn visit_u8(&self, value: &u8) -> String {
        value.to_string()
    }
}

impl<'a, C> Display for Toml<'a, C>
where
    C: 'a + Serialise,
{
    /// Format a Toml for Dispay.
    ///
    /// Formats the Toml for Display printing.
    ///
    /// # Example
    /// ```rust
    /// use shallot::serialise::Toml;
    ///
    /// let content: u8 = 1;
    /// let serialiser = Toml::new(&content);
    /// println!("{}", serialiser);
    /// ```
    /// # Error
    /// Will error if the underlying write macro fails.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.serialise())
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
        let content: u8 = 1;
        let expected = Toml { content: &content };
        let actual = Toml::new(&content);

        assert_eq!(expected, actual);
    }

    /// Toml::visit_u8 must correctly serialise a u8.
    ///
    /// The visit_u8 method must correctly serialise u8 values.
    #[test]
    fn toml_visit_u8() {
        let expected = "1";
        let content: u8 = 1;
        let toml = Toml::new(&content);
        let actual = toml.visit_u8(&content);

        assert_eq!(expected, actual);
        assert_eq!(expected, toml.serialise());
    }
}

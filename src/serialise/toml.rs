use crate::serialise::{Serialise, Serialiser};
use std::fmt::{self, Display, Formatter};

/// Toml.
///
/// Toml serialiser for the Shallot library. Will serialise and deserialise
/// Serialise implementing structs into TOML format.
///
/// # Examples
/// ```rust
/// use shallot::serialise::Toml;
///
/// let content: u8 = 1;
/// let serialiser = Toml::new(&content);
/// let data = serialiser.to_string();
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
    /// The expected output type.
    type Ok = String;
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
    /// ```
    /// use shallot::serialise::Toml;
    ///
    /// let content: u8 = 1;
    /// let serialiser = Toml::new(&content);
    /// println!("{}", serialiser);
    /// ```
    ///
    /// # Error
    /// Will error if the underlying write macro fails.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", "Toml")
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
    fn json_new() {
        let value: u8 = 1;
        let expected = Toml { content: &value };
        let actual = Toml::new(&value);

        assert_eq!(expected, actual);
    }
}

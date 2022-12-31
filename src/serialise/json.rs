//! Serialise::Json Module
//!
//! Contains implementation for the Json Serialiser.
use crate::serialise::{Serialise, Serialiser};
use std::fmt::{self, Display, Formatter};

/// Json.
///
/// Json serialiser for the Shallot library. Will serialise and deserialise
/// Serialise implementing structs into a JSON format.
///
/// # Examples
/// ```rust
/// use shallot::serialise::{Json, Serialiser};
///
/// let content: u8 = 1;
/// let serialiser = Json::new(&content);
/// let data = serialiser.serialise();
/// ```
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Json<'a, C>
where
    C: 'a + Serialise,
{
    /// The content to serialise.
    content: &'a C,
}

impl<'a, C> Json<'a, C>
where
    C: 'a + Serialise,
{
    /// Create a new Json.
    ///
    /// Create a new Json instance.
    ///
    /// # Example
    /// ```rust
    /// use shallot::serialise::Json;
    ///
    /// let content: u8 = 1;
    /// let serialiser = Json::new(&content);
    /// ```
    pub fn new(content: &'a C) -> Self {
        Self { content }
    }
}

impl<'a, C> Serialiser for Json<'a, C>
where
    C: 'a + Serialise,
{
    /// Serialise.
    ///
    /// Serialise the current serialiser.
    ///
    /// # Example
    /// ```rust
    /// use shallot::serialise::{Json, Serialiser};
    ///
    /// let content: u8 = 1;
    /// let serialised = Json::new(&content).serialise();
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
    /// use shallot::serialise::{Json, Serialiser};
    ///
    /// let content: u8 = 1;
    /// let serialiser = Json::new(&content);
    /// assert_eq!("1", serialiser.visit_u8(&content));
    /// ```
    fn visit_u8(&self, value: &u8) -> String {
        value.to_string()
    }
}

impl<'a, C> Display for Json<'a, C>
where
    C: 'a + Serialise,
{
    /// Format a Json for Dispay.
    ///
    /// Formats the Json for Display printing.
    ///
    /// # Example
    /// ```rust
    /// use shallot::serialise::Json;
    ///
    /// let content: u8 = 1;
    /// let serialiser = Json::new(&content);
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

    /// Json::new must create as per struct initialisation.
    ///
    /// The new method on Json must create an object as per the struct
    /// initialiser syntax.
    #[test]
    fn json_new() {
        let content: u8 = 1;
        let expected = Json { content: &content };
        let actual = Json::new(&content);

        assert_eq!(expected, actual);
    }

    /// Json::visit_u8 must correctly serialise a u8.
    ///
    /// The visit_u8 method must correctly serialise u8 values.
    #[test]
    fn json_visit_u8() {
        let expected = "1";
        let content: u8 = 1;
        let json = Json::new(&content);
        let actual = json.visit_u8(&content);

        assert_eq!(expected, actual);
        assert_eq!(expected, json.serialise());
    }
}

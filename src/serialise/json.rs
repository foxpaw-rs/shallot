use crate::serialise::{Serialise, Serialiser};
use std::fmt::{self, Display, Formatter};

/// Json.
///
/// Json serialiser for the Shallot library. Will serialise and deserialise
/// Serialise implementing structs into a JSON format.
///
/// # Examples
/// ```rust
/// use shallot::serialise::Json;
///
/// let content: u8 = 1;
/// let serialiser = Json::new(&content);
/// let data = serialiser.to_string();
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
    /// The expected output type.
    type Ok = String;
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
    /// ```
    /// use shallot::serialise::Json;
    ///
    /// let content: u8 = 1;
    /// let serialiser = Json::new(&content);
    /// println!("{}", serialiser);
    /// ```
    ///
    /// # Error
    /// Will error if the underlying write macro fails.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", "Json")
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
        let value: u8 = 1;
        let expected = Json { content: &value };
        let actual = Json::new(&value);

        assert_eq!(expected, actual);
    }
}

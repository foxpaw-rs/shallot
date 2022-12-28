use crate::serialise::{Serialise, Serialiser};
use std::fmt::{self, Display, Formatter};

/// Table.
///
/// Table serialiser for the Shallot library. Will serialise and deserialise
/// Serialise implementing structs into tabular format.
///
/// # Examples
/// ```rust
/// use shallot::serialise::Table;
///
/// let content: u8 = 1;
/// let serialiser = Table::new(&content);
/// let data = serialiser.to_string();
/// ```
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Table<'a, C>
where
    C: 'a + Serialise,
{
    /// The content to serialise.
    content: &'a C,
}

impl<'a, C> Table<'a, C>
where
    C: 'a + Serialise,
{
    /// Create a new Table.
    ///
    /// Create a new Table instance.
    ///
    /// # Example
    /// ```rust
    /// use shallot::serialise::Table;
    ///
    /// let content: u8 = 1;
    /// let serialiser = Table::new(&content);
    /// ```
    pub fn new(content: &'a C) -> Self {
        Self { content }
    }
}

impl<'a, C> Serialiser for Table<'a, C>
where
    C: 'a + Serialise,
{
    /// The expected output type.
    type Ok = String;
}

impl<'a, C> Display for Table<'a, C>
where
    C: 'a + Serialise,
{
    /// Format a Table for Dispay.
    ///
    /// Formats the Table for Display printing.
    ///
    /// # Example
    /// ```
    /// use shallot::serialise::Table;
    ///
    /// let content: u8 = 1;
    /// let serialiser = Table::new(&content);
    /// println!("{}", serialiser);
    /// ```
    ///
    /// # Error
    /// Will error if the underlying write macro fails.
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", "Table")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Table::new must create as per struct initialisation.
    ///
    /// The new method on Table must create an object as per the struct
    /// initialiser syntax.
    #[test]
    fn json_new() {
        let value: u8 = 1;
        let expected = Table { content: &value };
        let actual = Table::new(&value);

        assert_eq!(expected, actual);
    }
}

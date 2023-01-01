//! Serialise::Table Module
//!
//! Contains implementation for the Table Serialiser.
use crate::serialise::{Serialise, Serialiser};
use std::fmt::{self, Display, Formatter};

/// Table.
///
/// Table serialiser for the Shallot library. Will serialise Serialise
/// implementing structs into a Tabluar format.
///
/// # Examples
/// ```rust
/// use shallot::serialise::{Table, Serialiser};
///
/// let content: u8 = 1;
/// let serialiser = Table::new(&content);
/// let data = serialiser.serialise();
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
    /// Serialise.
    ///
    /// Serialise the current serialiser.
    ///
    /// # Example
    /// ```rust
    /// use shallot::serialise::{Table, Serialiser};
    ///
    /// let content: u8 = 1;
    /// let serialised = Table::new(&content).serialise();
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
    /// use shallot::serialise::{Table, Serialiser};
    ///
    /// let content: u8 = 1;
    /// let serialiser = Table::new(&content);
    /// assert_eq!("1", serialiser.visit_u8(&content));
    /// ```
    fn visit_u8(&self, value: &u8) -> String {
        value.to_string()
    }
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
    /// ```rust
    /// use shallot::serialise::Table;
    ///
    /// let content: u8 = 1;
    /// let serialiser = Table::new(&content);
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

    /// Table::new must create as per struct initialisation.
    ///
    /// The new method on Table must create an object as per the struct
    /// initialiser syntax.
    #[test]
    fn table_new() {
        let content: u8 = 1;
        let expected = Table { content: &content };
        let actual = Table::new(&content);

        assert_eq!(expected, actual);
    }

    /// Table::visit_u8 must correctly serialise a u8.
    ///
    /// The visit_u8 method must correctly serialise u8 values.
    #[test]
    fn table_visit_u8() {
        let expected = "1";
        let content: u8 = 1;
        let table = Table::new(&content);
        let actual = table.visit_u8(&content);

        assert_eq!(expected, actual);
        assert_eq!(expected, table.serialise());
    }
}

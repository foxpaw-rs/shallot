//! Serialise::Table Module
//!
//! Contains implementation for the Table Deerialiser.
use crate::deserialise::{Deserialise, Deserialiser, ParseError};
use std::marker::PhantomData;

/// Table.
///
/// Table deserialiser for the Shallot library. Will deserialise Deserialise
/// implementing structs into a tabular format. Note that if a Table object is
/// not immediately chained as shown in the example, the intended Deserialise
/// type `T` will likely need to be annotated.
///
/// # Examples
/// ```rust
/// use shallot::deserialise::{Table, Deserialiser, ParseError};
///
/// fn main() -> Result<(), ParseError> {
///     let content = "1";
///     let deserialiser = Table::new(content);
///     let data: u8 = deserialiser.deserialise()?;
///     Ok(())
/// }
/// ```
#[derive(Copy, Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Table<'c, T>  where T: Deserialise {
    /// The content to serialise.
    content: &'c str,

    /// The PhantonData housing the type to deserialise into.
    phantom: PhantomData<T>,
}

impl<'c, T> Table<'c, T> where T: Deserialise {
    /// Create a new Table.
    ///
    /// Create a new Table instance.
    ///
    /// # Example
    /// ```rust
    /// use shallot::deserialise::Table;
    ///
    /// let content = "1";
    /// let deserialiser: Table<'_, u8> = Table::new(&content);
    /// ```
    pub fn new(content: &'c str) -> Self {
        Self { 
            content,
            phantom: PhantomData
        }
    }
}

impl<'c, 'de, T> Deserialiser<'de, T> for Table<'c, T>
where
    T: Deserialise,
{
    /// Deserialise.
    ///
    /// Deserialise the current deserialiser.
    ///
    /// # Example
    /// ```rust
    /// use shallot::deserialise::{Deserialiser, Table, ParseError};
    ///
    /// fn main() -> Result<(), ParseError> {
    ///     let content = "1";
    ///     let deserialised: u8 = Table::new(content).deserialise()?;
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
    /// use shallot::deserialise::{Deserialiser, Table, ParseError};
    ///
    /// fn main() -> Result<(), ParseError> {
    ///     let content = "1";
    ///     let deserialiser: Table<'_, u8> = Table::new(&content);
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

    /// Table::new must create as per struct initialisation.
    ///
    /// The new method on Table must create an object as per the struct
    /// initialiser syntax.
    #[test]
    fn table_new() {
        let content = "1";
        let expected: Table<'_, u8> = Table { content: &content, phantom: PhantomData };
        let actual: Table<'_, u8> = Table::new(&content);

        assert_eq!(expected, actual);
    }

    /// Table::visit_u8 must correctly deserialise a u8.
    ///
    /// The visit_u8 method must correctly deserialise u8 values.
    #[test]
    fn table_visit_u8() {
        let content = "1";
        let table: Table<'_, u8> = Table::new(&content);

        assert_eq!("1", table.visit_u8(content).unwrap());
        assert_eq!(1_u8, table.deserialise().unwrap());
    }


    /// Table::visit_u8 must error on invalid u8.
    ///
    /// The visit_u8 method must correctly return the value, however, deserialise
    /// must correctly error on an invalid u8 token.
    #[test]
    fn table_visit_u8_invalid() {
        let content = "a";
        let table: Table<'_, u8> = Table::new(&content);

        assert_eq!("a", table.visit_u8(content).unwrap());
        assert_eq!(ParseError::new("Invalid value 'a' for type 'u8'."), table.deserialise().unwrap_err());
    }
}

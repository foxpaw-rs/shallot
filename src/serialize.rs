//! Serialize module which houses the Serialize and Serializer traits used to
//! handle the serialization process. Also houses the implementation of
//! Serialize on supported core items.

mod json;
pub use json::Json;

/// Trait to implement on serializable items. Defines how the item is
/// serialized.
pub trait Serialize {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer;
}

impl Serialize for () {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_unit()
    }
}

impl<A> Serialize for (A,)
where
    A: Serialize,
{
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_tuple_1(self)
    }
}

impl<A, B> Serialize for (A, B)
where
    A: Serialize,
    B: Serialize,
{
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_tuple_2(self)
    }
}

impl<A, B, C> Serialize for (A, B, C)
where
    A: Serialize,
    B: Serialize,
    C: Serialize,
{
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_tuple_3(self)
    }
}

impl<A, B, C, D> Serialize for (A, B, C, D)
where
    A: Serialize,
    B: Serialize,
    C: Serialize,
    D: Serialize,
{
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_tuple_4(self)
    }
}

impl<A, B, C, D, E> Serialize for (A, B, C, D, E)
where
    A: Serialize,
    B: Serialize,
    C: Serialize,
    D: Serialize,
    E: Serialize,
{
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_tuple_5(self)
    }
}

impl<A, B, C, D, E, F> Serialize for (A, B, C, D, E, F)
where
    A: Serialize,
    B: Serialize,
    C: Serialize,
    D: Serialize,
    E: Serialize,
    F: Serialize,
{
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_tuple_6(self)
    }
}

impl<A, B, C, D, E, F, G> Serialize for (A, B, C, D, E, F, G)
where
    A: Serialize,
    B: Serialize,
    C: Serialize,
    D: Serialize,
    E: Serialize,
    F: Serialize,
    G: Serialize,
{
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_tuple_7(self)
    }
}

impl<A, B, C, D, E, F, G, H> Serialize for (A, B, C, D, E, F, G, H)
where
    A: Serialize,
    B: Serialize,
    C: Serialize,
    D: Serialize,
    E: Serialize,
    F: Serialize,
    G: Serialize,
    H: Serialize,
{
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_tuple_8(self)
    }
}

impl<A, B, C, D, E, F, G, H, I> Serialize for (A, B, C, D, E, F, G, H, I)
where
    A: Serialize,
    B: Serialize,
    C: Serialize,
    D: Serialize,
    E: Serialize,
    F: Serialize,
    G: Serialize,
    H: Serialize,
    I: Serialize,
{
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_tuple_9(self)
    }
}

impl<A, B, C, D, E, F, G, H, I, J> Serialize for (A, B, C, D, E, F, G, H, I, J)
where
    A: Serialize,
    B: Serialize,
    C: Serialize,
    D: Serialize,
    E: Serialize,
    F: Serialize,
    G: Serialize,
    H: Serialize,
    I: Serialize,
    J: Serialize,
{
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_tuple_10(self)
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K> Serialize for (A, B, C, D, E, F, G, H, I, J, K)
where
    A: Serialize,
    B: Serialize,
    C: Serialize,
    D: Serialize,
    E: Serialize,
    F: Serialize,
    G: Serialize,
    H: Serialize,
    I: Serialize,
    J: Serialize,
    K: Serialize,
{
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_tuple_11(self)
    }
}

impl<A, B, C, D, E, F, G, H, I, J, K, L> Serialize for (A, B, C, D, E, F, G, H, I, J, K, L)
where
    A: Serialize,
    B: Serialize,
    C: Serialize,
    D: Serialize,
    E: Serialize,
    F: Serialize,
    G: Serialize,
    H: Serialize,
    I: Serialize,
    J: Serialize,
    K: Serialize,
    L: Serialize,
{
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_tuple_12(self)
    }
}

impl Serialize for bool {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_bool(self)
    }
}

impl Serialize for char {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_char(self)
    }
}

impl Serialize for f32 {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_f32(self)
    }
}

impl Serialize for f64 {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_f64(self)
    }
}

impl Serialize for i8 {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_i8(self)
    }
}

impl Serialize for i16 {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_i16(self)
    }
}

impl Serialize for i32 {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_i32(self)
    }
}

impl Serialize for i64 {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_i64(self)
    }
}

impl Serialize for i128 {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_i128(self)
    }
}

impl Serialize for isize {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_isize(self)
    }
}

impl Serialize for str {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_str(self)
    }
}

impl Serialize for String {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_string(self)
    }
}

impl Serialize for u8 {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_u8(self)
    }
}

impl Serialize for u16 {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_u16(self)
    }
}

impl Serialize for u32 {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_u32(self)
    }
}

impl Serialize for u64 {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_u64(self)
    }
}

impl Serialize for u128 {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_u128(self)
    }
}

impl Serialize for usize {
    /// Accept a serializer, allowing it to serialize this item. Note that this is
    /// an internal method used to serialize from the Serializer and is uncommon to
    /// use outside this library.
    fn accept<S>(&self, serializer: &S) -> S::Output
    where
        S: Serializer,
    {
        serializer.visit_usize(self)
    }
}

/// Trait to implement on an item that conducts the serialization, and defines
/// how data is serialized. Interaction with this should be done using the
/// serialize method, which in turn calls the required visit methods to
/// convert into the output type.
pub trait Serializer {
    /// The output type for this Serializer.
    type Output;

    /// Serialize the input into the required output type.
    fn serialize<S>(&self, input: &S) -> Self::Output
    where
        S: Serialize + ?Sized;

    /// Visit and serialize a bool type.
    fn visit_bool(&self, input: &bool) -> Self::Output;

    /// Visit and serialize a char type.
    fn visit_char(&self, input: &char) -> Self::Output;

    /// Visit and serialize a f32 type.
    fn visit_f32(&self, input: &f32) -> Self::Output;

    /// Visit and serialize a f64 type.
    fn visit_f64(&self, input: &f64) -> Self::Output;

    /// Visit and serialize an i8 type.
    fn visit_i8(&self, input: &i8) -> Self::Output;

    /// Visit and serialize an i16 type.
    fn visit_i16(&self, input: &i16) -> Self::Output;

    /// Visit and serialize an i32 type.
    fn visit_i32(&self, input: &i32) -> Self::Output;

    /// Visit and serialize an i64 type.
    fn visit_i64(&self, input: &i64) -> Self::Output;

    /// Visit and serialize an i128 type.
    fn visit_i128(&self, input: &i128) -> Self::Output;

    /// Visit and serialize an isize type.
    fn visit_isize(&self, input: &isize) -> Self::Output;

    /// Visit and serialize a str type.
    fn visit_str(&self, input: &str) -> Self::Output;

    /// Visit and serialize a String type.
    #[allow(clippy::ptr_arg)]
    fn visit_string(&self, input: &String) -> Self::Output;

    /// Visit and serialize a tuple type of size 1.
    #[allow(clippy::type_complexity)]
    fn visit_tuple_1<A>(&self, input: &(A,)) -> Self::Output
    where
        A: Serialize;

    /// Visit and serialize a tuple type of size 2.
    #[allow(clippy::type_complexity)]
    fn visit_tuple_2<A, B>(&self, input: &(A, B)) -> Self::Output
    where
        A: Serialize,
        B: Serialize;

    /// Visit and serialize a tuple type of size 3.
    #[allow(clippy::type_complexity)]
    fn visit_tuple_3<A, B, C>(&self, input: &(A, B, C)) -> Self::Output
    where
        A: Serialize,
        B: Serialize,
        C: Serialize;

    /// Visit and serialize a tuple type of size 4.
    #[allow(clippy::type_complexity)]
    fn visit_tuple_4<A, B, C, D>(&self, input: &(A, B, C, D)) -> Self::Output
    where
        A: Serialize,
        B: Serialize,
        C: Serialize,
        D: Serialize;

    /// Visit and serialize a tuple type of size 5.
    #[allow(clippy::type_complexity)]
    fn visit_tuple_5<A, B, C, D, E>(&self, input: &(A, B, C, D, E)) -> Self::Output
    where
        A: Serialize,
        B: Serialize,
        C: Serialize,
        D: Serialize,
        E: Serialize;

    /// Visit and serialize a tuple type of size 6.
    #[allow(clippy::type_complexity)]
    fn visit_tuple_6<A, B, C, D, E, F>(&self, input: &(A, B, C, D, E, F)) -> Self::Output
    where
        A: Serialize,
        B: Serialize,
        C: Serialize,
        D: Serialize,
        E: Serialize,
        F: Serialize;

    /// Visit and serialize a tuple type of size 7.
    #[allow(clippy::type_complexity)]
    fn visit_tuple_7<A, B, C, D, E, F, G>(&self, input: &(A, B, C, D, E, F, G)) -> Self::Output
    where
        A: Serialize,
        B: Serialize,
        C: Serialize,
        D: Serialize,
        E: Serialize,
        F: Serialize,
        G: Serialize;

    /// Visit and serialize a tuple type of size 8.
    #[allow(clippy::type_complexity)]
    fn visit_tuple_8<A, B, C, D, E, F, G, H>(
        &self,
        input: &(A, B, C, D, E, F, G, H),
    ) -> Self::Output
    where
        A: Serialize,
        B: Serialize,
        C: Serialize,
        D: Serialize,
        E: Serialize,
        F: Serialize,
        G: Serialize,
        H: Serialize;

    /// Visit and serialize a tuple type of size 9.
    #[allow(clippy::type_complexity)]
    fn visit_tuple_9<A, B, C, D, E, F, G, H, I>(
        &self,
        input: &(A, B, C, D, E, F, G, H, I),
    ) -> Self::Output
    where
        A: Serialize,
        B: Serialize,
        C: Serialize,
        D: Serialize,
        E: Serialize,
        F: Serialize,
        G: Serialize,
        H: Serialize,
        I: Serialize;

    /// Visit and serialize a tuple type of size 10.
    #[allow(clippy::type_complexity)]
    fn visit_tuple_10<A, B, C, D, E, F, G, H, I, J>(
        &self,
        input: &(A, B, C, D, E, F, G, H, I, J),
    ) -> Self::Output
    where
        A: Serialize,
        B: Serialize,
        C: Serialize,
        D: Serialize,
        E: Serialize,
        F: Serialize,
        G: Serialize,
        H: Serialize,
        I: Serialize,
        J: Serialize;

    /// Visit and serialize a tuple type of size 11.
    #[allow(clippy::type_complexity)]
    fn visit_tuple_11<A, B, C, D, E, F, G, H, I, J, K>(
        &self,
        input: &(A, B, C, D, E, F, G, H, I, J, K),
    ) -> Self::Output
    where
        A: Serialize,
        B: Serialize,
        C: Serialize,
        D: Serialize,
        E: Serialize,
        F: Serialize,
        G: Serialize,
        H: Serialize,
        I: Serialize,
        J: Serialize,
        K: Serialize;

    /// Visit and serialize a tuple type of size 12.
    #[allow(clippy::type_complexity)]
    fn visit_tuple_12<A, B, C, D, E, F, G, H, I, J, K, L>(
        &self,
        input: &(A, B, C, D, E, F, G, H, I, J, K, L),
    ) -> Self::Output
    where
        A: Serialize,
        B: Serialize,
        C: Serialize,
        D: Serialize,
        E: Serialize,
        F: Serialize,
        G: Serialize,
        H: Serialize,
        I: Serialize,
        J: Serialize,
        K: Serialize,
        L: Serialize;

    /// Visit and serialize an u8 type.
    fn visit_u8(&self, input: &u8) -> Self::Output;

    /// Visit and serialize an u16 type.
    fn visit_u16(&self, input: &u16) -> Self::Output;

    /// Visit and serialize an u32 type.
    fn visit_u32(&self, input: &u32) -> Self::Output;

    /// Visit and serialize an u64 type.
    fn visit_u64(&self, input: &u64) -> Self::Output;

    /// Visit and serialize an u128 type.
    fn visit_u128(&self, input: &u128) -> Self::Output;

    /// Visit and serialize a unit type.
    fn visit_unit(&self) -> Self::Output;

    /// Visit and serialize an usize type.
    fn visit_usize(&self, input: &usize) -> Self::Output;
}

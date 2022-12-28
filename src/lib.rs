//! Todo(Paul): Crate Documentation once 0.1 complete.
//!
//! # Usage
//!
//! # Supported Types
//! ## Primitive Types
//! * bool
//! * i8, i16, i32, i64, i128, isize
//! * u8, u16, u32, u64, u128, usize
//! * f32, f64
//! * char
//! * str
//!
//! ## Compound Types
//! * T
//! * [T]
//! * [T; 0] through [T; 32]
//! * Tuples up to size 16
//!
//! ## Common standard library Types
//! * String
//! * Option<T>
//! * Result<T, E>
//! * PhantomData<T>
//!
//! ## Wrapper Types
//! * Box<T>
//! * Cow<'a, T>
//! * Cell<T>
//! * RefCell<T>
//! * Mutex<T>
//! * RwLock<T>
//! * Rc<T>
//! * Arc<T>
//!
//! ## Collection Types
//! * BTreeMap<K, V>
//! * BTreeSet<T>
//! * BinaryHeap<T>
//! * HashMap<K, V, H>
//! * HashSet<T, H>
//! * LinkedList<T>
//! * VecDeque<T>
//! * Vec<T>
//!
//! ## FFI Types
//! * CStr
//! * CString
//! * OsStr
//! * OsString
//!
//! ## Miscellaneous standard library Types
//! * Duration
//! * SystemTime
//! * Path
//! * PathBuf
//! * Range<T>
//! * RangeInclusive<T>
//! * Bound<T>
//! * num::NonZero
//! * `!`
//!
//! ## Net Types
//! * IpAddr
//! * Ipv4Addr
//! * Ipv6Addr
//! * SocketAddr
//! * SocketAddrV4
//! * SocketAddrV6
//!
//! # Adding Serialisation and Deserialisation to Types
//!
//! # Custom Serialiser and Deserialiser
//!
//! For further detailed examples, refer to the documentation.

pub mod serialise;

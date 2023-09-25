//! # ESDE (SE-rialize and DE-serialize library)
//!
//! The idea is to easily store non-recursive data structures to files and read them back.
//! "Easy" in that context means that the format is understandable by humans and can therefore, in theory, produced from other languages/frameworks as well.
//!
//! This crate is focused on storing data in binary form, but can be adapted to store it in other formats.
//! It also provides derive macros (`Deserialize` and `Serialize`) to derive the corresponding traits for structs and enums.
//!
//! **note on cross-platform:**
//! There are no particular guarantees.
//! Most importantly, lengths of slices/vectors are stored as [`usize`]s, the length of which can vary from platform to platform.
//!
//! ## `Item`s
//! Items are the units in which data is serialized.
//! This can be anything.
//! I guess in most cases, the data structure is serialized into binary data to store it in a file, then a [`u8`] will be used.
//!
//! ## deserialization
//! You have an object that provides `Item`s, implementing the [`Sender`] trait.
//! This trait is automatically implemented for any [`std::io::Read`], "sending" [`u8`]s.
//!
//! Objects that can be deserialized from a stream of given `Item`s implement the [`Deserialize<Item>`] trait.
//! Implementations for some primitive types are provided, most others should be derivable with the corresponding and intuitively named derive macro.
//!
//! If an object is deserialized, its type must be known.
//! In other words: One must know the type before the type can be parsed.
//! No information what type is stored is provided by the library.
//!
//! ## serialization
//! You have an object than accepts `Item`s, implementing the [`Receiver`] trait.
//! This trait is automatically implemented for any [`std::io::Write`], receiving [`u8`]s;
//!
//! Any type that implements the [`Serialize<Item>`] trait can be serialized with any [`Receiver<Type = Item>`].
//! Some implementations of primitve types are provided.
//!
//! ## example
//!
//! ```rust
//! #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
//! struct Struct {
//!     another: AnotherStruct,
//!     an_enum: Enum,
//!     vector: Vec<u32>,
//!     option: Option<i64>,
//!     array: [String; 4],
//! }
//!
//! #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
//! struct AnotherStruct(u32, u32);
//!
//! #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
//! enum Enum {
//!     A,
//!     B(f64),
//!     C { x: AnotherStruct, y: u16 },
//! }
//!
//! fn main() -> std::io::Result<()> {
//!     const FILE: &str = "test.bin";
//!
//!     let example = Struct {
//!         another: AnotherStruct(69, 420),
//!         an_enum: Enum::B(69.420),
//!         vector: vec![69, 420, 1337],
//!         option: Some(-87),
//!         array: [
//!             String::from("one"),
//!             String::from("two"),
//!             String::from("three"),
//!             String::from("four"),
//!         ],
//!     };
//!
//!     {
//!         let mut file = File::create(FILE)?;
//!         file.auto_ser(example.clone())?;
//!         // NOTE Receiver::auto could be used here if File didn't implement Read + Write
//!     }
//!
//!     {
//!         let mut file = File::open(FILE)?;
//!         let person2: Struct = file.auto_de().map_err(Error::unwrap_sender)?;
//!         // NOTE Sender::auto could be used here if File didn't implement Read + Write
//!         println!("person1: {:#?}", example);
//!         println!("person2: {:#?}", person2);
//!         assert_eq!(example, person2);
//!         println!("indeed, they're equal")
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! ## planned future features
//! * [ ] attribute for ignoring fields when (de)serializing, they must implement [`Default`]
//! * [ ] handle generic types

mod es;
pub use es::*;

mod de;
pub use de::*;

mod generic_impls;
mod ingeneric_impls;

// pub use deser_derive::*;

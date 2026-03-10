//! CoreFoundation — RAII wrappers for CF types.
//!
//! Every type is a newtype around `CFRef<T>` which handles retain/release
//! automatically. Create/Copy results are wrapped without extra retain;
//! Get results must be wrapped with `::retain()`.
//!
//! ```rust,no_run
//! use malus::core_foundation::*;
//!
//! let s = CFString::new("hello");
//! let n = CFNumber::from_f64(3.14);
//! let b = CFBoolean::from_bool(true);
//! let d = CFData::new(b"bytes");
//!
//! let mut dict = CFMutableDictionary::new();
//! dict.set(s.as_type_ref(), n.as_type_ref());
//! ```

pub mod base;
pub mod string;
pub mod number;
pub mod boolean;
pub mod data;
pub mod array;
pub mod dictionary;

pub use base::{CFRef, CFTypeRef, CFAllocatorRef, CFIndex, CFTypeID, Boolean};
pub use string::CFString;
pub use number::{CFNumber, CFNumberType};
pub use boolean::CFBoolean;
pub use data::CFData;
pub use array::{CFArray, CFMutableArray};
pub use dictionary::{CFDictionary, CFMutableDictionary};

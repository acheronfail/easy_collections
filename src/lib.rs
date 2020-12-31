//! These are wrappers around `HashSet` and `HashMap` which make them a little more convenient to use.
//! They aren't intended for use in main applications, but were created to make prototyping and writing short programs easier.
//!
//! # Usage
//!
//! The struct `EasySet` wraps `HashSet` with some useful trait implementations.
//!
//! ```rust
//! use easy_collections::set;
//!
//! let a = &set!{1, 2, 3};
//! let b = &set!{2, 3, 4};
//! assert_eq!(a & b, set!{2, 3});        // intersection
//! assert_eq!(a | b, set!{1, 2, 3, 4});  // union
//! assert_eq!(a ^ b, set!{1, 4});        // symmetric difference
//! assert_eq!(a - b, set!{1});           // difference
//!
//! let c = &set!{1, 2, 3, 4};
//! assert!(a < c && b < c);              // subset
//! assert!(c > a && c > b);              // superset
//! assert!(a == a);                      // equality
//! ```
//!
//! The struct `EasyMap` wraps `HashMap` with some useful trait implementations.
//!
//! ```rust
//! use easy_collections::{EasyMap, map};
//!
//! // `42` here is the default value which is returned when no item exists in the map
//! // The default value is optional.
//! let map = map!{42; ("foo", 1), ("bar", 10), ("baz", 100)};
//! assert_eq!(map["foo"], 1);
//! assert_eq!(map["bar"], 10);
//! assert_eq!(map["baz"], 100);
//! assert_eq!(map["nope"], 42);
//! assert_eq!(map["nada"], 42);
//! assert_eq!(map["nuttin'"], 42);
//!
//! // If you want to create a map with just a single value, and no default, use a trailing comma:
//! let map: EasyMap<&str, (&str, &str)> = map!{("foo", "bar")};  // creates `EasyMap<_, (&str, &str)>` with `("foo", "bar")` as the default value
//! let map: EasyMap<&str, &str> = map!{("foo", "bar"),}; // creates `EasyMap<&str, &str>` with and `map["foo"] == "bar"`
//! ```
//!
//! Also, both `EasyMap` and `EasySet` deref to their underlying collections, for example:
//!
//! ```rust
//! use std::collections::{HashMap, HashSet};
//! use easy_collections::{EasyMap, EasySet, map, set};
//!
//! let easy: EasySet<_> = set!{"foo", "bar"};
//! let hash: &HashSet<_> = &*easy;
//! assert_eq!(&*easy, hash);
//!
//! let easy: EasyMap<_, _> = map!{("foo", "bar"),};
//! let hash: &HashMap<_, _> = &*easy;
//! assert_eq!(&*easy, hash);
//! ```

mod map;
mod set;

pub use map as easy_collections;
pub use set as easy_set;

pub use map::EasyMap;
pub use set::EasySet;

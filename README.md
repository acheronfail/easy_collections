[![crate](https://img.shields.io/crates/v/easy_collections)](https://crates.io/crates/easy_collections)
[![documentation](https://docs.rs/easy_collections/badge.svg)](https://docs.rs/easy_collections)
[![Average time to resolve an issue](https://isitmaintained.com/badge/resolution/acheronfail/easy_collections.svg)](https://isitmaintained.com/project/acheronfail/easy_collections "Average time to resolve an issue")
[![Percentage of issues still open](https://isitmaintained.com/badge/open/acheronfail/easy_collections.svg)](https://isitmaintained.com/project/acheronfail/easy_collections "Percentage of issues still open")

# easy_collections

These are wrappers around `HashSet` and `HashMap` which make them a little more convenient to use.
They aren't intended for use in main applications, but were created to make prototyping and writing short programs easier.

## Usage

The struct `EasySet` wraps `HashSet` with some useful trait implementations.

```rust
use easy_collections::set;

let a = &set!{1, 2, 3};
let b = &set!{2, 3, 4};
assert_eq!(a & b, set!{2, 3});        // intersection
assert_eq!(a | b, set!{1, 2, 3, 4});  // union
assert_eq!(a ^ b, set!{1, 4});        // symmetric difference
assert_eq!(a - b, set!{1});           // difference

let c = &set!{1, 2, 3, 4};
assert!(a < c && b < c);              // subset
assert!(c > a && c > b);              // superset
assert!(*a == set!{1, 2, 3});         // equality
```

The struct `EasyMap` wraps `HashMap` with some useful trait implementations.

```rust
use easy_collections::{EasyMap, map};

// `42` here is the default value which is returned when no item exists in the map
// The default value is optional.
let map = map!{42; "foo" => 1, "bar" => 10, "baz" => 100};
assert_eq!(map["foo"], 1);
assert_eq!(map["bar"], 10);
assert_eq!(map["baz"], 100);
assert_eq!(map["nope"], 42);
assert_eq!(map["nada"], 42);
assert_eq!(map["nuttin'"], 42);

let map: EasyMap<&str, &str> = map!{ "foo" => "bar" };
```

Also, both `EasyMap` and `EasySet` deref to their underlying collections, for example:

```rust
use std::collections::{HashMap, HashSet};
use easy_collections::{EasyMap, EasySet, map, set};

let easy: EasySet<_> = set!{"foo", "bar"};
let hash: &HashSet<_> = &*easy;
assert_eq!(&*easy, hash);

let easy: EasyMap<_, _> = map!{"foo" => "bar",};
let hash: &HashMap<_, _> = &*easy;
assert_eq!(&*easy, hash);
```

License: Unlicense OR MIT OR Apache-2.0
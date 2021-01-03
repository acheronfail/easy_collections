use std::{
    cmp::{Ord, Ordering, PartialOrd},
    collections::HashSet,
    hash::Hash,
    iter::FromIterator,
    ops::{BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref, Sub, SubAssign},
};

use paste::paste;

#[macro_export]
macro_rules! set {
    () => {
        $crate::EasySet::new()
    };
    {$($key:expr$(,)?)*} => {{
        let mut set = set!{};
        $(set.insert($key);)*
        set
    }};
}

/// A wrapper around `HashSet` which implements a lot of traits. One of the main benefits is that this map implements
/// the `BitAnd`, `BitOr`, `BitXor`, `Sub` and `Ord` traits in the same manner as Python's sets: https://docs.python.org/2/library/sets.html#set-objects
///
/// ```rust
/// use easy_collections::set;
///
/// let a = &set!{1, 2, 3};
/// let b = &set!{2, 3, 4};
/// // intersection
/// assert_eq!(a & b, set!{2, 3});
/// // union
/// assert_eq!(a | b, set!{1, 2, 3, 4});
/// // symmetric difference
/// assert_eq!(a ^ b, set!{1, 4});
/// // difference
/// assert_eq!(a - b, set!{1});
///
/// let c = &set!{1, 2, 3, 4};
/// // subset
/// assert!(a < c && b < c);
/// // superset
/// assert!(c > a && c > b);
/// // equality
/// assert!(a == a);
/// ```
///
/// You can also perform the same operations on types that implement `Into<EasySet<K>>`, for example:
/// ```rust
/// use easy_collections::set;
///
/// assert_eq!(set!{1, 2, 3} & [3, 4, 5].as_ref(), set!{3});
/// assert_eq!(set!{1, 2, 3} & vec![3, 4, 5], set!{3});
/// assert_eq!(set!{'b', 'a', 'r'} & String::from("baz"), set!{'b', 'a'});
/// assert_eq!(set!{"hello"} & std::collections::HashSet::new(), set!{});
/// ```
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct EasySet<K: Eq + Hash> {
    inner: HashSet<K>,
}

impl<K: Eq + Hash> EasySet<K> {
    /// Create a new `EasySet`.
    ///
    /// Note, there are macros to make this easier:
    /// ```rust
    /// use easy_collections::{EasySet, set};
    ///
    /// // create an empty set
    /// let set: EasySet<usize> = set!{};
    /// // create a set with values
    /// let set = set!{'a', 'b', 'c', 'd'};
    /// ```
    pub fn new() -> EasySet<K> {
        EasySet {
            inner: HashSet::new(),
        }
    }

    /// Same as `HashSet::insert`.
    pub fn insert(&mut self, k: K) -> bool {
        self.inner.insert(k)
    }

    /// Same as `HashSet::contains`.
    pub fn contains(&self, k: &K) -> bool {
        self.inner.contains(k)
    }

    /// Same as `HashSet::remove`.
    pub fn remove(&mut self, k: &K) -> bool {
        self.inner.remove(k)
    }

    /// Inserts the key into the set _if it wasn't in the set_. If it was in the set _it is removed_.
    ///
    /// ```rust
    /// use easy_collections::set;
    ///
    /// let mut set = set!{};
    /// set.toggle(1986);
    /// assert_eq!(set.contains(&1986), true);
    /// set.toggle(1986);
    /// assert_eq!(set.contains(&1986), false);
    ///```
    pub fn toggle(&mut self, k: K) -> bool {
        let contained_key = self.contains(&k);
        if self.contains(&k) {
            self.remove(&k);
        } else {
            self.insert(k);
        }

        contained_key
    }
}

impl<K: Eq + Hash> From<Vec<K>> for EasySet<K> {
    fn from(v: Vec<K>) -> Self {
        v.into_iter().collect()
    }
}

impl<K: Eq + Hash + Clone> From<&[K]> for EasySet<K> {
    fn from(v: &[K]) -> Self {
        v.iter().cloned().collect()
    }
}

impl From<String> for EasySet<char> {
    fn from(s: String) -> Self {
        s.chars().collect()
    }
}

impl<K: Eq + Hash> FromIterator<K> for EasySet<K> {
    fn from_iter<T: IntoIterator<Item = K>>(iter: T) -> Self {
        let mut set = set!();
        for k in iter {
            set.insert(k);
        }

        set
    }
}

impl<K: Eq + Hash> IntoIterator for EasySet<K> {
    type Item = K;
    type IntoIter = std::collections::hash_set::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<K: Eq + Hash> Deref for EasySet<K> {
    type Target = HashSet<K>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<K: Eq + Hash> PartialOrd for EasySet<K> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Eq + Hash> Ord for EasySet<K> {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.inner.is_subset(&other.inner) {
            Ordering::Less
        } else if self.inner.is_superset(&other.inner) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl<K: Eq + Hash + Clone> From<&EasySet<K>> for EasySet<K> {
    fn from(easy: &EasySet<K>) -> Self {
        easy.clone()
    }
}

impl<K: Eq + Hash> From<HashSet<K>> for EasySet<K> {
    fn from(inner: HashSet<K>) -> Self {
        EasySet { inner }
    }
}

impl<K: Eq + Hash + Clone> From<&HashSet<K>> for EasySet<K> {
    fn from(hash: &HashSet<K>) -> Self {
        EasySet {
            inner: (*hash).clone(),
        }
    }
}

impl<K: Eq + Hash> From<EasySet<K>> for HashSet<K> {
    fn from(easy: EasySet<K>) -> Self {
        easy.inner
    }
}

impl<K: Eq + Hash + Clone> From<&EasySet<K>> for HashSet<K> {
    fn from(easy: &EasySet<K>) -> Self {
        easy.inner.clone()
    }
}

macro_rules! impl_bit_op {
    ($trait:ty, $method:ident, $set_op:ident) => {
        paste! {
            impl<K: Eq + Hash + Clone, T: Into<EasySet<K>>> $trait<T> for &EasySet<K> {
                type Output = EasySet<K>;
                fn $method(self, rhs: T) -> Self::Output {
                    self.inner.$set_op(&rhs.into()).cloned().collect()
                }
            }
            impl<K: Eq + Hash + Clone, T: Into<EasySet<K>>> $trait<T> for EasySet<K> {
                type Output = Self;
                fn $method(self, rhs: T) -> Self::Output {
                    self.inner.$set_op(&rhs.into()).cloned().collect()
                }
            }
            // assign
            impl<K: Eq + Hash + Clone, T: Into<EasySet<K>>> [<$trait Assign>]<T> for EasySet<K> {
                fn [<$method _assign>](&mut self, rhs: T) {
                    *self = self.inner.$set_op(&rhs.into()).cloned().collect()
                }
            }
        }
    };
}

impl_bit_op!(BitAnd, bitand, intersection);
impl_bit_op!(BitOr, bitor, union);
impl_bit_op!(BitXor, bitxor, symmetric_difference);
impl_bit_op!(Sub, sub, difference);

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn macros() {
        let set: EasySet<char> = set!();
        assert_eq!(set.contains(&'a'), false);
        assert_eq!(set.contains(&'b'), false);
        assert_eq!(set.contains(&'c'), false);

        // without trailing comma
        let set = set! {'a', 'b', 'c'};
        assert_eq!(set.contains(&'a'), true);
        assert_eq!(set.contains(&'b'), true);
        assert_eq!(set.contains(&'c'), true);

        // with trailing comma
        let set = set! {'d', 'e', 'f',};
        assert_eq!(set.contains(&'d'), true);
        assert_eq!(set.contains(&'e'), true);
        assert_eq!(set.contains(&'f'), true);
    }

    #[test]
    fn deref() {
        let easy: EasySet<_> = set! {("foo", "bar"),};
        let hash: &HashSet<_> = &*easy;

        assert_eq!(&*easy, hash);
    }

    #[test]
    fn iter_via_deref() {
        let set = set! {'i', 't', 'e', 'r'};
        let mut values = vec![];
        for x in &*set {
            values.push(*x);
        }

        // the values could be in any order
        values.sort();
        assert_eq!(values, &['e', 'i', 'r', 't']);

        // ensure we can still use the set here
        assert_eq!(set, set! {'i', 't', 'e', 'r'});
    }

    #[test]
    fn into_iter() {
        let set = set! {'i', 't', 'e', 'r'};
        let mut values = vec![];
        for x in set {
            values.push(x);
        }

        // the values could be in any order
        values.sort();
        assert_eq!(values, &['e', 'i', 'r', 't']);
    }

    #[test]
    fn from_iter() {
        let v = vec!['i', 't', 'e', 'r'];
        let s = v.iter().collect::<EasySet<_>>();
        assert_eq!(s, set! {&'i', &'t', &'e', &'r'});
    }

    #[test]
    fn cmp() {
        let a = set! {1, 2, 3, 4};
        let b = set! {2, 3};
        let d = set! {5, 6};

        // a is a superset of b
        assert!(a > b);
        // b is a subset of a
        assert!(b < a);
        // b and b.clone() are equal and not super/sub sets of each other
        assert!(b == b.clone());
        // d is not equal, nor a super/sub set of any other set
        assert!(a != d && b != d);
    }

    macro_rules! test_op {
        ($name:ident, $op:tt, $op_assign:tt, $expected:expr) => {
            #[test]
            fn $name() {
                // standard op
                {
                    let a = set!{1, 2, 3};
                    let b = set!{3, 4, 5};
                    let c = a $op b;

                    let mut values = c.into_iter().collect::<Vec<_>>();
                    values.sort();
                    assert_eq!(values, $expected);
                }
                // assign
                {
                    let mut a = set!{1, 2, 3};
                    a $op_assign set!{3, 4, 5};

                    let mut values = a.into_iter().collect::<Vec<_>>();
                    values.sort();
                    assert_eq!(values, $expected);
                }
            }
        };
    }

    test_op!(ops_bitand, &, &=, [3]);
    test_op!(ops_bitor, |, |=, [1, 2, 3, 4, 5]);
    test_op!(ops_bitxor, ^, ^=, [1, 2, 4, 5]);
    test_op!(ops_sub, -, -=, [1, 2]);
}

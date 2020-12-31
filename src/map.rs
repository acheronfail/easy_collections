use std::{iter::FromIterator, collections::HashMap};
use std::hash::Hash;
use std::ops::{Index, IndexMut, Deref};

#[macro_export]
macro_rules! map {
    () => {
        $crate::EasyMap::new()
    };
    {$default:expr} => {
        $crate::EasyMap::new_with_default($default)
    };
    {$(($key:expr, $val:expr)$(,)?)*} => {{
        let mut map = map!{};
        $(map[$key] = $val;)*
        map
    }};
    {$default:expr; $(($key:expr, $val:expr)$(,)?)*} => {{
        let mut map = map!{$default};
        $(map[$key] = $val;)*
        map
    }};
}

/// A wrapper around `HashMap` that creates default values for empty keys.
/// It also provides convenience implementations for `Index` and `IndexMut`.
///
/// For example:
/// ```rust
/// use easy_collections::EasyMap;
///
/// let mut map = EasyMap::new();
/// assert_eq!(map['a'], 0); // default value for `usize` is `0`
/// // now set insert a value
/// map['a'] = 42_usize;
/// assert_eq!(map['a'], 42);
/// ```
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct EasyMap<K: Eq + Hash, V: Clone> {
    inner: HashMap<K, V>,
    default: V,
}

impl<K: Eq + Hash, V: Clone + Default> EasyMap<K, V> {
    /// Create a new `EasyMap`. The value `V` must implement `Default`.
    ///
    /// Note, that there are macros which make this easier:
    /// ```rust
    /// use easy_collections::map;
    ///
    /// let mut map = map!{};
    /// map[1] = (10, 4);
    /// ```
    ///
    /// And another to pre-populate a map with values:
    /// ```rust
    /// use easy_collections::map;
    ///
    /// let map = map!{("foo", "bar"), ("hello", "world")};
    /// assert_eq!(map["foo"], "bar");
    /// assert_eq!(map["hello"], "world");
    /// assert_eq!(map["not here"], "");
    /// ```
    pub fn new() ->  EasyMap<K, V> {
        EasyMap::new_with_default(V::default())
    }
}

impl<K: Eq + Hash, V: Clone> EasyMap<K, V> {
    /// Create a new `EasyMap`. The value `V` does not need to implement `Default`, instead you provide it with one here.
    ///
    /// Note, that there's a macro which makes this easier:
    /// ```rust
    /// use easy_collections::map;
    ///
    /// #[derive(Debug, Clone, PartialEq)]
    /// struct Foo(u32);
    ///
    /// let mut map = map!{Foo(1)};
    /// assert_eq!(map[1], Foo(1));
    /// assert_eq!(map[2], Foo(1));
    /// map[1] = Foo(1729);
    /// assert_eq!(map[1], Foo(1729));
    /// ```
    ///
    /// Or, the same while pre-populating the map with values:
    /// ```rust
    /// use easy_collections::map;
    ///
    /// let map = map!{42; ("foo", 1), ("bar", 10), ("baz", 100)};
    /// assert_eq!(map["foo"], 1);
    /// assert_eq!(map["bar"], 10);
    /// assert_eq!(map["baz"], 100);
    /// assert_eq!(map["nope"], 42);
    /// ```
    pub fn new_with_default(default: V) -> EasyMap<K, V> {
        EasyMap {
            inner: HashMap::new(),
            default,
        }
    }

    /// Same as `HashMap::insert`.
    ///
    /// NOTE: you probably just want to use the `IndexMut` trait for this:
    /// ```rust
    /// use easy_collections::EasyMap;
    ///
    /// let mut map = EasyMap::new();
    /// map[1] = "hello";
    /// ```
    pub fn insert(&mut self, k: K, v: V) -> Option<V> {
        self.inner.insert(k, v)
    }

    /// Same as `HashMap::remove`.
    pub fn remove(&mut self, k: K) -> Option<V> {
        self.inner.remove(&k)
    }
}

impl<K: Eq + Hash, V: Clone + Default> From<Vec<(K, V)>> for EasyMap<K, V> {
    fn from(v: Vec<(K, V)>) -> Self {
        v.into_iter().collect()
    }
}

impl<K: Eq + Hash + Clone, V: Clone + Default> From<&[(K, V)]> for EasyMap<K, V> {
    fn from(v: &[(K, V)]) -> Self {
        v.iter().cloned().collect()
    }
}

impl<K: Eq + Hash, V: Clone + Default> FromIterator<(K, V)> for EasyMap<K, V> {
    fn from_iter<T: IntoIterator<Item = (K, V)>>(iter: T) -> Self {
        let mut set = map!(V::default());
        for (k, v) in iter {
            set.insert(k, v);
        }

        set
    }
}

impl<K: Eq + Hash, V: Clone> IntoIterator for EasyMap<K, V> {
    type Item = (K, V);
    type IntoIter = std::collections::hash_map::IntoIter<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<K: Eq + Hash, V: Clone> Deref for EasyMap<K, V> {
    type Target = HashMap<K, V>;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<K: Eq + Hash, V: Clone> Index<K> for EasyMap<K, V> {
    type Output = V;
    fn index(&self, key: K) -> &Self::Output {
        self.inner.get(&key).unwrap_or(&self.default)
    }
}

impl<K: Eq + Hash, V: Clone> IndexMut<K> for EasyMap<K, V> {
    fn index_mut(&mut self, key: K) -> &mut Self::Output {
        self.inner.entry(key).or_insert(self.default.clone())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn macros() {
        // without default
        let map: EasyMap<char, usize> = map!{};
        assert_eq!(map['a'], 0);
        assert_eq!(map['b'], 0);
        assert_eq!(map['c'], 0);

        // with default
        let map: EasyMap<char, usize> = map!{1};
        assert_eq!(map['a'], 1);
        assert_eq!(map['b'], 1);
        assert_eq!(map['c'], 1);

        // without default & without trailing comma
        let map = map!{('a', 10), ('b', 20)};
        assert_eq!(map['a'], 10);
        assert_eq!(map['b'], 20);
        assert_eq!(map['c'], 0);

        // without default & with trailing comma
        let map = map!{('a', 100), ('b', 200), };
        assert_eq!(map['a'], 100);
        assert_eq!(map['b'], 200);
        assert_eq!(map['c'], 0);

        // with default & without trailing comma
        let map = map!{1; ('a', 10), ('b', 20)};
        assert_eq!(map['a'], 10);
        assert_eq!(map['b'], 20);
        assert_eq!(map['c'], 1);

        // with default & with trailing comma
        let map = map!{1; ('a', 100), ('b', 200), };
        assert_eq!(map['a'], 100);
        assert_eq!(map['b'], 200);
        assert_eq!(map['c'], 1);
    }

    #[test]
    fn index() {
        let mut map = EasyMap::new();
        map['a'] = 1;
        map['b'] = 2;
        map['c'] = 3;

        assert_eq!(map['a'], 1);
        assert_eq!(map['b'], 2);
        assert_eq!(map['c'], 3);
    }

    #[test]
    fn index_mut() {
        let mut map = map!(1; ('a', 1729));

        // test existing key
        let a = &mut map['a'];
        assert_eq!(*a, 1729);
        assert_eq!(map['a'], 1729);

        // test non-existent key
        let b = &mut map['b'];
        *b = 42;
        assert_eq!(*b, 42);
        assert_eq!(map['b'], 42);
        assert_eq!(map['c'], 1);
    }

    #[test]
    fn deref() {
        let easy: EasyMap<_, _> = map!{("foo", "bar"),};
        let hash: &HashMap<_, _> = &*easy;

        assert_eq!(&*easy, hash);
    }

    #[test]
    fn iter_via_deref() {
        let map = map!{('i', true), ('t', true), ('e', true), ('r', true)};
        let mut values = vec![];
        for (k, v) in &*map {
            values.push((*k, *v));
        }

        // the values could be in any order
        values.sort();
        assert_eq!(values, &[('e', true), ('i', true), ('r', true), ('t', true)]);

        // ensure we can still use the map here
        assert_eq!(map, map!{('i', true), ('t', true), ('e', true), ('r', true)});
    }

    #[test]
    fn into_iter() {
        let map = map!{('i', true), ('t', true), ('e', true), ('r', true)};
        let mut values = vec![];
        for x in map {
            values.push(x);
        }

        // the values could be in any order
        values.sort();
        assert_eq!(values, &[('e', true), ('i', true), ('r', true), ('t', true)]);
    }

    #[test]
    fn from_iter() {
        let v = vec![('i', true), ('t', true), ('e', true), ('r', true)];
        let s = v.into_iter().collect::<EasyMap<_, _>>();
        assert_eq!(s, map!{('i', true), ('t', true), ('e', true), ('r', true)});
    }
}

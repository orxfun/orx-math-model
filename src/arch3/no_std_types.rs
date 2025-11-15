// // set

// #[cfg(feature = "std")]
// pub type ColSet<T> = std::collections::HashSet<T>;
// #[cfg(not(feature = "std"))]
// pub type ColSet<T> = alloc::collections::BTreeSet<T>;

// map

#[cfg(feature = "std")]
pub type Map<K, V> = std::collections::HashMap<K, V>;
#[cfg(not(feature = "std"))]
pub type Map<K, V> = alloc::collections::BTreeMap<K, V>;

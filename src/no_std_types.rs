#[cfg(feature = "std")]
pub type ColSet<T> = std::collections::HashSet<T>;

#[cfg(not(feature = "std"))]
pub type ColSet<T> = alloc::collections::BTreeSet<T>;

use crate::symbols::Set;
#[cfg(not(feature = "std"))]
use alloc::collections::BTreeMap;
#[cfg(feature = "std")]
use std::{collections::HashMap, hash::Hash};

#[cfg(not(feature = "std"))]
type Map<N> = BTreeMap<N, usize>;
#[cfg(feature = "std")]
type Map<N> = HashMap<N, usize>;

pub struct SetPositions<'m> {
    map: Map<Set<'m>>,
}

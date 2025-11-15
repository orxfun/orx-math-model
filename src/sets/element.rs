use core::hash::Hash;

pub trait Element
where
    Self: PartialEq + Eq + Hash,
{
}

impl<E> Element for E where E: PartialEq + Eq + Hash {}

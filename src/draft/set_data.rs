use crate::draft::set::{Set, Set0};
use alloc::boxed::Box;

pub trait SetData0 {
    type Set: Set0;
    fn keys(&self) -> Box<dyn Iterator<Item = <Self::Set as Set>::Key> + '_>;
    fn element(&self, key: <Self::Set as Set>::Key) -> &<Self::Set as Set>::Elem;
}

use crate::draft::set::{Set, Set0, Set1};

pub trait SetData0 {
    type Set: Set0;

    fn keys(&self) -> impl Iterator<Item = <Self::Set as Set>::Key> + '_;

    fn element(&self, key: <Self::Set as Set>::Key) -> &<Self::Set as Set>::Elem;
}

pub trait SetData1 {
    type Set: Set1;

    fn keys(
        &self,
        i0: <<Self::Set as Set1>::S0 as Set>::Key,
    ) -> impl Iterator<Item = <Self::Set as Set>::Key> + '_;

    fn element(&self, key: <Self::Set as Set>::Key) -> &<Self::Set as Set>::Elem;
}

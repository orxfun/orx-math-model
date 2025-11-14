use crate::data::set_data::indices::{IndexValues, SetDepths};
use crate::{data::SetDataCore, symbols::sets::SetCore, Set, SetData};
use alloc::boxed::Box;

pub struct EmptySet<'m, const N: usize> {
    set: Set<'m, N>,
}

pub fn empty<'m, const N: usize>(set: Set<'m, N>) -> EmptySet<'m, N> {
    todo!()
}

impl<'m, const N: usize> SetDataCore<'m> for EmptySet<'m, N> {
    fn set(&self) -> SetCore<'m> {
        self.set.core()
    }

    fn elements(
        &'m self,
        _: &SetDepths<'m>,
        _: &IndexValues,
    ) -> Box<dyn Iterator<Item = usize> + '_> {
        Box::new([].into_iter())
    }
}

impl<'m, const N: usize> SetData<'m, N> for EmptySet<'m, N> {
    fn set(&self) -> Set<'m, N> {
        self.set
    }
}

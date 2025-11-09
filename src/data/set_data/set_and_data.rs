use crate::data::set_data::indices::{IndexValues, SetDepths};
use crate::Set;
use alloc::boxed::Box;

pub trait SetAndData<'m, const N: usize> {
    fn set(&self) -> Set<'m, N>;

    fn elements(
        &'m self,
        set_depths: &SetDepths<'m>,
        index_values: &IndexValues,
    ) -> Box<dyn Iterator<Item = usize> + '_>;
}

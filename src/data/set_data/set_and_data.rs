use crate::data::set_data::indices::{IndexValues, SetDepths};
use crate::{symbols::sets::SetCore, Set};
use alloc::boxed::Box;

pub trait SetDataCore<'m> {
    fn set(&self) -> SetCore<'m>;

    fn elements(
        &'m self,
        depths: &SetDepths<'m>,
        index_values: &IndexValues,
    ) -> Box<dyn Iterator<Item = usize> + '_>;
}

pub trait SetData<'m, const N: usize>: SetDataCore<'m> {
    fn set(&self) -> Set<'m, N>;
}

use crate::data::set_data::indices::{IndexValues, SetDepths};
use crate::symbols::SetCore;
use alloc::boxed::Box;

pub trait SetGen<const N: usize> {
    fn elements_by_dependencies(
        &self,
        depending_indices: [usize; N],
    ) -> Box<dyn Iterator<Item = usize> + '_>;

    fn elements<'m>(
        &self,
        set: SetCore<'m>,
        depths: SetDepths<'m>,
        index_values: &IndexValues,
    ) -> Box<dyn Iterator<Item = usize> + '_>;
}

use crate::data::set_data::indices::{IndexValues, SetDepths};
use alloc::boxed::Box;

pub trait SetAndData<'m> {
    fn elements(
        &'m self,
        depths: &SetDepths<'m>,
        index_values: &IndexValues,
    ) -> Box<dyn Iterator<Item = usize> + '_>;
}

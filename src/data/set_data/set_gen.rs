use crate::data::set_data::indices::{IndexValues, SetDepths};
use crate::symbols::sets::SetCore;
use alloc::boxed::Box;

pub trait SetGen<'m> {
    fn elements(
        &self,
        set: SetCore<'m>,
        depths: &SetDepths<'m>,
        index_values: &IndexValues,
    ) -> Box<dyn Iterator<Item = usize> + '_>;
}

pub trait SetGenNew<'m> {
    fn elements(
        &self,
        depths: &SetDepths<'m>,
        index_values: &IndexValues,
    ) -> Box<dyn Iterator<Item = usize> + '_>;
}

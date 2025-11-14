use crate::{
    data::set_data::indices::{IndexValues, SetDepths},
    symbols::sets::SetCore,
};
use alloc::boxed::Box;

pub trait SetData<'m> {
    fn set(&self) -> SetCore<'m>;

    fn elements(
        &'m self,
        depths: &SetDepths<'m>,
        index_values: &IndexValues,
    ) -> Box<dyn Iterator<Item = usize> + '_>;
}

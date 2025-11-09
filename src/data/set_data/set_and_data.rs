use crate::data::set_data::indices::{IndexValues, SetDepths};
use crate::symbols::sets::SetCore;
use crate::{Set, SetGen};
use alloc::boxed::Box;

pub trait SetAndDataCore<'m> {
    fn elements(
        &'m self,
        set_depths: &SetDepths<'m>,
        index_values: &IndexValues,
    ) -> Box<dyn Iterator<Item = usize> + '_>;
}

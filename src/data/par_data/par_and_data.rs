use crate::data::{set_data::SetDepths, IndexValuesIter};
use crate::symbols::pars::ParCore;

pub trait ParAndData<'m> {
    fn par(&self) -> ParCore<'m>;

    fn value(&self, depths: &SetDepths<'m>, index_values: &IndexValuesIter) -> f64;
}

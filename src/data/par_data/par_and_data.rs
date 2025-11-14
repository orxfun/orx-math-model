use crate::symbols::pars::ParCore;

pub trait ParData<'m> {
    fn par(&self) -> ParCore<'m>;

    fn value(&self, index_values: &[usize]) -> f64;
}

use crate::symbols::pars::ParCore;

pub trait ParAndData<'m> {
    fn par(&self) -> ParCore<'m>;

    fn value(&self, index_values: &[usize]) -> f64;
}

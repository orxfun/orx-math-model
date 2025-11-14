use crate::{symbols::pars::ParCore, Par};

pub trait ParDataCore<'m> {
    fn par(&self) -> ParCore<'m>;

    fn value(&self, index_values: &[usize]) -> f64;
}

pub trait ParData<'m, const N: usize>: ParDataCore<'m> {
    fn par(&self) -> Par<'m, N>;
}

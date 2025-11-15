use crate::{data::par_data::ParDataCore, symbols::pars::ParCore, Par, ParData};

pub struct ZeroPar<'m, const N: usize> {
    par: Par<'m, N>,
}

impl<'m, const N: usize> ZeroPar<'m, N> {
    pub fn new(par: Par<'m, N>) -> Self {
        Self { par }
    }
}

impl<'m, const N: usize> ParDataCore<'m> for ZeroPar<'m, N> {
    fn par(&self) -> ParCore<'m> {
        self.par.into()
    }

    fn value(&self, _: &[usize]) -> f64 {
        0.0
    }
}

impl<'m, const N: usize> ParData<'m, N> for ZeroPar<'m, N> {
    fn par(&self) -> Par<'m, N> {
        self.par
    }
}

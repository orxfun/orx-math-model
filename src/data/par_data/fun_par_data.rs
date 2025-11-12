use crate::data::par_data::par_and_data::ParAndData;
use crate::data::set_data::SetDepths;
use crate::data::{number::Number, IndexValuesIter};
use crate::symbols::pars::ParCore;
use core::marker::PhantomData;
use orx_self_or::SoR;

pub struct FunParData<'d, 'm, Data, N, T, F>
where
    N: Number,
    T: SoR<N>,
    F: Fn(&'d Data, IndexValuesIter<'_>) -> T,
{
    par: ParCore<'m>,
    data: &'d Data,
    fun: F,
    phantom: PhantomData<N>,
}

impl<'d, 'm, Data, N, T, F> ParAndData<'m> for FunParData<'d, 'm, Data, N, T, F>
where
    N: Number,
    T: SoR<N>,
    F: Fn(&'d Data, IndexValuesIter<'_>) -> T,
{
    fn par(&self) -> ParCore<'m> {
        self.par
    }

    fn value(&self, depths: &SetDepths<'m>, index_values: &IndexValuesIter) -> f64 {
        todo!()
    }
}

use crate::data::number::Number;
use crate::data::par_data::par_and_data::ParAndData;
use crate::symbols::pars::ParCore;
use core::marker::PhantomData;

pub struct FunParData<'d, 'm, Data, N, F>
where
    N: Number,
    F: Fn(&'d Data, &[usize]) -> N,
{
    par: ParCore<'m>,
    data: &'d Data,
    fun: F,
    phantom: PhantomData<N>,
}

impl<'d, 'm, Data, N, F> FunParData<'d, 'm, Data, N, F>
where
    N: Number,
    F: Fn(&'d Data, &[usize]) -> N,
{
    pub(crate) fn new(par: ParCore<'m>, data: &'d Data, fun: F) -> Self {
        Self {
            par,
            data,
            fun,
            phantom: PhantomData,
        }
    }
}

impl<'d, 'm, Data, N, F> ParAndData<'m> for FunParData<'d, 'm, Data, N, F>
where
    N: Number,
    F: Fn(&'d Data, &[usize]) -> N,
{
    fn par(&self) -> ParCore<'m> {
        self.par
    }

    #[inline(always)]
    fn value(&self, index_values: &[usize]) -> f64 {
        let number = (self.fun)(self.data, index_values);
        number.to_f64()
    }
}

use crate::data::number::Number;
use crate::data::par_data::par_and_data::ParDataCore;
use crate::symbols::pars::ParCore;
use core::marker::PhantomData;

pub struct FunParData<'d, 'm, const N: usize, Data, T, F>
where
    T: Number,
    F: Fn(&'d Data, &[usize]) -> T,
{
    par: ParCore<'m>,
    data: &'d Data,
    fun: F,
    phantom: PhantomData<T>,
}

impl<'d, 'm, const N: usize, Data, T, F> FunParData<'d, 'm, N, Data, T, F>
where
    T: Number,
    F: Fn(&'d Data, &[usize]) -> T,
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

impl<'d, 'm, const N: usize, Data, T, F> ParDataCore<'m> for FunParData<'d, 'm, N, Data, T, F>
where
    T: Number,
    F: Fn(&'d Data, &[usize]) -> T,
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

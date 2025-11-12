use crate::data::number::Number;
use crate::data::par_data::par_and_data::ParAndData;
use crate::symbols::pars::ParCore;
use core::marker::PhantomData;
use orx_self_or::SoR;

pub struct FunParData<'d, 'm, Data, N, T, F>
where
    N: Number,
    T: SoR<N>,
    F: Fn(&'d Data, &[usize]) -> T,
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
    F: Fn(&'d Data, &[usize]) -> T,
{
    fn par(&self) -> ParCore<'m> {
        self.par
    }

    #[inline(always)]
    fn value(&self, index_values: &[usize]) -> f64 {
        let number = (self.fun)(self.data, index_values);
        let number = *number.get_ref();
        number.to_f64()
    }
}

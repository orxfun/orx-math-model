use crate::data::{FunParData, Number};
use crate::symbols::pars::{Par, ParCore};
use orx_self_or::SoR;

impl<'m> Par<'m, 0> {
    pub fn data<'d, Data, N, T, F>(
        self,
        data: &'d Data,
        fun: F,
    ) -> FunParData<'d, 'm, Data, N, T, F>
    where
        N: Number,
        T: SoR<N>,
        F: Fn(&'d Data, &[usize]) -> T,
    {
        let par = ParCore::from(self);
        FunParData::new(par, data, fun)
    }
}

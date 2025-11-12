use crate::data::{FunParData, Number};
use crate::symbols::pars::{Par, ParCore};
use orx_self_or::SoR;

impl<'m> Par<'m, 0> {
    pub fn data<'d, Data, N, T, F>(
        self,
        data: &'d Data,
        fun: F,
    ) -> FunParData<'d, 'm, Data, N, T, impl Fn(&'d Data, &[usize]) -> T>
    where
        N: Number,
        T: SoR<N>,
        F: Fn(&'d Data) -> T,
    {
        let par = ParCore::from(self);

        let fun = move |data: &'d Data, indices: &[usize]| {
            debug_assert_eq!(indices.len(), 0);
            fun(data)
        };

        FunParData::new(par, data, fun)
    }
}

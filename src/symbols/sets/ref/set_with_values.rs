use crate::data::{FunSetAndDataD0, FunSetAndDataD1};
use crate::Set;
use orx_self_or::SoR;

impl<'m> Set<'m, 0> {
    pub fn data<'d, Data, I, T>(
        self,
        data: &'d Data,
        fun: fn(&'d Data) -> I,
    ) -> FunSetAndDataD0<'m, 'd, Data, I, T>
    where
        I: IntoIterator<Item = T>,
        T: SoR<usize>,
    {
        FunSetAndDataD0::new(self, data, fun)
    }
}

impl<'m> Set<'m, 1> {
    pub fn data<'d, Data, I, T>(
        self,
        data: &'d Data,
        fun: fn(&'d Data, usize) -> I,
    ) -> FunSetAndDataD1<'m, 'd, Data, I, T>
    where
        I: IntoIterator<Item = T>,
        T: SoR<usize>,
    {
        FunSetAndDataD1::new(self, data, fun)
    }
}

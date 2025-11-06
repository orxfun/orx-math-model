use crate::data::{FunSetAndDataD0, FunSetAndDataD1};
use crate::Set;

impl<'m> Set<'m, 0> {
    pub fn data<'d, Data, I>(
        self,
        data: &'d Data,
        fun: fn(&'d Data) -> I,
    ) -> FunSetAndDataD0<'m, 'd, Data, I>
    where
        I: Iterator<Item = usize>,
    {
        FunSetAndDataD0::new(self, data, fun)
    }
}

impl<'m> Set<'m, 1> {
    pub fn data<'d, Data, I>(
        self,
        data: &'d Data,
        fun: fn(&'d Data, usize) -> I,
    ) -> FunSetAndDataD1<'m, 'd, Data, I>
    where
        I: Iterator<Item = usize>,
    {
        FunSetAndDataD1::new(self, data, fun)
    }
}

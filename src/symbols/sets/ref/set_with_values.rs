use crate::data::FunSetAndDataD1;
use crate::symbols::{values::SetGen, Elements};
use crate::Set;
use alloc::boxed::Box;

impl<'m> Set<'m, 0> {
    pub fn values(self, elements: impl SetGen<0> + 'static) -> Self {
        let set = Box::new(elements);
        let elements = Elements::D0(set);
        // self.data().update_elements(elements);
        self
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

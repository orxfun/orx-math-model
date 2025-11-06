use crate::symbols::values::FunSet;
use crate::symbols::{values::SetGen, Elements};
use crate::Set;
use alloc::boxed::Box;

impl<'m> Set<'m, 0> {
    pub fn values(self, elements: impl SetGen<0> + 'static) -> Self {
        let set = Box::new(elements);
        let elements = Elements::D0(set);
        self.data().update_elements(elements);
        self
    }
}

impl<'m> Set<'m, 1> {
    pub fn values<I>(self, elements: Box<dyn Fn(usize) -> I>) -> Self
    where
        I: Iterator<Item = usize> + 'static,
    {
        let set = Box::new(elements);
        let elements = Elements::D1(set);
        self.data().update_elements(elements);
        // let fun = |data: &Data, [i]: [usize; 1]| elements(data, i);
        // let set = Box::new(FunSet::new(data, fun));
        // let elements = Elements::D1(set);
        // self.data().update_elements(elements);
        self
    }
}

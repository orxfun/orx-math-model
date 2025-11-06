use crate::symbols::{values::SetGen, Elements};
use crate::Set;
use alloc::boxed::Box;

impl<'m> Set<'m, 0> {
    pub fn values(self, elements: impl SetGen<0> + 'static) -> Self {
        let elements = Elements::D0(Box::new(elements));
        self.data().update_elements(elements);
        self
    }
}

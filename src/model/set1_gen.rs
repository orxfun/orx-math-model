use crate::model::Elements;
use alloc::boxed::Box;

pub struct Set1Gen {
    f: Box<dyn Fn(usize, &mut Elements)>,
}

impl Set1Gen {
    pub fn fill_elements(&self, i1: usize, elements: &mut Elements) {
        elements.clear();
        (self.f)(i1, elements)
    }
}

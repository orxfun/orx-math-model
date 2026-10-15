use crate::model::Elements;
use alloc::boxed::Box;

pub struct Set0Gen {
    f: Box<dyn Fn(&mut Elements)>,
}

impl Set0Gen {
    pub fn fill_elements(&self, elements: &mut Elements) {
        elements.clear();
        (self.f)(elements)
    }
}

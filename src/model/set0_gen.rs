use crate::model::Elements;
use alloc::boxed::Box;

pub struct Set0Gen {
    f: Box<dyn Fn(&mut Elements)>,
}

impl Default for Set0Gen {
    fn default() -> Self {
        Self {
            f: Box::new(|_| {}),
        }
    }
}

impl Set0Gen {
    pub fn fill_elements(&self, elements: &mut Elements) {
        elements.clear();
        (self.f)(elements)
    }
}

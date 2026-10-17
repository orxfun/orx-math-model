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
    pub fn new<I: IntoIterator<Item = usize> + 'static>(
        generator: impl Fn() -> I + 'static,
    ) -> Self {
        let f = Box::new(move |e: &mut Elements| e.fill_with(generator()));
        Self { f }
    }

    pub fn new2(f: Box<dyn Fn(&mut Elements)>) -> Self {
        Self { f }
    }

    pub fn fill(&self, elements: &mut Elements) {
        (self.f)(elements)
    }
}

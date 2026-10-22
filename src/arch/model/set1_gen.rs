use crate::model::Elements;
use alloc::boxed::Box;

pub struct Set1Gen {
    f: Box<dyn Fn(usize, &mut Elements)>,
}

impl Set1Gen {
    pub fn new<I: IntoIterator<Item = usize> + 'static>(
        generator: impl Fn(usize) -> I + 'static,
    ) -> Self {
        let f = Box::new(move |i, e: &mut Elements| e.fill_with(generator(i)));
        Self { f }
    }
}

use crate::model::Elements;
use alloc::boxed::Box;

pub struct Set0Gen<'d> {
    f: Box<dyn Fn(&mut Elements) + 'd>,
}

impl Default for Set0Gen<'_> {
    fn default() -> Self {
        Self {
            f: Box::new(|_| {}),
        }
    }
}

impl<'d> Set0Gen<'d> {
    pub fn new<I>(f: impl Fn() -> I + 'd) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        let g = move |e: &mut Elements| e.fill_with(f());
        Self { f: Box::new(g) }
    }

    pub fn fill(&self, elements: &mut Elements) {
        (self.f)(elements)
    }
}

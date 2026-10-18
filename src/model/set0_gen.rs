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
    pub fn new(f: Box<dyn Fn(&mut Elements)>) -> Self {
        Self { f }
    }

    pub fn fill(&self, elements: &mut Elements) {
        (self.f)(elements)
    }
}

pub struct Set0Gen2<'d> {
    f: Box<dyn Fn(&mut Elements) + 'd>,
}

impl Default for Set0Gen2<'_> {
    fn default() -> Self {
        Self {
            f: Box::new(|_| {}),
        }
    }
}

impl<'d> Set0Gen2<'d> {
    pub fn new(f: Box<dyn Fn(&mut Elements) + 'd>) -> Self {
        Self { f }
    }

    pub fn fill(&self, elements: &mut Elements) {
        (self.f)(elements)
    }
}

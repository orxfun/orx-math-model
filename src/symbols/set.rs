use crate::symbols::Idx;
use crate::Model;
use alloc::boxed::Box;
use alloc::vec::Vec;
use orx_imp_vec::Elem;

#[derive(Default)]
pub struct Elements {
    data: Vec<usize>,
}

impl Elements {
    pub fn fill_with(&mut self, values: impl IntoIterator<Item = usize>) {
        self.data.clear();
        self.data.extend(values);
    }

    pub fn as_slice(&self) -> &[usize] {
        &self.data
    }
}

#[derive(derive_new::new)]
pub struct SetData<'d> {
    f: Box<dyn Fn(&mut Elements) + 'd>,
}

impl Default for SetData<'_> {
    fn default() -> Self {
        Self {
            f: Box::new(|_| {}),
        }
    }
}

impl<'d> SetData<'d> {
    pub fn new_from_generator<I>(f: impl Fn() -> I + 'd) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        let f = move |e: &mut Elements| e.fill_with(f());
        Self::new(Box::new(f))
    }

    pub fn fill(&self, elements: &mut Elements) {
        (self.f)(elements)
    }
}

#[derive(derive_new::new, Clone, Copy)]
pub struct Set<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, SetData<'d>>,
}

impl<'m, 'd> Set<'m, 'd> {
    pub fn idx(self) -> Idx<'m, 'd> {
        self.m.idx(self)
    }
}

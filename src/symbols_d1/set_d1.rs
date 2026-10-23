use crate::model_references::SetRef;
use crate::symbols::{Elements, Set};
use alloc::boxed::Box;
use orx_imp_vec::Elem;

pub struct SetD1Data<'d> {
    f: Box<dyn Fn(usize, &mut Elements) + 'd>,
    s1: SetRef,
}

impl Default for SetD1Data<'_> {
    fn default() -> Self {
        // Self {
        //     f: Box::new(|_, _| {}),
        // }
        todo!()
    }
}

impl<'d> SetD1Data<'d> {
    pub fn new<I>(s1: Set<'_, 'd>, f: impl Fn(usize) -> I + 'd) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        let g = move |i1: usize, e: &mut Elements| e.fill_with(f(i1));
        todo!()
        // Self { s1, f: Box::new(g) }
    }

    pub fn fill(&self, i1: usize, elements: &mut Elements) {
        (self.f)(i1, elements)
    }
}

#[derive(derive_new::new, Clone, Copy)]
pub struct SetD1<'m, 'd>(pub(crate) Elem<'m, SetD1Data<'d>>);

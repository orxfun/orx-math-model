use crate::model_references::SetRef;
use crate::symbols::{Elements, Set};
use crate::Model;
use alloc::boxed::Box;
use orx_imp_vec::Elem;

pub struct SetD1Data<'d> {
    f: Box<dyn Fn(usize, &mut Elements) + 'd>,
    s0: SetRef,
}

impl<'d> SetD1Data<'d> {
    pub fn default<'m>(m: &'m Model<'d>, s0: Set<'m, 'd>) -> Self {
        let s0 = SetRef::new(m, s0);
        let f = Box::new(|_, _: &mut Elements| {});
        Self { f, s0 }
    }

    pub fn new<'m, I>(m: &'m Model<'d>, s0: Set<'m, 'd>, f: impl Fn(usize) -> I + 'd) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        let s0 = SetRef::new(m, s0);
        let f = move |i1: usize, e: &mut Elements| e.fill_with(f(i1));
        let f = Box::new(f);
        Self { f, s0 }
    }

    pub fn fill(&self, i0: usize, elements: &mut Elements) {
        (self.f)(i0, elements)
    }
}

#[derive(derive_new::new, Clone, Copy)]
pub struct SetD1<'m, 'd>(pub(crate) Elem<'m, SetD1Data<'d>>);

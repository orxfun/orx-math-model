use crate::model_references::{SetD1Ref, SetRef};
use crate::symbols::{Elements, Idx, RefIdxValue, Set, SetData};
use crate::Model;
use alloc::boxed::Box;
use orx_imp_vec::Elem;

pub struct SetD1Data<'d> {
    f: Box<dyn Fn(usize, &mut Elements) + 'd>,
    s0: SetRef,
}

impl<'d> SetD1Data<'d> {
    pub fn default(s0: Set<'_, 'd>) -> Self {
        let s0 = SetRef::new(s0);
        let f = |_, _: &mut Elements| {};
        Self { f: Box::new(f), s0 }
    }

    pub fn new<I>(s0: Set<'_, 'd>, f: impl Fn(usize) -> I + 'd) -> Self
    where
        I: IntoIterator<Item = usize>,
    {
        let s0 = SetRef::new(s0);
        let f = move |i1: usize, e: &mut Elements| e.fill_with(f(i1));
        Self { f: Box::new(f), s0 }
    }

    pub fn fill(&self, i0: usize, elements: &mut Elements) {
        (self.f)(i0, elements)
    }
}

#[derive(derive_new::new, Clone, Copy)]
pub struct SetD1<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, SetD1Data<'d>>,
}

impl<'m, 'd> SetD1<'m, 'd> {
    pub(crate) fn set(self, i: Idx<'m, 'd>) -> Set<'m, 'd> {
        assert!(core::ptr::eq(self.m, i.m), "TODO: error msg");

        let s1 = unsafe { self.el.get() };
        let idx = unsafe { i.el.get() };
        assert_eq!(s1.s0, idx.s0(), "TODO: err-msg");

        // TODO: safety explanation
        let p = unsafe { RefIdxValue::new(i) };
        let s1 = SetD1Ref::new(self);

        let m = self.m as *const Model;
        let f = move |e: &mut Elements| {
            let i0 = p.value();
            // SAFETY: set_d1 is tied to lifetime of `m`
            let fun = unsafe {
                let set_d1 = s1.set(&*m);
                &set_d1.el.get().f
            };
            fun(i0, e);
        };

        self.m.add_set(SetData::new(Box::new(f)))
    }
}

use crate::elements::Elements;
use crate::symbols::{Idx, RefIdxValue, Set, SetData, SetFun, SetRef};
use crate::Model;
use alloc::boxed::Box;
use orx_imp_vec::Elem;

#[derive(Clone, Copy)]
pub struct RefSetD1Fun<'d> {
    p: *const Box<dyn Fn(usize, &mut Elements) + 'd>,
}

impl<'d> RefSetD1Fun<'d> {
    /// # SAFETY
    ///
    /// * Created value must not outlive `'m` or `'d`.
    pub unsafe fn new<'m>(set: SetD1<'m, 'd>) -> Self {
        let el = unsafe { set.el.get() };
        Self { p: el.fun_ptr }
    }

    pub fn value(&self) -> &Box<dyn Fn(usize, &mut Elements) + 'd> {
        unsafe { &*self.p }
    }
}

#[derive(derive_new::new)]
pub struct SetD1Data<'d> {
    s0: SetRef,
    fun_ptr: *mut Box<dyn Fn(usize, &mut Elements) + 'd>,
}

impl<'d> SetD1Data<'d> {
    pub fn fill(&self, i0: usize, elements: &mut Elements) {
        let fun = unsafe { &*self.fun_ptr };
        fun(i0, elements);
    }

    pub fn update_fun(&mut self, fun: Box<dyn Fn(usize, &mut Elements) + 'd>) {
        let _ = unsafe { self.fun_ptr.replace(fun) };
    }

    pub fn update_fun_from_gen<I>(&mut self, fun: impl Fn(usize) -> I + 'd)
    where
        I: IntoIterator<Item = usize>,
    {
        let fun = move |i1: usize, e: &mut Elements| e.fill_with(fun(i1));
        self.update_fun(Box::new(fun));
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

        // SAFETY: p and fun_d1 is tied to lifetime of 'm and 'd due to return type Set<'m, 'd>.
        let (p, fun_d1) = unsafe { (RefIdxValue::new(i), RefSetD1Fun::new(self)) };
        let f = move |e: &mut Elements| {
            let i0 = p.value();
            let fun = fun_d1.value();
            fun(i0, e);
        };

        self.m.add_set(SetFun::new(Box::new(f)))
    }
}

use crate::elements::Elements;
use crate::symbols::{Idx, RefIdxValue, Set, SetData, SetFun, SetRef};
use crate::Model;
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// fun

#[derive(derive_new::new)]
pub struct SetD1Fun<'d>(Box<dyn Fn(usize, &mut Elements) + 'd>);

impl SetD1Fun<'_> {
    pub fn empty() -> Self {
        Self(Box::new(|_, _| {}))
    }

    pub fn fill(&self, i0: usize, elements: &mut Elements) {
        self.0(i0, elements)
    }
}

// data

#[derive(derive_new::new)]
pub struct SetD1Data<'d> {
    s0: SetRef,
    fun_ptr: *mut SetD1Fun<'d>,
}

impl<'d> SetD1Data<'d> {
    pub fn fill(&self, i0: usize, elements: &mut Elements) {
        unsafe { &*self.fun_ptr }.fill(i0, elements);
    }

    pub fn update_fun(&mut self, fun: SetD1Fun<'d>) {
        let _ = unsafe { self.fun_ptr.replace(fun) };
    }

    pub fn update_fun_from_gen<I>(&mut self, fun: impl Fn(usize) -> I + 'd)
    where
        I: IntoIterator<Item = usize>,
    {
        let fun = move |i1: usize, e: &mut Elements| e.fill_with(fun(i1));
        self.update_fun(SetD1Fun(Box::new(fun)));
    }
}

// symbol

#[derive(derive_new::new, Clone, Copy)]
pub struct SetD1<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, SetD1Data<'d>>,
}

impl<'m, 'd> SetD1<'m, 'd> {
    pub(crate) fn set(self, i: Idx<'m, 'd>) -> Set<'m, 'd> {
        struct RefSetD1Fun<'d>(*const SetD1Fun<'d>);

        impl<'d> RefSetD1Fun<'d> {
            /// # SAFETY
            ///
            /// * Created value must not outlive `'m` or `'d`.
            unsafe fn new<'m>(set: SetD1<'m, 'd>) -> Self {
                let el = unsafe { set.el.get() };
                Self(el.fun_ptr)
            }

            fn value(&self) -> &SetD1Fun<'d> {
                unsafe { &*self.0 }
            }
        }

        assert!(core::ptr::eq(self.m, i.m), "TODO: error msg");

        let s1 = unsafe { self.el.get() };
        let idx = unsafe { i.el.get() };
        assert_eq!(s1.s0, idx.s0(), "TODO: err-msg");

        // SAFETY: p and fun_d1 is tied to lifetime of 'm and 'd due to return type Set<'m, 'd>.
        let (p, fun_d1) = unsafe { (RefIdxValue::new(i), RefSetD1Fun::new(self)) };
        let f = move |e: &mut Elements| {
            let i0 = p.value();
            let fun = fun_d1.value();
            fun.0(i0, e);
        };

        self.m.add_set(SetFun::new(Box::new(f)))
    }
}

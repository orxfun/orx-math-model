use crate::elements::ElementsOfIdx;
use crate::symbols::{PseudoInt, PtrIntFun, SetOfIdx, SetOfIdxData, SetOfIdxFun, SetOfIdxRef};
use crate::Model;
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// fun

#[derive(derive_new::new)]
pub struct SetOfIdxD1Fun<'d>(Box<dyn Fn(usize, &mut ElementsOfIdx) + 'd>);

impl SetOfIdxD1Fun<'_> {
    pub fn empty() -> Self {
        Self(Box::new(|_, _| {}))
    }

    pub fn fill(&self, i0: usize, elements: &mut ElementsOfIdx) {
        self.0(i0, elements)
    }
}

// data

#[derive(derive_new::new)]
pub struct SetOfIdxD1Data<'d> {
    s0: SetOfIdxRef,
    fun: SetOfIdxD1Fun<'d>,
}

impl<'d> SetOfIdxD1Data<'d> {
    pub fn fill(&self, i0: usize, elements: &mut ElementsOfIdx) {
        self.fun.fill(i0, elements);
    }

    pub fn update_fun(&mut self, fun: SetOfIdxD1Fun<'d>) {
        self.fun = fun;
    }

    pub fn update_fun_from_gen<I>(&mut self, fun: impl Fn(usize) -> I + 'd)
    where
        I: IntoIterator<Item = usize>,
    {
        let fun = move |i1: usize, e: &mut ElementsOfIdx| e.fill_with(fun(i1));
        self.update_fun(SetOfIdxD1Fun(Box::new(fun)));
    }

    pub fn ptr_fun(p: *const SetOfIdxD1Data<'d>) -> *const SetOfIdxD1Fun<'d> {
        unsafe { core::ptr::addr_of!((*p).fun) }
    }
}

// symbol

#[derive(derive_new::new, Clone, Copy)]
pub struct SetOfIdxD1<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, SetOfIdxD1Data<'d>>,
}

impl<'m, 'd> SetOfIdxD1<'m, 'd> {
    pub(crate) fn at(self, i: impl Into<PseudoInt<'m, 'd>>) -> SetOfIdx<'m, 'd> {
        struct PtrSetD1Fun<'d>(*const SetOfIdxD1Fun<'d>);

        impl<'d> PtrSetD1Fun<'d> {
            /// # SAFETY
            ///
            /// * Created value must not outlive `'m` or `'d`.
            unsafe fn new<'m>(set: SetOfIdxD1<'m, 'd>) -> Self {
                Self(SetOfIdxD1Data::ptr_fun(set.el.as_ptr()))
            }

            fn val(&self) -> &SetOfIdxD1Fun<'d> {
                unsafe { &*self.0 }
            }
        }

        // SAFETY: fun_d1 is tied to lifetime of 'm and 'd due to return type Num<'m, 'd>.
        let fun_d1 = unsafe { PtrSetD1Fun::new(self) };

        let f = match i.into() {
            PseudoInt::Int(i) => {
                assert!(core::ptr::eq(self.m, i.m), "TODO: error msg");
                // SAFETY: p is tied to lifetime of 'm and 'd due to return type Num<'m, 'd>.
                let p = unsafe { PtrIntFun::new(i) };
                SetOfIdxFun::new(Box::new(move |e| fun_d1.val().0(p.fun().eval(), e)))
            }
            PseudoInt::Raw(i0) => SetOfIdxFun::new(Box::new(move |e| fun_d1.val().0(i0, e))),
        };

        self.m.add_set_of_idx(SetOfIdxData::new(f))
    }
}

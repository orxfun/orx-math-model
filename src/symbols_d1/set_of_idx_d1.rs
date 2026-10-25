use crate::elements::ElemCont;
use crate::ptr::RefPtr;
use crate::symbols::{Indices, IndicesData, IndicesFun, PseudoInt};
use crate::Model;
use alloc::boxed::Box;
use core::ops::Index;
use orx_imp_vec::Elem;

// fun

#[derive(derive_new::new)]
pub struct SetOfIdxD1Fun<'d>(Box<dyn Fn(usize, &mut ElemCont<usize>) + 'd>);

impl SetOfIdxD1Fun<'_> {
    pub fn empty() -> Self {
        Self(Box::new(|_, _| {}))
    }

    pub fn fill(&self, i0: usize, elements: &mut ElemCont<usize>) {
        self.0(i0, elements)
    }
}

// data

#[derive(derive_new::new)]
pub struct SetOfIdxD1Data<'d> {
    s0: RefPtr<IndicesFun<'d>>,
    fun: SetOfIdxD1Fun<'d>,
}

impl<'d> SetOfIdxD1Data<'d> {
    pub fn empty(s0: RefPtr<IndicesFun<'d>>) -> Self {
        Self::new(s0, SetOfIdxD1Fun::empty())
    }

    pub fn fill(&self, i0: usize, elements: &mut ElemCont<usize>) {
        self.fun.fill(i0, elements);
    }

    pub fn update_fun(&mut self, fun: SetOfIdxD1Fun<'d>) {
        self.fun = fun;
    }

    pub fn update_fun_from_gen<I>(&mut self, fun: impl Fn(usize) -> I + 'd)
    where
        I: IntoIterator<Item = usize>,
    {
        let fun = move |i1: usize, e: &mut ElemCont<usize>| e.fill(fun(i1));
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
    unsafe fn at_unchecked(self, i: impl Into<PseudoInt<'m, 'd>>) -> Indices<'d, 'd> {
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

        // SAFETY: lifetime is tied to lifetime of 'm and 'd due to return type
        let fun_d1 = unsafe { PtrSetD1Fun::new(self) };

        let f = match i.into() {
            PseudoInt::Int(i) => {
                assert!(core::ptr::eq(self.m, i.m), "TODO: error msg");
                // SAFETY: lifetime is tied to lifetime of 'm and 'd due to return type
                let p = unsafe { RefPtr::new(i.fun_ptr()) };
                IndicesFun::new(Box::new(move |e| fun_d1.val().0(p.val().eval(), e)))
            }
            PseudoInt::Raw(i0) => IndicesFun::new(Box::new(move |e| fun_d1.val().0(i0, e))),
        };

        let m = unsafe { &*(self.m as *const Model<'d>) };
        m.add_set_of_idx(IndicesData::new(f))
    }
}

// index

impl<'m, 'd, I: Into<PseudoInt<'m, 'd>>> Index<I> for SetOfIdxD1<'m, 'd> {
    type Output = Indices<'m, 'd>;

    fn index(&self, index: I) -> &Self::Output {
        // TODO: safety note: we are changing the lifetime from 'm,'d to 'd,'d
        self.m.add_sym_set(unsafe { self.at_unchecked(index) })
    }
}

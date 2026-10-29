use crate::data_structs::ElemCont;
use crate::ptr::RefPtr;
use crate::symbols::{ElIdx, PseudoInt};
use crate::Model;
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// fun

#[derive(derive_new::new)]
pub struct SetIdxFun<'d>(Box<dyn Fn(&mut ElemCont<usize>) + 'd>);

impl<'d> SetIdxFun<'d> {
    pub fn empty() -> Self {
        Self(Box::new(|_| {}))
    }

    pub fn range<'m>(
        begin: impl Into<PseudoInt<'m, 'd>>,
        exclusive_end: impl Into<PseudoInt<'m, 'd>>,
    ) -> Self
    where
        'd: 'm,
    {
        match (begin.into(), exclusive_end.into()) {
            (PseudoInt::Raw(a), PseudoInt::Raw(b)) => {
                let fill = move |e: &mut ElemCont<usize>| e.fill(a..b);
                SetIdxFun::new(Box::new(fill))
            }
            (PseudoInt::Raw(a), PseudoInt::Int(b)) => {
                // SAFETY: b is tied to lifetime of 'm and 'd
                let b = unsafe { RefPtr::new(b.fun_ptr()) };
                let fill = move |e: &mut ElemCont<usize>| e.fill(a..b.val().eval());
                SetIdxFun::new(Box::new(fill))
            }
            (PseudoInt::Int(a), PseudoInt::Raw(b)) => {
                let a = unsafe { RefPtr::new(a.fun_ptr()) };
                let fill = move |e: &mut ElemCont<usize>| e.fill(a.val().eval()..b);
                SetIdxFun::new(Box::new(fill))
            }
            (PseudoInt::Int(a), PseudoInt::Int(b)) => {
                let [a, b] = unsafe { [a, b].map(|x| RefPtr::new(x.fun_ptr())) };
                let fill = move |e: &mut ElemCont<usize>| e.fill(a.val().eval()..b.val().eval());
                SetIdxFun::new(Box::new(fill))
            }
        }
    }

    pub fn fill(&self, elements: &mut ElemCont<usize>) {
        self.0(elements)
    }
}

// symbol

#[derive(derive_new::new, Clone, Copy)]
pub struct SetIdx<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, SetIdxFun<'d>>,
}

impl<'m, 'd> SetIdx<'m, 'd> {
    pub(crate) fn fun_ptr(&self) -> RefPtr<SetIdxFun<'d>> {
        // TODO: safety note
        unsafe { RefPtr::new(self.el.as_ptr()) }
    }

    pub fn idx(self) -> ElIdx<'m, 'd> {
        self.m.element(self)
    }
}

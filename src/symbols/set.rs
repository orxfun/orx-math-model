use crate::data_structs::ElemCont;
use crate::ptr::RefPtr;
use crate::symbols::{Element, PseudoInt};
use crate::Model;
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// fun

#[derive(derive_new::new)]
pub struct SetFun<'d>(Box<dyn Fn(&mut ElemCont<usize>) + 'd>);

impl<'d> SetFun<'d> {
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
                SetFun::new(Box::new(fill))
            }
            (PseudoInt::Raw(a), PseudoInt::Int(b)) => {
                // SAFETY: b is tied to lifetime of 'm and 'd
                let b = unsafe { RefPtr::new(b.fun_ptr()) };
                let fill = move |e: &mut ElemCont<usize>| e.fill(a..b.val().eval());
                SetFun::new(Box::new(fill))
            }
            (PseudoInt::Int(a), PseudoInt::Raw(b)) => {
                let a = unsafe { RefPtr::new(a.fun_ptr()) };
                let fill = move |e: &mut ElemCont<usize>| e.fill(a.val().eval()..b);
                SetFun::new(Box::new(fill))
            }
            (PseudoInt::Int(a), PseudoInt::Int(b)) => {
                let [a, b] = unsafe { [a, b].map(|x| RefPtr::new(x.fun_ptr())) };
                let fill = move |e: &mut ElemCont<usize>| e.fill(a.val().eval()..b.val().eval());
                SetFun::new(Box::new(fill))
            }
        }
    }

    pub fn fill(&self, elements: &mut ElemCont<usize>) {
        self.0(elements)
    }
}

// symbol

#[derive(derive_new::new, Clone, Copy)]
pub struct Set<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, SetFun<'d>>,
}

impl<'m, 'd> Set<'m, 'd> {
    pub(crate) fn fun_ptr(&self) -> RefPtr<SetFun<'d>> {
        // TODO: safety note
        unsafe { RefPtr::new(self.el.as_ptr()) }
    }

    pub fn idx(self) -> Element<'m, 'd> {
        self.m.element(self)
    }
}

use crate::elements::ElemCont;
use crate::ptr::RefPtr;
use crate::symbols::{IdxDepr, PseudoInt};
use crate::Model;
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// fun

#[derive(derive_new::new)]
pub struct SetOfIdxFun<'d>(Box<dyn Fn(&mut ElemCont<usize>) + 'd>);

impl SetOfIdxFun<'_> {
    pub fn empty() -> Self {
        Self(Box::new(|_| {}))
    }

    pub fn fill(&self, elements: &mut ElemCont<usize>) {
        self.0(elements)
    }
}

// data

#[derive(derive_new::new)]
pub struct SetOfIdxData<'d> {
    fun: SetOfIdxFun<'d>,
}

impl<'d> SetOfIdxData<'d> {
    pub fn empty() -> Self {
        Self::new(SetOfIdxFun::empty())
    }

    pub fn range<'m>(
        begin: impl Into<PseudoInt<'m, 'd>>,
        exclusive_end: impl Into<PseudoInt<'m, 'd>>,
    ) -> Self
    where
        'd: 'm,
    {
        let fun = match (begin.into(), exclusive_end.into()) {
            (PseudoInt::Raw(a), PseudoInt::Raw(b)) => {
                let fill = move |e: &mut ElemCont<usize>| e.fill(a..b);
                SetOfIdxFun(Box::new(fill))
            }
            (PseudoInt::Raw(a), PseudoInt::Int(b)) => {
                // SAFETY: b is tied to lifetime of 'm and 'd
                let b = unsafe { RefPtr::new(b.fun_ptr()) };
                let fill = move |e: &mut ElemCont<usize>| e.fill(a..b.val().eval());
                SetOfIdxFun(Box::new(fill))
            }
            (PseudoInt::Int(a), PseudoInt::Raw(b)) => {
                let a = unsafe { RefPtr::new(a.fun_ptr()) };
                let fill = move |e: &mut ElemCont<usize>| e.fill(a.val().eval()..b);
                SetOfIdxFun(Box::new(fill))
            }
            (PseudoInt::Int(a), PseudoInt::Int(b)) => {
                let [a, b] = unsafe { [a, b].map(|x| RefPtr::new(x.fun_ptr())) };
                let fill = move |e: &mut ElemCont<usize>| e.fill(a.val().eval()..b.val().eval());
                SetOfIdxFun(Box::new(fill))
            }
        };
        Self::new(fun)
    }

    pub fn fill(&self, elements: &mut ElemCont<usize>) {
        self.fun.fill(elements);
    }

    pub fn update_fun(&mut self, fun: SetOfIdxFun<'d>) {
        self.fun = fun;
    }

    pub fn update_fun_from_gen<I>(&mut self, fun: impl Fn() -> I + 'd)
    where
        I: IntoIterator<Item = usize>,
    {
        let fun = move |e: &mut ElemCont<usize>| e.fill(fun());
        self.update_fun(SetOfIdxFun(Box::new(fun)));
    }
}

// symbol

#[derive(derive_new::new, Clone, Copy)]
pub struct SetOfIdx<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, SetOfIdxData<'d>>,
}

impl<'m, 'd> SetOfIdx<'m, 'd> {
    pub(crate) fn fun_ptr(&self) -> RefPtr<SetOfIdxFun<'d>> {
        // TODO: safety note
        unsafe { RefPtr::new(core::ptr::addr_of!((*self.el.as_ptr()).fun)) }
    }

    pub fn idx(self) -> IdxDepr<'m, 'd> {
        self.m.idx_depr(self)
    }
}

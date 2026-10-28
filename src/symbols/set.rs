use crate::data_structs::ElemCont;
use crate::ptr::RefPtr;
use crate::symbols::Element;
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

    pub fn fill(&self, elements: &mut ElemCont<usize>) {
        self.0(elements)
    }
}

// data

#[derive(derive_new::new)]
pub struct SetData<'d> {
    fun: SetFun<'d>,
}

impl<'d> SetData<'d> {
    pub fn empty() -> Self {
        Self::new(SetFun::empty())
    }

    pub fn fill(&self, elements: &mut ElemCont<usize>) {
        self.fun.fill(elements);
    }

    pub fn update_fun(&mut self, fun: SetFun<'d>) {
        self.fun = fun;
    }

    pub fn update_fun_from_gen<I>(&mut self, fun: impl Fn() -> I + 'd)
    where
        I: IntoIterator<Item = usize>,
    {
        let fun = move |e: &mut ElemCont<usize>| e.fill(fun());
        self.update_fun(SetFun(Box::new(fun)));
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

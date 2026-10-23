use crate::elements::Elements;
use crate::symbols::Idx;
use crate::Model;
use alloc::boxed::Box;
use orx_imp_vec::Elem;

#[derive(derive_new::new)]
pub struct SetFun<'d>(Box<dyn Fn(&mut Elements) + 'd>);

impl SetFun<'_> {
    pub fn empty() -> Self {
        Self(Box::new(|_| {}))
    }

    pub fn fill(&self, elements: &mut Elements) {
        self.0(elements)
    }
}

#[derive(derive_new::new, Clone, Copy, PartialEq, Debug)]
pub struct SetRef {
    i: usize,
}

#[derive(derive_new::new)]
pub struct SetData<'d> {
    fun_ptr: *mut SetFun<'d>,
}

impl<'d> SetData<'d> {
    pub fn fill(&self, elements: &mut Elements) {
        unsafe { &*self.fun_ptr }.fill(elements);
    }

    pub fn update_fun(&mut self, fun: SetFun<'d>) {
        let _ = unsafe { self.fun_ptr.replace(fun) };
    }

    pub fn update_fun_from_gen<I>(&mut self, fun: impl Fn() -> I + 'd)
    where
        I: IntoIterator<Item = usize>,
    {
        let fun = move |e: &mut Elements| e.fill_with(fun());
        self.update_fun(SetFun(Box::new(fun)));
    }
}

#[derive(derive_new::new, Clone, Copy)]
pub struct Set<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, SetData<'d>>,
}

impl<'m, 'd> Set<'m, 'd> {
    pub fn idx(self) -> Idx<'m, 'd> {
        self.m.idx(self)
    }
}

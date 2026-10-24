use crate::elements::Elements;
use crate::symbols::Idx;
use crate::Model;
use alloc::boxed::Box;
use orx_imp_vec::Elem;

#[derive(derive_new::new)]
pub struct SetOfIdxFun<'d>(Box<dyn Fn(&mut Elements) + 'd>);

impl SetOfIdxFun<'_> {
    pub fn empty() -> Self {
        Self(Box::new(|_| {}))
    }

    pub fn fill(&self, elements: &mut Elements) {
        self.0(elements)
    }
}

#[derive(derive_new::new, Clone, Copy, PartialEq, Debug)]
pub struct SetOfIdxRef {
    i: usize,
}

#[derive(derive_new::new)]
pub struct SetOfIdxData<'d> {
    fun: SetOfIdxFun<'d>,
}

impl<'d> SetOfIdxData<'d> {
    pub fn fill(&self, elements: &mut Elements) {
        self.fun.fill(elements);
    }

    pub fn update_fun(&mut self, fun: SetOfIdxFun<'d>) {
        self.fun = fun;
    }

    pub fn update_fun_from_gen<I>(&mut self, fun: impl Fn() -> I + 'd)
    where
        I: IntoIterator<Item = usize>,
    {
        let fun = move |e: &mut Elements| e.fill_with(fun());
        self.update_fun(SetOfIdxFun(Box::new(fun)));
    }
}

#[derive(derive_new::new, Clone, Copy)]
pub struct SetOfIdx<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, SetOfIdxData<'d>>,
}

impl<'m, 'd> SetOfIdx<'m, 'd> {
    pub fn idx(self) -> Idx<'m, 'd> {
        self.m.idx(self)
    }
}

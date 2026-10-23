use crate::elements::Elements;
use crate::symbols::Idx;
use crate::Model;
use alloc::boxed::Box;
use orx_imp_vec::Elem;

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct SetRef {
    i: usize,
}

impl SetRef {
    pub fn new<'m, 'd>(set: Set<'m, 'd>) -> Self {
        let i = set.el.position_in(&set.m.set).expect("TODO: error msg");
        Self { i }
    }

    pub fn set<'m, 'd>(self, m: &'m Model<'d>) -> Set<'m, 'd> {
        Set::new(m, m.set.get(self.i).expect("TODO: error msg"))
    }
}

#[derive(derive_new::new)]
pub struct SetData<'d> {
    fun_ptr: *mut Box<dyn Fn(&mut Elements) + 'd>,
}

impl<'d> SetData<'d> {
    pub fn fill(&self, elements: &mut Elements) {
        let fun = unsafe { &*self.fun_ptr };
        fun(elements)
    }

    pub fn update_fun(&mut self, fun: Box<dyn Fn(&mut Elements) + 'd>) {
        let _ = unsafe { self.fun_ptr.replace(fun) };
    }

    pub fn update_fun_from_gen<I>(&mut self, fun: impl Fn() -> I + 'd)
    where
        I: IntoIterator<Item = usize>,
    {
        let fun = move |e: &mut Elements| e.fill_with(fun());
        self.update_fun(Box::new(fun));
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

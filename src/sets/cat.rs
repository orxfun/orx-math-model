use crate::indices::{CatIdx, DIdx};
use crate::sets::{elements::Elements, DSet};
use crate::Model;
use alloc::boxed::Box;
use core::marker::PhantomData;
use orx_imp_vec::Elem;

// data

#[derive(derive_new::new)]
pub struct DCatSet<'d, K> {
    fun: Box<dyn Fn(&mut Elements<K>) + 'd>,
}

impl<'d, K> DCatSet<'d, K> {
    pub fn empty() -> Self {
        Self {
            fun: Box::new(|_| {}),
        }
    }

    pub fn set_fun(&mut self, fun: Box<dyn Fn(&mut Elements<K>) + 'd>) {
        self.fun = fun;
    }

    pub fn bind<I>(&mut self, elements: impl Fn() -> I + 'd)
    where
        I: IntoIterator<Item = K>,
    {
        self.set_fun(Box::new(move |e: &mut Elements<_>| e.fill(elements())));
    }

    pub fn fill(&self, elements: &mut Elements<K>) {
        (self.fun)(elements)
    }
}

// symbol

#[derive(derive_new::new, Clone, Copy)]
#[new(visibility = "pub(crate)")]
pub struct CatSet<'m, 'd, K> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, DSet<'d>>,
    p: PhantomData<K>,
}

impl<'m, 'd, K> CatSet<'m, 'd, K> {
    pub(crate) fn bind(&self) {
        //
    }
}

impl<'m, 'd> CatSet<'m, 'd, usize> {
    pub fn idx(self) -> CatIdx<'m, 'd, usize> {
        CatIdx::new(self.m, self.m.index(DIdx::empty_cat_int(self)))
    }
}

impl<'m, 'd> CatSet<'m, 'd, &'d str> {
    pub fn idx(self) -> CatIdx<'m, 'd, &'d str> {
        CatIdx::new(self.m, self.m.index(DIdx::empty_cat_str(self)))
    }
}

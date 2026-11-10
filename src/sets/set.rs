use crate::{elements::Elements, index_types::IdxIn};
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// data

pub enum DSet2<'d> {
    CatUsize(Box<dyn Fn(&mut Elements<usize>) + 'd>),
    CatStr(Box<dyn Fn(&mut Elements<&'d str>) + 'd>),
    Ord(Box<dyn Fn(&mut Elements<i64>) + 'd>),
}

impl<'d> DSet2<'d> {
    pub fn new_cat() -> Self {
        Self::CatUsize(Box::new(|_| {}))
    }

    pub fn new_ord() -> Self {
        Self::Ord(Box::new(|_| {}))
    }

    pub fn bind<I: IdxIn<'d>>(&mut self, fun: impl Fn(&mut Elements<I>) + 'd) {
        I::bind_set(self, fun);
    }
}

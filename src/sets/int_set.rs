use crate::{sets::elements::Elements, Model};
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// fun

#[derive(derive_new::new)]
pub struct IntSetFun<'d>(Box<dyn Fn(&mut Elements<usize>) + 'd>);

impl<'d> IntSetFun<'d> {
    pub fn empty() -> Self {
        Self(Box::new(|_| {}))
    }
}

// symbol

pub struct IntSet<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, IntSetFun<'d>>,
}

use crate::{sets::elements::Elements, Model};
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// fun

#[derive(derive_new::new)]
pub struct SetFun<'d, K>(Box<dyn Fn(&mut Elements<K>) + 'd>);

impl<'d, K> SetFun<'d, K> {
    pub fn empty() -> Self {
        Self(Box::new(|_| {}))
    }
}

// symbol

#[derive(derive_new::new)]
pub struct Set<'m, 'd, K> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, SetFun<'d, K>>,
}

use crate::{sets::elements::Elements, Model};
use alloc::boxed::Box;
use orx_imp_vec::Elem;

// fun

#[derive(derive_new::new)]
pub struct IndicesFun<'d>(Box<dyn Fn(&mut Elements<usize>) + 'd>);

impl<'d> IndicesFun<'d> {
    pub fn empty() -> Self {
        Self(Box::new(|_| {}))
    }
}

// symbol

pub struct Indices<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, IndicesFun<'d>>,
}

use crate::model_references::SetRef;
use crate::symbols::Set;
use crate::Model;
use orx_imp_vec::Elem;

pub struct IdxData {
    s0: SetRef,
    value: usize,
}

impl IdxData {
    pub fn new<'m, 'd>(s0: Set<'m, 'd>) -> Self {
        let s0 = SetRef::new(s0);
        Self { s0, value: 0 }
    }

    pub fn s0(&self) -> SetRef {
        self.s0
    }

    pub fn value(&self) -> usize {
        self.value
    }

    pub fn value_ptr(&self) -> *const usize {
        &self.value
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
    }
}

#[derive(derive_new::new, Clone, Copy)]
pub struct Idx<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, IdxData>,
}

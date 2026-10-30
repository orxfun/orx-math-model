use crate::{ptr::RefPtr, sets::SetFun, Model};
use orx_imp_vec::Elem;

// data

#[derive(derive_new::new)]
pub struct IdxData<'d, K: Copy> {
    s0: RefPtr<SetFun<'d, K>>,
    value: K,
}

impl<'d, K: Copy> IdxData<'d, K> {
    pub fn read(&self) -> K {
        self.value
    }

    pub fn write(&mut self, value: K) {
        self.value = value;
    }
}

// symbol

pub struct Idx<'m, 'd, K: Copy> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, IdxData<'d, K>>,
}

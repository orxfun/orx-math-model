use crate::ptr::RefPtr;
use crate::sets::{Set, SetFun};
use crate::Model;
use orx_imp_vec::Elem;

// data

pub struct IdxData<'d, K: Copy + Default> {
    s0: RefPtr<SetFun<'d, K>>,
    value: K,
}

impl<'d, K: Copy + Default> IdxData<'d, K> {
    pub fn new(set: Set<'_, 'd, K>) -> Self {
        let s0 = set.fun_ptr();
        let value = K::default();
        Self { s0, value }
    }

    pub fn read(&self) -> K {
        self.value
    }

    pub fn write(&mut self, value: K) {
        self.value = value;
    }
}

// symbol

#[derive(derive_new::new)]
#[new(visibility = "pub(crate)")]
pub struct Idx<'m, 'd, K: Copy + Default> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, IdxData<'d, K>>,
}

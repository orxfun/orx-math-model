use crate::Model;
use orx_imp_vec::Elem;

//data

#[derive(derive_new::new, Clone, Copy)]
pub struct ParIntData(usize);

impl ParIntData {
    pub fn value(&self) -> usize {
        self.0
    }

    pub fn ptr_value(p: *const ParIntData) -> *const usize {
        unsafe { core::ptr::addr_of!((*p).0) }
    }
}

// symbol

#[derive(derive_new::new, Clone, Copy)]
pub struct ParInt<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, ParIntData>,
}

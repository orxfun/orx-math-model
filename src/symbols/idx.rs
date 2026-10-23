use crate::symbols::SetRef;
use crate::Model;
use orx_imp_vec::Elem;

// ref

#[derive(Clone, Copy)]
pub struct PtrIdxValue(*const usize);

impl PtrIdxValue {
    /// # SAFETY
    ///
    /// * Created value must not outlive `'m` or `'d`.
    pub unsafe fn new<'m, 'd>(idx: Idx<'m, 'd>) -> Self {
        Self(core::ptr::addr_of!((*idx.el.as_ptr()).value))
    }

    /// # SAFETY
    ///
    /// This method is safe as long is it is created with the `new` method
    /// satisfying its safety requirements.
    pub fn value(self) -> usize {
        unsafe { *self.0 }
    }
}

// data

#[derive(derive_new::new)]
pub struct IdxData {
    s0: SetRef,
    value: usize,
}

impl IdxData {
    pub fn s0(&self) -> SetRef {
        self.s0
    }

    pub fn value(&self) -> usize {
        self.value
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
    }

    pub fn ptr_value(p: *const IdxData) -> *const usize {
        unsafe { core::ptr::addr_of!((*p).value) }
    }
}

// symbol

#[derive(derive_new::new, Clone, Copy)]
pub struct Idx<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, IdxData>,
}

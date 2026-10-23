use crate::symbols::SetRef;
use crate::Model;
use orx_imp_vec::Elem;

// ref

#[derive(Clone, Copy)]
pub struct RefIdxValue {
    p: *mut usize,
}

impl RefIdxValue {
    /// # SAFETY
    ///
    /// * Created value must not outlive `'m` or `'d`.
    pub unsafe fn new<'m, 'd>(idx: Idx<'m, 'd>) -> Self {
        let el = unsafe { idx.el.get() };
        Self { p: el.value_ptr }
    }

    /// # SAFETY
    ///
    /// This method is safe as long is it is created with the `new` method
    /// satisfying its safety requirements.
    pub fn value(self) -> usize {
        unsafe { *self.p }
    }
}

// data

#[derive(derive_new::new)]
pub struct IdxData {
    s0: SetRef,
    value_ptr: *mut usize,
}

impl IdxData {
    pub fn s0(&self) -> SetRef {
        self.s0
    }

    pub fn read_value(&self) -> usize {
        unsafe { *self.value_ptr }
    }

    pub fn set_value(&mut self, value: usize) {
        unsafe { self.value_ptr.write(value) };
    }
}

// symbol

#[derive(derive_new::new, Clone, Copy)]
pub struct Idx<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, IdxData>,
}

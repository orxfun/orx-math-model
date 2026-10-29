use crate::ptr::RefPtr;
use crate::symbols::SetStrFun;
use crate::Model;
use orx_imp_vec::Elem;

// data

#[derive(derive_new::new)]
pub struct ElStrData<'d> {
    s0: RefPtr<SetStrFun<'d>>,
    value: &'d str,
}

impl<'d> ElStrData<'d> {
    pub fn s0(&self) -> RefPtr<SetStrFun<'d>> {
        self.s0
    }

    pub fn value(&self) -> &'d str {
        self.value
    }

    pub fn set_value(&mut self, value: &'d str) {
        self.value = value;
    }
}

// symbol

#[derive(derive_new::new, Clone, Copy)]
pub struct ElStr<'m, 'd> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, ElStrData<'d>>,
}

impl<'m, 'd> ElStr<'m, 'd> {
    pub(crate) fn val_ptr(&self) -> *const &'d str {
        unsafe { core::ptr::addr_of!((*self.el.as_ptr()).value) }
    }
}

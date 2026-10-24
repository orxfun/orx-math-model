use crate::Model;
use orx_imp_vec::Elem;

// data

#[derive(derive_new::new, Clone, Copy)]
pub struct ParCoreData<T: Copy>(T);

impl<T: Copy> ParCoreData<T> {
    pub fn value(&self) -> T {
        self.0
    }
}

// symbol

#[derive(derive_new::new, Clone, Copy)]
pub struct ParCore<'m, 'd, T: Copy> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, ParCoreData<T>>,
}

impl<'m, 'd, T: Copy> ParCore<'m, 'd, T> {
    pub(crate) fn val_ptr(&self) -> *const T {
        unsafe { core::ptr::addr_of!((*self.el.as_ptr()).0) }
    }
}

// number

pub type ParData = ParCoreData<f64>;

pub type Par<'m, 'd> = ParCore<'m, 'd, f64>;

// int

pub type ParIntData = ParCoreData<usize>;

pub type ParInt<'m, 'd> = ParCore<'m, 'd, usize>;

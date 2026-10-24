use crate::Model;
use orx_imp_vec::Elem;

// data

#[derive(derive_new::new, Clone, Copy)]
pub struct ParCoreData<T: Copy>(T);

impl<T: Copy> ParCoreData<T> {
    pub fn value(&self) -> T {
        self.0
    }

    pub fn ptr_value(p: *const ParCoreData<T>) -> *const T {
        unsafe { core::ptr::addr_of!((*p).0) }
    }
}

// symbol

#[derive(derive_new::new, Clone, Copy)]
pub struct ParCore<'m, 'd, T: Copy> {
    pub(crate) m: &'m Model<'d>,
    pub(crate) el: Elem<'m, ParCoreData<T>>,
}

// number

pub type ParData = ParCoreData<f64>;

pub type Par<'m, 'd> = ParCore<'m, 'd, f64>;

// int

pub type ParIntData = ParCoreData<usize>;

pub type ParInt<'m, 'd> = ParCore<'m, 'd, usize>;

use crate::ptr::ValPtr;
use crate::symbols::{Int, IntFun, RefSetOfIdx};
use crate::Model;
use alloc::boxed::Box;
use core::ops::Add;
use orx_imp_vec::Elem;

// data

#[derive(derive_new::new)]
pub struct IdxData {
    s0: RefSetOfIdx,
    value: usize,
}

impl IdxData {
    pub fn s0(&self) -> RefSetOfIdx {
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

impl<'m, 'd> Idx<'m, 'd> {
    pub(crate) fn val_ptr(&self) -> *const usize {
        unsafe { core::ptr::addr_of!((*self.el.as_ptr()).value) }
    }
}

// ops - Idx

impl<'m, 'd> Add<Idx<'m, 'd>> for Idx<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn add(self, rhs: Idx<'m, 'd>) -> Self::Output {
        assert!(core::ptr::eq(self.m, rhs.m), "TODO: err-msg");

        // SAFETY: both are tied to lifetime of 'm and 'd due to return type
        let [a, b] = unsafe { [ValPtr::new(self.val_ptr()), ValPtr::new(rhs.val_ptr())] };
        let f = move || a.val() + b.val();
        self.m.add_int(IntFun::new(Box::new(f)))
    }
}

// ops - usize

impl<'m, 'd> Add<usize> for Idx<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn add(self, rhs: usize) -> Self::Output {
        // SAFETY: both are tied to lifetime of 'm and 'd due to return type
        let a = unsafe { ValPtr::new(self.val_ptr()) };
        let f = move || a.val() + rhs;
        self.m.add_int(IntFun::new(Box::new(f)))
    }
}

impl<'m, 'd> Add<Idx<'m, 'd>> for usize {
    type Output = Int<'m, 'd>;

    fn add(self, rhs: Idx<'m, 'd>) -> Self::Output {
        // SAFETY: both are tied to lifetime of 'm and 'd due to return type
        let b = unsafe { ValPtr::new(rhs.val_ptr()) };
        let f = move || self + b.val();
        rhs.m.add_int(IntFun::new(Box::new(f)))
    }
}

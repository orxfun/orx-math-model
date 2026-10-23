use crate::symbols::{Int, IntFun, SetRef};
use crate::Model;
use alloc::boxed::Box;
use core::ops::Add;
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

// ops - Idx

impl<'m, 'd> Add<Idx<'m, 'd>> for Idx<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn add(self, rhs: Idx<'m, 'd>) -> Self::Output {
        assert!(core::ptr::eq(self.m, rhs.m), "TODO: err-msg");

        // SAFETY: p and fun_d1 is tied to lifetime of 'm and 'd due to return type Sca<'m, 'd>.
        let [a, b] = unsafe { [PtrIdxValue::new(self), PtrIdxValue::new(rhs)] };
        let f = move || a.value() + b.value();
        self.m.add_int(IntFun::new(Box::new(f)))
    }
}

// ops - usize

impl<'m, 'd> Add<usize> for Idx<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn add(self, rhs: usize) -> Self::Output {
        // SAFETY: p and fun_d1 is tied to lifetime of 'm and 'd due to return type Sca<'m, 'd>.
        let a = unsafe { PtrIdxValue::new(self) };
        let f = move || a.value() + rhs;
        self.m.add_int(IntFun::new(Box::new(f)))
    }
}

impl<'m, 'd> Add<Idx<'m, 'd>> for usize {
    type Output = Int<'m, 'd>;

    fn add(self, rhs: Idx<'m, 'd>) -> Self::Output {
        // SAFETY: p and fun_d1 is tied to lifetime of 'm and 'd due to return type Sca<'m, 'd>.
        let b = unsafe { PtrIdxValue::new(rhs) };
        let f = move || self + b.value();
        rhs.m.add_int(IntFun::new(Box::new(f)))
    }
}

use core::ptr::eq;
use orx_imp_vec::Elem;

pub fn ref_eq<T>(a: &T, b: &T) -> bool {
    core::ptr::eq(a, b)
}

// val

#[derive(Clone, Copy)]
pub struct ValPtr<T: Copy>(*mut T);

impl<T: Copy> ValPtr<T> {
    /// # SAFETY
    ///
    /// * Created value must not outlive `'m` or `'d`.
    unsafe fn new(p: *mut T) -> Self {
        Self(p)
    }

    /// # SAFETY
    ///
    /// This method is safe as long is it is created with the `new` method
    /// satisfying its safety requirements.
    pub fn val(self) -> T {
        unsafe { *self.0 }
    }

    pub fn write(self, value: T) {
        unsafe { self.0.write(value) };
    }
}

impl<T: Copy> From<Elem<'_, T>> for ValPtr<T> {
    fn from(el: Elem<'_, T>) -> Self {
        // TODO: safety note
        unsafe { Self::new(el.as_mut_ptr()) }
    }
}

// ref

pub struct RefPtr<T: ?Sized>(*const T);

impl<T> Clone for RefPtr<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Copy for RefPtr<T> {}

impl<T: ?Sized> RefPtr<T> {
    /// # SAFETY
    ///
    /// * Created value must not outlive `'m` or `'d`.
    pub unsafe fn new(p: *const T) -> Self {
        Self(p)
    }

    /// # SAFETY
    ///
    /// This method is safe as long is it is created with the `new` method
    /// satisfying its safety requirements.
    pub fn val(&self) -> &T {
        unsafe { &*self.0 }
    }
}

impl<T> From<Elem<'_, T>> for RefPtr<T> {
    fn from(el: Elem<'_, T>) -> Self {
        // TODO: safety note
        unsafe { Self::new(el.as_ptr()) }
    }
}

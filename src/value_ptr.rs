#[derive(Clone, Copy)]
pub struct ValPtr<T: Copy>(*const T);

impl<T: Copy> ValPtr<T> {
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
    pub fn value(self) -> T {
        unsafe { *self.0 }
    }
}

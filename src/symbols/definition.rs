use alloc::string::String;
use core::cell::UnsafeCell;

#[derive(Default)]
pub struct Definition(UnsafeCell<String>);

impl Definition {
    pub fn set(&self, definition: impl Into<String>) {
        // SAFETY: Definition does not implement Send or Sync.
        // This can only be called from a sequential program without a race condition.
        // Each time we update, we entirely set the value of the field to a valid string.
        let x = unsafe { &mut *self.0.get() };
        *x = definition.into();
    }
}

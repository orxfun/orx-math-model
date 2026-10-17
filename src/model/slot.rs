use crate::model::Slots;

pub struct Slot<'a, T> {
    value: *const T,
    slots: &'a (),
}

impl<'a, T> Slot<'a, T> {
    pub fn new(slots: &'a Slots<T>, value: T) -> Self {
        let rf = slots.imp_push_get_ref(value);
        Self {
            value: rf,
            slots: &(),
        }
    }

    pub fn value(&self) -> &'a T {
        // SAFETY: Slots holding `value` is still alive and Slots is grow only
        unsafe { &*self.value }
    }

    pub fn set(&self, value: T) {
        let p = self.value as *mut T;
        // SAFETY: Slots holding `value` is still alive and Slots is grow only
        // and Slots is not Sync preventing any race condition
        unsafe { p.write(value) };
    }
}

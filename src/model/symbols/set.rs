use crate::model::{Set0Gen, Slot};

#[derive(derive_new::new)]
pub struct Set<'a> {
    slot: Slot<'a, Set0Gen>,
}

impl Set<'_> {
    pub(crate) fn value(&self) -> &Set0Gen {
        self.slot.value()
    }

    pub(crate) fn set(&self, value: Set0Gen) {
        self.slot.set(value);
    }
}

use crate::model::{Set0Gen, Set0Gen2, Slot};

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

#[derive(derive_new::new)]
pub struct Set2<'a, 'd> {
    slot: Slot<'a, Set0Gen2<'d>>,
}

impl<'d> Set2<'_, 'd> {
    pub(crate) fn value(&self) -> &Set0Gen2<'d> {
        self.slot.value()
    }

    pub(crate) fn set(&self, value: Set0Gen2<'d>) {
        self.slot.set(value);
    }
}

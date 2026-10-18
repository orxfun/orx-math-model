use crate::model::{Set0Gen, Slot};

#[derive(derive_new::new)]
pub struct Set2<'m, 'd> {
    slot: Slot<'m, Set0Gen<'d>>,
}

impl<'d> Set2<'_, 'd> {
    pub(crate) fn value(&self) -> &Set0Gen<'d> {
        self.slot.value()
    }

    pub(crate) fn set(&self, value: Set0Gen<'d>) {
        self.slot.set(value);
    }
}

use crate::model::Slot;

#[derive(derive_new::new)]
pub struct Par<'a> {
    slot: Slot<'a, i64>,
}

impl Par<'_> {
    pub(crate) fn value(&self) -> i64 {
        *self.slot.value()
    }

    pub(crate) fn set(&self, value: i64) {
        self.slot.set(value);
    }
}

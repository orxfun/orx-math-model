use crate::model::Slot;

#[derive(derive_new::new)]
pub struct ParD0<'a> {
    slot: Slot<'a, i64>,
}

impl ParD0<'_> {
    pub fn value(&self) -> i64 {
        *self.slot.value()
    }
}

use crate::idxs::IdxVal;
use crate::sets::DSet;
use crate::{Ptr, Set, Slot};

#[derive(derive_new::new)]
pub struct DIdx<'d> {
    set_ptr: Ptr<'d, DSet<'d>>,
    slot: Slot<'d, IdxVal<'d>>,
}

impl<'d> DIdx<'d> {
    pub fn read<S: Set>(&self) -> S::IdxRaw<'d> {
        self.slot.get().read::<S>()
    }

    pub fn write<S: Set>(&self, value: S::IdxRaw<'d>) {
        self.slot.get_mut().write::<S>(value);
    }
}

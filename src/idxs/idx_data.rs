use crate::idxs::IdxVal;
use crate::sets::DSet;
use crate::{Ptr, PtrMut, Set, Slot};

#[derive(derive_new::new)]
pub struct DIdx<'d> {
    set_ptr: Ptr<'d, DSet<'d>>,
    slot: Slot<'d, IdxVal<'d>>,
}

impl<'d> DIdx<'d> {
    pub fn val_ptr_mut(&self) -> PtrMut<'d, IdxVal<'d>> {
        self.slot.ptr_mut()
    }

    #[inline(always)]
    pub fn read<S: Set>(&self) -> S::IdxRaw<'d> {
        self.slot.get().read::<S>()
    }

    #[inline(always)]
    pub fn write<S: Set>(&self, value: S::IdxRaw<'d>) {
        self.slot.get_mut().write::<S>(value);
    }

    pub fn set_ptr(&self) -> Ptr<'d, DSet<'d>> {
        self.set_ptr
    }
}

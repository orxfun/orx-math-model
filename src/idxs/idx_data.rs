use crate::idxs::IdxVal;
use crate::scalars::DNum;
use crate::sets::DSet;
use crate::{AsF64, Ptr, PtrMut, Set, SetOrd, Slot, ValTyp};
use alloc::boxed::Box;

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

    pub fn ord_to_int(&self) -> DNum<'d> {
        let val = self.slot.ptr();
        let fun = Box::new(move || val.get().read::<SetOrd>().as_f64());
        DNum::new(ValTyp::Int, fun)
    }
}

use crate::pars::p1_fun::FP1;
use crate::pars::par_fun::ParFun;
use crate::sets::DSet;
use crate::{AsF64, DimVec, Ptr, Res, Set, Slot, ValTyp};
use alloc::boxed::Box;

#[derive(derive_new::new)]
pub struct DPars<'d> {
    sets: DimVec<Ptr<'d, DSet<'d>>>,
    slot: Slot<'d, ParFun<'d>>,
}

impl<'d> DPars<'d> {
    #[inline(always)]
    pub fn fun(&self) -> &ParFun<'d> {
        self.slot.get()
    }

    pub fn fun_ptr(&self) -> Ptr<'d, ParFun<'d>> {
        self.slot.ptr()
    }

    // bind

    pub fn d1_bind<V: AsF64, I0: Set>(
        &self,
        val_typ: ValTyp,
        fun: impl Fn(I0::IdxRaw<'d>) -> V + 'd,
    ) -> Res {
        debug_assert_eq!(self.fun().val_typ, val_typ);
        self.slot.bind(ParFun::d1_create::<V, I0>(val_typ, fun))
    }

    pub fn d2_bind<V: AsF64, I0: Set, I1: Set>(
        &self,
        val_typ: ValTyp,
        fun: impl Fn(I0::IdxRaw<'d>, I1::IdxRaw<'d>) -> V + 'd,
    ) -> Res {
        debug_assert_eq!(self.fun().val_typ, val_typ);
        self.slot.bind(ParFun::d2_create::<V, I0, I1>(val_typ, fun))
    }
}

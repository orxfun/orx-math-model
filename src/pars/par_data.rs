use crate::pars::p1_fun::FP1;
use crate::pars::p2_fun::FP2;
use crate::pars::ParFun;
use crate::sets::DSet;
use crate::{Dim, DimVec, Ptr, Res, ResValid, Set, Slot, Val};

#[derive(derive_new::new)]
pub struct DPars<'d> {
    sets: DimVec<Ptr<'d, DSet<'d>>>,
    slot: Slot<'d, ParFun<'d>>,
}

impl<'d> DPars<'d> {
    pub fn fun_ptr(&self) -> Ptr<'d, ParFun<'d>> {
        self.slot.ptr()
    }

    // bind

    pub fn d1_bind<V: Val, I0: Set>(&self, fun: FP1<'d, V, I0>) -> Res {
        let fun = ParFun::d1_create::<V, I0>(fun);
        self.slot.bind(fun)
    }

    pub fn d2_bind<V: Val, I0: Set, I1: Set>(&self, fun: FP2<'d, V, I0, I1>) -> Res {
        let fun = ParFun::d2_create::<V, I0, I1>(fun);
        self.slot.bind(fun)
    }

    // at

    pub fn dim(&self) -> Dim {
        self.sets.dim()
    }

    #[inline(always)]
    pub fn d1_at<V: Val, I0: Set>(&self, i0: I0::IdxRaw<'d>) -> V::Raw {
        self.slot.get().d1_at::<V, I0>(i0)
    }

    #[inline(always)]
    pub fn d2_at<V: Val, I0: Set, I1: Set>(
        &self,
        i0: I0::IdxRaw<'d>,
        i1: I1::IdxRaw<'d>,
    ) -> V::Raw {
        self.slot.get().d2_at::<V, I0, I1>(i0, i1)
    }
}

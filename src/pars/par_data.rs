use crate::pars::p1::FP1;
use crate::pars::p2::FP2;
use crate::pars::ParFun;
use crate::sets::DSet;
use crate::{Dim, DimVec, Ptr, Res, ResValid, Set, Slot, Val};

#[derive(derive_new::new)]
pub struct DPar<'d> {
    dim: Dim,
    sets: DimVec<Ptr<'d, DSet<'d>>>,
    slot: Slot<'d, ParFun<'d>>,
}

impl<'d> DPar<'d> {
    pub fn d1_bind<V: Val, I0: Set>(&self, fun: FP1<'d, V, I0>) -> Res {
        let fun = ParFun::d1_create::<V, I0>(fun);
        self.slot.bind(fun)
    }

    pub fn d2_bind<V: Val, I0: Set, I1: Set>(&self, fun: FP2<'d, V, I0, I1>) -> Res {
        let fun = ParFun::d2_create::<V, I0, I1>(fun);
        self.slot.bind(fun)
    }
}

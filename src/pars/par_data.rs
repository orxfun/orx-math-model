use crate::pars::ParFun;
use crate::sets::DSet;
use crate::{Dim, DimVec, Ptr, Res, ResValid, Set, Slot, Val};

#[derive(derive_new::new)]
pub struct DP<'d> {
    dim: Dim,
    sets: DimVec<Ptr<'d, DSet<'d>>>,
    slot: Slot<'d, ParFun<'d>>,
}

use crate::pars::p1::{FunP1, FP1};
use crate::sets::DSet;
use crate::{Dim, DimVec, Ptr, Res, ResValid, Set, Slot, Val};

#[derive(derive_new::new)]
pub struct DP<'d> {
    dim: Dim,
    sets: DimVec<Ptr<'d, DSet<'d>>>,
    slot: Slot<'d, FunP1<'d>>,
}

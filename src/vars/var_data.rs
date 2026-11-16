use crate::sets::DSet;
use crate::{Dim, DimVec, Ptr, VarType};

pub struct DV<'d> {
    typ: VarType,
    dim: Dim,
    sets: DimVec<Ptr<'d, DSet<'d>>>,
}

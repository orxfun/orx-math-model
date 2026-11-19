use crate::idxs::DIdx;
use crate::Ptr;

pub struct SumD2<'d> {
    i0: Ptr<'d, DIdx<'d>>,
    i1: Ptr<'d, DIdx<'d>>,
}

use crate::idxs::DIdx;
use crate::Ptr;

pub struct SumD1<'d> {
    i0: Ptr<'d, DIdx<'d>>,
}

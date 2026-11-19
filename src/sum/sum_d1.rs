use crate::idxs::DIdx;
use crate::lin_expr::DSum;
use crate::Ptr;

pub struct SumD1<'d> {
    data: Ptr<'d, DSum<'d>>,
}

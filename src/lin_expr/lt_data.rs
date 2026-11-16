use crate::scalars::{DVal, DVar};
use crate::Ptr;

pub struct DLinTerm<'d> {
    coef: Ptr<'d, DVal<'d>>,
    var: Ptr<'d, DVar<'d>>,
}

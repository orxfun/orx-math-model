use crate::scalars::{DVal, DVar};

pub struct LtData<'d> {
    coef: DVal<'d>,
    var: DVar<'d>,
}

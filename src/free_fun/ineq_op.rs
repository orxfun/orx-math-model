use crate::constraints::IneqTyp;

pub trait IneqOp {
    fn typ() -> IneqTyp;
}

#[allow(non_camel_case_types)]
pub struct leq;

impl IneqOp for leq {
    fn typ() -> IneqTyp {
        IneqTyp::Leq
    }
}

#[allow(non_camel_case_types)]
pub struct geq;

impl IneqOp for geq {
    fn typ() -> IneqTyp {
        IneqTyp::Geq
    }
}

#[allow(non_camel_case_types)]
pub struct eq;

impl IneqOp for eq {
    fn typ() -> IneqTyp {
        IneqTyp::Eq
    }
}

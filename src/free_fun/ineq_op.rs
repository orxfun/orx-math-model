use crate::constraints::IneqTyp;

pub trait IneqOp {
    fn typ(&self) -> IneqTyp;
}

#[allow(non_camel_case_types)]
pub struct leq;

impl IneqOp for leq {
    fn typ(&self) -> IneqTyp {
        IneqTyp::Leq
    }
}

#[allow(non_camel_case_types)]
pub struct geq;

impl IneqOp for geq {
    fn typ(&self) -> IneqTyp {
        IneqTyp::Geq
    }
}

#[allow(non_camel_case_types)]
pub struct eq;

impl IneqOp for eq {
    fn typ(&self) -> IneqTyp {
        IneqTyp::Eq
    }
}

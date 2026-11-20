use crate::{pars::DPars, scalars::DNum};
use crate::{AsF64, Int, IntParD0, Num, NumParD0, Ptr, SymWrap};
use alloc::boxed::Box;

pub struct BoundsD0<'d> {
    lower: BoundD0<'d>,
    upper: BoundD0<'d>,
}

// d0

pub enum BoundD0<'d> {
    None,
    Con(f64),
    Num(Ptr<'d, DNum<'d>>),
}

impl<'d, N: AsF64> From<N> for BoundD0<'d> {
    fn from(value: N) -> Self {
        Self::Con(value.as_f64())
    }
}

impl<'m, 'd> From<Int<'m, 'd>> for BoundD0<'d> {
    fn from(value: Int<'m, 'd>) -> Self {
        Self::Num(value.ptr())
    }
}

impl<'m, 'd> From<Num<'m, 'd>> for BoundD0<'d> {
    fn from(value: Num<'m, 'd>) -> Self {
        Self::Num(value.ptr())
    }
}

impl<'m, 'd> From<IntParD0<'m, 'd>> for BoundD0<'d> {
    fn from(value: IntParD0<'m, 'd>) -> Self {
        Self::Num(value.to_int().ptr())
    }
}

impl<'m, 'd> From<NumParD0<'m, 'd>> for BoundD0<'d> {
    fn from(value: NumParD0<'m, 'd>) -> Self {
        Self::Num(value.to_num().ptr())
    }
}

// d1

pub enum BoundD1<'d> {
    None,
    Con(f64),
    Num(Ptr<'d, DNum<'d>>),
    ParD1(Ptr<'d, DPars<'d>>),
}

// d2

pub enum BoundD2<'d> {
    None,
    Con(f64),
    Num(Ptr<'d, DNum<'d>>),
    ParD2(Ptr<'d, DPars<'d>>),
}

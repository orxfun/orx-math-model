use crate::{pars::DPars, scalars::DNum};
use crate::{
    AsF64, Int, IntParD0, IntParD1, IntParD2, Num, NumParD0, NumParD1, NumParD2, Ptr, Set, SymWrap,
};
use alloc::boxed::Box;

pub struct Bounds<T> {
    pub lower: T,
    pub upper: T,
}

macro_rules! impl_from_con {
    ($bound:ty) => {
        impl<'d, N: AsF64> From<N> for $bound {
            fn from(value: N) -> Self {
                Self::Con(value.as_f64())
            }
        }
    };
}

macro_rules! impl_from_num {
    ($bound:ty) => {
        impl<'m, 'd> From<Int<'m, 'd>> for $bound {
            fn from(value: Int<'m, 'd>) -> Self {
                Self::Num(value.ptr())
            }
        }

        impl<'m, 'd> From<Num<'m, 'd>> for $bound {
            fn from(value: Num<'m, 'd>) -> Self {
                Self::Num(value.ptr())
            }
        }
    };
}

// d0

pub enum BoundD0<'d> {
    None,
    Con(f64),
    Num(Ptr<'d, DNum<'d>>),
}

impl_from_con!(BoundD0<'d>);
impl_from_num!(BoundD0<'d>);

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

impl_from_con!(BoundD1<'d>);
impl_from_num!(BoundD1<'d>);

impl<'m, 'd, I0: Set> From<IntParD1<'m, 'd, I0>> for BoundD1<'d> {
    fn from(value: IntParD1<'m, 'd, I0>) -> Self {
        Self::ParD1(value.ptr())
    }
}

impl<'m, 'd, I0: Set> From<NumParD1<'m, 'd, I0>> for BoundD1<'d> {
    fn from(value: NumParD1<'m, 'd, I0>) -> Self {
        Self::ParD1(value.ptr())
    }
}

// d2

pub enum BoundD2<'d> {
    None,
    Con(f64),
    Num(Ptr<'d, DNum<'d>>),
    ParD2(Ptr<'d, DPars<'d>>),
}
impl_from_con!(BoundD2<'d>);
impl_from_num!(BoundD2<'d>);

impl<'m, 'd, I0: Set, I1: Set> From<IntParD2<'m, 'd, I0, I1>> for BoundD2<'d> {
    fn from(value: IntParD2<'m, 'd, I0, I1>) -> Self {
        Self::ParD2(value.ptr())
    }
}

impl<'m, 'd, I0: Set, I1: Set> From<NumParD2<'m, 'd, I0, I1>> for BoundD2<'d> {
    fn from(value: NumParD2<'m, 'd, I0, I1>) -> Self {
        Self::ParD2(value.ptr())
    }
}

use crate::{pars::DPars, scalars::DNum};
use crate::{
    AsF64, Int, IntParD0, IntParD1, IntParD2, Num, NumParD0, NumParD1, NumParD2, Ptr, Set, SymWrap,
};
use alloc::boxed::Box;

// bounds

#[repr(C)]
pub union UBounds<'d> {
    pub d0: Bounds<BoundD0<'d>>,
    pub d1: Bounds<BoundD1<'d>>,
    pub d2: Bounds<BoundD2<'d>>,
}

#[derive(derive_new::new, Clone, Copy)]
pub struct Bounds<T> {
    pub lower: T,
    pub upper: T,
}

// helper macros

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

        impl<'m, 'd> From<IntParD0<'m, 'd>> for $bound {
            fn from(value: IntParD0<'m, 'd>) -> Self {
                Self::Num(value.to_int().ptr())
            }
        }

        impl<'m, 'd> From<NumParD0<'m, 'd>> for $bound {
            fn from(value: NumParD0<'m, 'd>) -> Self {
                Self::Num(value.to_num().ptr())
            }
        }
    };
}

// d0

#[derive(Clone, Copy)]
pub enum BoundD0<'d> {
    None,
    Con(f64),
    Num(Ptr<'d, DNum<'d>>),
}

pub struct ResolvedBoundD0(Option<f64>);

impl ResolvedBoundD0 {
    pub fn eval(&self) -> Option<f64> {
        self.0
    }
}

impl_from_con!(BoundD0<'d>);
impl_from_num!(BoundD0<'d>);

impl<'d> BoundD0<'d> {
    #[inline(always)]
    pub fn eval(&self) -> Option<f64> {
        match self {
            Self::None => None,
            Self::Con(x) => Some(*x),
            Self::Num(x) => Some(x.get().eval()),
        }
    }

    pub fn resolve(&self) -> ResolvedBoundD0 {
        ResolvedBoundD0(match self {
            Self::None => None,
            Self::Con(x) => Some(*x),
            Self::Num(x) => Some(x.get().eval()),
        })
    }
}

// d1

#[derive(Clone, Copy)]
pub enum BoundD1<'d> {
    None,
    Con(f64),
    Num(Ptr<'d, DNum<'d>>),
    ParD1(Ptr<'d, DPars<'d>>),
}

pub enum ResolvedBoundD1<'d, I0: Set> {
    None,
    Con(f64),
    ParD1(&'d Box<dyn Fn(I0::IdxRaw<'d>) -> f64 + 'd>),
}

impl<'d, I0: Set> ResolvedBoundD1<'d, I0> {
    #[inline(always)]
    pub fn eval(&self, i0: I0::IdxRaw<'d>) -> Option<f64> {
        match self {
            Self::None => None,
            Self::Con(x) => Some(*x),
            Self::ParD1(x) => Some(x(i0)),
        }
    }
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

impl<'d> BoundD1<'d> {
    #[inline(always)]
    pub fn eval<I0: Set>(&self, i0: I0::IdxRaw<'d>) -> Option<f64> {
        match self {
            Self::None => None,
            Self::Con(x) => Some(*x),
            Self::Num(x) => Some(x.get().eval()),
            Self::ParD1(x) => Some(x.get().fun().d1_at::<I0>(i0)),
        }
    }

    pub fn resolve<I0: Set>(&self) -> ResolvedBoundD1<'d, I0> {
        match self {
            Self::None => ResolvedBoundD1::None,
            Self::Con(x) => ResolvedBoundD1::Con(*x),
            Self::Num(x) => ResolvedBoundD1::Con(x.get().eval()),
            Self::ParD1(x) => {
                let fun = unsafe { &x.get().fun().data.d1 };
                ResolvedBoundD1::ParD1(I0::unpack_p1_fun(fun))
            }
        }
    }
}

// d2

#[derive(Clone, Copy)]
pub enum BoundD2<'d> {
    None,
    Con(f64),
    Num(Ptr<'d, DNum<'d>>),
    ParD2(Ptr<'d, DPars<'d>>),
}

pub enum ResolvedBoundD2<'d, I0: Set, I1: Set> {
    None,
    Con(f64),
    ParD2(&'d Box<dyn Fn(I0::IdxRaw<'d>, I1::IdxRaw<'d>) -> f64 + 'd>),
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

impl<'d> BoundD2<'d> {
    #[inline(always)]
    pub fn eval<I0: Set, I1: Set>(&self, i0: I0::IdxRaw<'d>, i1: I1::IdxRaw<'d>) -> Option<f64> {
        match self {
            Self::None => None,
            Self::Con(x) => Some(*x),
            Self::Num(x) => Some(x.get().eval()),
            Self::ParD2(x) => Some(x.get().fun().d2_at::<I0, I1>(i0, i1)),
        }
    }

    pub fn resolve<I0: Set, I1: Set>(&self) -> ResolvedBoundD2<'d, I0, I1> {
        match self {
            Self::None => ResolvedBoundD2::None,
            Self::Con(x) => ResolvedBoundD2::Con(*x),
            Self::Num(x) => ResolvedBoundD2::Con(x.get().eval()),
            Self::ParD2(x) => {
                let fun = unsafe { &x.get().fun().data.d2 };
                ResolvedBoundD2::ParD2(I0::unpack_p2_fun::<I1>(fun))
            }
        }
    }
}

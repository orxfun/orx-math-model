use crate::{
    data_structs::funvec::FunVec,
    numerics::num::{RealNum, NEGINF, ONE, POSINF, ZERO},
};
use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

#[derive(Clone)]
pub enum Bounds<const D: usize> {
    Unbounded,
    Nonnegative,
    Nonpositive,
    BetweenZeroAndOne,

    NonnegativeWithConstUB(RealNum),
    NonnegativeWithParamUB(String, Rc<dyn FunVec<D, RealNum>>),

    NonpositiveWithConstLB(RealNum),
    NonpositiveWithParamLB(String, Rc<dyn FunVec<D, RealNum>>),

    ConstBounds(RealNum, RealNum),
    ParamBounds(
        String,
        Rc<dyn FunVec<D, RealNum>>,
        String,
        Rc<dyn FunVec<D, RealNum>>,
    ),
}

impl<const D: usize> Bounds<D> {
    pub fn get_bounds(&self, indices: [usize; D]) -> (RealNum, RealNum) {
        match self {
            Self::Unbounded => (NEGINF, POSINF),
            Self::Nonnegative => (ZERO, POSINF),
            Self::Nonpositive => (NEGINF, ZERO),
            Self::BetweenZeroAndOne => (ZERO, ONE),

            Self::NonnegativeWithConstUB(upper) => (ZERO, *upper),
            Self::NonnegativeWithParamUB(_, upper) => (ZERO, upper.value(indices)),

            Self::NonpositiveWithConstLB(lower) => (*lower, ZERO),
            Self::NonpositiveWithParamLB(_, lower) => (lower.value(indices), ZERO),

            Self::ConstBounds(lower, upper) => (*lower, *upper),
            Self::ParamBounds(_, lower, _, upper) => (lower.value(indices), upper.value(indices)),
        }
    }
}

impl<const D: usize> Debug for Bounds<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unbounded => write!(f, "Unbounded"),
            Self::Nonnegative => write!(f, "Nonnegative"),
            Self::Nonpositive => write!(f, "Nonpositive"),
            Self::BetweenZeroAndOne => write!(f, "BetweenZeroAndOne"),
            Self::NonnegativeWithConstUB(arg0) => {
                f.debug_tuple("NonnegativeWithConstUB").field(arg0).finish()
            }
            Self::NonnegativeWithParamUB(p, _) => {
                f.debug_tuple("NonnegativeWithParamUB").field(p).finish()
            }
            Self::NonpositiveWithConstLB(arg0) => {
                f.debug_tuple("NonpositiveWithConstLB").field(arg0).finish()
            }
            Self::NonpositiveWithParamLB(p, _) => {
                f.debug_tuple("NonpositiveWithParamLB").field(p).finish()
            }
            Self::ConstBounds(arg0, arg1) => f
                .debug_tuple("ConstBounds")
                .field(arg0)
                .field(arg1)
                .finish(),
            Self::ParamBounds(p1, _, p2, _) => {
                f.debug_tuple("ParamBounds").field(&(p1, p2)).finish()
            }
        }
    }
}
impl<const D: usize> Display for Bounds<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

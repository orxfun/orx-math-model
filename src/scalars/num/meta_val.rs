use crate::{
    scalars::{num::DNum, Int, Num},
    Symbol,
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ValTyp {
    Int,
    Num,
}

pub trait Val: Clone + Copy + 'static {
    type Sym<'m, 'd: 'm>;

    fn typ() -> ValTyp;

    fn sym<'m, 'd: 'm>(sym: Symbol<'m, 'd, DNum<'d>>) -> Self::Sym<'m, 'd>;
}

#[derive(Clone, Copy)]
pub struct ValInt;

impl Val for ValInt {
    type Sym<'m, 'd: 'm> = Int<'m, 'd>;

    fn typ() -> ValTyp {
        ValTyp::Int
    }

    fn sym<'m, 'd: 'm>(sym: Symbol<'m, 'd, DNum<'d>>) -> Self::Sym<'m, 'd> {
        Self::Sym::new(sym)
    }
}

#[derive(Clone, Copy)]
pub struct ValNum;

impl Val for ValNum {
    type Sym<'m, 'd: 'm> = Num<'m, 'd>;

    fn typ() -> ValTyp {
        ValTyp::Num
    }

    fn sym<'m, 'd: 'm>(sym: Symbol<'m, 'd, DNum<'d>>) -> Self::Sym<'m, 'd> {
        Self::Sym::new(sym)
    }
}

// as f64

#[inline(always)]
pub fn round_f64(x: f64) -> i64 {
    debug_assert!(x.is_finite());
    debug_assert!(x >= i64::MIN as f64);
    debug_assert!(x <= i64::MAX as f64);
    match x >= 0.0 {
        true => (x + 0.5) as i64,
        false => (x - 0.5) as i64,
    }
}

pub trait AsF64: 'static + Clone + Copy {
    fn as_f64(self) -> f64;
}

impl AsF64 for f64 {
    #[inline(always)]
    fn as_f64(self) -> f64 {
        self
    }
}

impl AsF64 for f32 {
    #[inline(always)]
    fn as_f64(self) -> f64 {
        self as f64
    }
}

impl AsF64 for i64 {
    #[inline(always)]
    fn as_f64(self) -> f64 {
        self as f64
    }
}

impl AsF64 for i32 {
    #[inline(always)]
    fn as_f64(self) -> f64 {
        self as f64
    }
}

impl AsF64 for u64 {
    #[inline(always)]
    fn as_f64(self) -> f64 {
        self as f64
    }
}

impl AsF64 for u32 {
    #[inline(always)]
    fn as_f64(self) -> f64 {
        self as f64
    }
}

impl AsF64 for isize {
    #[inline(always)]
    fn as_f64(self) -> f64 {
        self as f64
    }
}

impl AsF64 for usize {
    #[inline(always)]
    fn as_f64(self) -> f64 {
        self as f64
    }
}

// as i64

pub trait AsI64: AsF64 {}

impl AsI64 for i64 {}

impl AsI64 for i32 {}

impl AsI64 for u64 {}

impl AsI64 for u32 {}

impl AsI64 for isize {}

impl AsI64 for usize {}

use crate::scalars::{num::DNum, Int, Num};
use crate::{IntParD0, IntParD1, IntParD2, Model, NumParD0, NumParD1, NumParD2, Set, Symbol};
use alloc::string::ToString;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ValTyp {
    Int,
    Num,
}

pub trait Val: Clone + Copy + 'static {
    type Sym<'m, 'd: 'm>;

    fn typ() -> ValTyp;

    fn sym<'m, 'd: 'm>(sym: Symbol<'m, 'd, DNum<'d>>) -> Self::Sym<'m, 'd>;

    // pars
    type ParD0<'m, 'd: 'm>;
    type ParD1<'m, 'd: 'm, I0: Set>;
    type ParD2<'m, 'd: 'm, I0: Set, I1: Set>;
    fn par_d0<'m, 'd: 'm>(m: &'m Model<'d>, key: impl ToString) -> Self::ParD0<'m, 'd>;
    fn par_d1<'m, 'd: 'm, I0: Set>(
        m: &'m Model<'d>,
        key: impl ToString,
        s0: I0::SetSym<'m, 'd>,
    ) -> Self::ParD1<'m, 'd, I0>;
    fn par_d2<'m, 'd: 'm, I0: Set, I1: Set>(
        m: &'m Model<'d>,
        key: impl ToString,
        s0: I0::SetSym<'m, 'd>,
        s1: I1::SetSym<'m, 'd>,
    ) -> Self::ParD2<'m, 'd, I0, I1>;
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

    // pars
    type ParD0<'m, 'd: 'm> = IntParD0<'m, 'd>;
    type ParD1<'m, 'd: 'm, I0: Set> = IntParD1<'m, 'd, I0>;
    type ParD2<'m, 'd: 'm, I0: Set, I1: Set> = IntParD2<'m, 'd, I0, I1>;
    fn par_d0<'m, 'd: 'm>(m: &'m Model<'d>, key: impl ToString) -> Self::ParD0<'m, 'd> {
        m.par0_int(key)
    }
    fn par_d1<'m, 'd: 'm, I0: Set>(
        m: &'m Model<'d>,
        key: impl ToString,
        s0: I0::SetSym<'m, 'd>,
    ) -> Self::ParD1<'m, 'd, I0> {
        m.par1_int(key, s0)
    }
    fn par_d2<'m, 'd: 'm, I0: Set, I1: Set>(
        m: &'m Model<'d>,
        key: impl ToString,
        s0: I0::SetSym<'m, 'd>,
        s1: I1::SetSym<'m, 'd>,
    ) -> Self::ParD2<'m, 'd, I0, I1> {
        m.par2_int(key, s0, s1)
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

    // pars
    type ParD0<'m, 'd: 'm> = NumParD0<'m, 'd>;
    type ParD1<'m, 'd: 'm, I0: Set> = NumParD1<'m, 'd, I0>;
    type ParD2<'m, 'd: 'm, I0: Set, I1: Set> = NumParD2<'m, 'd, I0, I1>;

    fn par_d0<'m, 'd: 'm>(m: &'m Model<'d>, key: impl ToString) -> Self::ParD0<'m, 'd> {
        m.par0_num(key)
    }
    fn par_d1<'m, 'd: 'm, I0: Set>(
        m: &'m Model<'d>,
        key: impl ToString,
        s0: I0::SetSym<'m, 'd>,
    ) -> Self::ParD1<'m, 'd, I0> {
        m.par1_num(key, s0)
    }
    fn par_d2<'m, 'd: 'm, I0: Set, I1: Set>(
        m: &'m Model<'d>,
        key: impl ToString,
        s0: I0::SetSym<'m, 'd>,
        s1: I1::SetSym<'m, 'd>,
    ) -> Self::ParD2<'m, 'd, I0, I1> {
        m.par2_num(key, s0, s1)
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

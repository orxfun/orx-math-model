#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ValTyp {
    Int,
    Num,
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

pub trait AsF64 {
    #[inline(always)]
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

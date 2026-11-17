#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ValTyp {
    Int,
    Num,
}

// as f64

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
    fn as_f64(self) -> f64;
}

impl AsF64 for f64 {
    fn as_f64(self) -> f64 {
        self
    }
}

impl AsF64 for f32 {
    fn as_f64(self) -> f64 {
        self as f64
    }
}

impl AsF64 for i64 {
    fn as_f64(self) -> f64 {
        self as f64
    }
}

impl AsF64 for i32 {
    fn as_f64(self) -> f64 {
        self as f64
    }
}

impl AsF64 for u64 {
    fn as_f64(self) -> f64 {
        self as f64
    }
}

impl AsF64 for u32 {
    fn as_f64(self) -> f64 {
        self as f64
    }
}

impl AsF64 for isize {
    fn as_f64(self) -> f64 {
        self as f64
    }
}

impl AsF64 for usize {
    fn as_f64(self) -> f64 {
        self as f64
    }
}

// as i64

pub trait AsI64: AsF64 {
    fn as_i64(self) -> i64;
}

impl AsI64 for i64 {
    fn as_i64(self) -> i64 {
        self
    }
}

impl AsI64 for i32 {
    fn as_i64(self) -> i64 {
        self as i64
    }
}

impl AsI64 for u64 {
    fn as_i64(self) -> i64 {
        self as i64
    }
}

impl AsI64 for u32 {
    fn as_i64(self) -> i64 {
        self as i64
    }
}

impl AsI64 for isize {
    fn as_i64(self) -> i64 {
        self as i64
    }
}

impl AsI64 for usize {
    fn as_i64(self) -> i64 {
        self as i64
    }
}

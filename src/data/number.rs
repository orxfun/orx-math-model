pub trait Number: Clone + Copy {
    fn to_f64(self) -> f64;
}

impl Number for usize {
    #[inline(always)]
    fn to_f64(self) -> f64 {
        self as f64
    }
}

impl Number for u64 {
    #[inline(always)]
    fn to_f64(self) -> f64 {
        self as f64
    }
}

impl Number for u32 {
    #[inline(always)]
    fn to_f64(self) -> f64 {
        self as f64
    }
}

impl Number for isize {
    #[inline(always)]
    fn to_f64(self) -> f64 {
        self as f64
    }
}

impl Number for i64 {
    #[inline(always)]
    fn to_f64(self) -> f64 {
        self as f64
    }
}

impl Number for i32 {
    #[inline(always)]
    fn to_f64(self) -> f64 {
        self as f64
    }
}

impl Number for f64 {
    #[inline(always)]
    fn to_f64(self) -> f64 {
        self as f64
    }
}

impl Number for f32 {
    #[inline(always)]
    fn to_f64(self) -> f64 {
        self as f64
    }
}

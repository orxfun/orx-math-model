pub type RealNum = f32;
pub type IntNum = i32;

pub(crate) const ZERO: RealNum = 0.0;
pub(crate) const ONE: RealNum = 1.0;
pub(crate) const POSINF: RealNum = RealNum::INFINITY;
pub(crate) const NEGINF: RealNum = RealNum::NEG_INFINITY;

pub trait OptionalRealNum: Clone + Copy {
    fn unwrap_to_real_num(self) -> RealNum;
}
macro_rules! impl_intorealnum_num {
    ($num:ty) => {
        impl OptionalRealNum for $num {
            fn unwrap_to_real_num(self) -> RealNum {
                self.into_real_num()
            }
        }
        impl OptionalRealNum for Option<$num> {
            fn unwrap_to_real_num(self) -> RealNum {
                self.expect("Option<{number}> returned None")
                    .into_real_num()
            }
        }
    };
}
impl_intorealnum_num!(i64);
impl_intorealnum_num!(i32);
impl_intorealnum_num!(f64);
impl_intorealnum_num!(f32);
impl_intorealnum_num!(u64);
impl_intorealnum_num!(u32);
impl_intorealnum_num!(usize);

pub trait Num: Clone + Copy + ToString + 'static {
    fn into_real_num(self) -> RealNum;
    fn is_zero(self) -> bool;
    fn is_one(self) -> bool;
    fn is_minus_one(self) -> bool;
}
pub trait Int: Num {}

pub fn real_to_index(real: RealNum) -> usize {
    // todo! proper conversion
    real as usize
}

macro_rules! impl_num_for_float {
    ($x:ty) => {
        impl Num for $x {
            #[inline(always)]
            fn into_real_num(self) -> RealNum {
                self as RealNum
            }
            #[inline(always)]
            fn is_zero(self) -> bool {
                let num = self as RealNum;
                num < RealNum::EPSILON && num > -RealNum::EPSILON
            }
            #[inline(always)]
            fn is_one(self) -> bool {
                let num = self as RealNum;
                num < 1.0 + RealNum::EPSILON && num > 1.0 - RealNum::EPSILON
            }
            #[inline(always)]
            fn is_minus_one(self) -> bool {
                let num = self as RealNum;
                num < -1.0 + RealNum::EPSILON && num > -1.0 - RealNum::EPSILON
            }
        }
    };
}
macro_rules! impl_num_for_int {
    ($x:ty) => {
        impl Num for $x {
            #[inline(always)]
            fn into_real_num(self) -> RealNum {
                self as RealNum
            }
            #[inline(always)]
            fn is_zero(self) -> bool {
                self == 0
            }
            #[inline(always)]
            fn is_one(self) -> bool {
                self == 1
            }
            #[inline(always)]
            fn is_minus_one(self) -> bool {
                self == -1
            }
        }
    };
}
macro_rules! impl_num_for_int_unsigned {
    ($x:ty) => {
        impl Num for $x {
            #[inline(always)]
            fn into_real_num(self) -> RealNum {
                self as RealNum
            }
            #[inline(always)]
            fn is_zero(self) -> bool {
                self == 0
            }
            #[inline(always)]
            fn is_one(self) -> bool {
                self == 1
            }
            #[inline(always)]
            fn is_minus_one(self) -> bool {
                false
            }
        }
    };
}

impl_num_for_float!(f64);
impl_num_for_float!(f32);
impl_num_for_int_unsigned!(usize);
impl_num_for_int_unsigned!(u64);
impl_num_for_int_unsigned!(u32);
impl_num_for_int!(i64);
impl_num_for_int!(i32);

impl Int for usize {}
impl Int for u64 {}
impl Int for u32 {}
impl Int for i64 {}
impl Int for i32 {}

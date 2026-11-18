use crate::scalars::{DNum, ValInt, ValNum};
use crate::{Int, Num, SymWrap};
use core::ops::Mul;

// num/int + num/int

impl<'m, 'd> Mul<Num<'m, 'd>> for Num<'m, 'd> {
    type Output = Num<'m, 'd>;

    fn mul(self, rhs: Num<'m, 'd>) -> Self::Output {
        let data = DNum::mul(self.ptr(), rhs.ptr());
        self.m().val::<ValNum>(data)
    }
}

impl<'m, 'd> Mul<Int<'m, 'd>> for Num<'m, 'd> {
    type Output = Num<'m, 'd>;

    fn mul(self, rhs: Int<'m, 'd>) -> Self::Output {
        let data = DNum::mul(self.ptr(), rhs.ptr());
        self.m().val::<ValNum>(data)
    }
}

impl<'m, 'd> Mul<Num<'m, 'd>> for Int<'m, 'd> {
    type Output = Num<'m, 'd>;

    fn mul(self, rhs: Num<'m, 'd>) -> Self::Output {
        let data = DNum::mul(self.ptr(), rhs.ptr());
        self.m().val::<ValNum>(data)
    }
}

impl<'m, 'd> Mul<Int<'m, 'd>> for Int<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn mul(self, rhs: Int<'m, 'd>) -> Self::Output {
        let data = DNum::mul(self.ptr(), rhs.ptr());
        self.m().val::<ValInt>(data)
    }
}

// num + con

macro_rules! num_times_con {
    ($c:ty) => {
        impl<'m, 'd> Mul<$c> for Num<'m, 'd> {
            type Output = Num<'m, 'd>;

            fn mul(self, rhs: $c) -> Self::Output {
                let data = DNum::mul(self.ptr(), self.m().val_num_con(rhs).ptr());
                self.m().val::<ValNum>(data)
            }
        }

        impl<'m, 'd> Mul<Num<'m, 'd>> for $c {
            type Output = Num<'m, 'd>;

            fn mul(self, rhs: Num<'m, 'd>) -> Self::Output {
                rhs * self
            }
        }
    };
}

num_times_con!(f64);
num_times_con!(f32);
num_times_con!(i64);
num_times_con!(i32);
num_times_con!(u64);
num_times_con!(u32);
num_times_con!(usize);
num_times_con!(isize);

// int + icon

macro_rules! int_times_icon {
    ($c:ty) => {
        impl<'m, 'd> Mul<$c> for Int<'m, 'd> {
            type Output = Int<'m, 'd>;

            fn mul(self, rhs: $c) -> Self::Output {
                let data = DNum::mul(self.ptr(), self.m().val_num_con(rhs).ptr());
                self.m().val::<ValInt>(data)
            }
        }

        impl<'m, 'd> Mul<Int<'m, 'd>> for $c {
            type Output = Int<'m, 'd>;

            fn mul(self, rhs: Int<'m, 'd>) -> Self::Output {
                rhs * self
            }
        }
    };
}

int_times_icon!(i64);
int_times_icon!(i32);
int_times_icon!(u64);
int_times_icon!(u32);
int_times_icon!(usize);
int_times_icon!(isize);

// int + fcon

macro_rules! int_times_fcon {
    ($c:ty) => {
        impl<'m, 'd> Mul<$c> for Int<'m, 'd> {
            type Output = Num<'m, 'd>;

            fn mul(self, rhs: $c) -> Self::Output {
                let data = DNum::mul(self.ptr(), self.m().val_num_con(rhs).ptr());
                self.m().val::<ValNum>(data)
            }
        }

        impl<'m, 'd> Mul<Int<'m, 'd>> for $c {
            type Output = Num<'m, 'd>;

            fn mul(self, rhs: Int<'m, 'd>) -> Self::Output {
                rhs * self
            }
        }
    };
}

int_times_fcon!(f64);
int_times_fcon!(f32);

use crate::scalars::{DNum, ValInt, ValNum};
use crate::{Int, Num, SymWrap};
use core::ops::Sub;

// num/int + num/int

impl<'m, 'd> Sub<Num<'m, 'd>> for Num<'m, 'd> {
    type Output = Num<'m, 'd>;

    fn sub(self, rhs: Num<'m, 'd>) -> Self::Output {
        let data = DNum::sub(self.ptr(), rhs.ptr());
        self.m().val::<ValNum>(data)
    }
}

impl<'m, 'd> Sub<Int<'m, 'd>> for Num<'m, 'd> {
    type Output = Num<'m, 'd>;

    fn sub(self, rhs: Int<'m, 'd>) -> Self::Output {
        let data = DNum::sub(self.ptr(), rhs.ptr());
        self.m().val::<ValNum>(data)
    }
}

impl<'m, 'd> Sub<Num<'m, 'd>> for Int<'m, 'd> {
    type Output = Num<'m, 'd>;

    fn sub(self, rhs: Num<'m, 'd>) -> Self::Output {
        let data = DNum::sub(self.ptr(), rhs.ptr());
        self.m().val::<ValNum>(data)
    }
}

impl<'m, 'd> Sub<Int<'m, 'd>> for Int<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn sub(self, rhs: Int<'m, 'd>) -> Self::Output {
        let data = DNum::sub(self.ptr(), rhs.ptr());
        self.m().val::<ValInt>(data)
    }
}

// num + con

macro_rules! num_minus_con {
    ($c:ty) => {
        impl<'m, 'd> Sub<$c> for Num<'m, 'd> {
            type Output = Num<'m, 'd>;

            fn sub(self, rhs: $c) -> Self::Output {
                let data = DNum::sub(self.ptr(), self.m().val_num_con(-rhs).ptr());
                self.m().val::<ValNum>(data)
            }
        }

        impl<'m, 'd> Sub<Num<'m, 'd>> for $c {
            type Output = Num<'m, 'd>;

            fn sub(self, rhs: Num<'m, 'd>) -> Self::Output {
                let data = DNum::sub(rhs.m().val_num_con(self).ptr(), rhs.neg().ptr());
                rhs.m().val::<ValNum>(data)
            }
        }
    };
}

num_minus_con!(f64);
num_minus_con!(f32);
num_minus_con!(i64);
num_minus_con!(i32);
num_minus_con!(isize);

// int + icon

macro_rules! int_minus_icon {
    ($c:ty) => {
        impl<'m, 'd> Sub<$c> for Int<'m, 'd> {
            type Output = Int<'m, 'd>;

            fn sub(self, rhs: $c) -> Self::Output {
                let data = DNum::sub(self.ptr(), self.m().val_num_con(-rhs).ptr());
                self.m().val::<ValInt>(data)
            }
        }

        impl<'m, 'd> Sub<Int<'m, 'd>> for $c {
            type Output = Int<'m, 'd>;

            fn sub(self, rhs: Int<'m, 'd>) -> Self::Output {
                let data = DNum::sub(rhs.m().val_num_con(self).ptr(), rhs.neg().ptr());
                rhs.m().val::<ValInt>(data)
            }
        }
    };
}

int_minus_icon!(i64);
int_minus_icon!(i32);
int_minus_icon!(isize);

// int + fcon

macro_rules! int_minus_fcon {
    ($c:ty) => {
        impl<'m, 'd> Sub<$c> for Int<'m, 'd> {
            type Output = Num<'m, 'd>;

            fn sub(self, rhs: $c) -> Self::Output {
                let data = DNum::sub(self.ptr(), self.m().val_num_con(-rhs).ptr());
                self.m().val::<ValNum>(data)
            }
        }

        impl<'m, 'd> Sub<Int<'m, 'd>> for $c {
            type Output = Num<'m, 'd>;

            fn sub(self, rhs: Int<'m, 'd>) -> Self::Output {
                let data = DNum::sub(rhs.m().val_num_con(self).ptr(), rhs.neg().ptr());
                rhs.m().val::<ValNum>(data)
            }
        }
    };
}

int_minus_fcon!(f64);
int_minus_fcon!(f32);

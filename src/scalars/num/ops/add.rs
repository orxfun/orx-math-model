use crate::scalars::{DNum, ValInt, ValNum};
use crate::{Int, Num, SymWrap};
use core::ops::Add;

// num/int + num/int

impl<'m, 'd> Add<Num<'m, 'd>> for Num<'m, 'd> {
    type Output = Num<'m, 'd>;

    fn add(self, rhs: Num<'m, 'd>) -> Self::Output {
        let data = DNum::add(self.ptr(), rhs.ptr());
        self.m().val::<ValNum>(data)
    }
}

impl<'m, 'd> Add<Int<'m, 'd>> for Num<'m, 'd> {
    type Output = Num<'m, 'd>;

    fn add(self, rhs: Int<'m, 'd>) -> Self::Output {
        let data = DNum::add(self.ptr(), rhs.ptr());
        self.m().val::<ValNum>(data)
    }
}

impl<'m, 'd> Add<Num<'m, 'd>> for Int<'m, 'd> {
    type Output = Num<'m, 'd>;

    fn add(self, rhs: Num<'m, 'd>) -> Self::Output {
        let data = DNum::add(self.ptr(), rhs.ptr());
        self.m().val::<ValNum>(data)
    }
}

impl<'m, 'd> Add<Int<'m, 'd>> for Int<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn add(self, rhs: Int<'m, 'd>) -> Self::Output {
        let data = DNum::add(self.ptr(), rhs.ptr());
        self.m().val::<ValInt>(data)
    }
}

// num + con

macro_rules! num_plus_con {
    ($c:ty) => {
        impl<'m, 'd> Add<$c> for Num<'m, 'd> {
            type Output = Num<'m, 'd>;

            fn add(self, rhs: $c) -> Self::Output {
                let data = DNum::add(self.ptr(), self.m().val_num_con(rhs).ptr());
                self.m().val::<ValNum>(data)
            }
        }

        impl<'m, 'd> Add<Num<'m, 'd>> for $c {
            type Output = Num<'m, 'd>;

            fn add(self, rhs: Num<'m, 'd>) -> Self::Output {
                rhs + self
            }
        }
    };
}

num_plus_con!(f64);
num_plus_con!(f32);
num_plus_con!(i64);
num_plus_con!(i32);
num_plus_con!(u64);
num_plus_con!(u32);
num_plus_con!(usize);
num_plus_con!(isize);

// int + icon

macro_rules! int_plus_icon {
    ($c:ty) => {
        impl<'m, 'd> Add<$c> for Int<'m, 'd> {
            type Output = Int<'m, 'd>;

            fn add(self, rhs: $c) -> Self::Output {
                let data = DNum::add(self.ptr(), self.m().val_num_con(rhs).ptr());
                self.m().val::<ValInt>(data)
            }
        }

        impl<'m, 'd> Add<Int<'m, 'd>> for $c {
            type Output = Int<'m, 'd>;

            fn add(self, rhs: Int<'m, 'd>) -> Self::Output {
                rhs + self
            }
        }
    };
}

int_plus_icon!(i64);
int_plus_icon!(i32);
int_plus_icon!(u64);
int_plus_icon!(u32);
int_plus_icon!(usize);
int_plus_icon!(isize);

// int + fcon

macro_rules! int_plus_fcon {
    ($c:ty) => {
        impl<'m, 'd> Add<$c> for Int<'m, 'd> {
            type Output = Num<'m, 'd>;

            fn add(self, rhs: $c) -> Self::Output {
                let data = DNum::add(self.ptr(), self.m().val_num_con(rhs).ptr());
                self.m().val::<ValNum>(data)
            }
        }

        impl<'m, 'd> Add<Int<'m, 'd>> for $c {
            type Output = Num<'m, 'd>;

            fn add(self, rhs: Int<'m, 'd>) -> Self::Output {
                rhs + self
            }
        }
    };
}

int_plus_fcon!(f64);
int_plus_fcon!(f32);

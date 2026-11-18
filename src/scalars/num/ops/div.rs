use crate::scalars::{DNum, ValInt, ValNum};
use crate::{Int, Num, SymWrap};
use core::ops::Div;

// num/int / num/int

impl<'m, 'd> Div<Num<'m, 'd>> for Num<'m, 'd> {
    type Output = Num<'m, 'd>;

    fn div(self, rhs: Num<'m, 'd>) -> Self::Output {
        let data = DNum::div(self.ptr(), rhs.ptr());
        self.m().val::<ValNum>(data)
    }
}

impl<'m, 'd> Div<Int<'m, 'd>> for Num<'m, 'd> {
    type Output = Num<'m, 'd>;

    fn div(self, rhs: Int<'m, 'd>) -> Self::Output {
        let data = DNum::div(self.ptr(), rhs.ptr());
        self.m().val::<ValNum>(data)
    }
}

impl<'m, 'd> Div<Num<'m, 'd>> for Int<'m, 'd> {
    type Output = Num<'m, 'd>;

    fn div(self, rhs: Num<'m, 'd>) -> Self::Output {
        let data = DNum::div(self.ptr(), rhs.ptr());
        self.m().val::<ValNum>(data)
    }
}

impl<'m, 'd> Div<Int<'m, 'd>> for Int<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn div(self, rhs: Int<'m, 'd>) -> Self::Output {
        let data = DNum::div(self.ptr(), rhs.ptr());
        self.m().val::<ValInt>(data)
    }
}

// num/int / con

macro_rules! num_div_con {
    ($n:ty, $c:ty) => {
        impl<'m, 'd> Div<$c> for $n {
            type Output = Num<'m, 'd>;

            fn div(self, rhs: $c) -> Self::Output {
                let data = DNum::div(self.ptr(), self.m().val_num_con(rhs).ptr());
                self.m().val::<ValNum>(data)
            }
        }

        impl<'m, 'd> Div<$n> for $c {
            type Output = Num<'m, 'd>;

            fn div(self, rhs: $n) -> Self::Output {
                let data = DNum::div(rhs.m().val_num_con(self).ptr(), rhs.recip().ptr());
                rhs.m().val::<ValNum>(data)
            }
        }
    };
}

num_div_con!(Int<'m, 'd>, f64);
num_div_con!(Int<'m, 'd>, f32);
num_div_con!(Int<'m, 'd>, i64);
num_div_con!(Int<'m, 'd>, i32);
num_div_con!(Int<'m, 'd>, u64);
num_div_con!(Int<'m, 'd>, u32);
num_div_con!(Int<'m, 'd>, usize);
num_div_con!(Int<'m, 'd>, isize);

num_div_con!(Num<'m, 'd>, f64);
num_div_con!(Num<'m, 'd>, f32);
num_div_con!(Num<'m, 'd>, i64);
num_div_con!(Num<'m, 'd>, i32);
num_div_con!(Num<'m, 'd>, u64);
num_div_con!(Num<'m, 'd>, u32);
num_div_con!(Num<'m, 'd>, usize);
num_div_con!(Num<'m, 'd>, isize);

use super::add::val_num;
use crate::scalars::{DNum, ValInt, ValNum};
use crate::{Int, Num, SymWrap};
use alloc::boxed::Box;
use core::ops::Div;

// num/int / num/int

macro_rules! num_num {
    ($n1:ty, $n2:ty, $out:ty, $fout:ident) => {
        impl<'m, 'd> Div<$n2> for $n1 {
            type Output = $out;

            fn div(self, rhs: $n2) -> Self::Output {
                let [t1, t2] = [self.ptr(), rhs.ptr()];
                let fun = Box::new(move || t1.get().eval() / t2.get().eval());
                $fout(self.m(), fun)
            }
        }
    };
}

num_num!(Num<'m, 'd>, Num<'m, 'd>, Num<'m, 'd>, val_num);
num_num!(Num<'m, 'd>, Int<'m, 'd>, Num<'m, 'd>, val_num);
num_num!(Int<'m, 'd>, Num<'m, 'd>, Num<'m, 'd>, val_num);
num_num!(Int<'m, 'd>, Int<'m, 'd>, Num<'m, 'd>, val_num);

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

use super::add::{val_int, val_num};
use crate::scalars::{AsF64, DNum, ValInt, ValNum};
use crate::{Int, Num, SymWrap};
use alloc::boxed::Box;
use core::ops::Mul;

// num/int * num/int

macro_rules! num_num {
    ($n1:ty, $n2:ty, $out:ty, $fout:ident) => {
        impl<'m, 'd> Mul<$n2> for $n1 {
            type Output = $out;

            fn mul(self, rhs: $n2) -> Self::Output {
                let [t1, t2] = [self.ptr(), rhs.ptr()];
                let fun = Box::new(move || t1.get().eval() * t2.get().eval());
                $fout(self.m(), fun)
            }
        }
    };
}

num_num!(Num<'m, 'd>, Num<'m, 'd>, Num<'m, 'd>, val_num);
num_num!(Num<'m, 'd>, Int<'m, 'd>, Num<'m, 'd>, val_num);
num_num!(Int<'m, 'd>, Num<'m, 'd>, Num<'m, 'd>, val_num);
num_num!(Int<'m, 'd>, Int<'m, 'd>, Int<'m, 'd>, val_int);

// num * con

macro_rules! num_con {
    ($n1:ty, $n2:ty, $out:ty, $fout:ident) => {
        impl<'m, 'd> Mul<$n2> for $n1 {
            type Output = $out;

            fn mul(self, rhs: $n2) -> Self::Output {
                let t1 = self.ptr();
                let fun = Box::new(move || t1.get().eval() + rhs.as_f64());
                $fout(self.m(), fun)
            }
        }

        impl<'m, 'd> Mul<$n1> for $n2 {
            type Output = $out;

            fn mul(self, rhs: $n1) -> Self::Output {
                rhs + self
            }
        }
    };
}

num_con!(Num<'m, 'd>, f64, Num<'m, 'd>, val_num);
num_con!(Num<'m, 'd>, f32, Num<'m, 'd>, val_num);
num_con!(Num<'m, 'd>, i64, Num<'m, 'd>, val_num);
num_con!(Num<'m, 'd>, i32, Num<'m, 'd>, val_num);
num_con!(Num<'m, 'd>, u64, Num<'m, 'd>, val_num);
num_con!(Num<'m, 'd>, u32, Num<'m, 'd>, val_num);
num_con!(Num<'m, 'd>, usize, Num<'m, 'd>, val_num);
num_con!(Num<'m, 'd>, isize, Num<'m, 'd>, val_num);

num_con!(Int<'m, 'd>, f64, Num<'m, 'd>, val_num);
num_con!(Int<'m, 'd>, f32, Num<'m, 'd>, val_num);
num_con!(Int<'m, 'd>, i64, Int<'m, 'd>, val_int);
num_con!(Int<'m, 'd>, i32, Int<'m, 'd>, val_int);
num_con!(Int<'m, 'd>, u64, Int<'m, 'd>, val_int);
num_con!(Int<'m, 'd>, u32, Int<'m, 'd>, val_int);
num_con!(Int<'m, 'd>, usize, Int<'m, 'd>, val_int);
num_con!(Int<'m, 'd>, isize, Int<'m, 'd>, val_int);

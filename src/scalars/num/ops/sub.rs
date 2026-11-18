use super::add::{val_int, val_num};
use crate::scalars::{DNum, FNum, ValInt, ValNum};
use crate::{AsF64, Int, Model, Num, SymWrap, ValTyp};
use alloc::boxed::Box;
use core::ops::Sub;

// num/int - num/int

macro_rules! num_num {
    ($n1:ty, $n2:ty, $out:ty, $fout:ident) => {
        impl<'m, 'd> Sub<$n2> for $n1 {
            type Output = $out;

            fn sub(self, rhs: $n2) -> Self::Output {
                let [t1, t2] = [self.ptr(), rhs.ptr()];
                let fun = Box::new(move || t1.get().eval() - t2.get().eval());
                $fout(self.m(), fun)
            }
        }
    };
}

num_num!(Num<'m, 'd>, Num<'m, 'd>, Num<'m, 'd>, val_num);
num_num!(Num<'m, 'd>, Int<'m, 'd>, Num<'m, 'd>, val_num);
num_num!(Int<'m, 'd>, Num<'m, 'd>, Num<'m, 'd>, val_num);
num_num!(Int<'m, 'd>, Int<'m, 'd>, Int<'m, 'd>, val_int);

// num - con

macro_rules! num_minus_con {
    ($c:ty) => {
        impl<'m, 'd> Sub<$c> for Num<'m, 'd> {
            type Output = Num<'m, 'd>;

            fn sub(self, rhs: $c) -> Self::Output {
                let t1 = self.ptr();
                let fun = Box::new(move || t1.get().eval() - rhs.as_f64());
                val_num(self.m(), fun)
            }
        }

        impl<'m, 'd> Sub<Num<'m, 'd>> for $c {
            type Output = Num<'m, 'd>;

            fn sub(self, rhs: Num<'m, 'd>) -> Self::Output {
                let t2 = rhs.ptr();
                let fun = Box::new(move || self.as_f64() - t2.get().eval());
                val_num(rhs.m(), fun)
            }
        }
    };
}

num_minus_con!(f64);
num_minus_con!(f32);
num_minus_con!(i64);
num_minus_con!(i32);
num_minus_con!(isize);

// int - icon

macro_rules! int_minus_icon {
    ($c:ty) => {
        impl<'m, 'd> Sub<$c> for Int<'m, 'd> {
            type Output = Int<'m, 'd>;

            fn sub(self, rhs: $c) -> Self::Output {
                let t1 = self.ptr();
                let fun = Box::new(move || t1.get().eval() - rhs.as_f64());
                val_int(self.m(), fun)
            }
        }

        impl<'m, 'd> Sub<Int<'m, 'd>> for $c {
            type Output = Int<'m, 'd>;

            fn sub(self, rhs: Int<'m, 'd>) -> Self::Output {
                let t2 = rhs.ptr();
                let fun = Box::new(move || self.as_f64() - t2.get().eval());
                val_int(rhs.m(), fun)
            }
        }
    };
}

int_minus_icon!(i64);
int_minus_icon!(i32);
int_minus_icon!(isize);

// int - fcon

macro_rules! int_minus_fcon {
    ($c:ty) => {
        impl<'m, 'd> Sub<$c> for Int<'m, 'd> {
            type Output = Num<'m, 'd>;

            fn sub(self, rhs: $c) -> Self::Output {
                let t1 = self.ptr();
                let fun = Box::new(move || t1.get().eval() - rhs.as_f64());
                val_num(self.m(), fun)
            }
        }

        impl<'m, 'd> Sub<Int<'m, 'd>> for $c {
            type Output = Num<'m, 'd>;

            fn sub(self, rhs: Int<'m, 'd>) -> Self::Output {
                let t2 = rhs.ptr();
                let fun = Box::new(move || self.as_f64() - t2.get().eval());
                val_num(rhs.m(), fun)
            }
        }
    };
}

int_minus_fcon!(f64);
int_minus_fcon!(f32);

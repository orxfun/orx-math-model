use super::add::{val_int, val_num};
use crate::idxs::IdxOrd;
use crate::pars::{IntParD0, NumParD0};
use crate::{AsF64, Int, Num, SetOrd, SymWrap};
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

// num/int - idx

macro_rules! num_idx {
    ($n1:ty, $fout:ident) => {
        impl<'m, 'd> Sub<IdxOrd<'m, 'd>> for $n1 {
            type Output = $n1;

            fn sub(self, rhs: IdxOrd<'m, 'd>) -> Self::Output {
                let (t1, t2) = (self.ptr(), rhs.ptr());
                let fun =
                    Box::new(move || t1.get().eval().as_f64() - t2.get().read::<SetOrd>().as_f64());
                $fout(self.m(), fun)
            }
        }

        impl<'m, 'd> Sub<$n1> for IdxOrd<'m, 'd> {
            type Output = $n1;

            fn sub(self, rhs: $n1) -> Self::Output {
                let (t1, t2) = (self.ptr(), rhs.ptr());
                let fun =
                    Box::new(move || t1.get().read::<SetOrd>().as_f64() - t2.get().eval().as_f64());
                $fout(self.m(), fun)
            }
        }
    };
}

num_idx!(Num<'m, 'd>, val_num);
num_idx!(Int<'m, 'd>, val_int);

// num/int - par

macro_rules! num_par {
    ($n1:ty, $par:ty, $out:ty, $fout:ident) => {
        impl<'m, 'd> Sub<$par> for $n1 {
            type Output = $out;

            fn sub(self, rhs: $par) -> Self::Output {
                let (t1, t2) = (self.ptr(), rhs.ptr().get().fun_ptr());
                let fun = Box::new(move || t1.get().eval() - t2.get().d0_at());
                $fout(self.m(), fun)
            }
        }

        impl<'m, 'd> Sub<$n1> for $par {
            type Output = $out;

            fn sub(self, rhs: $n1) -> Self::Output {
                let (t1, t2) = (self.ptr().get().fun_ptr(), rhs.ptr());
                let fun = Box::new(move || t1.get().d0_at() - t2.get().eval());
                $fout(self.m(), fun)
            }
        }
    };
}

num_par!(Num<'m, 'd>, NumParD0<'m, 'd>, Num<'m, 'd>, val_num);
num_par!(Num<'m, 'd>, IntParD0<'m, 'd>, Num<'m, 'd>, val_num);
num_par!(Int<'m, 'd>, NumParD0<'m, 'd>, Num<'m, 'd>, val_num);
num_par!(Int<'m, 'd>, IntParD0<'m, 'd>, Int<'m, 'd>, val_int);

// num - con

macro_rules! num_con {
    ($n1:ty, $n2:ty, $out:ty, $fout:ident) => {
        impl<'m, 'd> Sub<$n2> for $n1 {
            type Output = $out;

            fn sub(self, rhs: $n2) -> Self::Output {
                let t1 = self.ptr();
                let fun = Box::new(move || t1.get().eval() - rhs.as_f64());
                $fout(self.m(), fun)
            }
        }

        impl<'m, 'd> Sub<$n1> for $n2 {
            type Output = $out;

            fn sub(self, rhs: $n1) -> Self::Output {
                let t2 = rhs.ptr();
                let fun = Box::new(move || self.as_f64() - t2.get().eval());
                $fout(rhs.m(), fun)
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

// idx - idx

impl<'m, 'd> Sub for IdxOrd<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn sub(self, rhs: Self) -> Self::Output {
        let [t1, t2] = [self.ptr(), rhs.ptr()];
        let fun = Box::new(move || (t1.get().read::<SetOrd>() - t2.get().read::<SetOrd>()) as f64);
        val_int(self.m(), fun)
    }
}

// idx - par

macro_rules! idx_par {
    ($par:ty, $out:ty, $fout:ident) => {
        impl<'m, 'd> Sub<$par> for IdxOrd<'m, 'd> {
            type Output = $out;

            fn sub(self, rhs: $par) -> Self::Output {
                let (t1, t2) = (self.ptr(), rhs.ptr().get().fun_ptr());
                let fun =
                    Box::new(move || (t1.get().read::<SetOrd>() - t2.get().d0_at_int()) as f64);
                $fout(self.m(), fun)
            }
        }

        impl<'m, 'd> Sub<IdxOrd<'m, 'd>> for $par {
            type Output = $out;

            fn sub(self, rhs: IdxOrd<'m, 'd>) -> Self::Output {
                let (t1, t2) = (self.ptr().get().fun_ptr(), rhs.ptr());
                let fun =
                    Box::new(move || (t1.get().d0_at_int() - t2.get().read::<SetOrd>()) as f64);
                $fout(self.m(), fun)
            }
        }
    };
}

idx_par!(NumParD0<'m, 'd>, Num<'m, 'd>, val_num);
idx_par!(IntParD0<'m, 'd>, Int<'m, 'd>, val_int);

// idx - con

macro_rules! idx_con {
    ($con:ty, $out:ty, $fout:ident) => {
        impl<'m, 'd> Sub<$con> for IdxOrd<'m, 'd> {
            type Output = $out;

            fn sub(self, rhs: $con) -> Self::Output {
                let t1 = self.ptr();
                let fun = Box::new(move || t1.get().read::<SetOrd>().as_f64() - rhs.as_f64());
                $fout(self.m(), fun)
            }
        }

        impl<'m, 'd> Sub<IdxOrd<'m, 'd>> for $con {
            type Output = $out;

            fn sub(self, rhs: IdxOrd<'m, 'd>) -> Self::Output {
                let t2 = rhs.ptr();
                let fun = Box::new(move || self.as_f64() - t2.get().read::<SetOrd>().as_f64());
                $fout(rhs.m(), fun)
            }
        }
    };
}

idx_con!(f64, Num<'m, 'd>, val_num);
idx_con!(f32, Num<'m, 'd>, val_num);
idx_con!(i64, Int<'m, 'd>, val_int);
idx_con!(i32, Int<'m, 'd>, val_int);
idx_con!(u64, Int<'m, 'd>, val_int);
idx_con!(u32, Int<'m, 'd>, val_int);
idx_con!(usize, Int<'m, 'd>, val_int);
idx_con!(isize, Int<'m, 'd>, val_int);

// par - con

macro_rules! par_con {
    ($par:ty, $con:ty, $out:ty, $fout:ident) => {
        impl<'m, 'd> Sub<$con> for $par {
            type Output = $out;

            fn sub(self, rhs: $con) -> Self::Output {
                let t1 = self.ptr().get().fun_ptr();
                let fun = Box::new(move || t1.get().d0_at() - rhs.as_f64());
                $fout(self.m(), fun)
            }
        }

        impl<'m, 'd> Sub<$par> for $con {
            type Output = $out;

            fn sub(self, rhs: $par) -> Self::Output {
                let t2 = rhs.ptr().get().fun_ptr();
                let fun = Box::new(move || self.as_f64() - t2.get().d0_at());
                $fout(rhs.m(), fun)
            }
        }
    };
}

par_con!(IntParD0<'m, 'd>, f64, Num<'m, 'd>, val_num);
par_con!(IntParD0<'m, 'd>, f32, Num<'m, 'd>, val_num);
par_con!(IntParD0<'m, 'd>, i64, Int<'m, 'd>, val_int);
par_con!(IntParD0<'m, 'd>, i32, Int<'m, 'd>, val_int);
par_con!(IntParD0<'m, 'd>, u64, Int<'m, 'd>, val_int);
par_con!(IntParD0<'m, 'd>, u32, Int<'m, 'd>, val_int);
par_con!(IntParD0<'m, 'd>, usize, Int<'m, 'd>, val_int);
par_con!(IntParD0<'m, 'd>, isize, Int<'m, 'd>, val_int);

par_con!(NumParD0<'m, 'd>, f64, Num<'m, 'd>, val_num);
par_con!(NumParD0<'m, 'd>, f32, Num<'m, 'd>, val_num);
par_con!(NumParD0<'m, 'd>, i64, Num<'m, 'd>, val_num);
par_con!(NumParD0<'m, 'd>, i32, Num<'m, 'd>, val_num);
par_con!(NumParD0<'m, 'd>, u64, Num<'m, 'd>, val_num);
par_con!(NumParD0<'m, 'd>, u32, Num<'m, 'd>, val_num);
par_con!(NumParD0<'m, 'd>, usize, Num<'m, 'd>, val_num);
par_con!(NumParD0<'m, 'd>, isize, Num<'m, 'd>, val_num);

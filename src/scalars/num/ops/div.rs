use super::add::val_num;
use crate::idxs::IdxOrd;
use crate::pars::{IntParD0, NumParD0};
use crate::scalars::AsF64;
use crate::{Int, Num, SetOrd, SymWrap};
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

// num/int / idx

macro_rules! num_idx {
    ($n1:ty, $fout:ident) => {
        impl<'m, 'd> Div<IdxOrd<'m, 'd>> for $n1 {
            type Output = Num<'m, 'd>;

            fn div(self, rhs: IdxOrd<'m, 'd>) -> Self::Output {
                let (t1, t2) = (self.ptr(), rhs.ptr());
                let fun =
                    Box::new(move || t1.get().eval().as_f64() / t2.get().read::<SetOrd>().as_f64());
                $fout(self.m(), fun)
            }
        }

        impl<'m, 'd> Div<$n1> for IdxOrd<'m, 'd> {
            type Output = Num<'m, 'd>;

            fn div(self, rhs: $n1) -> Self::Output {
                let (t1, t2) = (self.ptr(), rhs.ptr());
                let fun =
                    Box::new(move || t1.get().read::<SetOrd>().as_f64() / t2.get().eval().as_f64());
                $fout(self.m(), fun)
            }
        }
    };
}

num_idx!(Num<'m, 'd>, val_num);
num_idx!(Int<'m, 'd>, val_num);

// num/int / par

macro_rules! num_par {
    ($n1:ty, $par:ty, $out:ty, $fout:ident) => {
        impl<'m, 'd> Div<$par> for $n1 {
            type Output = $out;

            fn div(self, rhs: $par) -> Self::Output {
                let (t1, t2) = (self.ptr(), rhs.ptr().get().fun_ptr());
                let fun = Box::new(move || t1.get().eval() / t2.get().d0_at());
                $fout(self.m(), fun)
            }
        }

        impl<'m, 'd> Div<$n1> for $par {
            type Output = $out;

            fn div(self, rhs: $n1) -> Self::Output {
                let (t1, t2) = (self.ptr().get().fun_ptr(), rhs.ptr());
                let fun = Box::new(move || t1.get().d0_at() / t2.get().eval());
                $fout(self.m(), fun)
            }
        }
    };
}

num_par!(Num<'m, 'd>, NumParD0<'m, 'd>, Num<'m, 'd>, val_num);
num_par!(Num<'m, 'd>, IntParD0<'m, 'd>, Num<'m, 'd>, val_num);
num_par!(Int<'m, 'd>, NumParD0<'m, 'd>, Num<'m, 'd>, val_num);
num_par!(Int<'m, 'd>, IntParD0<'m, 'd>, Num<'m, 'd>, val_num);

// num/int / con

macro_rules! num_con {
    ($n1:ty, $n2:ty, $out:ty, $fout:ident) => {
        impl<'m, 'd> Div<$n2> for $n1 {
            type Output = $out;

            fn div(self, rhs: $n2) -> Self::Output {
                let t1 = self.ptr();
                let fun = Box::new(move || t1.get().eval() / rhs.as_f64());
                $fout(self.m(), fun)
            }
        }

        impl<'m, 'd> Div<$n1> for $n2 {
            type Output = $out;

            fn div(self, rhs: $n1) -> Self::Output {
                let t2 = rhs.ptr();
                let fun = Box::new(move || self.as_f64() / t2.get().eval());
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
num_con!(Int<'m, 'd>, i64, Num<'m, 'd>, val_num);
num_con!(Int<'m, 'd>, i32, Num<'m, 'd>, val_num);
num_con!(Int<'m, 'd>, u64, Num<'m, 'd>, val_num);
num_con!(Int<'m, 'd>, u32, Num<'m, 'd>, val_num);
num_con!(Int<'m, 'd>, usize, Num<'m, 'd>, val_num);
num_con!(Int<'m, 'd>, isize, Num<'m, 'd>, val_num);

// idx/idx

impl<'m, 'd> Div for IdxOrd<'m, 'd> {
    type Output = Num<'m, 'd>;

    fn div(self, rhs: Self) -> Self::Output {
        let [t1, t2] = [self.ptr(), rhs.ptr()];
        let fun = Box::new(move || (t1.get().read::<SetOrd>() / t2.get().read::<SetOrd>()) as f64);
        val_num(self.m(), fun)
    }
}

// idx / par

macro_rules! idx_par {
    ($par:ty, $out:ty, $fout:ident) => {
        impl<'m, 'd> Div<$par> for IdxOrd<'m, 'd> {
            type Output = $out;

            fn div(self, rhs: $par) -> Self::Output {
                let (t1, t2) = (self.ptr(), rhs.ptr().get().fun_ptr());
                let fun =
                    Box::new(move || (t1.get().read::<SetOrd>() / t2.get().d0_at_int()) as f64);
                $fout(self.m(), fun)
            }
        }

        impl<'m, 'd> Div<IdxOrd<'m, 'd>> for $par {
            type Output = $out;

            fn div(self, rhs: IdxOrd<'m, 'd>) -> Self::Output {
                let (t1, t2) = (self.ptr().get().fun_ptr(), rhs.ptr());
                let fun =
                    Box::new(move || (t1.get().d0_at_int() / t2.get().read::<SetOrd>()) as f64);
                $fout(self.m(), fun)
            }
        }
    };
}

idx_par!(NumParD0<'m, 'd>, Num<'m, 'd>, val_num);
idx_par!(IntParD0<'m, 'd>, Num<'m, 'd>, val_num);

// idx/con

macro_rules! idx_con {
    ($con:ty, $out:ty, $fout:ident) => {
        impl<'m, 'd> Div<$con> for IdxOrd<'m, 'd> {
            type Output = Num<'m, 'd>;

            fn div(self, rhs: $con) -> Self::Output {
                let t1 = self.ptr();
                let fun = Box::new(move || t1.get().read::<SetOrd>().as_f64() / rhs.as_f64());
                $fout(self.m(), fun)
            }
        }

        impl<'m, 'd> Div<IdxOrd<'m, 'd>> for $con {
            type Output = Num<'m, 'd>;

            fn div(self, rhs: IdxOrd<'m, 'd>) -> Self::Output {
                let t2 = rhs.ptr();
                let fun = Box::new(move || self.as_f64() / t2.get().read::<SetOrd>().as_f64());
                $fout(rhs.m(), fun)
            }
        }
    };
}

idx_con!(f64, Num<'m, 'd>, val_num);
idx_con!(f32, Num<'m, 'd>, val_num);
idx_con!(i64, Num<'m, 'd>, val_num);
idx_con!(i32, Num<'m, 'd>, val_num);
idx_con!(u64, Num<'m, 'd>, val_num);
idx_con!(u32, Num<'m, 'd>, val_num);
idx_con!(usize, Num<'m, 'd>, val_num);
idx_con!(isize, Num<'m, 'd>, val_num);

// par / con

macro_rules! par_con {
    ($par:ty, $con:ty, $out:ty, $fout:ident) => {
        impl<'m, 'd> Div<$con> for $par {
            type Output = $out;

            fn div(self, rhs: $con) -> Self::Output {
                let t1 = self.ptr().get().fun_ptr();
                let fun = Box::new(move || t1.get().d0_at() / rhs.as_f64());
                $fout(self.m(), fun)
            }
        }

        impl<'m, 'd> Div<$par> for $con {
            type Output = $out;

            fn div(self, rhs: $par) -> Self::Output {
                let t2 = rhs.ptr().get().fun_ptr();
                let fun = Box::new(move || self.as_f64() / t2.get().d0_at());
                $fout(rhs.m(), fun)
            }
        }
    };
}

par_con!(IntParD0<'m, 'd>, f64, Num<'m, 'd>, val_num);
par_con!(IntParD0<'m, 'd>, f32, Num<'m, 'd>, val_num);
par_con!(IntParD0<'m, 'd>, i64, Num<'m, 'd>, val_num);
par_con!(IntParD0<'m, 'd>, i32, Num<'m, 'd>, val_num);
par_con!(IntParD0<'m, 'd>, u64, Num<'m, 'd>, val_num);
par_con!(IntParD0<'m, 'd>, u32, Num<'m, 'd>, val_num);
par_con!(IntParD0<'m, 'd>, usize, Num<'m, 'd>, val_num);
par_con!(IntParD0<'m, 'd>, isize, Num<'m, 'd>, val_num);

par_con!(NumParD0<'m, 'd>, f64, Num<'m, 'd>, val_num);
par_con!(NumParD0<'m, 'd>, f32, Num<'m, 'd>, val_num);
par_con!(NumParD0<'m, 'd>, i64, Num<'m, 'd>, val_num);
par_con!(NumParD0<'m, 'd>, i32, Num<'m, 'd>, val_num);
par_con!(NumParD0<'m, 'd>, u64, Num<'m, 'd>, val_num);
par_con!(NumParD0<'m, 'd>, u32, Num<'m, 'd>, val_num);
par_con!(NumParD0<'m, 'd>, usize, Num<'m, 'd>, val_num);
par_con!(NumParD0<'m, 'd>, isize, Num<'m, 'd>, val_num);

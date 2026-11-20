use crate::idxs::IdxOrd;
use crate::pars::{IntParD0, NumParD0};
use crate::scalars::{DNum, FNum, ValInt, ValNum};
use crate::{AsF64, Int, Model, Num, SetOrd, SymWrap, ValTyp};
use alloc::boxed::Box;
use core::ops::Add;

pub fn val_num<'m, 'd>(m: &'m Model<'d>, fun: FNum<'d>) -> Num<'m, 'd> {
    m.val::<ValNum>(DNum::new(ValTyp::Num, fun))
}

pub fn val_int<'m, 'd>(m: &'m Model<'d>, fun: FNum<'d>) -> Int<'m, 'd> {
    m.val::<ValInt>(DNum::new(ValTyp::Int, fun))
}

// num/int + num/int

macro_rules! num_num {
    ($n1:ty, $n2:ty, $out:ty, $fout:ident) => {
        impl<'m, 'd> Add<$n2> for $n1 {
            type Output = $out;

            fn add(self, rhs: $n2) -> Self::Output {
                let [t1, t2] = [self.ptr(), rhs.ptr()];
                let fun = Box::new(move || t1.get().eval() + t2.get().eval());
                $fout(self.m(), fun)
            }
        }
    };
}

num_num!(Num<'m, 'd>, Num<'m, 'd>, Num<'m, 'd>, val_num);
num_num!(Num<'m, 'd>, Int<'m, 'd>, Num<'m, 'd>, val_num);
num_num!(Int<'m, 'd>, Num<'m, 'd>, Num<'m, 'd>, val_num);
num_num!(Int<'m, 'd>, Int<'m, 'd>, Int<'m, 'd>, val_int);

// num/int + idx

macro_rules! num_idx {
    ($n1:ty, $fout:ident) => {
        impl<'m, 'd> Add<IdxOrd<'m, 'd>> for $n1 {
            type Output = $n1;

            fn add(self, rhs: IdxOrd<'m, 'd>) -> Self::Output {
                let (t1, t2) = (self.ptr(), rhs.ptr());
                let fun =
                    Box::new(move || t1.get().eval().as_f64() + t2.get().read::<SetOrd>().as_f64());
                $fout(self.m(), fun)
            }
        }

        impl<'m, 'd> Add<$n1> for IdxOrd<'m, 'd> {
            type Output = $n1;

            fn add(self, rhs: $n1) -> Self::Output {
                rhs + self
            }
        }
    };
}

num_idx!(Num<'m, 'd>, val_num);
num_idx!(Int<'m, 'd>, val_int);

// num/int + par

macro_rules! num_par {
    ($n1:ty, $par:ty, $out:ty, $fout:ident) => {
        impl<'m, 'd> Add<$par> for $n1 {
            type Output = $out;

            fn add(self, rhs: $par) -> Self::Output {
                let (t1, t2) = (self.ptr(), rhs.ptr().get().fun_ptr());
                let fun = Box::new(move || t1.get().eval() + t2.get().d0_at());
                $fout(self.m(), fun)
            }
        }

        impl<'m, 'd> Add<$n1> for $par {
            type Output = $out;

            fn add(self, rhs: $n1) -> Self::Output {
                rhs + self
            }
        }
    };
}

num_par!(Num<'m, 'd>, NumParD0<'m, 'd>, Num<'m, 'd>, val_num);
num_par!(Num<'m, 'd>, IntParD0<'m, 'd>, Num<'m, 'd>, val_num);
num_par!(Int<'m, 'd>, NumParD0<'m, 'd>, Num<'m, 'd>, val_num);
num_par!(Int<'m, 'd>, IntParD0<'m, 'd>, Int<'m, 'd>, val_int);

// num + con

macro_rules! num_con {
    ($n1:ty, $n2:ty, $out:ty, $fout:ident) => {
        impl<'m, 'd> Add<$n2> for $n1 {
            type Output = $out;

            fn add(self, rhs: $n2) -> Self::Output {
                let t1 = self.ptr();
                let fun = Box::new(move || t1.get().eval() + rhs.as_f64());
                $fout(self.m(), fun)
            }
        }

        impl<'m, 'd> Add<$n1> for $n2 {
            type Output = $out;

            fn add(self, rhs: $n1) -> Self::Output {
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

// idx + idx

impl<'m, 'd> Add for IdxOrd<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn add(self, rhs: Self) -> Self::Output {
        let [t1, t2] = [self.ptr(), rhs.ptr()];
        let fun = Box::new(move || (t1.get().read::<SetOrd>() + t2.get().read::<SetOrd>()) as f64);
        val_int(self.m(), fun)
    }
}

// idx + par

macro_rules! idx_par {
    ($par:ty, $out:ty, $fout:ident) => {
        impl<'m, 'd> Add<$par> for IdxOrd<'m, 'd> {
            type Output = $out;

            fn add(self, rhs: $par) -> Self::Output {
                let (t1, t2) = (self.ptr(), rhs.ptr().get().fun_ptr());
                let fun =
                    Box::new(move || (t1.get().read::<SetOrd>() + t2.get().d0_at_int()) as f64);
                $fout(self.m(), fun)
            }
        }

        impl<'m, 'd> Add<IdxOrd<'m, 'd>> for $par {
            type Output = $out;

            fn add(self, rhs: IdxOrd<'m, 'd>) -> Self::Output {
                rhs + self
            }
        }
    };
}

idx_par!(NumParD0<'m, 'd>, Num<'m, 'd>, val_num);
idx_par!(IntParD0<'m, 'd>, Int<'m, 'd>, val_int);

// idx + con

macro_rules! idx_con {
    ($con:ty, $out:ty, $fout:ident) => {
        impl<'m, 'd> Add<$con> for IdxOrd<'m, 'd> {
            type Output = $out;

            fn add(self, rhs: $con) -> Self::Output {
                let t1 = self.ptr();
                let fun = Box::new(move || t1.get().read::<SetOrd>().as_f64() + rhs.as_f64());
                $fout(self.m(), fun)
            }
        }

        impl<'m, 'd> Add<IdxOrd<'m, 'd>> for $con {
            type Output = $out;

            fn add(self, rhs: IdxOrd<'m, 'd>) -> Self::Output {
                rhs + self
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

// par + con

macro_rules! par_con {
    ($par:ty, $con:ty, $out:ty, $fout:ident) => {
        impl<'m, 'd> Add<$con> for $par {
            type Output = $out;

            fn add(self, rhs: $con) -> Self::Output {
                let t1 = self.ptr().get().fun_ptr();
                let fun = Box::new(move || t1.get().d0_at() + rhs.as_f64());
                $fout(self.m(), fun)
            }
        }

        impl<'m, 'd> Add<$par> for $con {
            type Output = $out;

            fn add(self, rhs: $par) -> Self::Output {
                rhs + self
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

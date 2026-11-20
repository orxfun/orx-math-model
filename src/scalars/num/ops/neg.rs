use crate::idxs::IdxOrd;
use crate::pars::{IntParD0, NumParD0};
use crate::scalars::num::ops::add::{val_int, val_num};
use crate::{Int, Num, SetOrd, SymWrap};
use alloc::boxed::Box;
use core::ops::Neg;

impl<'m, 'd> Neg for IdxOrd<'m, 'd> {
    type Output = Int<'m, 'd>;

    fn neg(self) -> Self::Output {
        let t1 = self.ptr();
        let fun = Box::new(move || -t1.get().read::<SetOrd>() as f64);
        val_int(self.m(), fun)
    }
}

macro_rules! par0 {
    ($p:ty, $n:ty, $fout:ident) => {
        impl<'m, 'd> Neg for $p {
            type Output = $n;

            fn neg(self) -> Self::Output {
                let t1 = self.ptr().get().fun_ptr();
                let fun = Box::new(move || -t1.get().d0_at());
                $fout(self.m(), fun)
            }
        }
    };
}

par0!(NumParD0<'m, 'd>, Num<'m, 'd>, val_num);
par0!(IntParD0<'m, 'd>, Int<'m, 'd>, val_int);

macro_rules! num {
    ($n:ty, $fout:ident) => {
        impl<'m, 'd> Neg for $n {
            type Output = $n;

            fn neg(self) -> Self::Output {
                let t1 = self.ptr();
                let fun = Box::new(move || -t1.get().eval());
                $fout(self.m(), fun)
            }
        }
    };
}

num!(Num<'m, 'd>, val_num);
num!(Int<'m, 'd>, val_int);

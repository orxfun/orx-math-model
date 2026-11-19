use crate::scalars::num::ops::add::{val_int, val_num};
use crate::{Int, Num, SymWrap};
use alloc::boxed::Box;
use core::ops::Neg;

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

use crate::val_idx_pairs::D1ValIdxPair;
use crate::{
    Elements, Res, ResValid, Set, SetCatInt, SetCatStr, SetOrd, SetTyp, Val, ValInt, ValNum, ValTyp,
};
use alloc::boxed::Box;
use core::mem::ManuallyDrop;

pub type FP1<'d, V, I0> = Box<dyn Fn(<I0 as Set>::IdxRaw<'d>) -> <V as Val>::Raw + 'd>;

#[repr(C)]
pub union UFunP1<'d> {
    pub int_cint: ManuallyDrop<FP1<'d, ValInt, SetCatInt>>,
    pub int_cstr: ManuallyDrop<FP1<'d, ValInt, SetCatStr>>,
    pub int_ord: ManuallyDrop<FP1<'d, ValInt, SetOrd>>,

    pub num_cint: ManuallyDrop<FP1<'d, ValNum, SetCatInt>>,
    pub num_cstr: ManuallyDrop<FP1<'d, ValNum, SetCatStr>>,
    pub num_ord: ManuallyDrop<FP1<'d, ValNum, SetOrd>>,
}

pub struct FunP1<'d> {
    pub val_typ: ValTyp,
    pub i0_typ: SetTyp,
    data: UFunP1<'d>,
}

impl<'d> Drop for FunP1<'d> {
    fn drop(&mut self) {
        unsafe {
            match (self.val_typ, self.i0_typ) {
                (ValTyp::Int, SetTyp::CatInt) => ManuallyDrop::drop(&mut self.data.int_cint),
                (ValTyp::Int, SetTyp::CatStr) => ManuallyDrop::drop(&mut self.data.int_cstr),
                (ValTyp::Int, SetTyp::Ord) => ManuallyDrop::drop(&mut self.data.int_ord),

                (ValTyp::Num, SetTyp::CatInt) => ManuallyDrop::drop(&mut self.data.num_cint),
                (ValTyp::Num, SetTyp::CatStr) => ManuallyDrop::drop(&mut self.data.num_cstr),
                (ValTyp::Num, SetTyp::Ord) => ManuallyDrop::drop(&mut self.data.num_ord),
            }
        }
    }
}

impl<'d> FunP1<'d> {
    pub fn empty(val_typ: ValTyp, i0_typ: SetTyp) -> Self {
        let data = match (val_typ, i0_typ) {
            (ValTyp::Int, SetTyp::CatInt) => UFunP1 {
                int_cint: ManuallyDrop::new(Box::new(|_| Default::default())),
            },
            (ValTyp::Int, SetTyp::CatStr) => UFunP1 {
                int_cstr: ManuallyDrop::new(Box::new(|_| Default::default())),
            },
            (ValTyp::Int, SetTyp::Ord) => UFunP1 {
                int_ord: ManuallyDrop::new(Box::new(|_| Default::default())),
            },

            (ValTyp::Num, SetTyp::CatInt) => UFunP1 {
                num_cint: ManuallyDrop::new(Box::new(|_| Default::default())),
            },
            (ValTyp::Num, SetTyp::CatStr) => UFunP1 {
                num_cstr: ManuallyDrop::new(Box::new(|_| Default::default())),
            },
            (ValTyp::Num, SetTyp::Ord) => UFunP1 {
                num_ord: ManuallyDrop::new(Box::new(|_| Default::default())),
            },
        };
        Self {
            val_typ,
            i0_typ,
            data,
        }
    }

    pub fn new<V: Val, I0: Set>(fun: FP1<'d, V, I0>) -> Self
    where
        (V, I0): D1ValIdxPair<V = V, I0 = I0>,
    {
        let (val_typ, i0_typ) = (V::typ(), I0::typ());
        let data = <(V, I0) as D1ValIdxPair>::into_fun_union(fun);
        Self {
            val_typ,
            i0_typ,
            data,
        }
    }

    pub fn get<V: Val, I0: Set>(&self) -> &FP1<'d, V, I0>
    where
        (V, I0): D1ValIdxPair<V = V, I0 = I0>,
    {
        Res::dbg_assert_same_val(V::typ(), self.val_typ);
        Res::dbg_assert_same_set(I0::typ(), self.i0_typ);
        <(V, I0) as D1ValIdxPair>::fun_from_union(&self.data)
    }
}

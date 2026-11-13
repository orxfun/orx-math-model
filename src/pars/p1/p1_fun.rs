use crate::{
    val_idx_pairs::ValIdxPairD1, Elements, Res, ResValid, Set, SetCatInt, SetCatStr, SetOrd,
    SetTyp, Val, ValInt, ValNum, ValTyp,
};
use alloc::boxed::Box;
use core::mem::{transmute, ManuallyDrop};

pub type FP1<'d, V, I0> = Box<dyn Fn(<I0 as Set>::IdxRaw<'d>) -> <V as Val>::Raw + 'd>;

#[repr(C)]
pub union UFunP1<'d> {
    pub cint_int: ManuallyDrop<FP1<'d, ValInt, SetCatInt>>,
    pub cstr_int: ManuallyDrop<FP1<'d, ValInt, SetCatStr>>,
    pub ord_int: ManuallyDrop<FP1<'d, ValInt, SetOrd>>,

    pub cint_num: ManuallyDrop<FP1<'d, ValNum, SetCatInt>>,
    pub cstr_num: ManuallyDrop<FP1<'d, ValNum, SetCatStr>>,
    pub ord_num: ManuallyDrop<FP1<'d, ValNum, SetOrd>>,
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
                (ValTyp::Int, SetTyp::CatInt) => ManuallyDrop::drop(&mut self.data.cint_int),
                (ValTyp::Int, SetTyp::CatStr) => ManuallyDrop::drop(&mut self.data.cstr_int),
                (ValTyp::Int, SetTyp::Ord) => ManuallyDrop::drop(&mut self.data.ord_int),

                (ValTyp::Num, SetTyp::CatInt) => ManuallyDrop::drop(&mut self.data.cint_num),
                (ValTyp::Num, SetTyp::CatStr) => ManuallyDrop::drop(&mut self.data.cstr_num),
                (ValTyp::Num, SetTyp::Ord) => ManuallyDrop::drop(&mut self.data.ord_num),
            }
        }
    }
}

impl<'d> FunP1<'d> {
    pub fn empty(val_typ: ValTyp, i0_typ: SetTyp) -> Self {
        let data = match (val_typ, i0_typ) {
            (ValTyp::Int, SetTyp::CatInt) => UFunP1 {
                cint_int: ManuallyDrop::new(Box::new(|_| Default::default())),
            },
            (ValTyp::Int, SetTyp::CatStr) => UFunP1 {
                cstr_int: ManuallyDrop::new(Box::new(|_| Default::default())),
            },
            (ValTyp::Int, SetTyp::Ord) => UFunP1 {
                ord_int: ManuallyDrop::new(Box::new(|_| Default::default())),
            },

            (ValTyp::Num, SetTyp::CatInt) => UFunP1 {
                cint_num: ManuallyDrop::new(Box::new(|_| Default::default())),
            },
            (ValTyp::Num, SetTyp::CatStr) => UFunP1 {
                cstr_num: ManuallyDrop::new(Box::new(|_| Default::default())),
            },
            (ValTyp::Num, SetTyp::Ord) => UFunP1 {
                ord_num: ManuallyDrop::new(Box::new(|_| Default::default())),
            },
        };
        Self {
            val_typ,
            i0_typ,
            data,
        }
    }

    pub fn new<V: Val, I0: Set>(fun: FP1<'d, V, I0>) -> Self {
        let fun = ManuallyDrop::new(fun);
        let (val_typ, i0_typ) = (V::typ(), I0::typ());
        let data = match (val_typ, i0_typ) {
            (ValTyp::Int, SetTyp::CatInt) => UFunP1 {
                cint_int: unsafe { transmute(fun) },
            },
            (ValTyp::Int, SetTyp::CatStr) => UFunP1 {
                cstr_int: unsafe { transmute(fun) },
            },
            (ValTyp::Int, SetTyp::Ord) => UFunP1 {
                ord_int: unsafe { transmute(fun) },
            },

            (ValTyp::Num, SetTyp::CatInt) => UFunP1 {
                cint_num: unsafe { transmute(fun) },
            },
            (ValTyp::Num, SetTyp::CatStr) => UFunP1 {
                cstr_num: unsafe { transmute(fun) },
            },
            (ValTyp::Num, SetTyp::Ord) => UFunP1 {
                ord_num: unsafe { transmute(fun) },
            },
        };
        Self {
            val_typ,
            i0_typ,
            data,
        }
    }

    pub fn get<V: Val, I0: Set>(&self) -> &FP1<'d, V, I0>
    where
        (V, I0): ValIdxPairD1<V = V, I0 = I0>,
    {
        Res::dbg_assert_same_val(V::typ(), self.val_typ);
        Res::dbg_assert_same_set(I0::typ(), self.i0_typ);
        <(V, I0) as ValIdxPairD1>::fun_from_union(&self.data)
    }
}

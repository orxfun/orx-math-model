use crate::{
    Elements, Res, ResValid, Set, SetCatInt, SetCatStr, SetOrd, SetTyp, Val, ValInt, ValNum, ValTyp,
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

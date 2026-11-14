use crate::{Set, SetCatInt, SetCatStr, SetOrd, Val, ValInt, ValNum};
use alloc::boxed::Box;
use core::mem::ManuallyDrop;

pub type FP2<'d, V, I0, I1> =
    Box<dyn Fn(<I0 as Set>::IdxRaw<'d>, <I1 as Set>::IdxRaw<'d>) -> <V as Val>::Raw + 'd>;

#[repr(C)]
pub union UFunP2<'d> {
    pub int_cint_cint: ManuallyDrop<FP2<'d, ValInt, SetCatInt, SetCatInt>>,
    pub int_cint_cstr: ManuallyDrop<FP2<'d, ValInt, SetCatInt, SetCatStr>>,
    pub int_cint_ord: ManuallyDrop<FP2<'d, ValInt, SetCatInt, SetOrd>>,

    pub int_cstr_cint: ManuallyDrop<FP2<'d, ValInt, SetCatStr, SetCatInt>>,
    pub int_cstr_cstr: ManuallyDrop<FP2<'d, ValInt, SetCatStr, SetCatStr>>,
    pub int_cstr_ord: ManuallyDrop<FP2<'d, ValInt, SetCatStr, SetOrd>>,

    pub int_ord_cint: ManuallyDrop<FP2<'d, ValInt, SetOrd, SetCatInt>>,
    pub int_ord_cstr: ManuallyDrop<FP2<'d, ValInt, SetOrd, SetCatStr>>,
    pub int_ord_ord: ManuallyDrop<FP2<'d, ValInt, SetOrd, SetOrd>>,

    pub num_cint_cint: ManuallyDrop<FP2<'d, ValNum, SetCatInt, SetCatInt>>,
    pub num_cint_cstr: ManuallyDrop<FP2<'d, ValNum, SetCatInt, SetCatStr>>,
    pub num_cint_ord: ManuallyDrop<FP2<'d, ValNum, SetCatInt, SetOrd>>,

    pub num_cstr_cint: ManuallyDrop<FP2<'d, ValNum, SetCatStr, SetCatInt>>,
    pub num_cstr_cstr: ManuallyDrop<FP2<'d, ValNum, SetCatStr, SetCatStr>>,
    pub num_cstr_ord: ManuallyDrop<FP2<'d, ValNum, SetCatStr, SetOrd>>,

    pub num_ord_cint: ManuallyDrop<FP2<'d, ValNum, SetOrd, SetCatInt>>,
    pub num_ord_cstr: ManuallyDrop<FP2<'d, ValNum, SetOrd, SetCatStr>>,
    pub num_ord_ord: ManuallyDrop<FP2<'d, ValNum, SetOrd, SetOrd>>,
}

use crate::{Set, SetCatInt, SetCatStr, SetOrd};
use alloc::boxed::Box;
use core::mem::ManuallyDrop;

// core

pub type VP1<'d, I0> = Box<dyn Fn() -> <I0 as Set>::IdxRaw<'d> + 'd>;

// .

#[repr(C)]
pub union UFunV1<'d> {
    cint: ManuallyDrop<VP1<'d, SetCatInt>>,
    cstr: ManuallyDrop<VP1<'d, SetCatStr>>,
    ord: ManuallyDrop<VP1<'d, SetOrd>>,
}

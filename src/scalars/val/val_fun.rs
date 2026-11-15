use crate::scalars::val::{ScaInt, ScaNum, Val, ValTyp};
use crate::{Elements, Res, ResValid, Set, SetCatInt, SetCatStr, SetOrd, SetTyp};
use alloc::boxed::Box;
use core::mem::ManuallyDrop;

pub type FVal<'d, V> = Box<dyn Fn() -> <V as Val>::Raw + 'd>;

#[repr(C)]
pub union UFunVal<'d> {
    pub int: ManuallyDrop<FVal<'d, ScaInt>>,
    pub num: ManuallyDrop<FVal<'d, ScaNum>>,
}

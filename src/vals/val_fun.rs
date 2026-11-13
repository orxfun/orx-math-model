use crate::{Elements, Res, ResValid, Set, SetCatInt, SetCatStr, SetOrd, SetTyp, Val};
use alloc::boxed::Box;
use core::mem::{transmute, ManuallyDrop};

pub type FVal<'d, V> = Box<dyn Fn() -> <V as Val>::Raw + 'd>;

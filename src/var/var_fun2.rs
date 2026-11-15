use crate::{Set, SetCatInt, SetCatStr, SetOrd, SetTyp};
use alloc::boxed::Box;
use core::mem::{transmute, ManuallyDrop};

// core

type Idx<'d, I0, I1> = (<I0 as Set>::IdxRaw<'d>, <I1 as Set>::IdxRaw<'d>);

pub type FV2<'d, I0, I1> = Box<dyn Fn() -> Idx<'d, I0, I1> + 'd>;

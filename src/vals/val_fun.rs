use crate::{CatInt, CatStr, Elements, Ord, Res, ResValid, Set, SetTyp};
use alloc::boxed::Box;
use core::mem::{transmute, ManuallyDrop};

// pub type FVal<'d, V> = Box<dyn Fn() -> <V as Val> + 'd>;

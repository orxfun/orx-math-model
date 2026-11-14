use crate::{Set, Val};
use alloc::boxed::Box;

pub type FP2<'d, V, I0, I1> =
    Box<dyn Fn(<I0 as Set>::IdxRaw<'d>, <I1 as Set>::IdxRaw<'d>) -> <V as Val>::Raw + 'd>;

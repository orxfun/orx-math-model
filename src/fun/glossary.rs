use crate::{Elements, MetaSet};
use alloc::boxed::Box;

pub type FSet<'d, I0> = Box<dyn Fn(&mut Elements<<I0 as MetaSet>::IdxRaw<'d>>) + 'd>;

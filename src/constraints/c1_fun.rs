use crate::{constraints::DCon, Set};
use alloc::boxed::Box;

pub type FC1<'d, I0> = Box<dyn Fn(<I0 as Set>::IdxRaw<'d>) -> DCon<'d> + 'd>;

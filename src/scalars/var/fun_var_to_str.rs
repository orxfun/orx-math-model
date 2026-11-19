use crate::vars::DVars;
use crate::{ConVec, EIdx, Idx1, Idx2, Ptr, Set};
use alloc::boxed::Box;
use alloc::string::String;
use core::fmt::Write;

pub type FVarToStr<'d> = Box<dyn Fn(&mut String) -> Result<(), core::fmt::Error> + 'd>;

pub struct FVarToStrCtor;

impl FVarToStrCtor {
    pub fn d1_str<'d, I0: Set>(
        parent: Ptr<'d, DVars<'d>>,
        fun: impl Fn() -> Idx1<'d, I0> + 'd,
    ) -> FVarToStr<'d> {
        // TODO: use parent to add the variable key
        Box::new(move |f| {
            write!(f, "x_")?;
            I0::write_idx_str(f, fun())
        })
    }

    pub fn d2_str<'d, I0: Set, I1: Set>(
        parent: Ptr<'d, DVars<'d>>,
        fun: impl Fn() -> Idx2<'d, I0, I1> + 'd,
    ) -> FVarToStr<'d> {
        Box::new(move |f| {
            let (i0, i1) = fun();
            write!(f, "x_")?;
            I0::write_idx_str(f, i0)?;
            write!(f, "_")?;
            I1::write_idx_str(f, i1)
        })
    }
}

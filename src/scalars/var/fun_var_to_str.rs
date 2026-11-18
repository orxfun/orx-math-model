use crate::vars::DVars;
use crate::{ConVec, EIdx, Idx1, Idx2, Ptr, Set};
use alloc::boxed::Box;

type Formatter<'a> = core::fmt::Formatter<'a>;

pub type FVarStr<'d> =
    Box<dyn Fn(&mut core::fmt::Formatter<'_>) -> Result<(), core::fmt::Error> + 'd>;

pub struct FVarStrCtor;

impl FVarStrCtor {
    pub fn d1<'d, I0: Set>(
        parent: Ptr<'d, DVars<'d>>,
        fun: impl Fn() -> Idx1<'d, I0> + 'd,
    ) -> FVarStr<'d> {
        // TODO: use parent to add the variable key
        Box::new(move |f| I0::write_idx(f, fun()))
    }

    pub fn d2<'d, I0: Set, I1: Set>(
        parent: Ptr<'d, DVars<'d>>,
        fun: impl Fn() -> Idx2<'d, I0, I1> + 'd,
    ) -> FVarStr<'d> {
        Box::new(move |f| {
            let (i0, i1) = fun();
            I0::write_idx(f, i0)?;
            write!(f, "_")?;
            I1::write_idx(f, i1)
        })
    }
}

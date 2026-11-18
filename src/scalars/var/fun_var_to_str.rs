use crate::vars::DVars;
use crate::{ConVec, EIdx, Idx1, Idx2, Ptr, Set};
use alloc::boxed::Box;

type Formatter<'a> = core::fmt::Formatter<'a>;

pub type FVarStr<'d> = Box<dyn Fn(&mut Formatter<'_>) + 'd>;

pub struct FVarStrCtor;

impl FVarStrCtor {
    pub fn d1<'d, I0: Set>(
        parent: Ptr<'d, DVars<'d>>,
        fun: impl Fn() -> Idx1<'d, I0> + 'd,
    ) -> FVarStr<'d> {
        Box::new(move |f| {
            let i0 = fun();
            let arr = [I0::into_idx_enum(i0)];
            IdxEvalVec(ConVec::new(arr))
        })
    }

    pub fn d2<'d, I0: Set, I1: Set>(
        parent: Ptr<'d, DVars<'d>>,
        fun: impl Fn() -> Idx2<'d, I0, I1> + 'd,
    ) -> FVarStr<'d> {
        Box::new(move |f| {
            let (i0, i1) = fun();
            let arr = [I0::into_idx_enum(i0), I1::into_idx_enum(i1)];
            IdxEvalVec(ConVec::new(arr))
        })
    }
}

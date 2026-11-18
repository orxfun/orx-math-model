use crate::scalars::FVarIdx1;
use crate::{ConVec, EIdx, Set};
use alloc::{boxed::Box, string::String};

pub type Idx1<'d, I0> = <I0 as Set>::IdxRaw<'d>;
pub type Idx2<'d, I0, I1> = (<I0 as Set>::IdxRaw<'d>, <I1 as Set>::IdxRaw<'d>);

pub struct IdxEvalVec<'d>(ConVec<4, EIdx<'d>>);

pub type FVarIdx<'d> = Box<dyn Fn() -> IdxEvalVec<'d> + 'd>;

pub struct FVarIdxCtor;

impl FVarIdxCtor {
    pub fn d1<'d, I0: Set>(fun: impl Fn() -> Idx1<'d, I0> + 'd) -> FVarIdx<'d> {
        Box::new(move || {
            let i0 = fun();
            let arr = [I0::into_idx_enum(i0)];
            IdxEvalVec(ConVec::new(arr))
        })
    }

    pub fn d2<'d, I0: Set, I1: Set>(fun: impl Fn() -> Idx2<'d, I0, I1> + 'd) -> FVarIdx<'d> {
        Box::new(move || {
            let (i0, i1) = fun();
            let arr = [I0::into_idx_enum(i0), I1::into_idx_enum(i1)];
            IdxEvalVec(ConVec::new(arr))
        })
    }
}

use crate::scalars::FVarIdx1;
use crate::{ConVec, Set};
use alloc::{boxed::Box, string::String};

pub enum IdxEval<'d> {
    CatInt(usize),
    CatStr(&'d String),
    Ord(i64),
}

pub struct IdxEvalVec<'d>(ConVec<4, IdxEval<'d>>);

impl<'d> IdxEvalVec<'d> {
    pub fn new<const C: usize>(values: [IdxEval<'d>; C]) -> Self {
        Self(ConVec::new(values))
    }
}

pub type Idx1<'d, I0> = <I0 as Set>::IdxRaw<'d>;

pub type FVarIdx<'d> = Box<dyn Fn() -> IdxEvalVec<'d> + 'd>;

pub struct FVarIdxCtor;

impl FVarIdxCtor {
    pub fn d1<'d, I0: Set>(fun: impl Fn() -> Idx1<'d, I0> + 'd) -> FVarIdx<'d> {
        Box::new(move || IdxEvalVec(ConVec::new([IdxEval::CatInt(3)])))
    }
}

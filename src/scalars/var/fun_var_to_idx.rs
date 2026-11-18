use crate::ConVec;
use alloc::string::String;

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
